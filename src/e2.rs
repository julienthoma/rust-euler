fn fibonacci(n: u32) -> u32 {
  match n {
    0..=1 => 1,
    _ => fibonacci(n - 1) + fibonacci(n - 2),
  }
}

fn main() {
  let mut sum = 0;
  let mut n = 0;

  loop {
    let curr_fib = fibonacci(n);

    if curr_fib > 4_000_000 {
      break;
    }

    sum = if curr_fib % 2 == 0 { sum + curr_fib } else { sum };
    n += 1;
  }

  println!("{}", sum);
}
