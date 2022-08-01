use crate::{graph::Graph, person::Person, rand::BoolRNG};

struct Epidemic {
  connections: Graph,
  persons: Vec<Person>,
}

impl Epidemic {
  fn new(
    population: usize,
    density: f32,
    epidemic_density: f32,
  ) -> Epidemic {
    let connections = Graph::new(population, density);

    let mut bool_rng = BoolRNG::new(epidemic_density);

    let mut persons = Vec::with_capacity(population);
    for index in 0..population {
      persons.push(Person::new(index, bool_rng.sample()));
    }

    Epidemic {
      connections,
      persons,
    }
  }

  fn time_step(&mut self) {
    self.persons.retain_mut(|person| {
      if person.infected {
        person.time += 1;
        person.time < 5
      } else {
        true
      }
    })
  }
}
