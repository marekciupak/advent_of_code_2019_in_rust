use std::io;

pub fn run<R>(mut input: R)
where
  R: io::BufRead,
{
  let mut total = 0;
  let mut buffer = String::new();

  loop {
    if input.read_line(&mut buffer).unwrap() == 0 {
      break;
    }

    let mass: i32 = buffer.trim().parse().unwrap();
    total = total + mass / 3 - 2;

    buffer.clear();
  }

  println!("{}", total)
}
