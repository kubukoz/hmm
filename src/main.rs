mod add;
mod darwin;
mod files;
use add::add;
use darwin::rebuild_system;
use files::{ensure_config_file, open_rw_or_create};

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Home Manager Manager",
    about = "Manages your Home Manager config",
    version = env!("CARGO_PKG_VERSION"),

)]
enum Cmd {
    #[structopt(about = "Installs a program")]
    Add {
        #[structopt(required = true)]
        programs: Vec<String>,
    },
}

fn main() {
    let command = Cmd::from_args();

    match command {
        Cmd::Add { programs } => {
            let file_path = ensure_config_file();

            let mut file = open_rw_or_create(&file_path);

            add(programs, &mut file);

            rebuild_system()
        }
    };
}
