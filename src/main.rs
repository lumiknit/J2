/* luminkit's jump helper 2
 * Author: lumiknit (aasr4r4@gmail.com)
 * Version: 0.2.0-dev (240216)
 */

use std::process::{exit, Command};
use std::{env, fs, path, vec};

pub mod cli;
pub mod config;
pub mod fuzzy;
pub mod section;
pub mod sh_init;
pub mod ui_finder;

use config::Config;
use section::JoneSection;

fn get_executable_path(exe: &str) -> Option<String> {
  // Convert relative path to absolute path
  path::Path::new(exe)
    .canonicalize()
    .ok()
    .and_then(|p| p.to_str().map(|s| s.to_string()))
}

fn gather_all_paths(all: bool) -> Vec<String> {
  let config = Config::from_env();

  // Traverse all directories and gather paths
  let paths = std::sync::Mutex::new(vec![]);

  for base in config.find_base_paths.iter() {
    let mut builder = ignore::WalkBuilder::new(base);
    builder.standard_filters(true).hidden(!all);
    builder.build_parallel().run(|| {
      Box::new(|result| {
        if let Ok(entry) = result {
          let path = entry.path();
          if path.is_dir() {
            let mut paths = paths.lock().unwrap();
            paths.push(path.to_str().unwrap().to_string());
          }
        }
        ignore::WalkState::Continue
      })
    });
  }

  // Destruct paths from mutex wrapper
  paths.into_inner().unwrap()
}

fn cmd_find_first(_paths: &Vec<String>, _query: &String) {}

fn cmd_find_interactively(paths: &Vec<String>, query: &String) {
  let result = ui_finder::run(paths.clone(), query.clone());
  if let Some(result) = result {
    println!("{}", result);
  }
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
  cmd.spawn().expect("Failed to clone repo").wait().unwrap();
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

fn name_list_to_string(name: &Vec<String>, delimiter: &str) -> String {
  let joined = name.join(delimiter);
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
    cli::Command::Find { query, first, all } => {
      let paths = gather_all_paths(all);
      let query = name_list_to_string(&query, "");
      if first {
        cmd_find_first(&paths, &query);
      } else {
        cmd_find_interactively(&paths, &query);
      }
    }
    cli::Command::Clone { url, depth } => cmd_clone(&url, depth),
    cli::Command::JoneList => cmd_jone_list(),
    cli::Command::JoneNew { name } => {
      cmd_jone_new(&name_list_to_string(&name, " "));
    }
    cli::Command::JoneSections { name } => {
      cmd_jone_section_list(&name_list_to_string(&name, " "));
    }
    cli::Command::JoneLatest { name } => {
      cmd_jone_latest(&name_list_to_string(&name, " "));
    }
  }
}
