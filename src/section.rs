use chrono::Datelike;
use std::time;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum JoneSectionBase {
  Base36,
  Base10,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct JoneSection {
  pub year: u32,
  pub month: u32,
  pub day: u32,
  pub sys_time: time::SystemTime,
  pub rand: u32,
  pub base: JoneSectionBase,
}

impl JoneSection {
  pub fn gen() -> Self {
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

  pub fn base36_rand(&self) -> String {
    let mut r = self.rand;
    let mut rs = ['0'; 4];
    for i in 0..4 {
      rs[3 - i] = char::from_digit(r % 36, 36).unwrap();
      r /= 36;
    }
    rs.iter().collect::<String>()
  }

  pub fn to_base36(&self) -> String {
    format!(
      "{:02}{:01}{:01}-{}",
      self.year,
      char::from_digit(self.month, 36).unwrap(),
      char::from_digit(self.day, 36).unwrap(),
      self.base36_rand(),
    )
  }

  pub fn to_base10(&self) -> String {
    format!(
      "{:02}{:02}{:02}-{:04}",
      self.year, self.month, self.day, self.rand
    )
  }

  pub fn to_string(&self) -> String {
    match self.base {
      JoneSectionBase::Base36 => self.to_base36(),
      JoneSectionBase::Base10 => self.to_base10(),
    }
  }

  pub fn from_str(s: &str, sys_time: Option<time::SystemTime>) -> Option<Self> {
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
