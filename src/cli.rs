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
        // todo: support versions. Currently only adds a package from pkgs.vscode-extensions
        // default: latest
        // version: String,
    },
}
