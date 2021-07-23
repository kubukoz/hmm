mod add;
mod cli;
mod darwin;
mod files;
mod git;
mod nix;
mod types;
mod vscode;
mod vscode_search;

use std::path::PathBuf;

use crate::add::add;
use cli::Cmd;
use cli::Vscode;
use darwin::rebuild_system;
use files::{open_rw_or_create, root_path};

use git::git_commit;
use structopt::StructOpt;

fn main() {
    let command = Cmd::from_args();

    match command {
        Cmd::Add { programs } => {
            let relative_path = PathBuf::default().join("programs").join("auto.nix");

            let file_path = root_path().join(&relative_path);

            let mut file = open_rw_or_create(&file_path);

            let result = add(&programs, &mut file);

            if result.was_updated {
                rebuild_system();
                git_commit(
                    &vec![relative_path.as_path()],
                    format!("(hmm) Add {}", programs.join(", ")),
                )
                .expect("Couldn't commit");
            } else {
                println!("No new packages were added, skipping system rebuild")
            }
        }
        Cmd::Vscode(vsc) => match vsc {
            Vscode::Add { extensions } => {
                let relative_path = PathBuf::default()
                    .join("vscode")
                    .join("extensions")
                    .join("auto.nix");

                let file_path = root_path().join(&relative_path);

                let mut file = open_rw_or_create(&file_path);

                let result = add(&extensions, &mut file);

                if result.was_updated {
                    rebuild_system();

                    git_commit(
                        &vec![relative_path.as_path()],
                        format!("(hmm) Vscode: Add {}", extensions.join(", ")),
                    )
                    .expect("Couldn't commit");
                } else {
                    println!("No new packages were added, skipping system rebuild")
                }
            }
            Vscode::Managed(man) => match man {
                cli::Managed::Update => {
                    let result_main = vscode::managed_update(&mut open_rw_or_create(
                        &root_path()
                            .join("vscode")
                            .join("extensions")
                            .join("managed.nix"),
                    ));

                    let result_work = vscode::managed_update(&mut open_rw_or_create(
                        &root_path()
                            .join("work")
                            .join("vscode")
                            .join("extensions")
                            .join("managed.nix"),
                    ));

                    if result_main.was_updated || result_work.was_updated {
                        rebuild_system()
                    } else {
                        println!("No updates found, skipping system rebuild")
                    }
                }
            },
        },
    };
}
