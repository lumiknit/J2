use std::env;

pub struct Config {
  // Clone config
  pub repos_path: String,
  // Find config
  pub find_base_paths: Vec<String>,
  pub ignore_file_path: Option<String>,
  // Jone config
  pub jones_path: String,
}

fn get_var(name: &str) -> String {
  env::var(name).expect(&format!("Please set env ${}", name))
}

fn get_list_var(name: &str) -> Vec<String> {
  get_var(name).split(":").map(|s| s.to_string()).collect()
}

impl Config {
  pub fn from_env() -> Self {
    Self {
      repos_path: get_var("J2_REPOS_PATH"),
      find_base_paths: get_list_var("J2_FIND_BASE_PATHS"),
      ignore_file_path: env::var("J2_IGNORE").ok(),
      jones_path: get_var("J2_JONES_PATH"),
    }
  }
}
