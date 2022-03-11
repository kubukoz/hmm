use crate::{nix::nix_prefetch_url, vscode::Package};
use colored::Colorize;
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Manifest {
    version: String,
}

pub(crate) fn download_latest_extension(package: &Package, client: &Client) -> Package {
    let response = search_versions(package, client);

    let latest_version = response.version;

    let last_version_url =
    format!("https://{publisher}.gallery.vsassets.io/_apis/public/gallery/publisher/{publisher}/extension/{name}/{version}/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage",
        publisher = package.publisher,
        name = package.name,
        version = latest_version
    );

    // Skip download if the version didn't change
    let sha256 = if latest_version != package.version {
        println!(
            "{}",
            format!(
                "Updating {}.{} ({} -> {})",
                package.publisher, package.name, package.version, latest_version
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
        package.sha256.clone()
    };

    Package {
        name: package.name.clone(),
        publisher: package.publisher.clone(),
        version: latest_version,
        sha256,
    }
}

fn search_versions(package: &Package, client: &Client) -> Manifest {
    let api_url = format!(
        "https://{publisher}.gallery.vsassets.io/_apis/public/gallery/publisher/{publisher}/extension/{name}/latest/assetbyname/Microsoft.VisualStudio.Code.Manifest",
        publisher = package.publisher,
        name = package.name
    );

    client.get(api_url).send().unwrap().json().unwrap()
}
