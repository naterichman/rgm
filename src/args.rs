// CLI parser and app state
use clap::{Parser, Subcommand, ArgEnum};
use std::path::PathBuf;

#[derive(Parser, Debug, ArgEnum, PartialEq, Eq, Clone)]
pub enum ShellType{
    // TODO add other shells
    Zsh,
    Bash,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, name = "rgm")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
    /// Verbose output
    #[clap(short, long)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Tag repo(s).
    #[clap(arg_required_else_help = true)]
    Tag {
        tags: Vec<String>,
        #[clap(required = true, parse(from_os_str))]
        path: PathBuf,
    },
    /// Add alias to repo.
    #[clap(arg_required_else_help = true)]
    Alias {
        alias: String,
        #[clap(required = true, parse(from_os_str))]
        path: PathBuf,
    },
    /// Import git dir(s).
    #[clap(arg_required_else_help = true)]
    Import {
        #[clap(required = true, parse(from_os_str))]
        path: PathBuf,
    },

    /// Update the stored repos (status, remotes, branch, etc.)
    #[clap(arg_required_else_help = true)]
    Update{
        #[clap(required = false, parse(from_os_str))]
        path: Option<PathBuf>
    },

    /// Initialize RGM
    #[clap(arg_required_else_help = true)]
    Init {
        #[clap(required = true, arg_enum)]
        shell: ShellType
    }
}
