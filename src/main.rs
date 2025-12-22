use std::{fs, io::stdout};

use clap::Parser;

use crate::config::Config;

mod config;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(
    version,
    long_about = "A CLI that execute git pull command on repositories"
)]
struct Args {
    /// Generate example pulls config file.
    #[arg(long, default_value_t = false)]
    generate_example: bool,
}

fn main() {
    let args = Args::parse();

    if args.generate_example {
        serde_yaml::to_writer(stdout(), &Config::example()).unwrap();
        return;
    }

    let cfg_file_content = fs::read("./pulls.yaml").unwrap();

    let cfg = serde_yaml::from_slice::<config::Config>(&cfg_file_content).unwrap();
    println!("cfg: {cfg:?}");
}
