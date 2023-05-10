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

### `--ignore-group <group>`

Ignore packages in a specified group.

### `--ignore-ends <suffix>`

Ignore packages with names ending in a specified string.

### `--color <when>`

Specify when to enable coloring. Valid options are `always`, `never`, or `auto`.  
Default value `auto` only enables colors when outputting onto a tty.

## Dependencies

-   `libalpm` >= 13
-   `libcurl` >= 7.24
