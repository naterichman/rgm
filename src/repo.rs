use git2::{Error as GitError, ErrorCode, Repository, StatusOptions};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;
use walkdir::WalkDir;
use log::{error, info};

use crate::utils::config_file;
use crate::error::{Result, RgmError};

pub enum QueryOpts {
    Name,
    Tags,
    Alias,
    Any,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    //Bare repo,
    Bare,
    // Commits diverged from main remote (origin) TODO: customize main remote?
    // Ahead, Behind
    Diverged(usize, usize),
    // Even, and nothing modified
    Clean,
    // Modified work
    Dirty,
    // In a head detached state
    Detached,
    // TODO: Others? Merge/rebase in progress?
    Other,
}

impl Status {
    pub fn display(&self) -> &str {
        match self {
            Status::Bare => "Empty",
            // Todo
            Status::Diverged(_, _) => "Diverged",
            Status::Clean => "Clean",
            Status::Dirty => "Dirty",
            Status::Detached => "Detached",
            Status::Other => "Other",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repo {
    // Path to repo work dir
    pub path: PathBuf,
    // Repo name
    pub name: String,
    // Currently active branch name
    pub branch: String,
    // Status
    pub status: Option<Status>,
    // List of remotes
    pub remotes: Vec<String>,
    // Single valued alias
    pub alias: Option<String>,
    // List of tags
    pub tags: Vec<String>,
}

impl Repo {
    pub fn new(
        path: PathBuf,
        name: String,
        branch: String,
        status: Option<Status>,
        remotes: Vec<String>,
        alias: Option<String>,
        tags: Vec<String>,
    ) -> Self {
        Self {
            path,
            name,
            branch,
            status,
            remotes,
            alias,
            tags,
        }
    }

    pub fn add_tags(&mut self, add_tags: &Vec<String>) -> bool {
        let mut added = false;
        for tag in add_tags.iter() {
            if !self.tags.contains(&tag) {
                added = true;
                self.tags.push(tag.clone())
            }
        }
        added
    }

    pub fn add_alias(&mut self, alias: String) {
        self.alias = Some(alias)
    }

    pub fn query(&self, query_str: &str, opts: QueryOpts) -> bool {
        match opts {
            QueryOpts::Name => self.name.contains(query_str),
            _ => unimplemented!(),
        }
    }

    pub fn update(&mut self) {
        let raw = match Repository::open(&self.path.as_path()){
            Ok(r) => r,
            Err(msg) => {
                error!("{:?}", msg);
                return
            }
        };
        self.status = get_status(&raw);
        self.remotes = match raw.remotes() {
            Ok(remotes) => remotes.iter()
                .map(|x| x.unwrap().to_string())
                .collect(),
            Err(_) => Vec::new()
        }
    }

    pub fn from_raw(raw: Repository) -> std::result::Result<Self, GitError> {
        let head = raw.head()?;
        let rev = head.shorthand().unwrap();
        
        let status = get_status(&raw);
        let repo_path = raw.workdir().unwrap().to_path_buf();
        let name = String::from(repo_path.as_path().file_name().unwrap().to_str().unwrap());
        Ok(Repo {
            path: repo_path,
            name: name,
            branch: rev.to_string(),
            status: status,
            remotes: raw
                .remotes()?
                .iter()
                .map(|x| x.unwrap().to_string())
                .collect(),
            alias: None,
            tags: vec![],
        })
    }
}

fn get_status(raw: &Repository) -> Option<Status> {
    let mut stat_opts = StatusOptions::new();
    let status = match raw.statuses(Some(&mut stat_opts)) {
        Ok(status_raw) => {
            // TODO: determine between clean, detached, behind, ahead
            if status_raw.is_empty() {
                local_remote_diff(&raw, "origin").ok()
            } else {
                Some(Status::Dirty)
            }
        }
        Err(e) => {
            if e.code() == ErrorCode::BareRepo {
                Some(Status::Bare)
            } else {
                None
            }
        }
    };
    status
}

fn local_remote_diff(
    repo: &Repository,
    remote: &str,
) -> std::result::Result<Status, Box<dyn std::error::Error>> {
    // Get local head
    let head = repo.head()?;
    let local_head = head.peel_to_commit()?;
    let remote = format!("{}/{}", remote, head.shorthand().unwrap());
    // Get remote head
    let remote_head = repo
        .resolve_reference_from_short_name(&remote)?
        .peel_to_commit()?;
    // Get diff with `repo.graph_ahead_behind(local, remote)`
    let (behind, ahead) = repo.graph_ahead_behind(local_head.id(), remote_head.id())?;
    // Set Status:
    if (behind, ahead) == (0, 0) {
        Ok(Status::Clean)
    } else {
        Ok(Status::Diverged(behind, ahead))
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Meta {
    pub size: usize,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Repos {
    pub repos: Vec<Repo>,
    pub meta: Meta,
}


impl Repos {
    pub fn save(&self) -> Result<PathBuf> {
        let file_name = config_file();
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&file_name)
            .map_err(|err| RgmError {
                message: err.to_string(),
            })?;
        let json = serde_json::to_string(&self).map_err(|err| RgmError {
            message: err.to_string(),
        })?;
        file.write(&json.as_bytes()).map_err(|err| RgmError {
            message: err.to_string(),
        })?;
        return Ok(file_name);
    }

    pub fn load() -> Result<Self> {
        let mut file = OpenOptions::new()
            .read(true)
            .open(config_file())
            .map_err(|err| RgmError {
                message: err.to_string(),
            })?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|err| RgmError {
            message: err.to_string(),
        })?;
        let repos: Repos = serde_json::from_str::<Repos>(&contents).map_err(|err| RgmError {
            message: err.to_string(),
        })?;
        Ok(repos)
    }

    pub fn update(&mut self){
        for repo in self.repos.iter_mut() {
            repo.update();
        }
        if let Err(e) = self.save(){
            error!("{:?}", e);
        }
        info!("Updated repos")
    }

    pub fn longest_name(&self) -> usize {
        let mut longest = 0;
        for repo in self.repos.iter() {
            if repo.name.len() > longest {
                longest = repo.name.len()
            }
        }
        longest
    }

    pub fn from_dir(path: &PathBuf) -> Self {
        let mut walker = WalkDir::new(path.as_path()).into_iter();
        let mut repos: Vec<Repo> = Vec::new();
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
                    let raw = Repository::open(entry.path());
                    let repo = Repo::from_raw(raw.unwrap());
                    match repo {
                        Ok(v) => repos.push(v),
                        Err(s) => println!(
                            "Couldn't get repo info at path: {} err: {:?}",
                            entry.path().display(),
                            s
                        ),
                    }
                }
            }
        }
        Repos {
            meta: Meta { size: repos.len() },
            repos,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn empty_repo() -> Repo {
        Repo::new(
            PathBuf::from("/tmp/test"),
            String::from("test"),
            String::from("main"),
            Some(Status::Clean),
            vec![],
            vec![],
            vec![],
        )
    }
    
    #[test]
    fn test_repo_alias(){
        let repo = empty_repo();
        repo.add_alias(String::from("alias"));
        assert_eq!(&repo.alias, String::from("alias"))
    }

    #[test]
    fn test_repo_tags(){
        let repo = empty_repo();
        let mut tags = vec![String::from("tag1"), String::from("tag2")];
        repo.add_tags(&mut tags);
        
        assert_eq!(repo.tags.len(), 2);
        assert!(repo.tags.contains(&String::from("tag1")));
    }
}

