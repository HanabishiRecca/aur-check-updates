use crate::types::{Arr, Str};
use std::{
    fs::{self, DirEntry},
    io::Result,
    path::PathBuf,
};

const DB_DIR: &str = "sync";
const DB_EXT: &str = ".db";

macro_rules! R {
    ($e: expr) => {
        match $e {
            Ok(e) => e,
            Err(e) => return Some(Err(e)),
        }
    };
}

macro_rules! C {
    ($e: expr) => {
        if !$e {
            return None;
        }
    };
}

fn map(entry: Result<DirEntry>) -> Option<Result<Str>> {
    let entry = R!(entry);
    C!(R!(entry.metadata()).is_file());

    let mut name = entry.file_name().into_string().ok()?;
    C!(name.ends_with(DB_EXT));

    let len = name.len().checked_sub(DB_EXT.len())?;
    C!(len > 0);
    name.truncate(len);

    Some(Ok(Str::from(name)))
}

pub fn find_repos(dbpath: &str) -> Result<Arr<Str>> {
    fs::read_dir(PathBuf::from_iter([dbpath, DB_DIR]))?
        .filter_map(map)
        .collect()
}
