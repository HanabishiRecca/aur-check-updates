use crate::package::Pkg;
use crate::types::{Arr, Str};
use serde::Deserialize;
use serde_json::Result;
use std::collections::HashMap;

#[derive(Deserialize)]
struct Info {
    #[serde(rename = "Name")]
    name: Str,
    #[serde(rename = "Version")]
    ver: Str,
}

impl Info {
    fn into_kv(self) -> (Str, Str) {
        (self.name, self.ver)
    }
}

#[derive(Deserialize)]
struct Response {
    results: Arr<Info>,
}

pub fn args(pkgs: &[Pkg]) -> Str {
    pkgs.iter().flat_map(|pkg| ["&arg[]=", pkg.name()]).collect()
}

pub fn parse(data: &str) -> Result<HashMap<Str, Str>> {
    let results = serde_json::from_str::<Response>(data)?.results;
    Ok(results.into_iter().map(Info::into_kv).collect())
}
