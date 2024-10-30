use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::str::FromStr as _;
use std::time::Duration;

use anyhow::Context as _;

use crate::crate_name::CrateName;
use crate::crate_version::CrateVersion;
use crate::dirs;

pub struct Args {
    pub crate_name: Option<String>,
}

pub async fn execute(Args { crate_name }: Args) -> anyhow::Result<()> {
    let generated_dir = dirs::generated_dir()?;

    let target_crate_names = {
        match crate_name {
            None => {
                // read generated_dir (list all crate names)
                let mut target_crate_names = BTreeSet::new();
                for dir_entry in fs::read_dir(&generated_dir)? {
                    let dir_entry = dir_entry?;
                    if !dir_entry.file_type()?.is_dir() {
                        continue;
                    }

                    let crate_name =
                        CrateName::from_str(dir_entry.file_name().to_str().unwrap_or_default())?;
                    if !crate_name.as_ref().starts_with("googleapis-tonic-") {
                        continue;
                    }

                    target_crate_names.insert(crate_name);
                }
                target_crate_names
            }
            Some(target_crate_name) => {
                let crate_name = CrateName::from_str(&target_crate_name)?;
                let dir_path = generated_dir.join(crate_name.as_ref());
                if !dir_path.metadata()?.is_dir() {
                    anyhow::bail!("specified crate_name is not dir");
                }

                let mut target_crate_names = BTreeSet::new();
                target_crate_names.insert(crate_name);
                target_crate_names
            }
        }
    };

    let mut published = BTreeSet::new();
    for crate_name in target_crate_names {
        let sleep_time =
            match publish_recursive(&mut published, &generated_dir, &crate_name).await? {
                PublishResult::New => 600,
                PublishResult::Update => 60,
                PublishResult::UpToDate => 0,
            };
        if sleep_time > 0 {
            println!("Sleep {} secs", sleep_time);
            tokio::time::sleep(Duration::from_secs(sleep_time)).await;
        }
    }

    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum PublishResult {
    New,
    Update,
    UpToDate,
}

#[async_recursion::async_recursion]
async fn publish_recursive(
    published: &mut BTreeSet<CrateName>,
    generated_dir: &Path,
    crate_name: &CrateName,
) -> anyhow::Result<PublishResult> {
    if published.contains(crate_name) {
        return Ok(PublishResult::UpToDate);
    }

    let crate_dir_path = generated_dir.join(crate_name.as_ref());
    let (local_version, deps) = read_crate_metadata(&crate_dir_path)?;
    let remote_version = fetch_published_version(crate_name).await?;
    tokio::time::sleep(Duration::from_millis(100)).await;

    let already_published = match (remote_version.as_ref(), &local_version) {
        (None, _) => false,
        (Some(p), c) => p >= c,
    };

    if already_published {
        return Ok(PublishResult::UpToDate);
    } else {
        for dep in deps {
            let sleep_time = match publish_recursive(published, generated_dir, &dep).await? {
                PublishResult::New => 600,
                PublishResult::Update => 300,
                PublishResult::UpToDate => 0,
            };
            if sleep_time > 0 {
                println!("Sleep {} secs", sleep_time);
                tokio::time::sleep(Duration::from_secs(sleep_time)).await;
            }
        }

        println!("{} publish", crate_name);
        let status = Command::new("cargo")
            .args(["publish"])
            .current_dir(crate_dir_path)
            .status()?;
        if status.success() {
            published.insert(crate_name.to_owned());

            if remote_version.is_none() {
                Ok(PublishResult::New)
            } else {
                Ok(PublishResult::Update)
            }
        } else {
            anyhow::bail!("{} failed", crate_name);
        }
    }
}

fn read_crate_metadata(
    crate_dir_path: &Path,
) -> anyhow::Result<(CrateVersion, BTreeSet<CrateName>)> {
    let cargo_toml_path = crate_dir_path.join("Cargo.toml");
    let cargo_toml_content = fs::read_to_string(cargo_toml_path)?;
    let document = toml_edit::DocumentMut::from_str(&cargo_toml_content)?;
    let version = document["package"]["version"]
        .as_value()
        .context("package.version is not value")?
        .as_str()
        .context("package.version is not string")?;
    let version = CrateVersion::from_str(version)?;
    let dependencies = document["dependencies"]
        .as_table()
        .context("dependencies is not table")?
        .into_iter()
        .filter_map(|(k, _)| -> Option<anyhow::Result<CrateName>> {
            k.starts_with("googleapis-tonic-")
                .then_some(CrateName::from_str(k))
        })
        .collect::<anyhow::Result<BTreeSet<CrateName>>>()?;
    Ok((version, dependencies))
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
