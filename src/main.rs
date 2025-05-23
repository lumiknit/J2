/* luminkit's jump helper 2
 * Author: lumiknit (aasr4r4@gmail.com)
 * Version: 0.2.2 (241005)
 */

use std::process::{exit, Command};
use std::{env, fs, vec};

pub mod cli;
pub mod config;
pub mod fuzzy;
pub mod path;
pub mod section;
pub mod shell;
pub mod ui_finder;

use clap::Parser;
use config::Config;
use path::PathItem;
use section::JoneSection;
use shell::ShellType;

fn get_executable_path(exe: &str) -> Option<String> {
  // Convert relative path to absolute path
  std::path::Path::new(exe)
    .canonicalize()
    .ok()
    .and_then(|p| p.to_str().map(|s| s.to_string()))
}

fn gather_all_paths(
  base: Vec<String>,
  files: bool,
  all: bool,
) -> Vec<path::PathItem> {
  let config = Config::from_env();

  let base_paths = if base.is_empty() {
    config.find_base_paths.clone()
  } else {
    base
      .iter()
      .map(|s| s.split(":").map(|s| s.to_string()))
      .flatten()
      .collect()
  };

  // Convert base paths to names
  let base_paths = path::convert_base_paths_to_names(&base_paths);

  // Traverse all directories and gather paths
  let paths = std::sync::Mutex::new(vec![]);

  for base in base_paths.into_iter() {
    let mut builder = ignore::WalkBuilder::new(base.abs.clone());
    builder.standard_filters(true).hidden(!all);
    if let Some(p) = &config.ignore_file_path {
      if let Some(_err) = builder.add_ignore(p) {
        // eprintln!("Error to load ignore file({})\n{}", p, _err);
      }
    }
    builder.build_parallel().run(|| {
      Box::new(|result| {
        if let Ok(entry) = result {
          let path = entry.path();
          if path.is_dir() || files {
            let mut paths = paths.lock().unwrap();
            let abs = path.to_str().unwrap().to_string();
            let displayed = if path.starts_with(&base.abs) {
              base.displayed.clone() + ": " + &abs[base.abs.len()..].to_string()
            } else {
              abs.clone()
            };
            paths.push(PathItem { displayed, abs });
            return ignore::WalkState::Continue;
          }
        }
        ignore::WalkState::Skip
      })
    });
  }

  // Destruct paths from mutex wrapper
  paths.into_inner().unwrap()
}

fn cmd_find_first(paths: &Vec<path::PathItem>, query: &String) {
  let mut ed = fuzzy::EditDist::new();
  ed.update_query(&query.chars().collect());
  let mut min_dist = std::u32::MAX;
  let mut min_path = None;
  for path in paths {
    if let Some(cost) = ed.run(&path.displayed) {
      if cost < min_dist {
        min_dist = cost;
        min_path = Some(path);
      }
    }
  }
  if let Some(min_path) = min_path {
    println!("{}", min_path.abs);
  } else {
    exit(1);
  }
}

fn cmd_find_interactively(paths: &Vec<path::PathItem>, query: &String) {
  let result = ui_finder::run(paths.to_vec(), query);
  if let Some(result) = result {
    println!("{}", result);
  } else {
    exit(1);
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
  let path = std::path::Path::new(&config.repos_path).join(splitted[1]);
  fs::create_dir_all(path.clone()).expect("Failed to create directory");
  // Clone repo
  let mut cmd = Command::new("git");
  let mut cmd = cmd.arg("clone").arg(repo_url).arg(path.to_str().unwrap());
  if let Some(d) = depth {
    cmd = cmd.arg(format!("--depth={}", d));
  }
  cmd.spawn().expect("Failed to clone repo").wait().unwrap();
}

fn jone_list(config: &Config) {
  // Read jone directories
  let path = std::path::Path::new(&config.jones_path);
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
  let jone_path = std::path::Path::new(&config.jones_path).join(name);
  // Make directory
  fs::create_dir_all(&jone_path).expect("Failed to create directory");
  // Create jone file
  let section_name = JoneSection::gen().to_base10();
  let section_path = jone_path.join(section_name);
  fs::create_dir(std::path::Path::new(&section_path))
    .expect("Failed to create directory");
  section_path.to_str().unwrap().to_string()
}

fn jone_section_list(config: &Config, name: &str) -> vec::Vec<JoneSection> {
  let name = canonicalize_jone_name(name);
  let jone_path = std::path::Path::new(&config.jones_path).join(name);
  // Read jone directories
  let path = std::path::Path::new(&jone_path);
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

fn cmd_shell_init(shell: ShellType) {
  // Get args
  let args: Vec<String> = env::args().collect();
  let exe = get_executable_path(args[0].as_str()).unwrap_or(String::from("j2"));
  let script = match shell {
    ShellType::Sh => include_str!("init.sh"),
    ShellType::Pwsh => include_str!("init.ps1"),
  };
  let s = script
    .replace("<EXECUTABLE_PATH>", exe.as_str())
    .replace("<INIT_HELP>", include_str!("init_help.txt"));
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
    println!(
      "{}",
      std::path::Path::new(&config.jones_path)
        .join(name)
        .join(list[0].to_string())
        .to_str()
        .unwrap()
    )
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
  let parsed_command = cli::Cli::parse();
  match parsed_command.command {
    cli::Command::ShellInit { shell } => {
      let sh = if let Some(s) = shell {
        let parsed = ShellType::from_string(s.as_str());
        if let Some(parsed) = parsed {
          parsed
        } else {
          eprintln!("Invalid shell type '{}', available options: sh, pwsh", s);
          exit(1);
        }
      } else {
        ShellType::Sh
      };
      cmd_shell_init(sh)
    }
    cli::Command::Find {
      query,
      base,
      first,
      files,
      all,
    } => {
      let paths = gather_all_paths(base, files, all);
      let query = query.join("");
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
