mod aur;

use alpm::vercmp;
use std::{cmp::Ordering, collections::HashMap};

use crate::{
    error::R,
    print::{print_message, print_update},
};
use aur::request_updates;

#[cfg_attr(test, derive(Debug, PartialEq))]
enum Status {
    UpToDate,
    HasUpdate(String, String, String),
    NotInAUR(String, String),
}

fn print_status(status: Status, nlen: usize, vlen: usize) {
    use Status::*;
    match status {
        HasUpdate(name, ver, new_ver) => print_update(name, ver, new_ver, nlen, vlen, true),
        NotInAUR(name, ver) => print_update(name, ver, "[not found in AUR]", nlen, vlen, false),
        _ => {}
    }
}

fn gen_state(pkgs: Vec<(String, String)>, mut updates: HashMap<String, String>) -> Vec<Status> {
    use Ordering::*;
    use Status::*;
    pkgs.into_iter()
        .map(|(name, ver)| match updates.remove(&name) {
            Some(new_ver) => match vercmp(new_ver.as_str(), ver.as_str()) {
                Greater => HasUpdate(name, ver, new_ver),
                _ => UpToDate,
            },
            _ => NotInAUR(name, ver),
        })
        .collect()
}

fn count_updates(state: &[Status]) -> usize {
    use Status::*;
    state
        .iter()
        .filter(|status| matches!(status, HasUpdate(_, _, _)))
        .count()
}

fn calc_lengths(state: &[Status]) -> (usize, usize) {
    use Status::*;
    state.iter().fold((0, 0), |prev, status| match status {
        HasUpdate(name, ver, _) | NotInAUR(name, ver) => {
            (name.len().max(prev.0), ver.len().max(prev.1))
        }
        _ => prev,
    })
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

    let (nlen, vlen) = calc_lengths(&state);

    for status in state {
        print_status(status, nlen, vlen);
    }

    Ok(())
}

#[cfg(test)]
mod tests;
