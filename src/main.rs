mod graph;

fn main() {
  let g = graph::Graph::new(1000, 0.7);
  println!("density: {}", g.density());
}
