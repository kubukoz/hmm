# hm²

"Home Manager" Manager (hmm, hm²).

## About

The purpose of this program is to let me manage https://github.com/kubukoz/nix-config/ without opening an editor every time I want to install something (think `nix-env` but with home-manager / nix-darwin). Works on my machine, if you want it to work on yours feel free to fork/contribute.

## Features

Not many: for now, just adding programs to a specific file and running a specific command. Might be configurable in the future.

Provides zsh completions (generated to `./completions/zsh` at build time).

## Requirements

`nixfmt`

## Usage

```shell
$ hmm help

Home Manager Manager 0.5.0
Manages your Home Manager config

USAGE:
    hmm <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add       Installs programs
    help      Prints this message or the help of the given subcommand(s)
    vscode    Gateway to vscode programs
```

All commands commit the files they change with details about the change.

### add

Installs the given program(s):

- reads the target file (currently hardcoded as `~/.nixpkgs/programs/auto.nix`)
- parses it as a Nix array of string literals (`["foo" "bar"]`)
- adds all the parameters as lines
- sorts the lines, deduplicates them
- writes the result as a Nix string array to the same file

### vscode add

Installs the given vscode extension in `~/.nixpkgs/vscode/extensions/auto.nix`. Same rules as `add`.

### vscode managed update

Updates all vscode extensions in `~/.nixpkgs/vscode/extensions/managed.nix`:

- reads the file
- parses everything as nix and expects a list of vscode packages
- queries VS Code Marketplace for the latest version
- if the version doesn't match the one in the file, its artifact is prefetched
- the version/sha256 of the package are updated
- everything is written back to the same file
