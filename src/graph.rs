use rand::distributions::{Distribution, Uniform};

#[derive(Debug)]
pub struct Graph {
  pub size: usize,
  pub data: Vec<Vec<bool>>,
}

impl Graph {
  fn max_data_density(&self) -> f32 {
    (self.size as f32 - 1.0) / self.size as f32
  }

  fn fill(&mut self, density: f32) {
    let normalized_density =
      density / self.max_data_density();
    let threshold =
      (normalized_density * usize::MAX as f32) as usize;

    let uniform_rng = Uniform::from(0..usize::MAX);
    let mut rng = rand::thread_rng();

    for i in 0..self.size {
      self.data.push(Vec::with_capacity(self.size));

      for j in 0..self.size {
        if i < j {
          let random_number = uniform_rng.sample(&mut rng);
          self.data[i].push(random_number < threshold);
        } else if i == j {
          self.data[i].push(false);
        } else {
          let value = self.data[j][i];
          self.data[i].push(value);
        }
      }
    }
  }

  pub fn new(size: usize, density: f32) -> Graph {
    let mut graph = Graph {
      size,
      data: Vec::with_capacity(size),
    };

    graph.fill(density);

    graph
  }

  pub fn density(&self) -> f32 {
    let mut edges: usize = 0;

    for i in 0..self.size {
      for j in 0..self.size {
        if self.data[i][j] {
          edges += 1;
        }
      }
    }

    edges as f32 / (self.size * (self.size - 1)) as f32
  }
}
