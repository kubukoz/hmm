mod add;
mod cli;
mod darwin;
mod files;

use std::{
    fs::read_to_string,
    io::{Read, Write},
    process::{Command, Stdio},
};

use crate::add::add;
use cli::Cmd;
use darwin::rebuild_system;
use files::{ensure_config_file, open_rw_or_create};
use rowan::SyntaxNode;
use structopt::StructOpt;

use rnix::{
    parse as parse_nix,
    types::{List, Str, TypedNode, Wrapper},
    NixLanguage, StrPart,
};

fn main() {
    let str = read_to_string("/Users/jkoslowski/.nixpkgs/programs/auto.nix").unwrap();

    let parsed = parse_nix_string_list(str);

    dbg!(&parsed);

    let stringified = render_string_list(&parsed);
    let out = nixfmt_run(stringified);
    println!("{}", out);
    // let command = Cmd::from_args();

    // match command {
    //     Cmd::Add { programs } => {
    //         let file_path = ensure_config_file();

    //         let mut file = open_rw_or_create(&file_path);

    //         add(programs, &mut file);

    //         rebuild_system()
    //     }
    // };
}

fn parse_nix_string_list(str: String) -> Vec<String> {
    let nix_list = parse_nix(str.as_str())
        .as_result()
        .expect("Couldn't parse value as Nix AST")
        .root()
        .inner()
        .and_then(List::cast)
        .expect("Value wasn't a List");

    nix_list
        .items()
        .map(parse_nix_string_literal)
        .collect::<Vec<_>>()
}

fn parse_nix_string_literal(item: SyntaxNode<NixLanguage>) -> String {
    Str::cast(item)
        .expect("List element wasn't a String")
        .parts()
        .into_iter()
        .map(|part| match part {
            StrPart::Literal(lit) => lit,
            StrPart::Ast(_) => panic!("List element wasn't a string literal"),
        })
        .collect::<Vec<_>>()
        .join("")
}

fn nixfmt_run(input: String) -> String {
    let process = Command::new("nixfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    process
        .stdin
        .as_ref()
        .expect("Couldn't get stdin handle of nixfmt")
        .write_all(input.as_bytes())
        .expect("Couldn't write to nixfmt");

    // todo: read exit code?
    String::from_utf8(process.wait_with_output().expect("nixfmt failed").stdout)
        .expect("Couldn't read valid ")
}

fn render_string_list(values: &Vec<String>) -> String {
    let mut s = String::new();
    s.push('[');

    let values_string = values
        .clone()
        .iter_mut()
        .map(|v| {
            v.insert_str(0, "\"");
            v.push_str("\"");
            v.to_string()
        })
        .collect::<Vec<_>>()
        .join(" ");

    s.push_str(values_string.as_str());

    s.push(']');
    s
}
