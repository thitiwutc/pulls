use std::{
    fs,
    io::stdout,
    process::{Command, Stdio},
    sync::Arc,
    thread,
};

use clap::Parser;

mod config;
mod semaphore;
use crate::{
    config::{Config, RepositoryConfig},
    semaphore::Semaphore,
};

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
    let sem = Arc::new(Semaphore::new(cfg.num_threads));
    let mut handles = vec![];

    for repo in cfg.repositories {
        let sem = Arc::clone(&sem);

        handles.push(thread::spawn(move || {
            sem.acquire();

            println!("{}: git branch --show-current", &repo.dir);
            let prev_branch = String::from_utf8(
                Command::new("git")
                    .args(["branch", "--show-current"])
                    .current_dir(&repo.dir)
                    .output()
                    .unwrap()
                    .stdout,
            )
            .unwrap();
            let prev_branch_trimmed = prev_branch.trim();

            // Only checkout if we are not on the target branch already.
            if prev_branch_trimmed != repo.target_branch {
                println!("{}: git checkout", &repo.dir);
                let g_co_status = Command::new("git")
                    .args(["checkout", &repo.target_branch])
                    .stdout(Stdio::null())
                    .current_dir(&repo.dir)
                    .status()
                    .unwrap();
                if !g_co_status.success() {
                    eprintln!(
                        "git checkout branch={} of dir={} failed: {}",
                        repo.target_branch,
                        repo.dir,
                        g_co_status.code().unwrap_or(-1),
                    );
                }
            }

            println!("{}: git pull", &repo.dir);
            let g_pl_status = Command::new("git")
                .arg("pull")
                .stdout(Stdio::null())
                .current_dir(&repo.dir)
                .status()
                .unwrap();
            if !g_pl_status.success() {
                eprintln!(
                    "git pull branch={} of dir={} failed: {}",
                    repo.target_branch,
                    repo.dir,
                    g_pl_status.code().unwrap_or(-1),
                );
                checkout_prev_branch(&repo, prev_branch_trimmed);
            }
            if !repo.stay_in_target_branch {
                checkout_prev_branch(&repo, prev_branch_trimmed);
            }

            println!("{} ✅", repo.dir);
            sem.release();
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
}

fn checkout_prev_branch(repo: &RepositoryConfig, prev_branch: &str) {
    // Don't checkout again if we already in the prev_branch.
    if repo.target_branch == prev_branch {
        return;
    }

    let g_co_prev_br = Command::new("git")
        .args(["checkout", prev_branch])
        .stdout(Stdio::null())
        .current_dir(&repo.dir)
        .status()
        .unwrap();
    if !g_co_prev_br.success() {
        eprintln!(
            "git checkout branch={} of dir={} failed: {}",
            repo.target_branch,
            repo.dir,
            g_co_prev_br.code().unwrap_or(-1),
        );
    }
}
