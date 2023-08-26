pub static HELP_MSG: &str = r#"
luminkit's jump helper 2
Usage: J2 <COMMAND> [ARGS]
Commands:
  help: Print this help message
  version: Print the version
  init: Print the initialization script
  find <QUERY>: Find a directory
  clone <REPO_URL>: Clone a git repository
  jone-new [<NAME>]: Create a new jone (j-zone)
  jone-list: List jones
  jone-sections [<NAME>]: List sections in the jone
  jone-latest [<NAME>]: Print the latest section path in the jone. If no section exists, create a new one.
Environment variables:
  J2_REPOS_DIR: The directory where git repositories are stored
  J2_FIND_BASE_PATHS: The base paths to find directories (separated by ':')
  J2_IGNORES: The directories to ignore when finding (separated by ':')
  J2_JONE_PATH: The path to store jone files
  J2_EDITOR: The command name of editor to edit jone notes (e.g. vi)
"#;

pub fn print_help() {
  println!("{}", HELP_MSG.trim());
}