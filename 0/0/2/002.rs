// Copyright (C) 2013 Jorge Aparicio

use std::iter::AdditiveIterator;
use std::util::replace;

fn main() {
  let limit = 4000000;

  println(fibonacci().
          take_while(|&x| x < limit).
          filter(|x| x % 2 == 0).
          sum().
          to_str());
}

struct Fibonacci { curr: int, next: int }

fn fibonacci() -> Fibonacci {
  Fibonacci { curr: 1, next: 2 }
}

impl Iterator<int> for Fibonacci {
  fn next(&mut self) -> Option<int> {
    let new_next = self.curr + self.next;
    let new_curr = replace(&mut self.next, new_next);
    Some(replace(&mut self.curr, new_curr))
  }
}