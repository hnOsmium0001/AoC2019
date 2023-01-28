use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct Point {
  x: i32,
  y: i32,
}

fn read_asteroids() -> (Vec<Vec<char>>, Vec<Point>, i32, i32) {
  let file = File::open("../inputs/d10.txt").expect("No input.txt");
  let buf = BufReader::new(file);
  let lines: Vec<Vec<char>> = buf.lines()
    .map(|l| l.expect("Could not parse line from input.txt"))
    .map(|l| l.chars().collect())
    .collect();
  let mut asteroids = Vec::new();
  for (y, line) in lines.iter().enumerate() {
    for (x, c) in line.iter().enumerate() {
      match c {
        '.' => (),
        '#' => asteroids.push(Point { x: x as i32, y: y as i32 }),
        _ => panic!("Unknown input character {}!", c),
      }
    }
  }
  let width = lines[0].len() as i32;
  let height = lines.len() as i32;
  (lines, asteroids, width, height)
}

fn has_los(map: &Vec<Vec<char>>, base: &Point, target: &Point) -> bool {
  let dx = target.x - base.x;
  let dy = target.y - base.y;
  let orig_x_sign = dx > 0;
  let orig_y_sign = dy > 0;
  let mut x = target.x;
  let mut y = target.y;
  // TODO break condition
  while x - base.x > 0 == orig_x_sign && y - base.y > 0 == orig_y_sign {
    if map[y as usize][x as usize] == '#' {
      return false;
    }
    x += dx;
    y += dy;
  }
  true
}

fn part1() {
  let (map, asteroids, width, height) = read_asteroids();
  let mut best = &asteroids[0];
  let mut best_los = 0;
  for base in &asteroids {
    let mut los = 0;
    for target in &asteroids {
      if target == base {
        continue;
      }
      if has_los(&map, &base, &target) {
        los += 1;
      }
    }
    if los > best_los {
      best = &base;
      best_los = los;
    }
  }
  println!("{}", best_los);
}

fn main() {
  part1();
}
