// Smallest multiple https://projecteuler.net/problem=5

fn divisible(number: u32, max: u32) -> bool {
  let mut is_divisible = true;
  for n in 1..=max {
    if number % n != 0 {
      is_divisible = false;
    }
  }
  is_divisible
}

fn smallest_divisible_number(n: u32) -> u32{
  let mut i = 1;
  
  loop {
    if i % 1_000_000 == 0 {
      println!("{}", i);
    }
    if divisible(i, n) {
      break;
    }
    i = i + 1;
  }
  i
}

fn main() {
  println!("{}", smallest_divisible_number(20));
}
