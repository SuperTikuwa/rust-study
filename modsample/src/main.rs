mod myutil;

fn main() {
  for i in 1..4 {
    println!("{}の3倍={}", i, myutil::three_times(i))
  }
}
