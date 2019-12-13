use std::collections::{HashSet, VecDeque};
use std::fs;
use std::ops;

const ADD_INS: i64 = 1;
const ADD_SIZE: usize = 4;
const MULT_INS: i64 = 2;
const MULT_SIZE: usize = 4;
const INP_INS: i64 = 3;
const INP_SIZE: usize = 2;
const OUT_INS: i64 = 4;
const OUT_SIZE: usize = 2;
const JMPT_INS: i64 = 5;
const JMPT_SIZE: usize = 3;
const JMPF_INS: i64 = 6;
const JMPF_SIZE: usize = 3;
const TLS_INS: i64 = 7;
const TLS_SIZE: usize = 4;
const TEQ_INS: i64 = 8;
const TEQ_SIZE: usize = 4;
const SRL_INS: i64 = 9;
const SRL_SIZE: usize = 2;
const HALT_INS: i64 = 99;
const HALT_SIZE: usize = 1; 

fn read_input() -> Vec<i64> {
  let input = fs::read_to_string("../inputs/d11.txt").expect("No input.txt found");
  input.split(",")
    .map(|op| op.parse::<i64>().unwrap())
    .collect()
}

const TRIMMERS: [i64; 10] = [1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000];
fn digit_at(n: i64, i: usize) -> i64 {
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
  mem: Vec<i64>,
  stdin: VecDeque<i64>,
  stdout: VecDeque<i64>,

  state: State,
  iptr: usize,
  rptr: usize,
}

const POSITION: i64 = 0;
const IMMEDIATE: i64 = 1;
const RELATIVE: i64 = 2;

impl Interpreter {
  fn new(mem: Vec<i64>) -> Interpreter {
    Interpreter {
      mem: mem,
      stdin: VecDeque::new(),
      stdout: VecDeque::new(),
      state: State::Idle,
      iptr: 0,
      rptr: 0,
    }
  }

  fn get_param(&self, ofst: usize, modes: i64) -> i64 {
    match digit_at(modes, ofst - 1) {
      POSITION => {
        let addr = self.mem[self.iptr + ofst];
        self.mem[addr as usize]
      },
      IMMEDIATE => self.mem[self.iptr + ofst],
      RELATIVE => {
        // Preserve sign for computation of the offset and address
        let rptr_ofst = self.mem[self.iptr + ofst];
        let addr = self.rptr as i64 + rptr_ofst;
        self.mem[addr as usize]
      },
      _ => panic!(),
    }
  }

  fn get_addr(&self, ofst: usize, modes: i64) -> usize {
    match digit_at(modes, ofst - 1) {
      POSITION => self.mem[self.iptr + ofst] as usize,
      RELATIVE => {
        let rptr_ofst = self.mem[self.iptr + ofst];
        let addr = self.rptr as i64 + rptr_ofst;
        addr as usize
      },
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
        let addr_res = self.get_addr(3, modes);
        self.mem[addr_res] =
          self.get_param(1, modes) +
          self.get_param(2, modes);
        self.iptr += ADD_SIZE;
      },
      MULT_INS => {
        let addr_res = self.get_addr(3, modes);
        self.mem[addr_res] =
          self.get_param(1, modes) *
          self.get_param(2, modes);
        self.iptr += MULT_SIZE;
      },
      INP_INS => {
        let addr_res = self.get_addr(1, modes);
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
        self.stdout.push_front(self.get_param(1, modes));
        self.iptr += OUT_SIZE;
      },
      JMPT_INS => {
        if self.get_param(1, modes) == 1 {
          self.iptr = self.get_param(2, modes) as usize;
        } else {
          self.iptr += JMPT_SIZE;
        }
      },
      JMPF_INS => {
        if self.get_param(1, modes) == 0 {
          self.iptr = self.get_param(2, modes) as usize;
        } else {
          self.iptr += JMPF_SIZE;
        }
      },
      TLS_INS => {
        let par1 = self.get_param(1, modes);
        let par2 = self.get_param(2, modes);
        let addr_res = self.get_addr(3, modes);
        self.mem[addr_res] = if par1 < par2 { 1 } else { 0 };
        self.iptr += TLS_SIZE;
      },
      TEQ_INS => {
        let par1 = self.get_param(1, modes);
        let par2 = self.get_param(2, modes);
        let addr_res = self.get_addr(3, modes);
        self.mem[addr_res] = if par1 == par2 { 1 } else { 0 };
        self.iptr += TEQ_SIZE;
      },
      SRL_INS => {
        let i = self.get_param(1, modes);
        let new_rptr = self.rptr as i64 + i;
        self.rptr = new_rptr as usize;
        self.iptr += SRL_SIZE;
      },
      HALT_INS => {
        self.state = State::Halted;
        return;
      },
      _ => panic!("Invalid instruction {}!", ins),
    };

    self.state = State::Idle;
  }

  fn try_pop_output(&mut self) -> Option<i64> {
    self.stdout.pop_back()
  }

  fn pop_output(&mut self) -> i64 {
    self.stdout.pop_back().expect("No output from program!")
  }
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Vector2i {
  x: i64,
  y: i64,
}

impl ops::AddAssign<Self> for Vector2i {
    fn add_assign(&mut self, other: Self) {
      self.x += other.x;
      self.y += other.y;
    }
}

impl ops::Add<Self> for Vector2i {
  type Output = Vector2i;

  fn add(self, rhs: Self) -> Self {
    Vector2i {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    }
  }
}

impl Vector2i {
  fn to_idx(&self, canvas: &Canvas) -> usize {
    (self.x + self.y * canvas.width as i64) as usize
  }
}

struct Canvas {
  map: Vec<i64>,
  width: usize,
  height: usize,
}

impl ops::Index<usize> for Canvas {
  type Output = i64;

  fn index(&self, i: usize) -> &Self::Output {
    &self.map[i]
  }
}

impl ops::IndexMut<usize> for Canvas {
  fn index_mut(&mut self, i: usize) -> &mut Self::Output {
    &mut self.map[i]
  }
}

impl Canvas {
  fn new(width: usize, height: usize) -> Canvas {
    Canvas {
      map: vec![0; width * height],
      width: width,
      height: height,
    }
  }
}

struct Turtle {
  canvas: Canvas,
  pos: Vector2i,
  dir: Vector2i,
  controller: Interpreter,
}

const LEFT: i64 = 0;
const RIGHT: i64 = 1;

fn rotate_vec(vec: &Vector2i, op: i64) -> Vector2i {
  match op {
    LEFT => Vector2i { x: -vec.y, y: vec.x },
    RIGHT => Vector2i { x: vec.y, y: -vec.x },
    _ => panic!("Invalid operation {}!", op),
  }
}

impl Turtle {
  fn new(mem: Vec<i64>, width: usize, height: usize) -> Turtle {
    Turtle {
      canvas: Canvas::new(width, height),
      pos: Vector2i {
        x: width as i64 / 2,
        y: height as i64 / 2
      },
      dir: Vector2i { x: 0, y: 1 }, // Facing up
      controller: Interpreter::new(mem),
    }
  }

  fn execute<F>(&mut self, mut on_paint: F)
    where F: FnMut(usize) {
    if self.controller.state == State::Halted {
      return;
    }

    loop {
      self.controller.step();
      match self.controller.state {
        State::Halted => return,
        State::Interrupted => {
          let idx = self.pos.to_idx(&self.canvas);
          self.controller.stdin.push_back(self.canvas[idx]);
        },
        State::Idle => if self.controller.stdout.len() >= 2 {
          let idx = self.pos.to_idx(&self.canvas);
          // Paint the canvas
          self.canvas[idx] = self.controller.pop_output();
          // Rotate the robot
          let op = self.controller.pop_output();
          self.dir = rotate_vec(&self.dir, op);
          // Move the robot
          self.pos += self.dir;

          on_paint(idx);
        },
        State::Running => panic!("Interpretere didn't finish running!"),
      }
    }
  }
}

fn print_canvas(canvas: &Canvas) {
  let mut map = String::new();

  let w = canvas.width as i64;
  let h = canvas.height as i64;
  let mut curr = Vector2i { x: 0, y: h - 1 };
  loop {
    let i = curr.to_idx(&canvas);

    map.push_str(match canvas.map[i] {
      0 => ".",
      1 => "#",
      _ => panic!(),
    });

    curr.x += 1;
    if curr.x >= w {
      curr.x = 0;
      curr.y -= 1;
      map.push_str("\n");
      if curr.y < 0 {
        break;
      }
    }
  }

  println!("{}", map);
}

fn part1() {
  let mut opcodes = read_input();
  for _ in 0..1024 {
    opcodes.push(0);
  }
  
  let mut turtle = Turtle::new(opcodes, 128, 128);
  let mut painted_pos = HashSet::new();
  turtle.execute(|i| {
    painted_pos.insert(i);
  });

  print_canvas(&turtle.canvas);
  println!("{}", painted_pos.len());
}

fn part2() {
  let mut opcodes = read_input();
  for _ in 0..1024 {
    opcodes.push(0);
  }
  
  let mut turtle = Turtle::new(opcodes, 128, 128);

  // Start on a white panel
  let middle = 128 / 2;
  turtle.canvas.map[middle + middle * turtle.canvas.width] = 1;

  turtle.execute(|i| {});

  print_canvas(&turtle.canvas);
}

fn main() {
  // part1();
  part2();
}
