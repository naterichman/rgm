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
                repos.save();
                println!("Saved repos to")
            },
        },
        None => {
            let repos = Repos::load();
            match repos {
                Ok(r) => {
                    for val in r.repos.iter() {
                        println!("{:?}", val)
                    }
                },
                Err(e) => println!("{:?}", e)
            }
        }
    }
}
