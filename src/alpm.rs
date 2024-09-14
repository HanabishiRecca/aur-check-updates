use crate::{
    types::{Arr, Str},
    utils,
};
use alpm::{Alpm, Db, Event, Result, SigLevel};

pub fn init(dbpath: &str) -> Result<Alpm> {
    let alpm = Alpm::new("/", dbpath)?;

    alpm.set_event_cb((), |e, _| {
        if let Event::DatabaseMissing(event) = e.event() {
            crate::print::warning(format_args!(
                "database file for '{}' does not exist",
                event.dbname()
            ))
        }
    });

    Ok(alpm)
}

pub fn load_dbs<'a>(alpm: &'a Alpm, repos: &[impl AsRef<str>]) -> Result<Arr<&'a Db>> {
    repos
        .iter()
        .map(|repo| alpm.register_syncdb(repo.as_ref(), SigLevel::NONE))
        .collect()
}

pub fn find_foreign_packages(
    dbpath: &str,
    repos: &[impl AsRef<str>],
    ignores: &[impl AsRef<str>],
    ignore_groups: &[impl AsRef<str>],
    ignore_suffixes: &[impl AsRef<str>],
) -> Result<Arr<(Str, Str)>> {
    let alpm = init(dbpath)?;
    let dbs = load_dbs(&alpm, repos)?;
    let ignores = utils::to_hashset(ignores);
    let ignore_groups = utils::to_hashset(ignore_groups);

    Ok(alpm
        .localdb()
        .pkgs()
        .into_iter()
        .filter_map(|pkg| {
            let name = pkg.name();
            utils::filter!(ignores.is_empty() || !ignores.contains(name));
            utils::filter!(
                ignore_groups.is_empty() || !pkg.groups().iter().any(|g| ignore_groups.contains(g))
            );
            utils::filter!(
                ignore_suffixes.is_empty()
                    || !ignore_suffixes.iter().any(|s| name.ends_with(s.as_ref()))
            );
            utils::filter!(dbs.iter().all(|db| db.pkg(name).is_err()));
            Some((Str::from(name), Str::from(pkg.version().as_str())))
        })
        .collect())
}
