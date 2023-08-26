use std::env;

pub struct Config {
  // Clone config
  pub repos_path: String,
  // Find config
  pub find_base_paths: Vec<String>,
  pub ignores: Vec<String>,
  // Jone config
  pub jones_path: String,
}

impl Config {
  pub fn from_env() -> Self {
    let repos_path =
      env::var("J2_REPOS_PATH").expect("Please set env $J2_REPOS_PATH");
    let find_base_paths = env::var("J2_FIND_BASE_PATHS")
      .expect("Please set env $J2_FIND_BASE_PATHS")
      .split(":")
      .map(|s| s.to_string())
      .collect();
    let ignores = env::var("J2_IGNORES")
      .expect("Please set env $J2_IGNORES")
      .split(":")
      .map(|s| s.to_string())
      .collect();
    let jones_path =
      env::var("J2_JONES_PATH").expect("Please set env $J2_JONES_PATH");
    Self {
      repos_path,
      find_base_paths,
      ignores,
      jones_path,
    }
  }
}
