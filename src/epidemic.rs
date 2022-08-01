use crate::{graph::Graph, person::Person, rand::BoolRng};

pub struct Epidemic {
  bool_rng: BoolRng,
  transmission_time: usize,
  reinfection_time: usize,

  connections: Graph,
  pub persons: Vec<Person>,
}

impl Epidemic {
  pub fn new(
    alpha: f32,
    transmission_time: usize,
    reinfection_time: usize,
    population: usize,
    density: f32,
    epidemic_density: f32,
  ) -> Epidemic {
    let connections = Graph::new(population, density);

    let mut bool_rng = BoolRng::new(epidemic_density);

    let mut persons = (0..population)
      .map(|_| Person::new(bool_rng.sample()))
      .collect::<Vec<Person>>();

    // Calculate infected connections.
    for i in 0..population {
      for j in 0..population {
        if persons[i].infected && connections.get(i, j) {
          persons[j].infected_connections += 1;
        }
      }
    }

    Epidemic {
      bool_rng: BoolRng::new(alpha),
      transmission_time,
      reinfection_time,

      connections,
      persons,
    }
  }

  pub fn time_step(&mut self) {
    // Every person has a chance alpha of getting infected
    // for each of its infected connections.
    // Every time a new person is infected its connections
    // will have their number of infected connections
    // increased by 1.
    self
      .persons
      .iter()
      .enumerate()
      // A person can't be infected twice.
      .filter(|(_, person)| !person.infected)
      // A person can only be infected by another infected
      // person.
      .filter(|(_, person)| person.infected_connections > 0)
      // A person can only be reinfected after the
      // reinfection time.
      .filter(|(_, person)| person.time == 0)
      // Take the indexes.
      .map(|(index, _)| index)
      // "unborrow" self.persons.
      .collect::<Vec<usize>>()
      .iter()
      .for_each(|i| {
        if self.bool_rng.sample_weighted(
          self.persons[*i].infected_connections,
        ) {
          self.persons[*i].infected = true;

          for j in 0..self.connections.size {
            if self.connections.get(*i, j) {
              self.persons[j].infected_connections += 1;
            }
          }
        }
      });

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

        if self.persons[i].time > self.transmission_time {
          self.persons[i].infected = false;
          self.persons[i].time = self.reinfection_time;

          for j in 0..self.connections.size {
            if self.connections.get(i, j) {
              self.persons[j].infected_connections -= 1;

              // A recovered person can no longer transmit.
              self.connections.set(i, j, false);
              self.connections.set(j, i, false);
            }
          }
        }
      } else if self.persons[i].time > 0 {
        self.persons[i].time -= 1;
      }
    }
  }
}
