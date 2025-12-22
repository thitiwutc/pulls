use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_num_threads")]
    pub num_threads: u8,
    pub repositories: Vec<RepositoryConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepositoryConfig {
    pub dir: String,
    pub branch: String,
    #[serde(default = "default_checkout_prev_branch")]
    pub checkout_prev_branch: bool,
}

fn default_num_threads() -> u8 {
    4
}

fn default_checkout_prev_branch() -> bool {
    true
}

impl Config {
    /// example returns example config.
    pub fn example() -> Config {
        Config {
            num_threads: 4,
            repositories: vec![RepositoryConfig {
                dir: String::from("path-to-my-repo"),
                branch: String::from("main"),
                checkout_prev_branch: true,
            }],
        }
    }
}
