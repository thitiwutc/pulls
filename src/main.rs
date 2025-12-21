use std::fs;

mod config;

fn main() {
    let cfg_file_content = fs::read("./config.yaml").unwrap();

    let cfg = serde_yaml::from_slice::<config::Config>(&cfg_file_content).unwrap();
    println!("cfg: {cfg:?}");
}
