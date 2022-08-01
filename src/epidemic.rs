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
    // Increase the infected time of the infected persons.
    // If a person infected time is equal to 5 this person
    // is healthy now.
    // Every time a person gets healthy all its connections
    // have their infected connections decreased by one.
    // Recovered persons "looses" its connections with all
    // other persons as it can't transmit anymore.
    for i in 0..self.connections.size {
      if self.persons[i].infected {
        self.persons[i].time += 1;

        if self.persons[i].time >= 5 {
          self.persons[i].infected = false;
          // This and the lines bellow ensure this person
          // won't be infected again.
          self.persons[i].infected_connections = 0;

          for j in 0..self.connections.size {
            if self.connections.get(i, j) {
              self.persons[j].infected_connections -= 1;

              // A recovered person can no longer transmit.
              self.connections.set(i, j, false);
              self.connections.set(j, i, false);
            }
          }
        }
      }
    }

    // Infect 1 person per unit of time if still there are
    // healthy persons.
    // The new infected will always be the person with the
    // most infected connections.
    // Every time a new person is infected its connections
    // will have their number of infected connections
    // increased by 1.
    let new_infected_index = self
      .persons
      .iter()
      .enumerate()
      // A person can't be infected twice.
      .filter(|(_, person)| !person.infected)
      // A person can only be infected by another infected
      // person.
      .filter(|(_, person)| person.infected_connections > 0)
      .max_by(|(_, r), (_, l)| {
        r.infected_connections.cmp(&l.infected_connections)
      })
      .map(|(index, _)| index);

    if let Some(i) = new_infected_index {
      self.persons[i].infected = true;

      for j in 0..self.connections.size {
        if self.connections.get(i, j) {
          self.persons[j].infected_connections += 1;
        }
      }
    }
  }
}
