use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Home Manager Manager",
    about = "Manages your Home Manager config",
    version = env!("CARGO_PKG_VERSION"),

)]
pub(crate) enum Cmd {
    #[structopt(about = "Installs programs")]
    Add {
        #[structopt(required = true)]
        programs: Vec<String>,
    },
    #[structopt(about = "Gateway to vscode programs")]
    Vscode(Vscode),
}

#[derive(StructOpt, Debug)]
pub(crate) enum Vscode {
    #[structopt(about = "Installs extensions")]
    Add {
        #[structopt(required = true)]
        extensions: Vec<String>,
    },
    Managed(Managed),
}

#[derive(StructOpt, Debug)]
pub(crate) enum Managed {
    #[structopt(about = "Updates all managed extensions")]
    Update,
    // todo: add - install the latest version as managed
}
