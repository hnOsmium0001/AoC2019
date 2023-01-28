use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn read_input() -> Vec<String> {
  let file = File::open("../inputs/d6.txt").expect("No input.txt");
  let buf = BufReader::new(file);
  buf.lines()
    .map(|l| l.expect("Could not parse line from input.txt"))
    .collect()
}

fn parse_input(raw: &Vec<String>) -> (HashMap<&str, &str>, HashMap<&str, Vec<&str>>) {
  let mut res = HashMap::new();
  let mut rev_res = HashMap::new();
  for orbit in raw {
    let mut raw = orbit.split(")");
    let center = raw.next().expect("Invalid input!");
    let orbiter = raw.next().expect("Invalid input!");
    res.insert(orbiter, center);
    
    let entries = match rev_res.entry(center) {
      Entry::Occupied(o) => o.into_mut(),
      Entry::Vacant(v) => v.insert(Vec::new())
    };
    entries.push(orbiter);
  }
  (res, rev_res)
}

fn part1() {
  let raw = read_input();
  let (parsed, starts) = parse_input(&raw);

  let direct_orbits = parsed.len();
  let indirect_orbits = 0;

  let iorbit = HashMap::new();
  

  println!("{}", direct_orbits + indirect_orbits);
}

fn main() {
  part1();
}
