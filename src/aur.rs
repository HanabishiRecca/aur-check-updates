use crate::types::{Arr, Str};
use serde::Deserialize;
use serde_json::Error;
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

impl Pkg {
    fn into_kv(self) -> (Str, Str) {
        (self.name, self.ver)
    }
}

pub fn args(pkgs: &[(impl AsRef<str>, impl AsRef<str>)]) -> Str {
    pkgs.iter().flat_map(|(name, _)| ["&arg[]=", name.as_ref()].into_iter()).collect()
}

pub fn parse(data: &str) -> Result<HashMap<Str, Str>, Error> {
    let results = serde_json::from_str::<Response>(data)?.results;
    Ok(results.into_iter().map(Pkg::into_kv).collect())
}
