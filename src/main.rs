mod epidemic;
mod graph;
mod path;
mod person;
mod rand;

fn print_epidemic(
  time: usize,
  epidemic: &epidemic::Epidemic,
) {
  println!(
    "epidemic({time}): {:?}",
    epidemic
      .persons
      .iter()
      .map(|p| (if p.infected { "i" } else { "h" }, p.time))
      .collect::<Vec<_>>()
  );
}

fn main() {
  let time = 2;

  let mut epidemic =
    epidemic::Epidemic::new(0.3, 1, 10, 1000, 0.1, 0.01);

  print_epidemic(0, &epidemic);

  for _ in 0..time {
    epidemic.time_step();
  }

  print_epidemic(time, &epidemic);
}
