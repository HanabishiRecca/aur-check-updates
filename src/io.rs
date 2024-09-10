use crate::types::{Arr, Str};
use std::{
    fs::{self, DirEntry},
    io::Result,
    path::PathBuf,
};

const DB_EXT: &str = ".db";

fn is_db(name: &str) -> bool {
    name.len() > DB_EXT.len() && name.ends_with(DB_EXT)
}

macro_rules! res {
    ($e: expr) => {
        match $e {
            Ok(e) => e,
            Err(e) => return Some(Err(e)),
        }
    };
}

fn map(entry: Result<DirEntry>) -> Option<Result<Str>> {
    let entry = res!(entry);
    let mut name = entry.file_name().into_string().ok()?;
    (is_db(&name) && res!(entry.metadata()).is_file()).then_some(())?;
    name.truncate(name.len() - DB_EXT.len());
    Some(Ok(Str::from(name)))
}

pub fn find_repos(dbpath: &str) -> Result<Arr<Str>> {
    fs::read_dir(PathBuf::from_iter([dbpath, "sync"]))?
        .filter_map(map)
        .collect::<Result<Arr<_>>>()
}
