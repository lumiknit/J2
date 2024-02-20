use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Command {
  #[clap(about = "Initialization script for shell")]
  ShellInit,

  #[clap(about = "Execute fuzzy find")]
  Find {
    /// Query string
    query: Vec<String>,

    /// Without GUI, just pick the first match
    #[clap(short, long)]
    first: bool,

    /// Find all hidden files
    #[clap(short, long)]
    all: bool,
  },

  #[clap(about = "Clone a repository")]
  Clone {
    /// URL of git remote repository
    url: String,

    /// Depth of the clone
    #[clap(short, long)]
    depth: Option<u32>,
  },

  #[clap(about = "Create a new jone")]
  JoneNew {
    /// Name of the jone
    name: Vec<String>,
  },

  #[clap(about = "List all jones")]
  JoneList,

  #[clap(about = "List sections in the jone")]
  JoneSections {
    /// Name of the jone
    name: Vec<String>,
  },

  #[clap(about = "Show the latest section in the jone")]
  JoneLatest {
    /// Name of the jone
    name: Vec<String>,
  },
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(propagate_version = true)]
pub struct Cli {
  #[clap(subcommand)]
  pub command: Command,
}

pub fn parse_command() -> Cli {
  Cli::parse()
}
