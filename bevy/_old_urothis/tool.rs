#![feature(io_error_more)]
pub mod bin;
pub mod lib;

use clap::{Parser, Subcommand};
use const_format::concatcp;
use shadow_rs::shadow;
shadow!(build);
use env_logger::Builder;
use log::LevelFilter;

const ABOUT: &str = concatcp!(
    "INTERNAL Mnemosyne tool",
    " | Commit: ",
    build::SHORT_COMMIT,
    " | Built: ",
    build::BUILD_TIME_2822
);

#[derive(Debug, Parser)]
#[clap(about = ABOUT, long_about = None, propagate_version = true, version = build::PKG_VERSION)]
pub struct Cli {
    #[clap(short, long)]
    verbose: bool,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(arg_required_else_help = true, name = "generate")]
    Generate(bin::tool::commands::generate::Generator),
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut builder = Builder::new();
    builder.filter_level(LevelFilter::Info);
    builder.init();
    let cli = Cli::parse();
    match cli.command {
        Commands::Generate(generate) => {
            match generate.command.unwrap() {
                bin::tool::commands::generate::GeneratorCommands::ObjectData(_data) => {}
            };
            Ok(())
        }
    }
}
