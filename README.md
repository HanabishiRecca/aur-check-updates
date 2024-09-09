# aur-check-updates

A very basic CLI app for checking updates from **Arch User Repository** (AUR).

    $ aur-check-updates
    :: Checking AUR updates...
    foo 1.0 => 2.0
    bar 1.0 => [not found in AUR]

The app is designed to perform only this simple task, so it does **not** build packages nor pull VCS updates.  
If you need a fully-featured AUR helper, consider using [`paru`](https://github.com/morganamilo/paru) instead.

## Usage

    $ aur-check-updates [<option>...]

| Option                  | Description                                                               |
| ----------------------- | ------------------------------------------------------------------------- |
| `--ignore <package>`    | Do not check updates for a package.<sup>1</sup>                           |
| `--ignoregroup <group>` | Do not check updates for packages in a group.<sup>1</sup>                 |
| `--color <when>`        | Specify when to enable coloring: `always`, `never` or `auto`.<sup>2</sup> |
| `--timeout <ms>`        | Set a timeout for network connection in milliseconds.<sup>3</sup>         |
| `--dbpath <path>`       | Alternate database location.                                              |
| `--repo <name>`         | Override working repository.<sup>4</sup>                                  |
| `-h`, `--help`          | Display help message and exit.                                            |

1. Ignore options can be used more than once. Multiple packages/groups can be specified by separating them with a comma.
2. Default value is `auto`, it enables coloring only when tty is detected.
3. Default value is `5000`.
4. Can be used more than once. Multiple repositories can be specified by separating them with a comma. By default the app reads all repositories from `dbpath/sync` directory.

### Example

    $ aur-check-updates --ignore foo,bar --ignoregroup custom --color never --timeout 10000

Ignores `foo` and `bar` packages, ignores all packages in `custom` group, disables coloring, sets timeout to 10 seconds.

## Download

You can download prebuilt binaries from [releases](https://github.com/HanabishiRecca/aur-check-updates/releases) page.

## Building from the source

**Rust 1.70 or up is required.**

Install dependencies:

-   `libalpm`
-   `libcurl`

Install Rust compiler and run:

    $ cargo build --release

## Packages

**AUR**

-   [`aur-check-updates`](https://aur.archlinux.org/packages/aur-check-updates)
-   [`aur-check-updates-bin`](https://aur.archlinux.org/packages/aur-check-updates-bin)
