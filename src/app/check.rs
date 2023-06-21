mod aur;

use alpm::vercmp;
use std::{cmp::Ordering, collections::HashMap};

use crate::{
    error::R,
    print::{print_message, print_package, print_update},
};
use aur::request_updates;

#[cfg_attr(test, derive(Debug, PartialEq))]
enum Status {
    UpToDate,
    HasUpdate(String, String, String),
    NotInAUR(String),
}

use Status::*;

fn print_status(status: Status) {
    match status {
        HasUpdate(name, ver, new_ver) => print_update(name, ver, new_ver),
        NotInAUR(name) => print_package(name, "is not in AUR"),
        _ => {}
    }
}

fn gen_state(pkgs: Vec<(String, String)>, mut updates: HashMap<String, String>) -> Vec<Status> {
    use Ordering::*;
    pkgs.into_iter()
        .map(|(name, ver)| match updates.remove(&name) {
            Some(new_ver) => match vercmp(new_ver.as_str(), ver.as_str()) {
                Greater => HasUpdate(name, ver, new_ver),
                _ => UpToDate,
            },
            _ => NotInAUR(name),
        })
        .collect()
}

fn count_updates(state: &[Status]) -> usize {
    state
        .iter()
        .filter(|status| matches!(status, HasUpdate(_, _, _)))
        .count()
}

pub fn check_updates(pkgs: Vec<(String, String)>, timeout: u64) -> R<()> {
    if pkgs.is_empty() {
        print_message("no packages to check");
        return Ok(());
    }

    let updates = request_updates(pkgs.iter(), timeout)?;
    let state = gen_state(pkgs, updates);

    if count_updates(&state) == 0 {
        print_message("no updates");
    }

    state.into_iter().for_each(print_status);
    Ok(())
}

#[cfg(test)]
mod tests;
