use clap::Parser;
use gitr::cli::Args;

fn main() {
    let args = Args::parse();
    args.cmd.execute();
}
