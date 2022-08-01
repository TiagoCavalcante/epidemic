use rand::distributions::{Distribution, Uniform};

#[derive(Debug)]
pub struct Graph {
  size: usize,
  data: Vec<bool>,
}

impl Graph {
  fn get(&self, row: usize, col: usize) -> bool {
    self.data[row * self.size + col]
  }

  fn set(&mut self, row: usize, col: usize, value: bool) {
    self.data[row * self.size + col] = value
  }

  pub fn density(&self) -> f32 {
    let mut edges: usize = 0;

    for row in 0..self.size {
      for col in 0..self.size {
        if self.get(row, col) {
          edges += 1;
        }
      }
    }

    edges as f32 / (self.size * (self.size - 1)) as f32
  }

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
      for j in 0..self.size {
        if i < j {
          let random_number = uniform_rng.sample(&mut rng);
          self.set(i, j, random_number < threshold);
        } else if i == j {
          self.set(i, j, false);
        } else {
          self.set(i, j, self.get(j, i));
        }
      }
    }
  }

  pub fn new(size: usize, density: f32) -> Graph {
    let mut graph = Graph {
      size,
      data: vec![false; size * size],
    };

    graph.fill(density);

    graph
  }
}
