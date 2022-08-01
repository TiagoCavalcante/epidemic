use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

pub struct BoolRng {
  uniform_rng: Uniform<usize>,
  rng: ThreadRng,
  threshold: usize,
}

impl BoolRng {
  pub fn new(probability: f32) -> BoolRng {
    let uniform_rng: Uniform<usize> =
      Uniform::from(0..usize::MAX);
    let rng: ThreadRng = rand::thread_rng();

    BoolRng {
      uniform_rng,
      rng,
      threshold: (probability * usize::MAX as f32) as usize,
    }
  }

  pub fn sample(&mut self) -> bool {
    self.uniform_rng.sample(&mut self.rng) < self.threshold
  }

  pub fn sample_weighted(&mut self, weight: usize) -> bool {
    (0..weight)
      .map(|_| {
        self.uniform_rng.sample(&mut self.rng)
          < self.threshold
      })
      .any(|v| v)
  }
}
