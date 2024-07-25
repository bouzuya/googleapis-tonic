#[derive(clap::Parser)]
struct Cli {
    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    Build,
}

fn main() {
    let cli = <Cli as clap::Parser>::parse();
    match cli.subcommand {
        Subcommand::Build => {
            println!("Build");
        }
    }
}
