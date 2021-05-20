use rand::Rng;

fn main() {
  let mut input = String::new();

  std::io::stdin().read_line(&mut input).ok();
  let s = input.trim();

  for _i in 0..3 {
    println!("{}", s);
  }
}
