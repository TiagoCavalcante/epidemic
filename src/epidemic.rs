use crate::{graph::Graph, person::Person, rand::BoolRNG};

#[derive(Debug)]
pub struct Epidemic {
  connections: Graph,
  pub persons: Vec<Person>,
}

impl Epidemic {
  pub fn new(
    population: usize,
    density: f32,
    epidemic_density: f32,
  ) -> Epidemic {
    let connections = Graph::new(population, density);

    let mut bool_rng = BoolRNG::new(epidemic_density);

    let mut persons = (0..population)
      .map(|_| Person::new(bool_rng.sample(), 0))
      .collect::<Vec<Person>>();

    for i in 0..population {
      for j in 0..population {
        if persons[i].infected && connections.get(i, j) {
          persons[j].infected_connections += 1;
        }
      }
    }

    Epidemic {
      connections,
      persons,
    }
  }

  pub fn time_step(&mut self) {
    for i in 0..self.connections.size {
      if self.persons[i].infected {
        self.persons[i].time += 1;

        if self.persons[i].time == 5 {
          // It is healthy now.
          self.persons[i].infected = false;

          for j in 0..self.connections.size {
            if self.connections.get(i, j) {
              println!("i = {i}, j = {j}");
              self.persons[j].infected_connections -= 1;

              // This person can no longer transmit.
              self.connections.set(i, j, false);
              self.connections.set(j, i, false);
            }
          }
        }
      }
    }

    if let Some((index, person)) = self
      .persons
      .iter_mut()
      // A person can't be infected twice.
      .filter(|person| !person.infected)
      .enumerate()
      .max_by(|(_, r), (_, l)| {
        r.infected_connections.cmp(&l.infected_connections)
      })
    {
      person.infected = true;

      for j in 0..self.connections.size {
        if self.connections.get(index, j) {
          self.persons[j].infected_connections += 1;
        }
      }
    }
  }
}
