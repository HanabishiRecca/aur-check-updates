use alpm::{Alpm, Event, SigLevel};
use configparser::ini::Ini;
use std::collections::HashMap;

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

pub fn find_foreign_packages(
    ignores: Vec<String>,
    ignore_groups: Vec<String>,
) -> R<Vec<(String, String)>> {
    let config = Ini::new_cs().load("/etc/pacman.conf")?;

    let alpm = Alpm::new(
        "/",
        get_config_option(&config, "DBPath").unwrap_or("/var/lib/pacman"),
    )?;

    alpm.set_event_cb(0, |e, _| {
        if let Event::DatabaseMissing(event) = e.event() {
            print::warning(format_args!(
                "database file for '{}' does not exist (use 'pacman -Sy' to download)",
                event.dbname()
            ))
        }
    });

    let mut repos = Vec::new();

    for (s, _) in config {
        if s.as_str() == "options" {
            continue;
        }
        repos.push(alpm.register_syncdb(s.as_str(), SigLevel::NONE)?);
    }

    let mut pkgs = Vec::new();

    for pkg in alpm.localdb().pkgs() {
        let name = pkg.name();
        if !ignores.is_empty() && ignores.iter().any(|s| name == s) {
            continue;
        }
        if !ignore_groups.is_empty() {
            let groups = pkg.groups();
            if ignore_groups.iter().any(|s| groups.iter().any(|g| g == s)) {
                continue;
            }
        }
        if repos.iter().any(|db| db.pkg(name).is_ok()) {
            continue;
        }
        pkgs.push((name.to_string(), pkg.version().to_string()));
    }

    Ok(pkgs)
}
