pub mod fuzzy;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let query = "oob";
    let target1 = "foobar";
    let target2 = "out-of-bound";

    let qcs = query.to_string().chars().collect();
    let mut ed = crate::fuzzy::EditDist::new();
    ed.update_query(&qcs);
    let cost1 = ed.run(&target1.to_string());
    let cost2 = ed.run(&target2.to_string());

    println!("\n---");
    println!("{} for {}: {:?}", target1, query, cost1);
    println!("{} for {}: {:?}", target2, query, cost2);
  }
}
