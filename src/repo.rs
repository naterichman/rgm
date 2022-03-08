use git2::{Repository, Error as GitError, StatusOptions, ErrorCode};
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use std::fmt;
use std::convert;
use serde::{Deserialize, Serialize};
use crate::error::{Result,RgmError};
use walkdir::WalkDir;

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
    Other
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
        tags: Vec<String>
    ) -> Self {
        Self {
            path,
            name,
            branch,
            status,
            remotes,
            alias,
            tags
        }
    }

    pub fn update(&mut self) {
        // TODO: fetch remotes, branch, status from repo by path
        let repo_raw = Repository::open(&self.path.as_path());
        unimplemented!()
    }

    pub fn add_tags(&mut self, tags: &mut Vec<String>) {
        self.tags.append(tags);
    }

    pub fn add_alias(&mut self, alias: String) {
        self.alias = Some(alias)
    }
}

impl convert::TryFrom<Repository> for Repo {
    type Error = GitError;
    fn try_from(raw: Repository) -> std::result::Result<Self, Self::Error> {
        let head = raw.head()?;
        let rev = head.shorthand().unwrap();

        let mut stat_opts = StatusOptions::new();
        let status = match raw.statuses(Some(&mut stat_opts)) {
            Ok(status_raw) => {
                // TODO: determine between clean, detached, behind, ahead 
                if status_raw.is_empty(){
                    local_remote_diff(&raw, "origin").ok()
                }
                else { Some(Status::Dirty) }
            }
            Err(e) => {
                if e.code() == ErrorCode::BareRepo { Some(Status::Bare) }
                else { None }
            }
        };
        let repo_path = raw.workdir().unwrap().to_path_buf();
        let name = String::from(repo_path
            .as_path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap());
        Ok(Repo {
            path: repo_path,
            name: name,
            branch: rev.to_string(),
            status: status,
            remotes: raw.remotes()?.iter().map(|x| x.unwrap().to_string()).collect(),
            alias: None,
            tags: vec![]
        })
    }
}

impl fmt::Display for Repo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Proper display
        write!(f, "{}", self.path.display())
    }
}

fn local_remote_diff(repo: &Repository, remote: &str) -> std::result::Result<Status, Box<dyn std::error::Error>> {
    // Get local head
    let head = repo.head()?;
    let local_head = head.peel_to_commit()?;
    let remote = format!("{}/{}", remote, head.shorthand().unwrap());
    // Get remote head
    let remote_head = repo.resolve_reference_from_short_name(&remote)?
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

#[derive(Deserialize, Serialize)]
pub struct Repos(Vec<Repo>);

impl Repos {
    pub fn save(&self, path: &PathBuf) -> Result<()> {
        let mut file = File::create(path)
            .map_err(|err| RgmError { message: err.to_string() })?;
        let json = serde_json::to_string(&self.0)
            .map_err(|err| RgmError { message: err.to_string() })?;
        file.write(&json.as_bytes())
            .map_err(|err| RgmError { message: err.to_string() });
        return Ok(())
    }
    
    pub fn load(path: &PathBuf) -> Result<Self> {
        let mut file = File::open(path)
            .map_err(|err| RgmError { message: err.to_string() })?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|err| RgmError { message: err.to_string() })?;
        let repos: Repos = serde_json::from_str(&contents)
            .map_err(|err| RgmError { message: err.to_string() })?;
        Ok(repos)
    }
}

impl From<&PathBuf> for Repos {
    fn from(path: &PathBuf) -> Self {
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
                    let repo = Repo::try_from(raw.unwrap());
                    match repo {
                        Ok(v) => repos.push(v),
                        Err(s) => println!("Couldn't get repo info at path {:?}", s),
                    }
                }
            }
        }
        Repos(repos)
    }
}
