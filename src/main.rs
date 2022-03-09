use args::{Cli, Commands};
use clap::Parser;
use repo::Repos;
use std::process;

mod args;
mod repo;
mod error;

fn usage(){
    println!("rgm PATH")
}

fn get_repos_or_exit() -> Repos {
    let repos = Repos::load();
    match repos {
        Ok(r) => r,
        Err(_) => process::exit(1)
    }
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(command) => match command {
            Commands::Tag{mut tags, path} => {
                let mut repos = get_repos_or_exit();
                for r in repos.repos.iter_mut(){
                    // if r.path is a subdirectory of path
                    if r.path.starts_with(&path){
                        r.add_tags(&mut tags);
                    }
                }
            },
            Commands::Alias{alias, path} => {
                let mut repos = get_repos_or_exit();
                // Match on path
                for r in repos.repos.iter_mut() {
                    if r.path == path {
                        println!("Adding alias {} to {}",&alias,&r.name);
                        r.add_alias(alias);
                        break
                    }
                }
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
