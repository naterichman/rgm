use git2::{Repository, Error as GitError, StatusOptions, ErrorCode};
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use std::fmt;
use std::convert;
use serde::Serialize;
use crate::error::{Result,RgmError};

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
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
    fn try_from(raw: Repository) -> Result<Self, Self::Error> {
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

fn local_remote_diff(repo: &Repository, remote: &str) -> Result<Status, Box<dyn std::error::Error>> {
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

#[derive(Serialize)]
pub struct Repos(Vec<Repo>);

impl Repos {
    pub fn save(&self, path: PathBuf) -> Result<()> {
        let mut file = File::create(path)
            .map_err(|err| RgmError { message: err.to_string() })?;
        let json = serde_json::to_string(&self.0)
            .map_err(|err| RgmError { message: err.to_string() })?;
        file.write(&json.as_bytes())
            .map(|_| ())
            .map_err(|err| RgmError { message: err.to_string() })
    }
}

impl From<Vec<PathBuf>> for Repos {
    fn from(paths: Vec<PathBuf>) -> Self {
        unimplemented!()
    }
}
