use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn read_input() -> Vec<String> {
  let file = File::open("../inputs/d1.txt").expect("No input.txt");
  let buf = BufReader::new(file);
  buf.lines()
    .map(|l| l.expect("Could not parse line from input.txt"))
    .collect()
}

fn part1() {
  let lines = read_input();
  let mut total: i32 = 0;
  for line in lines {
    let mass = line.parse::<i32>().unwrap();
    total += mass / 3 - 2;
  }
  println!("{}", total);
}

fn part2() {
  let lines = read_input();
  let mut total: i32 = 0;
  for line in lines {
    let mass = line.parse::<i32>().unwrap();
    let mut fuel = mass / 3 - 2;
    while fuel > 0 {
      total += fuel;
      fuel = fuel / 3 - 2;
    }
  }
  println!("{}", total);
}

fn main() {
    // part1();
    part2();
}
