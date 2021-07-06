use std::{collections::HashMap, fs::File};

use crate::{
    files::read_file,
    nix::{parse_nix_attributes_list, Attrs},
};

pub(crate) fn update(file: &mut File) {
    let packages = parse_nix_attributes_list(read_file(file))
        .into_iter()
        .map(decode_vscode_package)
        .collect::<Vec<_>>();

    dbg!(packages);

    todo!()
}

fn decode_vscode_package(attrs: Attrs) -> Package {
    let mut indexed: HashMap<String, String> = HashMap::default();

    attrs.0.into_iter().for_each(|attr| {
        indexed.insert(attr.name, attr.value);
    });

    Package {
        name: indexed.get("name").expect("name missing").to_owned(),
        publisher: indexed
            .get("publisher")
            .expect("publisher missing")
            .to_owned(),
        version: indexed.get("version").expect("version missing").to_owned(),
        sha256: indexed.get("sha256").expect("sha256 missing").to_owned(),
    }
}

#[derive(Debug)]
struct Package {
    name: String,
    publisher: String,
    version: String,
    sha256: String,
}
