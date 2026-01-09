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
    pub target_branch: String,
    #[serde(default = "default_checkout_prev_branch")]
    pub stay_in_target_branch: bool,
}

fn default_num_threads() -> u8 {
    4
}

fn default_checkout_prev_branch() -> bool {
    false
}

impl Config {
    /// example returns example config.
    pub fn example() -> Config {
        Config {
            num_threads: 4,
            repositories: vec![RepositoryConfig {
                dir: String::from("path-to-my-repo"),
                target_branch: String::from("main"),
                stay_in_target_branch: false,
            }],
        }
    }
}
