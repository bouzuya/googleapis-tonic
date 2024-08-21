use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr as _;
use std::time::Duration;

use anyhow::Context as _;

use crate::crate_name::CrateName;
use crate::crate_version::CrateVersion;
use crate::state::State;

pub async fn execute() -> anyhow::Result<()> {
    let crates_dir = PathBuf::from("crates");
    let generated_dir = PathBuf::from("generated");

    let state_file = crates_dir.join("xtask").join("state.json");
    let state = State::load(&state_file)?;

    for crate_name in state.publish_order() {
        let published_version = fetch_published_version(crate_name).await?;
        tokio::time::sleep(Duration::from_millis(100)).await;
        let crate_version = state
            .crate_versions()
            .get(crate_name)
            .with_context(|| format!("{} crate version not found", crate_name))?;
        let already_published = match (published_version.as_ref(), crate_version) {
            (None, _) => false,
            (Some(p), c) => p >= c,
        };
        if already_published {
            println!(
                "{} {} has already been published",
                crate_name, crate_version
            );
        } else {
            let status = Command::new("cargo")
                .args(["publish"])
                .current_dir(generated_dir.join(crate_name.to_string()))
                .status()?;
            if status.success() {
                println!("{} published", crate_name);
                if published_version.is_none() {
                    tokio::time::sleep(Duration::from_secs(600)).await;
                } else {
                    tokio::time::sleep(Duration::from_secs(300)).await;
                }
            } else {
                anyhow::bail!("{} failed", crate_name);
            }
        }
    }

    Ok(())
}

async fn fetch_published_version(crate_name: &CrateName) -> anyhow::Result<Option<CrateVersion>> {
    #[derive(Debug, serde::Deserialize)]
    struct CrateResponse {
        #[serde(rename = "crate")]
        krate: CrateResponseCrate,
    }
    #[derive(Debug, serde::Deserialize)]
    struct CrateResponseCrate {
        max_version: String,
    }
    let client = reqwest::Client::new();
    let response = client
        .request(
            reqwest::Method::GET,
            format!("https://crates.io/api/v1/crates/{}", crate_name),
        )
        .header(reqwest::header::USER_AGENT, "googleapis-tonic-xtask")
        .send()
        .await?;
    if !response.status().is_success() {
        if response.status().as_u16() == 404 {
            return Ok(None);
        } else {
            let response_body = response.text().await?;
            anyhow::bail!("{:?}", response_body);
        }
    }
    let response_body = response.json::<CrateResponse>().await?;
    let max_version = response_body.krate.max_version;
    Ok(Some(CrateVersion::from_str(&max_version)?))
}
