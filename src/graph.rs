use crate::rand::BoolRNG;

#[derive(Debug)]
pub struct Graph {
  pub size: usize,
  data: Vec<bool>,
}

impl Graph {
  pub fn get(&self, row: usize, col: usize) -> bool {
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
    let mut bool_rng =
      BoolRNG::new(density / self.max_data_density());

    for i in 0..self.size {
      for j in 0..self.size {
        if i < j {
          self.set(i, j, bool_rng.sample());
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
