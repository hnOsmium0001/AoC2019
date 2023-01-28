use std::collections::VecDeque;
use std::fs;

const ADD_INS: i32 = 1;
const ADD_SIZE: usize = 4;
const MULT_INS: i32 = 2;
const MULT_SIZE: usize = 4;
const INP_INS: i32 = 3;
const INP_SIZE: usize = 2;
const OUT_INS: i32 = 4;
const OUT_SIZE: usize = 2;
const JMPT_INS: i32 = 5;
const JMPT_SIZE: usize = 3;
const JMPF_INS: i32 = 6;
const JMPF_SIZE: usize = 3;
const TLS_INS: i32 = 7;
const TLS_SIZE: usize = 4;
const TEQ_INS: i32 = 8;
const TEQ_SIZE: usize = 4;
const HALT_INS: i32 = 99;
const HALT_SIZE: usize = 1; 

fn read_input() -> Vec<i32> {
  let input = fs::read_to_string("inputs/day05.txt").expect("No input.txt found");
  input.split(",")
    .map(|op| op.parse::<i32>().unwrap())
    .collect()
}

const TRIMMERS: [i32; 6] = [1, 10, 100, 1000, 10000, 100000];
fn digit_at(n: i32, i: usize) -> i32 {
  (n / TRIMMERS[i]) % 10
}

const POSITION: i32 = 0;
const IMMEDIATE: i32 = 1;

fn get_param(mem: &Vec<i32>, idx: usize, ofst: usize, modes: i32) -> i32 {
  match digit_at(modes, ofst - 1) {
    POSITION => mem[mem[idx + ofst] as usize],
    IMMEDIATE => mem[idx + ofst],
    _ => panic!(),
  }
}

fn execute(inputs: &mut VecDeque<i32>) -> Vec<i32> {
  let mut out = Vec::new();
  let mut mem = read_input();
  let mut iptr: usize = 0;
  loop {
    let ins = mem[iptr] % 100;
    let modes = (mem[iptr] - ins) / 100;
    match ins {
      ADD_INS => {
        let addr_res = mem[iptr + 3] as usize;
        mem[addr_res] =
          get_param(&mem, iptr, 1, modes) +
          get_param(&mem, iptr, 2, modes);
        iptr += ADD_SIZE;
      },
      MULT_INS => {
        let addr_res = mem[iptr + 3] as usize;
        mem[addr_res] =
          get_param(&mem, iptr, 1, modes) *
          get_param(&mem, iptr, 2, modes);
        iptr += MULT_SIZE;
      },
      INP_INS => {
        let addr_res = mem[iptr + 1] as usize;
        mem[addr_res] = inputs.pop_back().unwrap_or(0);
        iptr += INP_SIZE;
      },
      OUT_INS => {
        out.push(get_param(&mem, iptr, 1, modes));
        iptr += OUT_SIZE;
      },
      JMPT_INS => {
        if get_param(&mem, iptr, 1, modes) != 0 {
          iptr = get_param(&mem, iptr, 2, modes) as usize;
        } else {
          iptr += JMPT_SIZE;
        }
      },
      JMPF_INS => {
        if get_param(&mem, iptr, 1, modes) == 0 {
          iptr = get_param(&mem, iptr, 2, modes) as usize;
        } else {
          iptr += JMPF_SIZE;
        }
      },
      TLS_INS => {
        let par1 = get_param(&mem, iptr, 1, modes);
        let par2 = get_param(&mem, iptr, 2, modes);
        let addr_res = mem[iptr + 3] as usize;
        mem[addr_res] = if par1 < par2 { 1 } else { 0 };
        iptr += TLS_SIZE;
      },
      TEQ_INS => {
        let par1 = get_param(&mem, iptr, 1, modes);
        let par2 = get_param(&mem, iptr, 2, modes);
        let addr_res = mem[iptr + 3] as usize;
        mem[addr_res] = if par1 == par2 { 1 } else { 0 };
        iptr += TEQ_SIZE;
      },
      HALT_INS => break,
      _ => (),
    };
  }
  out
}

fn part1() {
  let mut inputs = VecDeque::new();
  inputs.push_back(1);
  let out = execute(&mut inputs);
  println!("{:#?}", out);
}

fn part2() {
  let mut inputs = VecDeque::new();
  inputs.push_back(5);
  let out = execute(&mut inputs);
  println!("{:#?}", out);
}

fn main() {
  // part1();
  part2();
}
