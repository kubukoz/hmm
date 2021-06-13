use std::{env::var, fs::create_dir_all, path::Path};

include!("src/cli.rs");

fn main() {
    let target = Path::new(&var("OUT_DIR").expect("OUT_DIR not specified"))
        .join("completions")
        .join("zsh");

    create_dir_all(&target).expect("Couldn't create directories");

    Cmd::clap().gen_completions("hmm", structopt::clap::Shell::Zsh, target);
}