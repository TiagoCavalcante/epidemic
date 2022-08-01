mod epidemic;
mod graph;
mod path;
mod person;
mod rand;

fn main() {
  let size = 800;
  let steps = 100;

  let mut epidemic =
    epidemic::Epidemic::new(size, 0.9, 0.2);

  println!(
    "epidemic(0): {:?}",
    epidemic
      .persons
      .iter()
      .map(|p| (if p.infected { "i" } else { "h" }, p.time))
      .collect::<Vec<_>>()
  );

  for _ in 0..steps {
    epidemic.time_step();
  }

  println!(
    "epidemic({steps}): {:?}",
    epidemic
      .persons
      .iter()
      .map(|p| (if p.infected { "i" } else { "h" }, p.time))
      .collect::<Vec<_>>()
  );
}
