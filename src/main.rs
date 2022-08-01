mod graph;
mod path;
mod person;
mod rand;

fn main() {
  let size = 10;
  let g = graph::Graph::new(size, 0.01);

  let mut bool_rng = rand::BoolRNG::new(0.1);

  let mut persons = vec![];
  for index in 0..size {
    persons
      .push(person::Person::new(index, bool_rng.sample()));
  }

  println!("density: {}", g.density());
  println!("density: {:?}", persons);
}
