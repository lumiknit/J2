use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum Command {
  #[clap(about = "Initialization script for shell")]
  ShellInit {
    /// Type of shell
    #[clap(help = "Type of shell (sh, pwsh)")]
    shell: Option<String>,
  },

  #[clap(about = "Execute fuzzy find")]
  Find {
    /// Query string
    query: Vec<String>,

    /// Base paths to search.
    /// If not specified, use J2_FIND_BASE_PATHS
    #[clap(short, long)]
    base: Vec<String>,

    /// Without GUI, just pick the first match
    #[clap(short = '1', long)]
    first: bool,

    /// Allow (non-directory) files to be included
    #[clap(short, long)]
    files: bool,

    /// Allow hidden files to be included
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

#[derive(Parser)]
#[command(author, version, about)]
#[command(propagate_version = true)]
pub struct Cli {
  #[clap(subcommand)]
  pub command: Command,
}
