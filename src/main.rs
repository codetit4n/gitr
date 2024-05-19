use clap::Parser;
mod cli;
mod utils;

fn main() {
    use cli::Args;

    let args = Args::parse();
    args.cmd.execute();
}
