mod bytes_type;
mod command;
mod feature_name;
mod ident;
mod map_type;
mod module;
mod modules;
mod package_name;
mod proto_dir;
mod proto_file;

#[derive(clap::Parser)]
struct Cli {
    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    Build,
    Test,
}

fn main() -> anyhow::Result<()> {
    let cli = <Cli as clap::Parser>::parse();
    match cli.subcommand {
        Subcommand::Build => self::command::build::execute(),
        Subcommand::Test => self::command::test::execute(),
    }
}
