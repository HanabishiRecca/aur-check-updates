use crate::{
    print,
    types::{Arr, Str},
    utils,
};
use alpm::{Alpm, AnyEvent, Db, Event, Package, Result, SigLevel};
use std::collections::HashSet;

fn event(e: AnyEvent, _: &mut ()) {
    if let Event::DatabaseMissing(event) = e.event() {
        print::warning(format_args!("database file for '{}' does not exist", event.dbname()));
    }
}

fn init(dbpath: &str) -> Result<Alpm> {
    let alpm = Alpm::new("/", dbpath)?;
    alpm.set_event_cb((), event);
    Ok(alpm)
}

fn load_dbs<'a>(alpm: &'a Alpm, repos: &[impl AsRef<str>]) -> Result<Arr<&'a Db>> {
    repos.iter().map(|repo| alpm.register_syncdb(repo.as_ref(), SigLevel::NONE)).collect()
}

macro_rules! C {
    ($e: expr) => {
        ($e).then_some(())?
    };
}

fn map_pkg(
    pkg: &Package, ignores: &HashSet<&str>, ignore_groups: &HashSet<&str>,
    ignore_suffixes: &[impl AsRef<str>], dbs: &[&Db],
) -> Option<(Str, Str)> {
    let name = pkg.name();
    C!(ignores.is_empty() || !ignores.contains(name));
    C!(ignore_groups.is_empty() || !pkg.groups().iter().any(|g| ignore_groups.contains(g)));
    C!(ignore_suffixes.is_empty() || !ignore_suffixes.iter().any(|s| name.ends_with(s.as_ref())));
    C!(dbs.iter().all(|db| db.pkg(name).is_err()));
    Some((Str::from(name), Str::from(pkg.version().as_str())))
}

pub fn find_foreign_packages(
    dbpath: &str, repos: &[impl AsRef<str>], ignores: &[impl AsRef<str>],
    ignore_groups: &[impl AsRef<str>], ignore_suffixes: &[impl AsRef<str>],
) -> Result<Arr<(Str, Str)>> {
    let alpm = init(dbpath)?;
    let dbs = load_dbs(&alpm, repos)?;
    let ignores = utils::to_hashset(ignores);
    let ignore_groups = utils::to_hashset(ignore_groups);
    let map = move |pkg| map_pkg(pkg, &ignores, &ignore_groups, ignore_suffixes, &dbs);
    Ok(alpm.localdb().pkgs().into_iter().filter_map(map).collect())
}
