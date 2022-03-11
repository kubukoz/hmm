use std::{collections::HashMap, fs::File};

use crate::{
    files::{read_file, write_file},
    nix::{nixfmt_run, parse_nix_attributes_list, render_nix_attributes_list, Attr, Attrs},
    types::{Add, Update, UpdateKind, UpdateResult},
    vscode_search::download_latest_extension,
};
use reqwest::blocking::Client;

pub(crate) fn managed_update(file: &mut File) -> UpdateResult<Update> {
    let client = Client::new();

    let mut updates: Vec<Update> = Vec::default();

    let updated_packages = parse_nix_attributes_list(read_file(file))
        .into_iter()
        .map(Package::from_attrs)
        .map(|p| {
            let result = download_latest_extension(&p, &client);

            if p != result {
                updates.push(Update {
                    program: p.publisher + "." + p.name.as_str(),
                    from: p.version,
                    to: result.version.clone(),
                });
            }

            result
        })
        .map(Package::to_attrs)
        .collect::<Vec<_>>();

    write_file(
        nixfmt_run(render_nix_attributes_list(&updated_packages)),
        file,
    );

    UpdateResult {
        updates,
        kind: UpdateKind::Update,
    }
}

pub(crate) fn managed_add(extensions: &Vec<String>, file: &mut File) -> UpdateResult<Add> {
    let client = Client::new();

    let mut updates: Vec<Add> = Vec::default();

    let mut packages = parse_nix_attributes_list(read_file(file))
        .into_iter()
        .map(Package::from_attrs)
        .collect::<Vec<_>>();

    extensions
        .into_iter()
        .map(|e| {
            let p = download_latest_extension(&Package::from_publisher_name(e), &client);

            if !packages.contains(&p) {
                updates.push(Add {
                    program: p.publisher.clone() + "." + p.name.as_str().clone(),
                    version: Some(p.version.clone()),
                });

                packages.push(p);
            }
        })
        .for_each(drop);

    let packages = packages.into_iter().map(Package::to_attrs).collect();

    write_file(nixfmt_run(render_nix_attributes_list(&packages)), file);
    UpdateResult {
        updates,
        kind: UpdateKind::Add,
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Package {
    pub name: String,
    pub publisher: String,
    pub version: String,
    pub sha256: String,
}

impl Package {
    fn to_attrs(self) -> Attrs {
        Attrs(vec![
            Attr {
                name: "name".to_string(),
                value: self.name,
            },
            Attr {
                name: "publisher".to_string(),
                value: self.publisher,
            },
            Attr {
                name: "version".to_string(),
                value: self.version,
            },
            Attr {
                name: "sha256".to_string(),
                value: self.sha256,
            },
        ])
    }

    fn from_attrs(attrs: Attrs) -> Package {
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

    fn from_publisher_name(e: &str) -> Package {
        let mut split = e.split('.');

        Package {
            publisher: split.next().unwrap().to_owned(),
            name: split.next().unwrap().to_owned(),
            version: "".to_string(),
            sha256: "".to_string(),
        }
    }
}
