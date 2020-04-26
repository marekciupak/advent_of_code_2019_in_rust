use std::io;

pub fn run<R>(mut input: R)
where
  R: io::BufRead,
{
  let mut buffer = String::new();

  loop {
    if input.read_line(&mut buffer).unwrap() == 0 {
      break;
    }

    println!("{}", buffer);

    buffer.clear();
  }
}
