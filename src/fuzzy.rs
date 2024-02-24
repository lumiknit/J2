pub struct EditDist {
  q: Vec<char>,
  d: [Vec<u32>; 2],
  target_chars: Vec<char>,
}

impl EditDist {
  pub fn new() -> Self {
    Self {
      q: vec![],
      d: [vec![], vec![]],
      target_chars: vec![],
    }
  }

  pub fn update_query<'a>(&'a mut self, q_vec: &Vec<char>) -> &'a mut Self {
    self.q.clear();
    self.q.extend(q_vec.iter().map(|c| c.to_ascii_lowercase()));
    self.d[0].resize(self.q.len() + 1, 0);
    self.d[1].resize(self.q.len() + 1, 0);
    self
  }

  fn is_ascii_sep(c: char) -> bool {
    ('\x00' <= c && c < '0')
      || ('9' < c && c < 'A')
      || ('Z' < c && c < 'a')
      || ('z' < c && c < '\x7f')
  }

  const COST_INSERT: u32 = 2;
  const COST_MATCH_NON_CONTD: u32 = 1;
  const COST_MATCH_NON_ABBREV: u32 = 7;

  pub fn run(&mut self, target: &String) -> Option<u32> {
    // If the query is empty, just return inverse of length
    if self.q.is_empty() {
      // Then length is a cost
      return Some(target.len() as u32);
    }

    // Convert target string into chars
    self.target_chars.clear();
    self
      .target_chars
      .extend(target.chars().map(|c| c.to_ascii_lowercase()));

    // Check if the query is contained in the target in linear time

    // Find first hit
    let first_hit = {
      let q_first = self.q[0];
      let mut h = None;
      for i in 0..self.target_chars.len() {
        if self.target_chars[i] == q_first {
          h = Some(i);
          break;
        }
      }
      h?
    };

    // Then, find the rest of the query
    {
      let mut q_iter = self.q.iter();
      let mut q = *q_iter.next().unwrap();
      for i in first_hit..self.target_chars.len() {
        if self.target_chars[i] == q {
          if let Some(n) = q_iter.next() {
            q = *n;
          } else {
            break;
          }
        }
      }
      if let Some(_) = q_iter.next() {
        return None;
      }
    }

    // If the query is contained in the target, calculate the edit distance
    self.d[(first_hit + 1) % 2].fill(u32::MAX);

    let mut pp = if first_hit > 0 {
      self.target_chars[first_hit - 1]
    } else {
      '\x00'
    };
    let mut i = 0;
    for (idx, &pc) in self.target_chars.iter().skip(first_hit).enumerate() {
      // Because of skip, add first_hit to idx
      let idx = idx + first_hit;

      // Check if the previous character is a separator;
      let after_sep = Self::is_ascii_sep(pp);

      // Calculate index of d
      i = idx % 2;
      let zi = 1 - i;

      // Traverse of query string
      let mut qp = '\x01';
      for (j, &qc) in self.q.iter().enumerate() {
        let mut cost: u32 = u32::MAX;
        if qc == pc {
          if j == 0 {
            cost = Self::COST_INSERT.saturating_mul(idx as u32);
          } else {
            cost = self.d[zi][j - 1];
          }
          if !after_sep {
            cost = cost.saturating_add(Self::COST_MATCH_NON_ABBREV);
          }
          if qp != pp {
            cost = cost.saturating_add(Self::COST_MATCH_NON_CONTD);
          }
        }
        // Just insert from previous
        cost = cost.min(self.d[zi][j].saturating_add(Self::COST_INSERT));
        self.d[i][j] = cost;
        qp = qc;
      }
      pp = pc;
    }
    Some(self.d[i][self.q.len() - 1])
  }
}
