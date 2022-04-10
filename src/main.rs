use args::{Cli, Commands};
use clap::Parser;
use logging::setup_log;
use log::{info, error};
use std::io;

use crate::repo::Repos;
use crate::screen::Screen;

mod args;
mod error;
mod input;
mod logging;
mod repo;
mod repoitem;
mod repoview;
mod screen;
pub mod sorting;
mod utils;

fn usage() {
    println!("rgm PATH")
}

fn main() {
    setup_log().unwrap();
    log::info!("Set up logging");
    let cli = Cli::parse();
    match cli.command {
        Some(command) => match command {
            Commands::Tag { mut tags, path } => {
                let mut repos = utils::get_repos_or_exit();
                for r in repos.repos.iter_mut() {
                    // if r.path is a subdirectory of path
                    if r.path.starts_with(&path) {
                        r.add_tags(&mut tags);
                    }
                }
            }
            Commands::Alias { alias, path } => {
                let mut repos = utils::get_repos_or_exit();
                // Match on path
                for r in repos.repos.iter_mut() {
                    if r.path == path {
                        println!("Adding alias {} to {}", &alias, &r.name);
                        r.add_alias(alias);
                        break;
                    }
                }
            }
            Commands::Import { path } => {
                let repos = Repos::from_dir(&path);
                match repos.save() {
                    Ok(p) => println!("Saved {} repos to {}", &repos.meta.size, p.display()),
                    Err(e) => println!("Error saving repos: {}", e),
                }
            },
            Commands::Update { path } => {
                let repos = Repos::load();
                match repos {
                    // TODO progress bar for updating
                    Ok(mut r) => {
                        if let Some(p) = path {
                            // TODO: Update only paths in given directory
                            unimplemented!()
                        } else {
                            r.update();
                        }
                    },
                    Err(e) => error!("{:?}", e),
                }
            }
        },
        None => {
            let repos = Repos::load();
            match repos {
                Ok(r) => {
                    //r.update();
                    let mut screen = Screen::new(r);
                    let mut out = io::stdout();
                    if let Err(e) = screen.run(out) {
                        error!("{:?}", e)
                    }
                }
                Err(e) => error!("{:?}", e),
            }
        }
    }
}
