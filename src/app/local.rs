use alpm::{Alpm, Error::DbNotNull, Event, SigLevel};
use std::{collections::HashSet, fs, path};

use crate::{error::R, print::print_warning};

macro_rules! every {
    ($($e:expr),+ $(,)?) => {
        $(($e)) && +
    };
}

const DB_EXT: &str = ".db";

fn is_db(name: &str) -> bool {
    name.len() > DB_EXT.len() && name.ends_with(DB_EXT)
}

pub fn find_repos(dbpath: &str) -> R<HashSet<String>> {
    let path = String::from_iter([dbpath, path::MAIN_SEPARATOR_STR, "sync"]);
    let mut repos = HashSet::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;

        let Ok(mut name) = entry.file_name().into_string() else {
            continue;
        };

        if !is_db(&name) {
            continue;
        }

        if !entry.metadata()?.is_file() {
            continue;
        }

        name.truncate(name.len() - DB_EXT.len());
        repos.insert(name);
    }

    Ok(repos)
}

pub fn find_foreign_packages(
    dbpath: &str,
    repos: HashSet<String>,
    ignores: HashSet<String>,
    ignore_groups: HashSet<String>,
) -> R<Vec<(String, String)>> {
    let alpm = Alpm::new("/", dbpath)?;

    alpm.set_event_cb((), |e, _| {
        if let Event::DatabaseMissing(event) = e.event() {
            print_warning(format_args!(
                "database file for '{}' does not exist (use 'pacman -Sy' to download)",
                event.dbname()
            ))
        }
    });

    let repos = repos
        .into_iter()
        .filter_map(|repo| match alpm.register_syncdb(repo, SigLevel::NONE) {
            Err(DbNotNull) => None,
            r => Some(r),
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(alpm
        .localdb()
        .pkgs()
        .into_iter()
        .filter_map(|pkg| {
            let name = pkg.name();
            every!(
                ignores.is_empty() || !ignores.contains(name),
                ignore_groups.is_empty() || !pkg.groups().iter().any(|g| ignore_groups.contains(g)),
                repos.iter().all(|db| db.pkg(name).is_err()),
            )
            .then(|| (name.to_string(), pkg.version().to_string()))
        })
        .collect())
}
