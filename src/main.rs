mod add;
mod cli;
mod darwin;
mod files;
mod nix;
mod types;
mod vscode;
mod vscode_search;

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

            let result = add(programs, &mut file);

            if result.was_updated {
                rebuild_system()
            } else {
                println!("No new packages were added, skipping system rebuild")
            }
        }
        Cmd::Vscode(vsc) => match vsc {
            Vscode::Add { extensions } => {
                let file_path = root_path()
                    .join("vscode")
                    .join("extensions")
                    .join("auto.nix");

                let mut file = open_rw_or_create(&file_path);

                let result = add(extensions, &mut file);

                if result.was_updated {
                    rebuild_system()
                } else {
                    println!("No new packages were added, skipping system rebuild")
                }
            }
            Vscode::Managed(man) => match man {
                cli::Managed::Update => {
                    let file_path = root_path()
                        .join("vscode")
                        .join("extensions")
                        .join("managed.nix");

                    let mut file = open_rw_or_create(&file_path);

                    let result = vscode::managed_update(&mut file);

                    if result.was_updated {
                        rebuild_system()
                    } else {
                        println!("No updates found, skipping system rebuild")
                    }
                }
            },
        },
    };
}
