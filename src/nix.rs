use std::{
    io::Write,
    process::{Command, Stdio},
};

use rowan::SyntaxNode;

use rnix::{
    parse as parse_nix,
    types::{List, Str, TypedNode, Wrapper},
    NixLanguage, StrPart,
};

pub(crate) fn parse_nix_string_list(str: String) -> Vec<String> {
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

pub(crate) fn nixfmt_run(input: String) -> String {
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

pub(crate) fn render_nix_string_list(values: &Vec<String>) -> String {
    let mut s = String::new();
    s.push_str(prelude("This file should always parse as an array of string literals.").as_str());
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

fn prelude(details: &str) -> String {
    format!(
        r#"
    #
    # This file was written by hmm (Home Manager Manager).
    # Don't edit it manually!
    # {}
    #
    "#,
        details
    )
    .to_string()
}
