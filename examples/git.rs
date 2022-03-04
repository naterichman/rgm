use git2::{Repository, StatusOptions};
use std::env;
use std::path::PathBuf;

fn help(){
    println!("usage: git [path]\nSimple example to help visualize git2 statuses.")
}

fn handle_git(val: String) {
    let path = PathBuf::from(val).canonicalize().unwrap();
    let repo = Repository::open(path.as_path()).unwrap();
    let mut stat_opts = StatusOptions::new();
    match repo.statuses(Some(&mut stat_opts)) {
        Ok(v) => match v.len() {
            0 => println!("No status"),
            _ => {
                for i in 0..v.len() {
                    let status = v.get(i).unwrap();
                    println!("{:?} {:?}", status.status(), status.path());
                }
            }
        }
        Err(e) => println!("{:?}", e)
    };

}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => help(),
        2 => {
            match args[1].parse() {
                Ok(val) => handle_git(val),
                _ => help()
            }
        },
        _ => help()
    }
}
