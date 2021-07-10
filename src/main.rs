mod add;
mod cli;
mod darwin;
mod files;
mod nix;
mod vscode;

use crate::add::add;
use cli::Cmd;
use cli::Vscode;
use darwin::rebuild_system;
use files::{open_rw_or_create, root_path};

use structopt::StructOpt;

fn main() {
    let command = Cmd::from_args();

    match command {
        Cmd::Add { programs } => {
            let file_path = root_path().join("programs").join("auto.nix");

            let mut file = open_rw_or_create(&file_path);

            add(programs, &mut file);

            rebuild_system()
        }
        Cmd::Vscode(vsc) => match vsc {
            Vscode::Add { extensions } => {
                let file_path = root_path()
                    .join("vscode")
                    .join("extensions")
                    .join("auto.nix");

                let mut file = open_rw_or_create(&file_path);

                add(extensions, &mut file);

                rebuild_system()
            }
            Vscode::Managed(man) => match man {
                cli::Managed::Update => {
                    let file_path = root_path()
                        .join("vscode")
                        .join("extensions")
                        .join("managed.nix");

                    let mut file = open_rw_or_create(&file_path);

                    vscode::update(&mut file);

                    rebuild_system()
                }
            },
        },
    };
}
