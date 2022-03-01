use crate::args::Args;
use std::path::PathBuf;

#[derive(Debug)]
pub struct State {
    pub path: Option<PathBuf>,
    pub verbose: bool,
}

impl From<Args> for State {
    fn from(args: Args) -> Self {
        let path = args
            .path
            .map(|val| PathBuf::from(val).canonicalize().unwrap());
        Self {
            path,
            verbose: args.verbose,
        }
    }
}
