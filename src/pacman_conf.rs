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

fn read_config(mut reader: impl BufRead) -> R<(Option<String>, Vec<String>)> {
    let mut line = String::new();
    let mut in_options = false;
    let mut dbpath = None;
    let mut repos = Vec::new();

    loop {
        line.clear();

        if reader.read_line(&mut line)? == 0 {
            break;
        }

        if skip_line(&line) {
            continue;
        }

        match parse_section(&line) {
            Some("options") => {
                in_options = true;
                continue;
            }
            Some(s) => {
                in_options = false;
                repos.push(s.to_string());
                continue;
            }
            _ => {}
        }

        if !in_options || dbpath.is_some() {
            continue;
        }

        if let Some(("DBPath", value)) = parse_option(&line) {
            dbpath = Some(value.to_string());
        }
    }

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
#[options]
DBPath = /not/in/section
                            
[core]
DBPath = /core/fake/path
Include = /etc/pacman.d/mirrorlist

    [options]    
#foo
RootDir = /
#[foo]
#DBPath = /comment/fake/path/
DBPaths = /wrong/option/name/
    DBPath    =    /var/lib/pacman/    
DBPath = /first/fake/path/

    [extra]    
Include = /etc/pacman.d/mirrorlist

    [community]    
Include = /etc/pacman.d/mirrorlist

[options]
DBPath = /second/fake/path
",
        ));

        let (dbpath, repos) = read_config(reader)?;
        assert_eq!(dbpath.as_deref(), Some("/var/lib/pacman/"));
        assert_eq!(repos, ["core", "extra", "community"]);
        Ok(())
    }
}
