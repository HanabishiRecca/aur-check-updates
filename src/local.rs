use alpm::{Alpm, Event, SigLevel};
use configparser::ini::Ini;
use std::collections::{HashMap, HashSet};

use crate::{error::R, print};

fn get_config_option<'a>(
    config: &'a HashMap<String, HashMap<String, Option<String>>>,
    name: &str,
) -> Option<&'a str> {
    config
        .get("options")?
        .get(name)?
        .as_ref()
        .map(|s| s.as_str())
}

macro_rules! every {
    ($($e:expr),+ $(,)?) => {
        $(($e)) && +
    };
}

pub fn find_foreign_packages(
    ignores: HashSet<String>,
    ignore_groups: HashSet<String>,
) -> R<Vec<(String, String)>> {
    let config = Ini::new_cs().load("/etc/pacman.conf")?;

    let alpm = Alpm::new(
        "/",
        get_config_option(&config, "DBPath").unwrap_or("/var/lib/pacman"),
    )?;

    alpm.set_event_cb((), |e, _| {
        if let Event::DatabaseMissing(event) = e.event() {
            print::warning(format_args!(
                "database file for '{}' does not exist (use 'pacman -Sy' to download)",
                event.dbname()
            ))
        }
    });

    let repos = config
        .into_iter()
        .filter(|(s, _)| s != "options")
        .map(|(s, _)| alpm.register_syncdb(s, SigLevel::NONE))
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
