mod bytes_type;
mod command;
mod crate_name;
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

#[derive(clap::Parser)]
struct Cli {
    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    Build,
    Publish,
    ShowGoogleapisVersion,
    Test,
    UpdateGoogleapis,
}

fn main() -> anyhow::Result<()> {
    let cli = <Cli as clap::Parser>::parse();
    match cli.subcommand {
        Subcommand::Build => self::command::build::execute(),
        Subcommand::Publish => self::command::publish::execute(),
        Subcommand::ShowGoogleapisVersion => self::command::show_googleapis_version::execute(),
        Subcommand::Test => self::command::test::execute(),
        Subcommand::UpdateGoogleapis => self::command::update_googleapis::execute(),
    }
}
