use std::thread;
use std::time::Duration;

use std::time::Instant;
use std::fs::read_to_string; // use instead of std::fs::File
use std::path::Path;

use git2::Repository;
mod config;

use crate::config::*;

fn main() {
    let config = Config::new("env.json".to_string());
    println!("{:?}", config);

    let start = Instant::now();
    // let repo = match Repository::clone(&env.repo, "./test") {
    //     Ok(repo) => repo,
    //     Err(e) => panic!("failed to clone: {}", e),
    // };
    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    // let config_json = Path::new("./test/config.json");
    // match Path::exists(config_json) {
    //     e => {
    //         if !e {
    //             println!("None");
    //         } else {
    //             println!("Exists");
    //         }
    //     },
    //     _ => {}
    // }
}