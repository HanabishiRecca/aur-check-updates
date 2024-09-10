use crate::types::{Arr, Str};
use serde::Deserialize;
use serde_json::{from_str, Error};
use std::collections::HashMap;

#[derive(Deserialize)]
struct Response {
    results: Arr<Pkg>,
}

#[derive(Deserialize)]
struct Pkg {
    #[serde(rename = "Name")]
    name: Str,
    #[serde(rename = "Version")]
    ver: Str,
}

pub fn url(endpoint: &str, pkgs: &[(impl AsRef<str>, impl AsRef<str>)]) -> Str {
    let args = pkgs
        .iter()
        .flat_map(|(name, _)| ["&arg[]=", name.as_ref()].into_iter());

    [endpoint, "?"].into_iter().chain(args).collect()
}

pub fn parse(data: &str) -> Result<HashMap<Str, Str>, Error> {
    Ok(from_str::<Response>(data)?
        .results
        .into_vec() // rust-lang/rust#59878
        .into_iter()
        .map(|pkg| (pkg.name, pkg.ver))
        .collect())
}
