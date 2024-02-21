pub struct FuzzyContext {
  original_query: String,
  query_for_contain: String,
}

impl FuzzyContext {
  pub fn new(query: &str) -> Self {
    let lower = query.to_lowercase();
    Self {
      original_query: query.to_string(),
      query_for_contain: lower.chars().collect(),
    }
  }

  pub fn contained_in(&self, target: &String) -> bool {
    // Check if the query is contained in the target in linear time
    let mut iter = self.query_for_contain.chars().peekable();
    for c in target.to_lowercase().chars() {
      let q = *iter.peek().unwrap_or(&' ');
      if c == q && iter.next().is_none() {
        return true;
      }
    }
    false
  }
}
