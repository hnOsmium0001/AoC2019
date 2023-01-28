use std::fs;

const ADD: usize = 1;
const MULT: usize = 2;
const HALT: usize = 99;

fn read_input() -> Vec<usize> {
  let input = fs::read_to_string("inputs/day02.txt").expect("No input.txt found");
  input.split(",")
    .map(|op| op.parse::<usize>().unwrap())
    .collect()
}

fn execute(noun: usize, verb: usize) -> usize {
  let mut mem = read_input();
  mem[1] = noun;
  mem[2] = verb;

  let mut iptr: usize = 0;
  loop {
    match mem[iptr] {
      ADD => {
        let addr_oprd1 = mem[iptr + 1];
        let addr_oprd2 = mem[iptr + 2];
        let addr_res = mem[iptr + 3];
        mem[addr_res] = mem[addr_oprd1] + mem[addr_oprd2];
      },
      MULT => {
        let addr_oprd1 = mem[iptr + 1];
        let addr_oprd2 = mem[iptr + 2];
        let addr_res = mem[iptr + 3];
        mem[addr_res] = mem[addr_oprd1] * mem[addr_oprd2];
      },
      HALT => break,
      _ => (),
    }
    iptr += 4;
  }

  mem[0]
}

fn part1() {
  println!("{}", execute(12, 2));
}

fn part2() {
  for n in 0..=99 {
    for v in 0..=99 {
      let res = execute(n, v);
      if res == 19690720 {
        println!("noun: {}, verb: {}", n, v);
        return;
      }
    }
  }
}

fn main() {
  // part1();
  part2();
}
