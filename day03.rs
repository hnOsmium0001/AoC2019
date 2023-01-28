use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::cmp;

fn read_input() -> Vec<String> {
  let file = File::open("../inputs/d3.txt").expect("No input.txt");
  let buf = BufReader::new(file);
  buf.lines()
    .map(|l| l.expect("Could not parse line from input.txt"))
    .collect()
}

#[derive(Debug, PartialEq, Clone)]
struct Point {
  x: i32,
  y: i32,
}

#[derive(Debug, PartialEq)]
enum Line {
  Horizontal { p1: Point, p2: Point },
  Vertical { p1: Point, p2: Point },
}

impl Line {
  fn is_in(&self, pt: &Point) -> bool {
    match self {
      Line::Horizontal { p1, p2 } =>
        pt.y == p1.y &&
        ((pt.x - p1.x > 0) == (p2.x - pt.x > 0)),
      Line::Vertical { p1, p2 } =>
        pt.x == p1.x &&
        ((pt.y - p1.y > 0) == (p2.y - pt.y > 0)),
    }
  }

  fn length(&self) -> i32 {
    match self {
      Line::Horizontal { p1, p2 } => (p2.x - p1.x).abs(),
      Line::Vertical { p1, p2 } => (p2.y - p1.y).abs(),
    }
  }

  fn dist_from_p1(&self, pt: &Point) -> i32 {
    match self {
      Line::Horizontal { p1, .. } => (p1.x - pt.x).abs(),
      Line::Vertical { p1, .. } => (p1.y - pt.y).abs(),
    }
  }
}

fn read_wire(input: &Vec<&str>) -> Vec<Line> {
  let mut last = Point { x: 0, y: 0 };
  let mut res = Vec::new();
  for item in input {
    let dir = &item[..1];
    let dist = &item[1..].parse::<i32>().unwrap();
    match dir {
      "L" => res.push(Line::Horizontal {
        p1: Point { x: last.x, y: last.y },
        p2: Point { x: last.x - dist, y: last.y },
      }),
      "R" => res.push(Line::Horizontal {
        p1: Point { x: last.x, y: last.y },
        p2: Point { x: last.x + dist, y: last.y },
      }),
      "U" => res.push(Line::Vertical {
        p1: Point { x: last.x, y: last.y },
        p2: Point { x: last.x, y: last.y + dist },
      }),
      "D" => res.push(Line::Vertical {
        p1: Point { x: last.x, y: last.y },
        p2: Point { x: last.x, y: last.y - dist },
      }),
      _ => (),
    }
    
    match res.last() {
      Some(line) => match line {
        Line::Horizontal { p2, .. } | Line::Vertical { p2, .. } => last = p2.clone(),
      },
      None => break,
    }
  }
  res
}
fn read_wires() -> (Vec<Line>, Vec<Line>) {
  let input = read_input();
  let data0 = input[0].split(",").collect();
  let data1 = input[1].split(",").collect();
  (read_wire(&data0), read_wire(&data1))
}

fn intersection(line1: &Line, line2: &Line) -> Option<Point> {
  let res = match line1 {
    Line::Horizontal { p1: h, .. } => match line2 {
      Line::Horizontal { .. } => None,
      Line::Vertical { p2: v, .. } => Some(Point { x: v.x, y: h.y, }),
    },
    Line::Vertical { p1: v, .. } => match line2 {
      Line::Horizontal { p2: h, .. } => Some(Point { x: v.x, y: h.y, }),
      Line::Vertical { .. } => None,
    }
  };

  match res {
    Some(pt) => {
      if line1.is_in(&pt) && line2.is_in(&pt) {
       Some(pt)
      } else {
       None
      }
    },
    None => None,
  }
}

fn dist_between(pt1: &Point, pt2: &Point) -> i32 {
  return (pt1.x - pt2.x).abs() + (pt1.y - pt2.y).abs();
}

fn part1() {
  let (first, second) = read_wires();
  let origin = Point { x: 0, y: 0 };
  let mut closest = i32::max_value();

  for (i, fline) in first.iter().enumerate() {
    for (j, sline) in second.iter().enumerate() {
      if i == 0 && j == 0 {
        continue;
      }

      let dist = match intersection(&fline, &sline) {
        Some(pt) => dist_between(&origin, &pt),
        None => i32::max_value(), 
      };
      closest = cmp::min(closest, dist);
    }
  }
  println!("{}", closest);
}

fn length_back(lines: &Vec<Line>, idx: usize) -> i32 {
  let mut len = 0;
  for line in lines.iter().rev().skip(cmp::min(lines.len(), lines.len() - idx)) {
    len += line.length();
  }
  len
}

fn part2() {
  let (first, second) = read_wires();
  let origin = Point { x: 0, y: 0 };

  let mut shortest = i32::max_value();

  for (i, fline) in first.iter().enumerate() {
    for (j, sline) in second.iter().enumerate() {
      if i == 0 && j == 0 {
        continue;
      }

      let len = match intersection(&fline, &sline) {
        Some(pt) => 
          length_back(&first, i) + fline.dist_from_p1(&pt) +
          length_back(&second, j) + sline.dist_from_p1(&pt),
        None => i32::max_value(), 
      };
      shortest = cmp::min(shortest, len);
    }
  }
  println!("{}", shortest);
}

fn main() {
  // part1();
  part2();
}
