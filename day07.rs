use std::collections::VecDeque;
use std::fs;
use std::cmp;

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
  let input = fs::read_to_string("../inputs/d7.txt").expect("No input.txt found");
  input.split(",")
    .map(|op| op.parse::<i32>().unwrap())
    .collect()
}

const TRIMMERS: [i32; 10] = [1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000];
fn digit_at(n: i32, i: usize) -> i32 {
  (n / TRIMMERS[i]) % 10
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
  Idle,
  Running,
  Interrupted,
  Halted,
}

impl State {
  fn can_continue(&self) -> bool {
    match self {
      State::Idle | State::Running => true,
      State::Interrupted | State::Halted => false, 
    }
  }
}

#[derive(Debug, Clone)]
struct Interpreter {
  mem: Vec<i32>,
  stdin: VecDeque<i32>,
  stdout: Vec<i32>,

  state: State,
  iptr: usize,
}

const POSITION: i32 = 0;
const IMMEDIATE: i32 = 1;

impl Interpreter {
  fn new(mem: Vec<i32>) -> Interpreter {
    Interpreter {
      mem: mem,
      stdin: VecDeque::new(),
      stdout: Vec::new(),
      state: State::Idle,
      iptr: 0,
    }
  }

  fn get_param(&self, idx: usize, ofst: usize, modes: i32) -> i32 {
    match digit_at(modes, ofst - 1) {
      POSITION => self.mem[self.mem[idx + ofst] as usize],
      IMMEDIATE => self.mem[idx + ofst],
      _ => panic!(),
    }
  }

  fn execute(&mut self) {
    loop {
      self.step();
      if !self.state.can_continue() {
        break;
      }
    }
  }

  fn step(&mut self) {
    self.state = State::Running;

    let ins = self.mem[self.iptr] % 100;
    let modes = (self.mem[self.iptr] - ins) / 100;
    match ins {
      ADD_INS => {
        let addr_res = self.mem[self.iptr + 3] as usize;
        self.mem[addr_res] =
          self.get_param(self.iptr, 1, modes) +
          self.get_param(self.iptr, 2, modes);
        self.iptr += ADD_SIZE;
      },
      MULT_INS => {
        let addr_res = self.mem[self.iptr + 3] as usize;
        self.mem[addr_res] =
          self.get_param(self.iptr, 1, modes) *
          self.get_param(self.iptr, 2, modes);
        self.iptr += MULT_SIZE;
      },
      INP_INS => {
        let addr_res = self.mem[self.iptr + 1] as usize;
        match self.stdin.pop_front() {
          Some(inp) => {
            self.mem[addr_res] = inp;
            self.iptr += INP_SIZE;
          },
          None => {
            self.state = State::Interrupted;
            return;
          },
        }
      },
      OUT_INS => {
        self.stdout.push(self.get_param(self.iptr, 1, modes));
        self.iptr += OUT_SIZE;
      },
      JMPT_INS => {
        if self.get_param(self.iptr, 1, modes) != 0 {
          self.iptr = self.get_param(self.iptr, 2, modes) as usize;
        } else {
          self.iptr += JMPT_SIZE;
        }
      },
      JMPF_INS => {
        if self.get_param(self.iptr, 1, modes) == 0 {
          self.iptr = self.get_param(self.iptr, 2, modes) as usize;
        } else {
          self.iptr += JMPF_SIZE;
        }
      },
      TLS_INS => {
        let par1 = self.get_param(self.iptr, 1, modes);
        let par2 = self.get_param(self.iptr, 2, modes);
        let addr_res = self.mem[self.iptr + 3] as usize;
        self.mem[addr_res] = if par1 < par2 { 1 } else { 0 };
        self.iptr += TLS_SIZE;
      },
      TEQ_INS => {
        let par1 = self.get_param(self.iptr, 1, modes);
        let par2 = self.get_param(self.iptr, 2, modes);
        let addr_res = self.mem[self.iptr + 3] as usize;
        self.mem[addr_res] = if par1 == par2 { 1 } else { 0 };
        self.iptr += TEQ_SIZE;
      },
      HALT_INS => {
        self.state = State::Halted;
        return;
      },
      _ => panic!("Invalid instruction {}!", ins),
    };

    self.state = State::Idle;
  }
}

fn part1() {
  let opcodes = read_input();
  let mut max = 0;
  for i in 0..5 {
    for j in 0..5 {
      if i == j {
        continue;
      }
      for k in 0..5 {
        if i == k || j == k {
          continue;
        }
        for m in 0..5 {
          if i == m || j == m || k == m {
            continue;
          }
          for n in 0..5 {
            if i == n || j == n || k == n || m == n {
              continue;
            }

            let phases = [i, j, k, m, n];
            let mut signal = 0;
            for ii in 0..5 {
              let mut interpreter = Interpreter::new(opcodes.clone());
              interpreter.stdin.push_back(phases[ii]);
              interpreter.stdin.push_back(signal);
              interpreter.execute();
              signal = *interpreter.stdout.last().expect("No output from amplifier program!");
            }
            max = cmp::max(max, signal);
          }
        }
      }
    }
  }
  println!("{}", max);
}

fn part2() {
  let opcodes = read_input();
  let mut max = 0;
  for i in 5..10 {
    for j in 5..10 {
      if i == j {
        continue;
      }
      for k in 5..10 {
        if i == k || j == k {
          continue;
        }
        for m in 5..10 {
          if i == m || j == m || k == m {
            continue;
          }
          for n in 5..10 {
            if i == n || j == n || k == n || m == n {
              continue;
            }

            let phases = [i, j, k, m, n];
            let mut instances = vec![Interpreter::new(opcodes.clone()); 5];
            let last_idx = instances.len() - 1;

            let mut idx = 0;
            let mut signal = 0;
            loop {
              let interpreter = &mut instances[idx];
              // First time running this instance
              if interpreter.state == State::Idle {
                interpreter.stdin.push_back(phases[idx]);
              }
              interpreter.stdin.push_back(signal);

              // Execute the program until halt or interrupt
              interpreter.execute();
              // Even though we don't explicitly wait for outputs, it is guaranteed (AoC description)
              // that the program will output a signal before it requests the next input signal
              match interpreter.state {
                // When the last amplifier halts, all other amplifiers must halted too
                // therefore we can safely take the final output now
                State::Halted if idx == last_idx => break,
                State::Halted | State::Interrupted => signal = *interpreter.stdout.last().expect("No output from amplifier program!"),
                _ => panic!("Invalid state after executing to halt or interrupted!"),
              }
              idx = if idx + 1 < instances.len() { idx + 1 } else { 0 };
            }
            
            max = cmp::max(max, signal);
          }
        }
      }
    }
  }
  println!("{}", max);
}

fn main() {
  // part1();
  part2();
}
