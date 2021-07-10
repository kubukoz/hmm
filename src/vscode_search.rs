use crate::{nix::nix_prefetch_url, vscode::Package};
use chrono::{DateTime, Utc};
use colored::Colorize;
use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::json;

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

pub(crate) fn download_latest_extension(package: Package, client: &Client) -> Package {
    let response: SearchResults = search_versions(&package, client);

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
        println!(
            "{}",
            format!(
                "Updating {}.{} ({} -> {})",
                package.publisher, package.name, package.version, last_version.version
            )
            .green()
        );
        nix_prefetch_url(last_version_url)
    } else {
        println!(
            "{}",
            format!(
                "Skipping update for {}.{} because it didn't change ({})",
                package.publisher, package.name, package.version
            )
            .blue()
        );
        package.sha256
    };

    Package {
        name: package.name,
        publisher: package.publisher,
        version: last_version.version,
        sha256,
    }
}

fn search_versions(package: &Package, client: &Client) -> SearchResults {
    let api_url = "https://marketplace.visualstudio.com/_apis/public/gallery/extensionquery?api-version=6.1-preview";

    let request = json!({
        "filters": [
            {"criteria": [{
                "filterType": 7,
                "value": format!("{}.{}", package.publisher, package.name)
            }]},

        ],
        "flags": 103
    });

    client
        .post(api_url)
        .json(&request)
        .send()
        .unwrap()
        .json()
        .unwrap()
}
