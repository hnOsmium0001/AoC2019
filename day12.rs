use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::ops;

fn read_input() -> Vec<String> {
  let file = File::open("../inputs/d12.txt").expect("No input.txt");
  let buf = BufReader::new(file);
  buf.lines()
    .map(|l| l.expect("Could not parse line from input.txt"))
    .collect()
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Vector3i {
  x: i32,
  y: i32,
  z: i32,
}

impl ops::AddAssign<Self> for Vector3i {
    fn add_assign(&mut self, other: Self) {
      self.x += other.x;
      self.y += other.y;
      self.z += other.z;
    }
}

impl ops::Add<Self> for Vector3i {
  type Output = Vector3i;

  fn add(self, rhs: Self) -> Self {
    Vector3i {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    }
  }
}

fn read_moons() -> (Vec<Vector3i>, Vec<Vector3i>) {
  let raw = read_input();
  let mut pos = Vec::new();
  let mut vel = Vec::new();
  for line in raw {
    let data = &line[1..(line.len() - 1)];
    let data = data.replace(&['x', 'y', 'z', '=', ' '][..], "");
    let mut data = data.split(",");
    pos.push(Vector3i {
      x: data.next().unwrap().parse::<i32>().unwrap(),
      y: data.next().unwrap().parse::<i32>().unwrap(),
      z: data.next().unwrap().parse::<i32>().unwrap(),
    });
    vel.push(Vector3i {
      x: 0,
      y: 0,
      z: 0,
    })
  }
  (pos, vel)
}

fn sign_of(n: i32, m: i32) -> i32 {
  if n == m { 0 } else if n < m { 1 } else { -1 }
}

fn count_energy(positions: &Vec<Vector3i>, velocities: &Vec<Vector3i>) -> i32 {
  let mut energy = 0;
  for i in 0..positions.len() {
    let pos = positions[i];
    let kinetic = pos.x.abs() + pos.y.abs() + pos.z.abs();
    let vel = velocities[i];
    let potential = vel.x.abs() + vel.y.abs() + vel.z.abs();
    energy += kinetic * potential;
  }
  energy
}

fn part1() {
  let (mut positions, mut velocities) = read_moons();
  for _ in 0..1000 {
    // Apply gravity
    for i in 0..positions.len() {
      for j in (i + 1)..positions.len() {
        let pos = &positions[i];
        let pos2 = &positions[j];

        let vel = &mut velocities[i];
        vel.x += sign_of(pos.x, pos2.x);
        vel.y += sign_of(pos.y, pos2.y);
        vel.z += sign_of(pos.z, pos2.z);
        let vel = &mut velocities[j];
        vel.x += sign_of(pos2.x, pos.x);
        vel.y += sign_of(pos2.y, pos.y);
        vel.z += sign_of(pos2.z, pos.z);
      }
    }

    // Apply velocity
    for i in 0..positions.len() {
      positions[i] += velocities[i];
    }
  }
  println!("{}", count_energy(&positions, &velocities));
}

fn part2_force_simulate() {
  let (mut positions, mut velocities) = read_moons();
  let original_pos = positions.clone();
  let mut iterations = 0;
  loop {
    // Apply gravity
    for i in 0..positions.len() {
      for j in (i + 1)..positions.len() {
        let pos = &positions[i];
        let pos2 = &positions[j];

        let vel = &mut velocities[i];
        vel.x += sign_of(pos.x, pos2.x);
        vel.y += sign_of(pos.y, pos2.y);
        vel.z += sign_of(pos.z, pos2.z);
        let vel = &mut velocities[j];
        vel.x += sign_of(pos2.x, pos.x);
        vel.y += sign_of(pos2.y, pos.y);
        vel.z += sign_of(pos2.z, pos.z);
      }
    }

    // Apply velocity
    for i in 0..positions.len() {
      positions[i] += velocities[i];
    }

    iterations += 1;

    let mut same = true;
    for (orig, pos) in original_pos.iter().zip(positions.iter()) {
      if orig != pos {
        same = false;
        break;
      }
    }
    if same {
      break;
    }
  }
  println!("{}", iterations);
}

fn part2() {

}

fn main() {
  // part1();
  // part2_force_simulate();
  part2();
}
