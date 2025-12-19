use crate::print::ColorMode;
use crate::{alpm, aur, cli, io, package, print, request, utils};
use std::env;
use std::error::Error;

const DEFAULT_COLOR_MODE: ColorMode = ColorMode::Auto;
const DEFAULT_DBPATH: &str = "/var/lib/pacman";
const DEFAULT_IGNORES: &[&str] = &[];
const DEFAULT_IGNORE_GROUPS: &[&str] = &[];
const DEFAULT_IGNORE_SUFFIXES: &[&str] = &["-debug"];
const DEFAULT_ENDPOINT: &str = "https://aur.archlinux.org/rpc/v5/info";
const DEFAULT_TIMEOUT: u64 = 5000;
const DEFAULT_SHOW_UPDATED: bool = false;
const DEFAULT_SHOW_FAILED: bool = true;
const DEFAULT_RAW: bool = false;

macro_rules! default {
    ($option: expr, $default: expr) => {
        match $option {
            Some(value) => value,
            _ => $default,
        }
    };
}

pub fn run() -> Result<bool, Box<dyn Error>> {
    print::set_color_mode(DEFAULT_COLOR_MODE);

    let Some(config) = cli::read_args(env::args().skip(1))? else {
        return Ok(true);
    };

    let raw = default!(config.raw(), DEFAULT_RAW);
    let color_mode = default!(config.color_mode(), DEFAULT_COLOR_MODE);
    print::set_color_mode(if raw { ColorMode::Never } else { color_mode });

    if !raw {
        print::header("Checking AUR updates...");
    }

    let dbpath = default!(config.dbpath(), DEFAULT_DBPATH);
    let repos = default!(config.repos(), &io::find_repos(dbpath)?);
    let names = default!(config.ignores(), &utils::to_arr(DEFAULT_IGNORES));
    let groups = default!(config.ignore_groups(), &utils::to_arr(DEFAULT_IGNORE_GROUPS));
    let suffixes = default!(config.ignore_suffixes(), &utils::to_arr(DEFAULT_IGNORE_SUFFIXES));
    let packages = alpm::find(dbpath, repos, names, groups, suffixes)?;

    if packages.is_empty() {
        if !raw {
            print::message("no packages to check");
        }
        return Ok(false);
    }

    let endpoint = default!(config.endpoint(), DEFAULT_ENDPOINT);
    let timeout = default!(config.timeout(), DEFAULT_TIMEOUT);
    let response = request::send(endpoint, aur::args(&packages).as_bytes(), timeout)?;

    let updates = aur::parse(core::str::from_utf8(&response)?)?;
    let show_updated = default!(config.show_updated(), DEFAULT_SHOW_UPDATED);
    let show_failed = default!(config.show_failed(), DEFAULT_SHOW_FAILED);
    let state = package::into_state(packages, updates, show_updated, show_failed);

    if !raw && !state.has_updates() {
        print::message("no updates");
    }

    let (nlen, vlen) = if raw { (0, 0) } else { state.lengths() };

    for record in state.into_records() {
        record.print(nlen, vlen);
    }

    Ok(false)
}
