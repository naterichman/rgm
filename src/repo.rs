use git2::{Repository, Error as GitError, StatusOptions, ErrorCode};
use std::path::PathBuf;
use std::fmt;
use std::convert;


#[derive(Debug)]
pub enum Status {
    // Commits ahead of main remote (origin) TODO: customize main remote?
    Ahead,
    //Bare repo,
    Bare,
    // Commits behind main remote
    Behind,
    // Even, and nothing modified
    Clean,
    // Modified work
    Dirty,
    // In a head detached state
    Detached,
    // TODO: Others? Merge/rebase in progress?
    Other
}

#[derive(Debug)]
pub struct Repo {
    // Path to repo work dir
    pub path: PathBuf,
    // Repo name
    pub name: String,
    // Currently active branch name
    pub branch: String,
    // Status
    pub status: Status,
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
        status: Status,
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
                if status_raw.is_empty(){ Status::Clean }
                else { Status::Dirty }
            }
            Err(e) => {
                if e.code() == ErrorCode::BareRepo { Status::Bare }
                return Err(e)
            }
        }
        unimplemented!()
    }

}

impl fmt::Display for Repo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Proper display
        write!(f, "{}", self.path.display())
    }
}
