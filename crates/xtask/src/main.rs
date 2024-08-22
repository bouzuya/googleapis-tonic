mod bytes_type;
mod command;
mod crate_name;
mod crate_version;
mod feature_name;
mod googleapis_version;
mod ident;
mod map_type;
mod module;
mod modules;
mod proto_dir;
mod proto_file;
mod proto_file_path;
mod protobuf_package_name;
mod sha1hash;
mod state;

#[derive(clap::Parser)]
struct Cli {
    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    /// Generate crates and update state.json. (state.json + googleapis/ -> state.json (updated) + generated/*)
    Build {
        #[arg(long)]
        force_update: bool,
    },
    /// Publish the generated/* crates to crates.io
    Publish,
    /// Show googleapis version (read googleapis/)
    ShowGoogleapisVersion,
    /// Test generated/* crates
    Test,
    /// Update googleapis/ (git submodule update)
    UpdateGoogleapis,
    /// Update README.md
    UpdateReadme,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = <Cli as clap::Parser>::parse();
    match cli.subcommand {
        Subcommand::Build { force_update } => self::command::build::execute(force_update),
        Subcommand::Publish => self::command::publish::execute().await,
        Subcommand::ShowGoogleapisVersion => self::command::show_googleapis_version::execute(),
        Subcommand::Test => self::command::test::execute(),
        Subcommand::UpdateGoogleapis => self::command::update_googleapis::execute(),
        Subcommand::UpdateReadme => self::command::update_readme::execute(),
    }
}
