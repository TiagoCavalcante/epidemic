mod graph;
mod path;

use crate::path::Path;

fn main() {
  let g = graph::Graph::new(800, 0.01);
  println!("density: {}", g.density());
  println!("paths: {:?}", g.find_paths(4, 70, 3));
}
