# hmm

"Home Manager" Manager.

```
Home Manager Manager 0.1.0
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

## Features

Not many: for now, just adding programs to a specific file and running a specific command. Might be configurable in the future.

Provides zsh completions (generated to `./completions/zsh` at build time).

## Commands

### add

Installs the given program(s): reads the target file (currently hardcoded as `~/.nixpkgs/programs/auto.txt`),
adds all the parameters as lines, sorts the lines and writes them back. Finally, triggers `darwin-rebuild switch` and expects it to succeed.
