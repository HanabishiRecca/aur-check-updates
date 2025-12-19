use crate::package::Pkg;
use crate::types::{Arr, Str};
use crate::{print, utils};
use alpm::{Alpm, AlpmList, AnyEvent, Db, Event, Package, Result, SigLevel};

fn event(e: AnyEvent, _: &mut ()) {
    if let Event::DatabaseMissing(event) = e.event() {
        print::warning(format_args!("database file for '{}' does not exist", event.dbname()));
    }
}

pub struct Query {
    alpm: Alpm,
}

impl Query {
    pub fn new(dbpath: &str) -> Result<Self> {
        let alpm = Alpm::new("/", dbpath)?;
        alpm.set_event_cb((), event);
        Ok(Self { alpm })
    }

    fn load_db(&self, name: &str) -> Result<&Db> {
        self.alpm.register_syncdb(name, SigLevel::NONE)
    }

    fn load_dbs(&self, repos: &[Str]) -> Result<Arr<&Db>> {
        repos.iter().map(move |name| self.load_db(name)).collect()
    }

    fn pkgs(&self) -> AlpmList<'_, &Package> {
        self.alpm.localdb().pkgs()
    }
}

struct Dbs<'a> {
    dbs: Arr<&'a Db>,
}

impl<'a> Dbs<'a> {
    fn new(dbs: Arr<&'a Db>) -> Self {
        Self { dbs }
    }

    fn has(&self, pkg: &Package) -> bool {
        let name = pkg.name();
        self.dbs.iter().any(move |db| db.pkg(name).is_ok())
    }
}
pub struct Filter<'a> {
    names: &'a [Str],
    groups: &'a [Str],
    suffixes: &'a [Str],
}

impl<'a> Filter<'a> {
    pub fn new(names: &'a [Str], groups: &'a [Str], suffixes: &'a [Str]) -> Self {
        Self { names, groups, suffixes }
    }

    fn has_name(&self, name: &str) -> bool {
        utils::contains(self.names, name)
    }

    fn has_group(&self, group: &str) -> bool {
        utils::contains(self.groups, group)
    }

    fn test_groups(&self, groups: AlpmList<&str>) -> bool {
        !self.groups.is_empty() && groups.iter().any(move |group| self.has_group(group))
    }

    fn test_suffixes(&self, name: &str) -> bool {
        self.suffixes.iter().any(move |suffix| name.ends_with(suffix.as_ref()))
    }

    fn test(&self, pkg: &Package) -> bool {
        let name = pkg.name();
        self.has_name(name) || self.test_groups(pkg.groups()) || self.test_suffixes(name)
    }
}

fn to_pkg(pkg: &Package) -> Pkg {
    Pkg::new(Str::from(pkg.name()), Str::from(pkg.version().as_str()))
}

pub fn find(
    dbpath: &str, repos: &[Str], names: &[Str], groups: &[Str], suffixes: &[Str],
) -> Result<Arr<Pkg>> {
    let query = Query::new(dbpath)?;
    let dbs = Dbs::new(query.load_dbs(repos)?);
    let filter = Filter::new(names, groups, suffixes);
    let pkgs = query.pkgs().into_iter().filter(move |pkg| !dbs.has(pkg) && !filter.test(pkg));
    Ok(pkgs.map(to_pkg).collect())
}
