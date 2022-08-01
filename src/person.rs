#[derive(Debug)]
pub struct Person {
  index: usize,
  infected: bool,
  time: usize,
}

impl Person {
  pub fn new(index: usize, infected: bool) -> Person {
    Person {
      index,
      infected,
      time: 0,
    }
  }
}
