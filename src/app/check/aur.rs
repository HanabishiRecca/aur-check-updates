use curl::easy::Easy;
use serde::Deserialize;
use serde_json::from_str;
use std::{collections::HashMap, time::Duration};

use crate::error::R;

#[derive(Deserialize)]
struct Response {
    results: Vec<Pkg>,
}

#[derive(Deserialize)]
struct Pkg {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Version")]
    ver: String,
}

fn request(url: &str, timeout: u64) -> R<String> {
    let mut easy = Easy::new();
    easy.url(url)?;
    easy.timeout(Duration::from_millis(timeout))?;

    let mut result = Vec::new();
    let mut transfer = easy.transfer();

    transfer.write_function(|data| {
        result.extend_from_slice(data);
        Ok(data.len())
    })?;

    transfer.perform()?;
    drop(transfer);

    Ok(String::from_utf8(result)?)
}

pub fn request_updates<'a>(
    pkgs: impl Iterator<Item = &'a (String, String)>,
    endpoint: &str,
    timeout: u64,
) -> R<HashMap<String, String>> {
    let url: String = [endpoint, "?"]
        .into_iter()
        .chain(pkgs.flat_map(|(name, _)| ["&arg[]=", name].into_iter()))
        .collect();

    Ok(from_str::<Response>(&request(&url, timeout)?)?
        .results
        .into_iter()
        .map(|Pkg { name, ver }| (name, ver))
        .collect())
}
