mod args;
mod repo;
mod state;
use args::Args;
use clap::Parser;
use git2::Repository;
use state::State;
use std::path::PathBuf;
use walkdir::WalkDir;

fn main() {
    let args = Args::parse();
    let state = State::from(args);
    if let Some(val) = state.path {
        let targets = generate_repos(val);
        for target in targets {
            let repo = Repository::open(target.as_path());
            match repo {
                Ok(v) => println!("{} {:?}", v.workdir().unwrap().display(), v.state()),
                Err(s) => println!("Couldn't get repo info at path {:?}", target),
            }
        }
    } else {
        unimplemented!();
    }
}

fn generate_repos(path: PathBuf) -> Vec<PathBuf> {
    println!("Looking at {:?}", path);
    let mut walker = WalkDir::new(path.as_path()).into_iter();
    let mut targets: Vec<PathBuf> = Vec::new();
    loop {
        let entry = match walker.next() {
            None => break,
            Some(Err(_)) => unimplemented!(), //Eventually debug log
            Some(Ok(entry)) => entry,
        };
        let ft = entry.file_type();
        // Don't care about files
        if ft.is_file() {
            continue;
        } else {
            // Skip hidden directories
            if entry
                .file_name()
                .to_str()
                .map(|s| s.starts_with('.'))
                .unwrap_or(false)
            {
                walker.skip_current_dir();
                continue;
            }
            let g_dir = entry.path().join(".git");
            // Found a git subdirectory, no need to recurse in this dir anymore.
            if g_dir.exists() && g_dir.is_dir() {
                walker.skip_current_dir();
                targets.push(entry.path().to_path_buf())
            }
        }
    }
    targets
}
