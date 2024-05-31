use std::error::Error;

use clap::{Parser, Subcommand};

use topo::{extract, render, render_many};

#[derive(Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Render(render::Args),
    RenderMany(render_many::Args),
    Extract(extract::Args),
}

impl Command {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        match self {
            Self::Render(args) => render::run(args),
            Self::RenderMany(args) => render_many::run(args),
            Self::Extract(args) => extract::run(args),
        }
    }
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let args = Args::parse();
    args.command.run()
}
