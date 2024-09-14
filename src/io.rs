use crate::{
    types::{Arr, Str},
    utils::filter,
};
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

fn map(entry: Result<DirEntry>) -> Option<Result<Str>> {
    let entry = R!(entry);
    filter!(R!(entry.metadata()).is_file());

    let mut name = entry.file_name().into_string().ok()?;
    filter!(name.ends_with(DB_EXT));

    let len = name.len().checked_sub(DB_EXT.len())?;
    filter!(len > 0);
    name.truncate(len);

    Some(Ok(Str::from(name)))
}

pub fn find_repos(dbpath: &str) -> Result<Arr<Str>> {
    fs::read_dir(PathBuf::from_iter([dbpath, DB_DIR]))?
        .filter_map(map)
        .collect()
}
