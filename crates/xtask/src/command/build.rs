mod build_crate;
mod build_crates;

use crate::proto_dir::ProtoDir;

pub fn execute() -> anyhow::Result<()> {
    let proto_dir = "crates/xtask/googleapis";
    let src_dir = "crates/googleapis-tonic/src";

    let proto_dir = ProtoDir::load(proto_dir)?;

    build_crate::build_crate(src_dir, &proto_dir)?;
    build_crates::build_crates(src_dir, &proto_dir)?;

    Ok(())
}
