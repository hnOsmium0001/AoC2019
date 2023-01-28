use std::fs;
use std::convert::TryInto;

fn read_input() -> String {
  fs::read_to_string("../inputs/d8.txt").expect("Cannot find input.txt!")
}

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn part1() {
  let input = read_input();
  let mut layers = Vec::new();

  let mut layer = [0; 9];
  let mut i = 0;
  for c in input.chars() {
    layer[c.to_digit(10).expect("Invalid input!") as usize] += 1;
    if i < WIDTH * HEIGHT - 1 {
      i += 1;
    } else {
      layers.push(layer);
      layer = [0; 9];
      i = 0;
    }
  }

  let mut max_0_layer = &layers[0];
  for layer in &layers {
    if layer[0] < max_0_layer[0] {
      max_0_layer = &layer;
    }
  }
  println!("{}", max_0_layer[1] * max_0_layer[2]);
}

const BLACK: i32 = 0;
const WHITE: i32 = 1;
const TRANSPARENT: i32 = 2;
const PENDING: i32 = -1;

fn part2() {
  let input = read_input();
  let mut result = [PENDING; WIDTH * HEIGHT];

  let mut i = 0;
  for c in input.chars() {
    if result[i] == PENDING {
      result[i] = match c.to_digit(10).expect("Invalid input!").try_into().unwrap() {
        BLACK => BLACK,
        WHITE => WHITE,
        TRANSPARENT => PENDING,
        _ => panic!("Invalid input!"),
      }
    }

    i = if i + 1 < result.len() { i + 1 } else { 0 };
  }

  let mut out = String::new();
  let mut lines = 0;
  for c in result.iter() {
    out.push_str(match *c {
      WHITE => "#",
      BLACK => " ",
      _ => "?",
    });
    if (out.len() - lines) % WIDTH == 0 {
      lines += 1;
      out.push_str("\n");
    }
  }
  println!("{}", out);
}

fn main() {
  // part1();
  part2();
}
