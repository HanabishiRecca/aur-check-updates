use std::{
    fs::File,
    io::{BufRead, BufReader, Error as IOError},
};

use crate::error::R;

fn get_file() -> Result<File, IOError> {
    File::open("/etc/pacman.conf")
}

fn skip_line(s: &str) -> bool {
    s.is_empty() || s.starts_with('#')
}

fn parse_section(s: &str) -> Option<&str> {
    s.trim().strip_prefix('[')?.strip_suffix(']')
}

fn parse_option(s: &str) -> Option<(&str, &str)> {
    s.split_once('=')
        .map(|(name, value)| (name.trim(), value.trim()))
}

fn read_config(reader: impl BufRead) -> R<(Option<String>, Vec<String>)> {
    let mut dbpath = None;
    let mut search = false;

    let repos = reader
        .lines()
        .filter_map(|line| {
            let s = match line {
                Ok(s) => s,
                e => return Some(e),
            };

            if skip_line(&s) {
                return None;
            }

            match parse_section(&s) {
                Some("options") => {
                    search = true;
                    return None;
                }
                Some(s) => {
                    search = false;
                    return Some(Ok(s.to_string()));
                }
                _ => {}
            }

            if !search {
                return None;
            }

            if let Some(("DBPath", value)) = parse_option(&s) {
                dbpath = Some(value.to_string());
            }

            None
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok((dbpath, repos))
}

pub fn get_configuration() -> R<(Option<String>, Vec<String>)> {
    read_config(BufReader::new(get_file()?))
}

#[cfg(test)]
mod tests {
    use std::{
        assert_eq,
        io::{BufReader, Cursor},
    };

    use super::read_config;
    use crate::error::R;

    #[test]
    fn config() -> R<()> {
        let reader = BufReader::new(Cursor::new(
            b"
# /etc/pacman.conf
# GENERAL OPTIONS
    [options]    
#foo
RootDir = /
#[foo]
DBPath = /first/fake/path/
    DBPath    =    /var/lib/pacman/    
DBPaths = /wrong/option/name/
#DBPath = /comment/fake/path/

    [core]    
DBPath = /core/fake/path
Include = /etc/pacman.d/mirrorlist

    [extra]    
Include = /etc/pacman.d/mirrorlist

    [community]    
Include = /etc/pacman.d/mirrorlist

#[options]
DBPath = /options/fake/path
",
        ));

        let (dbpath, repos) = read_config(reader)?;
        assert_eq!(dbpath.as_deref(), Some("/var/lib/pacman/"));
        assert_eq!(repos, ["core", "extra", "community"]);
        Ok(())
    }
}
