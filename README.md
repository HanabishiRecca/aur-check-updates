# aur-check-updates

A very basic CLI app for checking updates from **Arch User Repository** (AUR).

The app is designed to perform only this simple task, so it does **not** build packages nor pull VCS updates.  
If you need a full-featured AUR helper, consider using [`paru`](https://github.com/morganamilo/paru) instead.

Example:

    $ aur-check-updates
    :: Checking AUR updates...
    foo 1.0.0 => 2.0.0
    bar is not in AUR

## Options

### `--ignore <package>`

Do not check updates for a package.  
Multiple packages can be specified by separating them with a comma.

Example to ignore `foo` package:

    $ aur-check-updates --ignore foo

### `--ignoregroup <group>`

Do not check updates for packages in a group.  
Multiple groups can be specified by separating them with a comma.

Example to ignore packages in `custom` group:

    $ aur-check-updates --ignoregroup custom

### `--color <when>`

Specify when to enable coloring. Valid options are `always`, `never`, or `auto`.  
Default value `auto` only enables colors when outputting onto a tty.

Example to disable coloring:

    $ aur-check-updates --color never

## Dependencies

-   `libalpm` >= 13
-   `libcurl` >= 7.24
