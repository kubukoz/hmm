include!("src/cli.rs");

fn main() {
    Cmd::clap().gen_completions("hmm", structopt::clap::Shell::Zsh, "./completions/zsh");
}
