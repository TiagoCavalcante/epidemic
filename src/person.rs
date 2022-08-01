#[derive(Debug)]
pub struct Person {
  pub infected: bool,
  pub infected_connections: usize,
  pub time: usize,
}

impl Person {
  pub fn new(
    infected: bool,
    infected_connections: usize,
  ) -> Person {
    Person {
      infected,
      infected_connections,
      time: 0,
    }
  }
}
