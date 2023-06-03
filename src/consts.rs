macro_rules! C {
    ($n: ident, $v: expr $(,)?) => {
        pub const $n: &str = $v;
    };
}

C!(PACMAN_CONF, "/etc/pacman.conf");
C!(DEFAULT_DBPATH, "/var/lib/pacman/");
C!(AUR_ENDPOINT, "https://aur.archlinux.org/rpc/v5/info");
