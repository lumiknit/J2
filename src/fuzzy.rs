pub fn contained_in(query: &Vec<char>, target: &String) -> bool {
  if query.is_empty() {
    return true;
  }
  // Check if the query is contained in the target in linear time
  let mut q_iter = query.iter().peekable();
  for c in target.chars() {
    if let Some(&q) = q_iter.peek() {
      if c.to_ascii_lowercase() == (*q).to_ascii_lowercase() {
        q_iter.next();
      }
    } else {
      return true;
    }
  }
  q_iter.peek().is_none()
}

pub fn edit_distance(q: &Vec<char>, target: &String) -> u32 {
  let q: Vec<char> = q.iter().map(|c| c.to_ascii_lowercase()).collect();
  let p: Vec<char> = target.chars().map(|c| c.to_ascii_lowercase()).collect();
  let mut d = vec![vec![0; p.len()]; 2];
  for i in 0..p.len() {
    d[1][i] = 0 as u32;
  }
  for i in 0..q.len() {
    let qc = q[i];
    let qp = if i == 0 { '\x00' } else { q[i - 1] };
    for j in 0..p.len() {
      let pc = p[j];
      let pp = if j == 0 { '\x01' } else { p[j - 1] };
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
