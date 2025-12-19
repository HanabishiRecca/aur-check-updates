use crate::types::{Arr, Str};
use std::fs::{self, DirEntry};
use std::io::Result;
use std::path::PathBuf;

const DB_DIR: &str = "sync";
const DB_EXT: &str = ".db";

macro_rules! err {
    ($e: expr) => {
        match $e {
            Ok(e) => e,
            Err(e) => return Some(Err(e)),
        }
    };
}

macro_rules! test {
    ($e: expr) => {
        ($e).then_some(())?
    };
}

fn map_entry(entry: Result<DirEntry>) -> Option<Result<Str>> {
    let entry = err!(entry);
    test!(err!(entry.metadata()).is_file());

    let mut name = entry.file_name().into_string().ok()?;
    test!(name.ends_with(DB_EXT));

    let len = name.len().checked_sub(DB_EXT.len())?;
    test!(len > 0);
    name.truncate(len);

    Some(Ok(Str::from(name)))
}

pub fn find_repos(dbpath: &str) -> Result<Arr<Str>> {
    let path = PathBuf::from_iter([dbpath, DB_DIR]);
    fs::read_dir(path)?.filter_map(map_entry).collect()
}
