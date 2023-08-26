/* luminkit's jump helper 2
 * Author: lumiknit (aasr4r4@gmail.com)
 * Version: 0.0.1 (230825)
 */

use std::{env, fs, path, vec};
use std::process::{exit, Command};

pub mod config;
pub mod section;
pub mod sh_init;
pub mod help_msg;

use config::Config;
use section::JoneSection;

const VERSION: &str = "0.0.1";

fn get_executable_path(exe: &str) -> Option<String> {
  // Convert relative path to absolute path
  path::Path::new(exe)
    .canonicalize()
    .ok()
    .and_then(|p| p.to_str().map(|s| s.to_string()))
}

fn print_version() {
  println!("lumiknit's jump helper v{}", VERSION);
}

fn print_init(exe: &str) {
  let exe = get_executable_path(exe).unwrap_or(String::from("j2"));
  let s = sh_init::SH_INIT.replace("<EXECUTABLE_PATH>", exe.as_str());
  println!("{}", s.trim());
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Dir {
  path: String,
  name: String,
  loss: u32,
}

fn edit_distance(
  path: &str,
  query: &str,
) -> u32 {
  let p = path.as_bytes();
  let q = query.as_bytes();
  let mut d = vec![vec![0; p.len()]; 2];
  for i in 0..p.len() {
    d[1][i] = 0 as u32;
  }
  for i in 0..q.len() {
    let qc = q[i];
    let qp = if i == 0 { 1 } else { q[i - 1] };
    for j in 0..p.len() {
      let pc = p[j];
      let pp = if j == 0 { 0 } else { p[j - 1] };
      let mut costs = vec![0];
      if qc == pc {
        if j == 0 {
          costs.push(200);
        } else {
          costs.push(d[(i + 1) % 2][j - 1] + if qp == pp { 200 } else { 100 });
        }
      }
      d[i % 2][j] = *costs.iter().max().unwrap();
    }
  }
  d[(q.len() + 1) % 2].iter().max().unwrap_or(&0).clone() + 4096 - (p.len() as u32)
}

fn gather_directories(
  config: &Config,
  result: &mut Vec<Dir>,
  root: &str,
  original_query: &str,
  query: &str,
  dir: &path::Path,
) {
  if !dir.is_dir() {
    return;
  }
  let filename = dir.file_name().unwrap().to_str().unwrap();
  if config.ignores.contains(&filename.to_string()) {
    return;
  }
  let path_name: String = format!("/{}", filename.to_lowercase());
  let mut query_chars = query.chars().peekable();
  for c in path_name.chars() {
    if let Some(qc) = query_chars.peek() {
      if c == *qc {
        query_chars.next();
      }
    }
  }
  if query_chars.peek().is_none() {
    // Found!
    let path = dir.to_str().unwrap();
    result.push(Dir {
      path: path.to_string(),
      name: filename.to_string(),
      loss: edit_distance(path.strip_prefix(root).unwrap_or(path), original_query),
    });
  } else {
    let new_query = query_chars.collect::<String>();
    // Find recursively
    if let Ok(entries) = dir.read_dir() {
      // Check directory contains .git
      let mut has_git = false;
      for entry in dir.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name();
        if file_name.is_some()
          && file_name.unwrap().to_str().unwrap() == ".git"
        {
          has_git = true;
          break;
        }
      }
      if has_git {
        return;
      }
      for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name();
        let hidden = file_name.is_some()
          && file_name.unwrap().to_str().unwrap().starts_with(".");
        if path.is_dir() && !hidden {
          gather_directories(config, result, root, original_query, new_query.as_str(), &path);
        }
      }
    }
  }
}

fn find_path(config: &Config, search: &str) -> Vec<String> {
  let mut result: Vec<Dir> = vec![];
  for base_path in &config.find_base_paths {
    let path = path::Path::new(base_path);
    gather_directories(config, &mut result, base_path, search, search, path);
  }
  result.sort_by(|a, b| b.loss.cmp(&a.loss));
  result.iter().map(|d| d.path.clone()).collect()
}

fn clone(config: &Config, repo_url: &str) {
  // Split repo_url by protocal
  let url_clone = repo_url.clone();
  let splitted: Vec<&str> = url_clone.split("://").collect();
  if splitted.len() != 2 {
    println!("Invalid repo url: {}", repo_url);
    exit(1)
  }
  // Mkdir
  fs::create_dir_all(path::Path::new(&format!(
    "{}/{}",
    config.repos_path, splitted[1]
  )))
  .expect("Failed to create directory");
  // Clone repo
  Command::new("git")
    .arg("clone")
    .arg(repo_url)
    .arg(format!("{}/{}", config.repos_path, splitted[1]))
    .spawn()
    .expect("Failed to clone repo")
    .wait()
    .unwrap();
}

fn jone_list(config: &Config) {
  // Read jone directories
  let path = path::Path::new(&config.jones_path);
  if let Ok(entries) = path.read_dir() {
    for entry in entries {
      let file_name = entry.ok().and_then(|e| e.file_name().into_string().ok());
      if file_name.is_none() {
        continue;
      }
      let file_name = file_name.unwrap();
      if file_name.starts_with(".") {
        continue;
      }
      println!("{}", file_name);
    }
  }
}

fn empty_jone_name() -> String {
  String::from("_")
}

fn canonicalize_jone_name(name: &str) -> String {
  // Convert to lowercase and replace whitespaces into underscores
  let mut result = String::new();
  for c in name.chars() {
    if c.is_whitespace() {
      result.push('_');
    } else {
      result.push(c.to_lowercase().next().unwrap());
    }
  }
  result
}

fn jone_new(config: &Config, name: &str) -> String {
  let name = canonicalize_jone_name(name);
  let jone_path = format!("{}/{}", config.jones_path, name);
  // Make directory
  fs::create_dir_all(&jone_path).expect("Failed to create directory");
  // Create jone file
  let section_name = JoneSection::gen().to_base36();
  let section_path = format!("{}/{}", jone_path, section_name);
  fs::create_dir(path::Path::new(&section_path))
    .expect("Failed to create directory");
  section_path
}

fn jone_section_list(config: &Config, name: &str) -> vec::Vec<JoneSection> {
  let name = canonicalize_jone_name(name);
  let jone_path = format!("{}/{}", config.jones_path, name);
  // Read jone directories
  let path = path::Path::new(&jone_path);
  let entries = path.read_dir();
  if entries.is_err() {
    return vec![];
  }
  let entries = entries.unwrap();
  let mut list = vec![];
  for entry in entries {
    if entry.is_err() {
      continue;
    }
    let entry = entry.unwrap();
    let file_name = entry.file_name().into_string();
    if file_name.is_err() {
      continue;
    }
    let file_name = file_name.unwrap();
    let created = entry.metadata().and_then(|m| m.created()).ok();
    if let Some(section) = JoneSection::from_str(file_name.as_str(), created) {
      list.push(section);
    }
  }
  list.sort_by(|a, b| b.cmp(a));
  list
}

fn run(args: Vec<String>) {
  let l = args.len();
  if l <= 1 {
    help_msg::print_help();
    return;
  }
  match args[1].as_str() {
    "version" => {
      print_version();
    }
    "init" => {
      print_init(args[0].as_str());
    }
    "find" => {
      let config = Config::from_env();
      let query = args[2..].join("").to_lowercase();
      let result = find_path(&config, query.as_str());
      for path in result {
        println!("{}", path);
      }
    }
    "clone" => {
      if args.len() < 3 {
        println!("Usage: J2 clone <REPO_URL>");
        exit(1);
      }
      let url = args[2..].join("");
      let config = Config::from_env();
      clone(&config, url.as_str());
    }
    "jone-new" => {
      let config = Config::from_env();
      let name = if args.len() < 3 {
        empty_jone_name()
      } else {
        args[2..].join(" ")
      };
      let p = jone_new(&config, name.as_str());
      println!("{}", p);
    }
    "jone-list" => {
      let config = Config::from_env();
      jone_list(&config);
    }
    "jone-sections" => {
      let config = Config::from_env();
      let name = if args.len() < 3 {
        empty_jone_name()
      } else {
        args[2..].join(" ")
      };
      let list = jone_section_list(&config, name.as_str());
      for section in list {
        println!("{}", section.to_string());
      }
    }
    "jone-latest" => {
      let config = Config::from_env();
      let name = if args.len() < 3 {
        empty_jone_name()
      } else {
        args[2..].join(" ")
      };
      let list = jone_section_list(&config, name.as_str());
      if list.len() > 0 {
        println!("{}/{}/{}", config.jones_path, name, list[0].to_string());
      } else {
        let p = jone_new(&config, name.as_str());
        println!("{}", p);
      }
    }
    _ => {
      help_msg::print_help();
    }
  }
}

fn main() {
  run(env::args().collect());
}
