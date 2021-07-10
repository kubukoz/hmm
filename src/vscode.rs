use std::{collections::HashMap, fs::File};

use crate::{
    files::{read_file, write_file},
    nix::{
        nix_prefetch_url, nixfmt_run, parse_nix_attributes_list, render_nix_attributes_list, Attr,
        Attrs,
    },
};
use chrono::{DateTime, Utc};
use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::json;

pub(crate) fn update(file: &mut File) {
    let packages = parse_nix_attributes_list(read_file(file))
        .into_iter()
        .map(decode_vscode_package)
        .collect::<Vec<_>>();

    let updated_packages = packages
        .into_iter()
        .map(download_latest_extension)
        .map(Package::to_attrs)
        .collect::<Vec<_>>();

    write_file(
        nixfmt_run(render_nix_attributes_list(&&updated_packages)),
        file,
    );
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

#[derive(Deserialize, Debug)]
struct SearchResults {
    results: Vec<SearchResult>,
}
#[derive(Deserialize, Debug)]
struct SearchResult {
    extensions: Vec<Extension>,
}

#[derive(Deserialize, Debug)]
struct Extension {
    versions: Vec<Version>,
}

#[derive(Deserialize, Debug)]
struct Version {
    version: String,
    #[serde(rename = "lastUpdated")]
    last_updated: DateTime<Utc>,
    files: Vec<Asset>,
}

#[derive(Deserialize, Debug)]
struct Asset {
    #[serde(rename = "assetType")]
    asset_type: String,
    source: String,
}

fn download_latest_extension(package: Package) -> Package {
    let client = Client::new();

    let req = json!({
        "filters": [
            {"criteria": [{
                "filterType": 7,
                "value": format!("{}.{}", package.publisher, package.name)
            }]},

        ],
        "flags": 103
    });

    let response: SearchResults = client.post("https://marketplace.visualstudio.com/_apis/public/gallery/extensionquery?api-version=6.1-preview").json(&req).send().unwrap().json().unwrap();

    let last_version = response
        .results
        .into_iter()
        .flat_map(|r| r.extensions)
        .flat_map(|e| e.versions)
        .max_by_key(|a| a.last_updated)
        .unwrap();

    let last_version_url = last_version
        .files
        .into_iter()
        .find(|a| a.asset_type == "Microsoft.VisualStudio.Services.VSIXPackage")
        .unwrap()
        .source;

    // Skip download if the version didn't change
    let sha256 = if last_version.version != package.version {
        nix_prefetch_url(last_version_url)
    } else {
        package.sha256
    };

    Package {
        name: package.name,
        publisher: package.publisher,
        version: last_version.version,
        sha256,
    }
}

#[derive(Debug)]
struct Package {
    name: String,
    publisher: String,
    version: String,
    sha256: String,
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
}
