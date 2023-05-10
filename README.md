# aur-checkupdates

Very basic CLI app for checking updates from **Arch User Repository** (AUR).

This app does **not**:

-   build packages
-   account VCS updates

The only goal is a simple AUR version check, nothing more.  
If you need a full-featured AUR helper, consider using [`paru`](https://github.com/morganamilo/paru) instead.

Example:

    $ aur-checkupdates
    :: Checking AUR updates...
    foo 1.0.0 => 2.0.0
    bar is not in AUR

## Options

### `--ignoregroup <group>`

Do not check updates for all packages in a `group`.  
Multiple groups can be specified by separating them with a comma.

Example to ignore packages in `custom` group:

    $ aur-checkupdates --ignoregroup "custom"

### `--ignoresuffix <suffix>`

Do not check updates for all packages with names ending in a `suffix`.  
Multiple suffixes can be specified by separating them with a comma.

Example to ignore packages with `-custom` suffix:

    $ aur-checkupdates --ignoresuffix "-custom"

### `--color <when>`

Specify when to enable coloring. Valid options are `always`, `never`, or `auto`.  
Default value `auto` only enables colors when outputting onto a tty.

Example to disable coloring:

    $ aur-checkupdates --color never

## Dependencies

-   `libalpm` >= 13
-   `libcurl` >= 7.24
