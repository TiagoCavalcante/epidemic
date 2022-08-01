// use crate::path::Path;

mod epidemic;
mod graph;
mod path;
mod person;
mod rand;

fn main() {
  let size = 8;
  // let g = graph::Graph::new(size, 0.2);

  // let mut bool_rng = rand::BoolRNG::new(0.1);

  // let persons = (0..size)
  //   .map(|_| person::Person::new(bool_rng.sample(), 0))
  //   .collect::<Vec<person::Person>>();

  // println!("density: {}", g.density());
  // println!("persons: {:?}", persons);
  // println!("paths: {:?}", g.find_paths(4, 7, 3));

  let mut epidemic =
    epidemic::Epidemic::new(size, 0.2, 0.4);
  println!(
    "epidemic(0): {:?}",
    epidemic
      .persons
      .iter()
      .map(|p| p.infected_connections)
      .collect::<Vec<_>>()
  );

  println!(
    "epidemic(0): {:?}",
    epidemic
      .persons
      .iter()
      .map(|p| (if p.infected { "i" } else { "h" }, p.time))
      .collect::<Vec<_>>()
  );
  epidemic.time_step(); // 1
  epidemic.time_step(); // 2
  epidemic.time_step(); // 3
  epidemic.time_step(); // 4
  epidemic.time_step(); // 5
  epidemic.time_step(); // 6
  epidemic.time_step(); // 7
  epidemic.time_step(); // 8
  epidemic.time_step(); // 9
  epidemic.time_step(); // 10
  epidemic.time_step(); // 11
  epidemic.time_step(); // 12
  epidemic.time_step(); // 13
  epidemic.time_step(); // 14
  println!(
    "epidemic(5): {:?}",
    epidemic
      .persons
      .iter()
      .map(|p| (if p.infected { "i" } else { "h" }, p.time))
      .collect::<Vec<_>>()
  );
}
