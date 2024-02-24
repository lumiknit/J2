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
    let mut ln_iter = last_name.chars();
    let mut name = "".to_string();

    loop {
      let c = if let Some(c) = ln_iter.next() { c } else { '_' };
      name.push(c);

      if !set.contains(&name) {
        break;
      }
    }

    set.insert(name.clone());

    paths.push(PathItem {
      displayed: name,
      abs: base_path.clone(),
    });
  }

  paths
}
