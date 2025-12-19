use crate::types::{Arr, Str};
use std::iter;

pub fn to_arr(input: &[impl AsRef<str>]) -> Arr<Str> {
    input.iter().map(AsRef::as_ref).map(Str::from).collect()
}

pub fn contains(list: &[impl AsRef<str>], value: &str) -> bool {
    list.iter().any(move |v| v.as_ref() == value)
}

pub fn str_diff(a: &str, b: &str) -> usize {
    iter::zip(a.bytes(), b.bytes()).position(|(a, b)| a != b).unwrap_or(0)
}
