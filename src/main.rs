/* luminkit's jump helper 2
 * Author: lumiknit (aasr4r4@gmail.com)
 * Version: 0.2.0-dev (240216)
 */

use std::process::{exit, Command};
use std::{env, fs, path, vec};

pub mod cli;
pub mod config;
pub mod section;
pub mod sh_init;

use config::Config;
use section::JoneSection;

fn get_executable_path(exe: &str) -> Option<String> {
  // Convert relative path to absolute path
  path::Path::new(exe)
    .canonicalize()
    .ok()
    .and_then(|p| p.to_str().map(|s| s.to_string()))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Dir {
  path: String,
  name: String,
  loss: u32,
}

fn edit_distance(path: &str, query: &str) -> u32 {
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
  d[(q.len() + 1) % 2].iter().max().unwrap_or(&0).clone() + 4096
    - (p.len() as u32)
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
  let path_name = filename.to_lowercase();
  let mut query_chars = query.char_indices().peekable();
  let p = query_chars.peek();
  if p.is_some() && p.unwrap().1 == '/' {
    query_chars.next();
  }
  for c in path_name.chars() {
    if let Some(qc) = query_chars.peek() {
      if c == qc.1 {
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
      loss: edit_distance(
        path.strip_prefix(root).unwrap_or(path),
        original_query,
      ),
    });
  } else {
    let new_query = &query[query_chars.peek().unwrap().0..];
    // Find recursively
    if let Ok(entries) = dir.read_dir() {
      // Check directory contains .git
      let mut has_git = false;
      for entry in dir.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name();
        if file_name.is_some() && file_name.unwrap().to_str().unwrap() == ".git"
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
          gather_directories(
            config,
            result,
            root,
            original_query,
            new_query,
            &path,
          );
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

fn clone(config: &Config, repo_url: &str, depth: Option<u32>) {
  // Split repo_url by protocal
  let url_clone = repo_url;
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
  let mut cmd = Command::new("git");
  let mut cmd = cmd
    .arg("clone")
    .arg(repo_url)
    .arg(format!("{}/{}", config.repos_path, splitted[1]));
  if let Some(d) = depth {
  	cmd = cmd.arg(format!("--depth={}", d));
  }
  cmd.spawn()
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

const EMPTY_JONE_NAME: &str = "_";

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
  match args[1].as_str() {
    "find" => {
      let config = Config::from_env();
      let query = args[2..].join("").to_lowercase();
      let result = find_path(&config, query.as_str());
      for path in result {
        println!("{}", path);
      }
    }
    _ => unreachable!(),
  }
}

fn cmd_shell_init() {
  // Get args
  let args: Vec<String> = env::args().collect();
  let exe = get_executable_path(args[0].as_str()).unwrap_or(String::from("j2"));
  let s = sh_init::SH_INIT.replace("<EXECUTABLE_PATH>", exe.as_str());
  println!("{}", s.trim());
}

fn cmd_clone(url: &String, depth: Option<u32>) {
	let config = Config::from_env();
	clone(&config, url, depth);
}

fn cmd_jone_new(name: &String) {
  let config = Config::from_env();
  let p = jone_new(&config, name);
  println!("{}", p);
}

fn cmd_jone_list() {
  let config = Config::from_env();
  jone_list(&config);
}

fn cmd_jone_section_list(name: &String) {
  let config = Config::from_env();
  let list = jone_section_list(&config, name);
  for section in list {
    println!("{}", section.to_string());
  }
}

fn cmd_jone_latest(name: &String) {
  let config = Config::from_env();
  let list = jone_section_list(&config, name);
  if list.len() > 0 {
    println!("{}/{}/{}", config.jones_path, name, list[0].to_string());
  } else {
    let p = jone_new(&config, name);
    println!("{}", p);
  }
}

fn name_list_to_string(name: &Vec<String>) -> String {
  let joined = name.join(" ");
  let trimmed = joined.trim();
  if trimmed.len() <= 0 {
    EMPTY_JONE_NAME.to_string()
  } else {
    trimmed.to_string()
  }
}

fn main() {
  // Parse command line arguments
  let parsed_command = cli::parse_command();
  match parsed_command.command {
    cli::Command::ShellInit => cmd_shell_init(),
    cli::Command::Clone {
    	url, depth } => cmd_clone(&url, depth),
    cli::Command::JoneList => cmd_jone_list(),
    cli::Command::JoneNew { name } => {
      cmd_jone_new(&name_list_to_string(&name));
    }
    cli::Command::JoneSections { name } => {
      cmd_jone_section_list(&name_list_to_string(&name));
    }
    cli::Command::JoneLatest { name } => {
      cmd_jone_latest(&name_list_to_string(&name));
    }
    _ => unimplemented!(),
  }
}
