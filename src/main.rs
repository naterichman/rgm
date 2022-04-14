use args::{Cli, Commands, ShellType};
use clap::Parser;
use logging::setup_log;
use log::{info, error};
use std::{io, fs};

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
    utils::clear_shell_file();
    let cli = Cli::parse();
    match cli.command {
        Some(command) => match command {
            Commands::Tag { mut tags, path } => {
                let path = match fs::canonicalize(path) {
                    Ok(p) => p,
                    Err(e) => {
                        println!("Could not parse input {:?}", e);
                        return
                    }
                };
                println!("Adding tags {:?} to repos in {}", &tags, path.display());
                let mut repos = utils::get_repos_or_exit();
                let mut applied = 0;
                for r in repos.repos.iter_mut() {
                    // if r.path is a subdirectory of path
                    if r.path.starts_with(&path) {
                        if r.add_tags(&mut tags){
                            applied += 1;
                        }
                    }
                }
                println!("Applied tags to {} repos, saving", applied);
                println!("{:?}", repos);
                if let Err(e) = repos.save(){
                    println!("{:?}", e);
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
                if let Err(e) = repos.save(){
                    println!("{:?}", e);
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
            },
            Commands::Init { shell } => {
                match shell {
                    ShellType::Zsh => utils::zsh_init(),
                    ShellType::Bash => utils::bash_init(),
                    _ => {}
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
