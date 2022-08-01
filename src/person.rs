pub struct Person {
  pub infected: bool,
  pub infected_connections: usize,
  pub time: usize,
}

impl Person {
  pub fn new(infected: bool) -> Person {
    Person {
      infected,
      infected_connections: 0,
      time: 0,
    }
  }
}
