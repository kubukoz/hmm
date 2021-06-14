use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Home Manager Manager",
    about = "Manages your Home Manager config",
    version = env!("CARGO_PKG_VERSION"),

)]
pub(crate) enum Cmd {
    #[structopt(about = "Installs a program")]
    Add {
        #[structopt(required = true)]
        programs: Vec<String>,
    },
    #[structopt(about = "Gateway to vscode programs")]
    Vscode(Vscode),
}

#[derive(StructOpt, Debug)]
pub(crate) enum Vscode {
    #[structopt(about = "Installs an extension")]
    Add {
        #[structopt(required = true)]
        extension: String,
        // default: latest
        version: String,
    },
}
