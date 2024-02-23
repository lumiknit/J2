pub struct EditDist<'a> {
  q: &'a Vec<char>,
  d: [Vec<i32>; 2],
}

impl<'a> EditDist<'a> {
  pub fn new(q: &'a Vec<char>) -> Self {
    let l = q.len();
    Self {
      q,
      d: [vec![0; l], vec![0; l]],
    }
  }

  pub fn contained_in(&self, target: &String) -> bool {
    if self.q.is_empty() {
      return true;
    }
    // Check if the query is contained in the target in linear time
    let mut q_iter = self.q.iter();
    let mut q = *q_iter.next().unwrap();
    for c in target.chars() {
      if c.to_ascii_lowercase() == q {
        if let Some(n) = q_iter.next() {
          q = *n;
        } else {
          return true;
        }
      }
    }
    false
  }

  fn is_ascii_sep(c: char) -> bool {
    ('\x00' <= c && c < '0')
      || ('9' < c && c < 'A')
      || ('Z' < c && c < 'a')
      || ('z' < c && c < '\x7f')
  }

  pub fn run(&mut self, target: &String) -> i32 {
    for i in 0..self.q.len() {
      self.d[1][i] = 0;
    }
    let mut pp = '\x00';
    let mut i = 0;
    for (idx, pc) in target.chars().map(|c| c.to_ascii_lowercase()).enumerate()
    {
      let after_sep = Self::is_ascii_sep(pc);
      i = idx % 2;
      let mut qp = '\x01';
      for (j, &qc) in self.q.iter().enumerate() {
        let mut cost = 0;
        if qc == pc {
          if j == 0 {
            cost = cost.max(50);
          } else {
            if qp == pp {
              cost = cost.max(self.d[1 - i][j - 1] + 200);
            } else if after_sep {
              cost = cost.max(self.d[1 - i][j - 1] + 150);
            } else {
              cost = cost.max(self.d[1 - i][j - 1] + 50);
            }
          }
        }
        self.d[i][j] = cost;
        qp = qc;
      }
      pp = pc;
    }
    self.d[i].iter().max().unwrap_or(&0).clone() - target.len() as i32
  }
}
