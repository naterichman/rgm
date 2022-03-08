use args::{Cli, Commands};
use clap::Parser;
use repo::Repos;
use std::path::PathBuf;

mod args;
mod repo;
mod error;

fn usage(){
    println!("rgm PATH")
}


fn main() {
    let cli = Cli::parse();
    let cache_path = PathBuf::from("~/.rgm.json").canonicalize().unwrap();
    match cli.command {
        Some(command) => match command {
            Commands::Tag{tags, path} => {
                unimplemented!()
            },
            Commands::Alias{alias, path} => {
                unimplemented!()
            },
            Commands::Import{path} => {
                let repos = Repos::from(&path);
                repos.save(&cache_path);
                println!("Saved repos to {}", cache_path.display())
            },
        },
        None => {
            // List git directories,
            println!("List git repos")
        }
    }
}
