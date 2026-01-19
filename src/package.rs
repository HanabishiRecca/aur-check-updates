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
}

impl Pkg {
    pub fn new(name: Str, ver: Str) -> Self {
        Self { name, ver }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ver(&self) -> &str {
        &self.ver
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Record {
    pkg: Pkg,
    status: Status,
}

impl Record {
    fn new(pkg: Pkg, status: Status) -> Self {
        Self { pkg, status }
    }

    pub fn print(&self, nlen: usize, vlen: usize) {
        use Status::*;
        match &self.status {
            UpToDate => print::package(&self.pkg, nlen),
            HasUpdate(new_ver) => print::update(&self.pkg, new_ver, nlen, vlen),
            NotInAUR => print::not_found(&self.pkg, nlen, vlen),
        }
    }

    fn has_update(&self) -> bool {
        matches!(self.status, Status::HasUpdate(_))
    }

    fn max_len(&self, (nlen, vlen): (usize, usize)) -> (usize, usize) {
        (nlen.max(self.pkg.name().len()), vlen.max(self.pkg.ver().len()))
    }
}

pub struct State {
    updates: Arr<Record>,
}

impl State {
    fn new(updates: Arr<Record>) -> Self {
        Self { updates }
    }

    pub fn has_updates(&self) -> bool {
        self.updates.iter().any(Record::has_update)
    }

    pub fn lengths(&self) -> (usize, usize) {
        self.updates.iter().fold((0, 0), |acc, pkg| pkg.max_len(acc))
    }

    pub fn into_records(self) -> Arr<Record> {
        self.updates
    }
}

struct Task {
    updates: HashMap<Str, Str>,
    keep_updated: bool,
    keep_failed: bool,
}

impl Task {
    fn new(updates: HashMap<Str, Str>, keep_updated: bool, keep_failed: bool) -> Self {
        Self { updates, keep_updated, keep_failed }
    }

    fn map_update(&mut self, pkg: Pkg) -> Option<Record> {
        use Status::*;
        match self.updates.remove(pkg.name()) {
            Some(new_ver) => match alpm::vercmp(new_ver.as_ref(), pkg.ver()) {
                Ordering::Greater => Some(Record::new(pkg, HasUpdate(new_ver))),
                _ => self.keep_updated.then_some(Record::new(pkg, UpToDate)),
            },
            _ => self.keep_failed.then_some(Record::new(pkg, NotInAUR)),
        }
    }

    fn into_state(mut self, pkgs: Arr<Pkg>) -> State {
        State::new(pkgs.into_iter().filter_map(|pkg| self.map_update(pkg)).collect())
    }
}

pub fn into_state(
    pkgs: Arr<Pkg>, updates: HashMap<Str, Str>, keep_updated: bool, keep_failed: bool,
) -> State {
    Task::new(updates, keep_updated, keep_failed).into_state(pkgs)
}
