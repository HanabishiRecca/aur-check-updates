use crate::types::{Arr, Str};
use std::{collections::HashSet, iter};

pub fn copy(input: &[impl AsRef<str>]) -> Arr<Str> {
    input.iter().map(AsRef::as_ref).map(Str::from).collect()
}

pub fn to_hashset(source: &[impl AsRef<str>]) -> HashSet<&str> {
    HashSet::from_iter(source.iter().map(AsRef::as_ref))
}

pub fn str_diff(a: &str, b: &str) -> usize {
    iter::zip(a.bytes(), b.bytes())
        .position(|(a, b)| a != b)
        .unwrap_or_default()
}
