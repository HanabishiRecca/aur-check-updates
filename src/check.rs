use alpm::vercmp;
use std::cmp::Ordering::*;

use crate::aur::request_updates;
use crate::error::R;
use crate::print;

enum Status {
    UpToDate,
    HasUpdate(String, String, String),
    NotInAUR(String),
}

use Status::*;

pub fn check_updates(pkgs: Vec<(String, String)>) -> R<()> {
    if pkgs.is_empty() {
        print::message(format_args!("no packages to check"));
        return Ok(());
    }

    let mut updates = request_updates(pkgs.iter().map(|(name, _)| name.as_str()))?;

    let stat: Vec<_> = pkgs
        .into_iter()
        .map(|(name, ver)| match updates.remove(name.as_str()) {
            Some(new_ver) => match vercmp(new_ver.as_str(), ver.as_str()) {
                Greater => HasUpdate(name, ver, new_ver),
                _ => UpToDate,
            },
            _ => NotInAUR(name),
        })
        .collect();

    let count = stat
        .iter()
        .filter(|s| matches!(s, HasUpdate(_, _, _)))
        .count();

    if count == 0 {
        print::message(format_args!("no updates"));
    }

    for s in stat {
        match s {
            HasUpdate(name, ver, new_ver) => print::update(
                format_args!("{name}"),
                format_args!("{ver}"),
                format_args!("{new_ver}"),
            ),
            NotInAUR(name) => print::package(format_args!("{name}"), format_args!("is not in AUR")),
            _ => {}
        }
    }

    Ok(())
}
