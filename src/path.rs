// Path structure

use std::collections::HashSet;

#[derive(Clone)]
pub struct PathItem {
  // Displayed path (also used for query)
  pub displayed: String,

  // Absolute path (real paths)
  pub abs: String,
}

pub fn convert_base_paths_to_names(base_paths: &Vec<String>) -> Vec<PathItem> {
  let mut set = HashSet::new();
  let mut paths = Vec::new();
  for base_path in base_paths {
    let last_name = base_path
      .split("/")
      .map(|s| s.trim())
      .filter(|b| !b.is_empty())
      .last()
      .unwrap();
    let mut name = last_name.to_string();
    let mut c = 0;

    while set.contains(&name) {
      name = format!("{}-{}", last_name, c);
      c += 1;
    }
    set.insert(name.clone());

    paths.push(PathItem {
      displayed: name,
      abs: base_path.clone(),
    });
  }

  paths
}
