pub enum ShellType {
  Sh,
  Pwsh,
}

impl ShellType {
  pub fn to_str(&self) -> &str {
    match self {
      ShellType::Sh => "sh",
      ShellType::Pwsh => "pwsh",
    }
  }

  pub fn from_string(s: &str) -> Option<Self> {
    match s.to_lowercase().as_str() {
      "sh" => Some(ShellType::Sh),
      "bash" => Some(ShellType::Sh),
      "zsh" => Some(ShellType::Sh),
      "fish" => Some(ShellType::Sh),
      "pwsh" => Some(ShellType::Pwsh),
      "powershell" => Some(ShellType::Pwsh),
      _ => None,
    }
  }
}
