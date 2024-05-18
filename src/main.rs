use clap::Parser;
mod cli;

fn main() {
    let args = cli::Args::parse();
    match args.cmd {
        cli::Commands::Init => unimplemented!(),
        cli::Commands::Add => unimplemented!(),
    }
}
