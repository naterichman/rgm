use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    pub path: Option<String>,

    #[clap(short, long)]
    pub verbose: bool,
}
