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

| Option                      | Description                                                                                                                           |
| --------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| `-u`, `--updated`           | Also show up-to-date packages.                                                                                                        |
| `-n`, `--nofailed`          | Hide packages not found in AUR.                                                                                                       |
| `--ignore <packages>`       | Do not check updates for packages. <sup>1</sup>                                                                                       |
| `--ignoregroup <groups>`    | Do not check updates for package groups. <sup>1</sup>                                                                                 |
| `--ignoresuffix <suffixes>` | Do not check updates for packages with suffixes. Default value is `-debug`, to exclude autogenerated debug packages. <sup>1</sup>     |
| `--color <when>`            | Specify when to enable coloring: `always`, `never` or `auto`. Default value is `auto`, it enables coloring only when tty is detected. |
| `--dbpath <path>`           | Alternate database location. Default value is `/var/lib/pacman`.                                                                      |
| `--repos <names>`           | Override working repositories. By default all repositories from `dbpath/sync` directory are used. <sup>1</sup>                        |
| `--endpoint <url>`          | Change AUR info endpoint url. Default value is `https://aur.archlinux.org/rpc/v5/info`.                                               |
| `--timeout <ms>`            | Set a timeout for network connection in milliseconds. Default value is `5000`.                                                        |
| `-h`, `--help`              | Display the help message.                                                                                                             |

1. Multiple values could be specified using a comma-separated list.

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
