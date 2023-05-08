use alpm::{Alpm, Event, SigLevel};
use configparser::ini::Ini;
use std::collections::HashMap;

use crate::error::R;

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

pub fn find_foreign_packages() -> R<Vec<(String, String)>> {
    let config = Ini::new_cs().load("/etc/pacman.conf")?;

    let alpm = Alpm::new(
        "/",
        get_config_option(&config, "DBPath").unwrap_or("/var/lib/pacman"),
    )?;

    alpm.set_event_cb(0, |e, _| if let Event::DatabaseMissing(event) = e.event() {
        println!("\x1b[1;33mwarning:\x1b[0m database file for '{}' does not exist (use 'pacman -Sy' to download)", event.dbname());
    });

    let mut repos = Vec::new();

    for (s, _) in config {
        if s.as_str() == "options" {
            continue;
        }
        repos.push(alpm.register_syncdb(s.as_str(), SigLevel::NONE)?);
    }

    Ok(alpm
        .localdb()
        .pkgs()
        .into_iter()
        .filter_map(|pkg| {
            let name = pkg.name();
            if name.ends_with('-') {
                return None;
            }
            if repos.iter().any(|db| db.pkg(name).is_ok()) {
                return None;
            }
            Some((name.to_string(), pkg.version().to_string()))
        })
        .collect())
}
