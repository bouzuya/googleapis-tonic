use crate::dirs;
use crate::googleapis_version::GoogleapisVersion;

pub fn execute() -> anyhow::Result<()> {
    let googleapis_dir = dirs::googleapis_dir()?;
    let version = GoogleapisVersion::load_from_googleapis_dir(googleapis_dir)?;
    println!("{}", version);
    Ok(())
}
