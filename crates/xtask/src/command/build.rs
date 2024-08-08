mod build_crate;
mod build_crates;

use std::path::PathBuf;

use crate::proto_dir::ProtoDir;

pub fn execute() -> anyhow::Result<()> {
    let crates_dir = PathBuf::from("crates");
    let proto_dir = crates_dir.join("xtask").join("googleapis");
    let proto_dir = ProtoDir::load(proto_dir)?;
    let version = "0.0.0";

    let googleapis_tonic_src_dir = build_crate::build_crate(&crates_dir, &proto_dir, version)?;
    build_crates::build_crates(&googleapis_tonic_src_dir, &proto_dir, version)?;

    Ok(())
}
