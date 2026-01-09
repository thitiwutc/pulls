use std::{
    fs,
    io::stdout,
    process::{Command, Stdio},
};

use clap::Parser;

use crate::config::{Config, RepositoryConfig};

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

    for repo in cfg.repositories {
        let prev_branch = String::from_utf8(
            Command::new("git")
                .args(["branch", "--show-current"])
                .current_dir(&repo.dir)
                .stdout(Stdio::null())
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap();
        let prev_branch_trimmed = prev_branch.trim();

        dbg!(prev_branch_trimmed);

        let g_co_status = Command::new("git")
            .args(["checkout", &repo.branch])
            .stdout(Stdio::null())
            .current_dir(&repo.dir)
            .status()
            .unwrap();
        if !g_co_status.success() {
            eprintln!(
                "git checkout branch={} of dir={} failed: {}",
                repo.branch,
                repo.dir,
                g_co_status.code().unwrap_or(-1),
            );
            continue;
        }

        let g_pl_status = Command::new("git")
            .arg("pull")
            .stdout(Stdio::null())
            .current_dir(&repo.dir)
            .status()
            .unwrap();
        if !g_pl_status.success() {
            eprintln!(
                "git pull branch={} of dir={} failed: {}",
                repo.branch,
                repo.dir,
                g_pl_status.code().unwrap_or(-1),
            );
            checkout_prev_branch(&repo, prev_branch_trimmed);
            continue;
        }
        if repo.checkout_prev_branch {
            checkout_prev_branch(&repo, prev_branch_trimmed);
        }

        println!("{}", repo.dir);
    }
}

fn checkout_prev_branch(repo: &RepositoryConfig, prev_branch: &str) {
    let g_co_prev_br = Command::new("git")
        .args(["checkout", prev_branch])
        .stdout(Stdio::null())
        .current_dir(&repo.dir)
        .status()
        .unwrap();
    if !g_co_prev_br.success() {
        eprintln!(
            "git checkout branch={} of dir={} failed: {}",
            repo.branch,
            repo.dir,
            g_co_prev_br.code().unwrap_or(-1),
        );
    }
}
