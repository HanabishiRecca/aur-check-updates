use crate::types::{Arr, Str};
use std::collections::HashSet;

pub fn copy(input: &[impl AsRef<str>]) -> Arr<Str> {
    input.iter().map(AsRef::as_ref).map(Str::from).collect()
}

pub fn to_hashset(source: &[impl AsRef<str>]) -> HashSet<&str> {
    HashSet::from_iter(source.iter().map(AsRef::as_ref))
}

macro_rules! filter {
    ($e: expr) => {
        if !$e {
            return None;
        }
    };
}

pub(crate) use filter;
