# hm²

"Home Manager" Manager (hmm, hm²).

## About

The purpose of this program is to let me manage https://github.com/kubukoz/nix-config/ without opening an editor every time I want to install something (think `nix-env` but with home-manager / nix-darwin). Works on my machine, if you want it to work on yours feel free to fork/contribute.

## Features

Not many: for now, just adding programs to a specific file and running a specific command. Might be configurable in the future.

Provides zsh completions (generated to `./completions/zsh` at build time).

## Requirements

`nixfmt`, nix-darwin

## Usage

```shell
$ hmm help

Home Manager Manager 0.3.0
Manages your Home Manager config

USAGE:
    hmm <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add     Installs a program
    help    Prints this message or the help of the given subcommand(s)
```

### add

Installs the given program(s):

- reads the target file (currently hardcoded as `~/.nixpkgs/programs/auto.nix`)
- parses it as a Nix array of string literals (`["foo" "bar"]`)
- adds all the parameters as lines
- sorts the lines, deduplicates them
- writes the result as a Nix string array to the same file
- finally, triggers `darwin-rebuild switch` and expects it to succeed.
