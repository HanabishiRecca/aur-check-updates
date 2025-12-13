#[cfg(test)]
mod tests;

use crate::print;
use crate::types::{Arr, Str};
use std::cmp::Ordering;
use std::collections::HashMap;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum Status {
    UpToDate,
    HasUpdate(Str),
    NotInAUR,
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Pkg {
    name: Str,
    ver: Str,
    status: Status,
}

impl Pkg {
    pub fn new(name: Str, ver: Str, status: Status) -> Self {
        Pkg { name, ver, status }
    }

    pub fn print(&self, nlen: usize, vlen: usize) {
        use Status::*;
        match &self.status {
            UpToDate => print::package(&self.name, &self.ver, nlen),
            HasUpdate(new_ver) => print::update(&self.name, &self.ver, new_ver, nlen, vlen),
            NotInAUR => print::not_found(&self.name, &self.ver, nlen, vlen),
        }
    }

    fn has_update(&self) -> bool {
        matches!(self.status, Status::HasUpdate(_))
    }

    fn lengths(&self) -> (usize, usize) {
        (self.name.len(), self.ver.len())
    }
}

fn map_update(
    name: Str, ver: Str, updates: &mut HashMap<Str, Str>, keep_updated: bool, keep_failed: bool,
) -> Option<Pkg> {
    use Status::*;
    match updates.remove(&name) {
        Some(new_ver) => match alpm::vercmp(new_ver.as_ref(), ver.as_ref()) {
            Ordering::Greater => Some(Pkg::new(name, ver, HasUpdate(new_ver))),
            _ => keep_updated.then_some(Pkg::new(name, ver, UpToDate)),
        },
        _ => keep_failed.then_some(Pkg::new(name, ver, NotInAUR)),
    }
}

pub fn into_state(
    packages: impl IntoIterator<Item = (Str, Str)>, mut updates: HashMap<Str, Str>,
    keep_updated: bool, keep_failed: bool,
) -> Arr<Pkg> {
    let map = move |(name, ver)| map_update(name, ver, &mut updates, keep_updated, keep_failed);
    packages.into_iter().filter_map(map).collect()
}

pub fn count_updates(state: &[Pkg]) -> usize {
    state.iter().filter(|pkg| pkg.has_update()).count()
}

pub fn calc_lengths(state: &[Pkg]) -> (usize, usize) {
    state.iter().fold((0, 0), |prev, pkg| prev.max(pkg.lengths()))
}
