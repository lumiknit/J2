/* luminkit's jump helper 2
 * Author: lumiknit (aasr4r4@gmail.com)
 * Version: 0.0.1 (230825)
 */

use std::{env, fs, path, time, vec};
use std::process::{exit, Command};

use chrono::Datelike;

struct Config {
  // Clone config
  repos_path: String,
  // Find config
  find_base_paths: Vec<String>,
  ignores: Vec<String>,
  // Jone config
  jone_path: String,
}

impl Config {
  fn from_env() -> Self {
    let repos_path =
      env::var("J2_REPOS_DIR").expect("Please set env $J2_REPOS_DIR");
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
    let jone_path =
      env::var("J2_JONE_PATH").expect("Please set env $J2_JONE_PATH");
    Self {
      repos_path,
      find_base_paths,
      ignores,
      jone_path,
    }
  }
}

fn print_help() {
  let s = "
luminkit's jump helper 2
Usage: J2 <COMMAND> [ARGS]
Commands:
  help: Print this help message
  init: Print the initialization script
  find <QUERY>: Find a directory
  clone <REPO_URL>: Clone a git repository
  jone-new [<NAME>]: Create a new jone (j-zone)
  jone-list: List jones
  jone-sections [<NAME>]: List sections in the jone
Environment variables:
  J2_REPOS_DIR: The directory where git repositories are stored
  J2_FIND_BASE_PATHS: The base paths to find directories (separated by ':')
  J2_IGNORES: The directories to ignore when finding (separated by ':')
  J2_JONE_PATH: The path to store jone files
  J2_EDITOR: The command name of editor to edit jone notes (e.g. vi)
";
  println!("{}", s.trim());
}

fn get_executable_path(exe: &str) -> Option<String> {
  // Convert relative path to absolute path
  path::Path::new(exe)
    .canonicalize()
    .ok()
    .and_then(|p| p.to_str().map(|s| s.to_string()))
}

fn print_init(exe: &str) {
  let exe = get_executable_path(exe).unwrap_or(String::from("j2"));
  let s = format!(
        "
# luminkit's jump helper 2
# To initialize this for your shell, run:
# eval \"$(j2 init)\"
# To initialize this for your shell permanently, add the above line to your shell's rc file.
__J2=\"{}\"
J() {{
    case \"$1\" in
        a) echo \"ASD\";;
        *) echo \"Invalid command: $1\";;
    esac
    $__J2
}}
",
        exe
    );
  println!("{}", s.trim());
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Dir {
  path: String,
  name: String,
  loss: u32,
}

fn gather_directories(
  config: &Config,
  result: &mut Vec<Dir>,
  query: &str,
  dir: &path::Path,
  loss: u32,
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
  let mut l = loss;
  let mut dl = 1; // penalty of mismatch
  for c in path_name.chars() {
    if let Some(qc) = query_chars.peek() {
      if c == *qc {
        query_chars.next();
        dl = 1;
      } else {
        l += dl;
        dl = 0;
      }
    }
  }
  if query_chars.peek().is_none() {
    // Found!
    result.push(Dir {
      path: dir.to_str().unwrap().to_string(),
      name: filename.to_string(),
      loss: 0,
    });
  } else {
    let new_query = query_chars.collect::<String>();
    // Find recursively
    if let Ok(entries) = dir.read_dir() {
      for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name();
        let hidden = file_name.is_some()
          && file_name.unwrap().to_str().unwrap().starts_with(".");
        if path.is_dir() && !hidden {
          gather_directories(config, result, new_query.as_str(), &path, l);
        }
      }
    }
  }
}

fn find_path(config: &Config, search: &str) -> Vec<String> {
  let mut result: Vec<Dir> = vec![];
  for base_path in &config.find_base_paths {
    let path = path::Path::new(base_path);
    gather_directories(config, &mut result, search, path, 0);
  }
  result.sort_by(|a, b| a.loss.cmp(&b.loss));
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

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum JoneSectionBase {
  Base36,
  Base10,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct JoneSection {
  pub year: u32,
  pub month: u32,
  pub day: u32,
  pub sys_time: time::SystemTime,
  pub rand: u32,
  pub base: JoneSectionBase,
}

impl JoneSection {
  fn gen() -> Self {
    let now = chrono::Local::now();
    Self {
      sys_time: time::SystemTime::now(),
      year: now.year_ce().1 % 100,
      month: now.month(),
      day: now.day(),
      rand: rand::random::<u32>() % 36_u32.pow(4),
      base: JoneSectionBase::Base36,
    }
  }

  fn base36_rand(&self) -> String {
    let mut r = self.rand;
    let mut rs = ['0'; 4];
    for i in 0..4 {
      rs[3 - i] = char::from_digit(r % 36, 36).unwrap();
      r /= 36;
    }
    rs.iter().collect::<String>()
  }

  fn to_base36(&self) -> String {
    format!(
      "{:02}{:01}{:01}-{}",
      self.year,
      char::from_digit(self.month, 36).unwrap(),
      char::from_digit(self.day, 36).unwrap(),
      self.base36_rand(),
    )
  }

  fn to_base10(&self) -> String {
    format!(
      "{:02}{:02}{:02}-{:04}",
      self.year, self.month, self.day, self.rand
    )
  }

  fn to_string(&self) -> String {
    match self.base {
      JoneSectionBase::Base36 => self.to_base36(),
      JoneSectionBase::Base10 => self.to_base10(),
    }
  }

  fn from_str(s: &str, sys_time: Option<time::SystemTime>) -> Option<Self> {
    if s.len() == 9 && s[4..5].contains('-') {
      // Base36
      Some(Self {
        year: u32::from_str_radix(&s[0..2], 10).ok()?,
        month: u32::from_str_radix(&s[2..3], 36).ok()?,
        day: u32::from_str_radix(&s[3..4], 36).ok()?,
        sys_time: sys_time.unwrap_or(time::SystemTime::now()),
        rand: u32::from_str_radix(&s[5..9], 36).ok()?,
        base: JoneSectionBase::Base36,
      })
    } else if s.len() == 11 && s[6..7].contains("-") {
      // Base10
      Some(Self {
        year: u32::from_str_radix(&s[0..2], 10).ok()?,
        month: u32::from_str_radix(&s[2..4], 10).ok()?,
        day: u32::from_str_radix(&s[4..6], 10).ok()?,
        sys_time: sys_time.unwrap_or(time::SystemTime::now()),
        rand: u32::from_str_radix(&s[7..11], 36).ok()?,
        base: JoneSectionBase::Base10,
      })
    } else {
      None
    }
  }
}

fn jone_list(config: &Config) {
  // Read jone directories
  let path = path::Path::new(&config.jone_path);
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

fn jone_new(config: &Config, name: &str) {
  let name = canonicalize_jone_name(name);
  let jone_path = format!("{}/{}", config.jone_path, name);
  // Make directory
  fs::create_dir_all(&jone_path).expect("Failed to create directory");
  // Create jone file
  let section_name = JoneSection::gen().to_base36();
  let section_path = format!("{}/{}", jone_path, section_name);
  fs::create_dir(path::Path::new(&section_path))
    .expect("Failed to create directory");
  print!("{}", section_path);
}

fn jone_section_list(config: &Config, name: &str) {
  let name = canonicalize_jone_name(name);
  let jone_path = format!("{}/{}", config.jone_path, name);
  // Read jone directories
  let path = path::Path::new(&jone_path);
  let entries = path.read_dir();
  if entries.is_err() {
    return;
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
  for section in list {
    println!("{}", section.to_string());
  }
}

fn run(args: Vec<String>) {
  let l = args.len();
  if l <= 1 {
    print_help();
    return;
  }
  match args[1].as_str() {
    "init" => {
      print_init(args[0].as_str());
    }
    "find" => {
      let config = Config::from_env();
      let query = args[2..].join("");
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
      jone_new(&config, name.as_str());
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
      jone_section_list(&config, name.as_str());
    }
    _ => {
      print_help();
    }
  }
}

fn main() {
  run(env::args().collect());
}
