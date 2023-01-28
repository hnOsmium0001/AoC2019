use std::fs;

fn read_input() -> (u32, u32) {
  let s = match fs::read_to_string("../inputs/d4.txt") {
    Ok(res) => res,
    Err(_) => String::from(""),
  };
  let start = s[..6].parse::<u32>().expect("Not a number in input.txt");
  let end = s[7..].parse::<u32>().expect("Not a number in input.txt");
  (start, end)
}

fn rule_double_digits(s: &str) -> bool {
  let mut nit = s.chars().skip(1);
  for c in s.chars() {
    match nit.next() {
      Some(n) => {
        if c == n {
          return true;
        }
      },
      None => break,
    }
  }
  false
}

fn rule_never_decrease(s: &str) -> bool {
  let mut nit = s.chars().skip(1);
  for c in s.chars() {
    match nit.next() {
      Some(n) => {
        if n.to_digit(10) < c.to_digit(10) {
          return false;
        }
      },
      None => break
    }
  }
  true
}

fn part1() {
  let (start, end) = read_input();
  let mut count = 0;
  for i in start..=end {
    let s = i.to_string();
    if rule_double_digits(&s) && rule_never_decrease(&s) {
      count += 1;
    }
  }
  println!("{}", count);
}

fn new_rule_double_digits(s: &str) -> bool {
  let mut nit = s.chars().skip(1);
  let mut repeat_count = 0;
  for c in s.chars() {
    match nit.next() {
      Some(n) => {
        if c == n {
          repeat_count += 1;
        } else {
          if repeat_count == 1 {
            return true;
          }
          repeat_count = 0;
        }
      },
      None => break,
    }
  }
  repeat_count == 1
}

fn part2() {
  let (start, end) = read_input();
  let mut count = 0;
  for i in start..=end {
    let s = i.to_string();
    if new_rule_double_digits(&s) && rule_never_decrease(&s) {
      count += 1;
    }
  }
  println!("{}", count);
}

fn main() {
  // part1();
  part2();
}
