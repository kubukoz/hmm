mod add;
mod cli;
mod darwin;
mod files;
mod nix;

use crate::add::add;
use cli::Cmd;
use darwin::rebuild_system;
use files::{ensure_config_file, open_rw_or_create};

use structopt::StructOpt;

fn main() {
    let command = Cmd::from_args();

    match command {
        Cmd::Add { programs } => {
            let file_path = ensure_config_file();

            let mut file = open_rw_or_create(&file_path);

            add(programs, &mut file);

            rebuild_system()
        }
        Cmd::Vscode(vsc) => {
            panic!("kalm")
        }
    };
}
