mod pacman_conf;

use alpm::{Alpm, Error::DbNotNull, Event, SigLevel};
use std::collections::HashSet;

use crate::{error::*, print::*};
use pacman_conf::*;

macro_rules! every {
    ($($e:expr),+ $(,)?) => {
        $(($e)) && +
    };
}

pub fn find_foreign_packages(
    ignores: HashSet<String>,
    ignore_groups: HashSet<String>,
) -> R<Vec<(String, String)>> {
    let (dbpath, repos) = get_configuration()?;
    let alpm = Alpm::new("/", dbpath.as_deref().unwrap_or("/var/lib/pacman/"))?;

    alpm.set_event_cb((), |e, _| {
        if let Event::DatabaseMissing(event) = e.event() {
            warning(format_args!(
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
