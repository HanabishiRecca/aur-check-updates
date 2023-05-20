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
