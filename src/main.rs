mod epidemic;
mod graph;
mod path;
mod person;
mod rand;

fn main() {
  let size = 1000;
  let time = 2;

  let mut epidemic = epidemic::Epidemic::new(
    0.3,
    1,
    10,
    size,
    0.1,
    0.01,
  );

  println!(
    "epidemic(0): {:?}",
    epidemic
      .persons
      .iter()
      .map(|p| (if p.infected { "i" } else { "h" }, p.time))
      .collect::<Vec<_>>()
  );

  for _ in 0..time {
    epidemic.time_step();
  }

  println!(
    "epidemic({time}): {:?}",
    epidemic
      .persons
      .iter()
      .map(|p| (if p.infected { "i" } else { "h" }, p.time))
      .collect::<Vec<_>>()
  );
}
