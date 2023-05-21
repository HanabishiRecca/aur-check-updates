use std::io::Cursor;

use super::*;

#[test]
fn config() -> R<()> {
    let reader = BufReader::new(Cursor::new(include_bytes!("test_config.in")));
    let (dbpath, repos) = read_config(reader)?;
    assert_eq!(dbpath.as_deref(), Some("/var/lib/pacman/"));
    assert_eq!(repos, ["core", "extra", "community"]);
    Ok(())
}

#[test]
fn config_no_dbpath() -> R<()> {
    let reader = BufReader::new(Cursor::new(include_bytes!("test_config_no_dbpath.in")));
    let (dbpath, repos) = read_config(reader)?;
    assert!(dbpath.is_none());
    assert_eq!(repos, ["core"]);
    Ok(())
}

#[test]
fn config_empty() -> R<()> {
    let reader = BufReader::new(Cursor::new([]));
    let (dbpath, repos) = read_config(reader)?;
    assert!(dbpath.is_none());
    assert!(repos.is_empty());
    Ok(())
}
