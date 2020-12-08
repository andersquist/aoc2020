use super::utils::parse;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

lazy_static! {
  static ref CMD_RE: Regex = Regex::new(r"^(jmp|nop|acc) ([+-]\d+)$").unwrap();
}

#[derive(Debug, Default)]
struct Instr {
  cmd: Option<Cmd>,
}

impl FromStr for Instr {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut instr = Instr::default();
    let caps = CMD_RE
      .captures(s)
      .ok_or_else(|| format!("No regex match for [{}]", s))?;
    let cmd = caps.get(1).map_or("", |m| m.as_str());
    let val = caps
      .get(2)
      .map_or("", |m| m.as_str())
      .replace("+", "")
      .parse::<i32>()
      .map_err(|e| e.to_string())?;
    instr.cmd = match cmd {
      "nop" => Some(Cmd::Nop(val)),
      "acc" => Some(Cmd::Acc(val)),
      "jmp" => Some(Cmd::Jmp(val)),
      _ => None,
    };

    Ok(instr)
  }
}

// Runs the code and returns if a loop is detected
// or it has excuted the last instruction
fn run_code(instructions: &Vec<Cmd>) -> (i32, bool) {
  let mut acc = 0;
  let mut pointer: i32 = 0;
  let mut exec_count = HashSet::new();

  loop {
    exec_count.insert(pointer);
    match &instructions[pointer as usize] {
      Cmd::Acc(val) => {
        acc += val;
        pointer += 1;
      }
      Cmd::Jmp(val) => {
        pointer += val;
      }
      _ => {
        pointer += 1;
      }
    }
    if exec_count.contains(&pointer) {
      return (acc, true);
    } else if pointer == instructions.len() as i32 - 1 {
      return (acc, false);
    }
  }
}

pub fn day_8_puzzle_2() -> Result<(), Error> {
  let p = Path::new("inputs/day8.txt");
  let mut cmds: Vec<Cmd> = parse::<Instr>(&p)?.map(|i| i.cmd.unwrap()).collect();

  for i in 0..cmds.len() {
    let new_cmd = match &cmds[i] {
      Cmd::Nop(v) => Some(Cmd::Jmp(*v)),
      Cmd::Jmp(v) => Some(Cmd::Nop(*v)),
      _ => None,
    };
    if new_cmd.is_some() {
      let prev = std::mem::replace(&mut cmds[i], new_cmd.unwrap());
      let (acc, infinite) = run_code(&cmds);
      if !infinite {
        println!("Terminated with acc: {}", acc);
        return Ok(());
      }
      let _new = std::mem::replace(&mut cmds[i], prev);
    }
  }
  println!("No solution found!");
  Ok(())
}

pub fn day_8_puzzle_1() -> Result<(), Error> {
  let p = Path::new("inputs/day8.txt");
  let cmds = parse::<Instr>(&p)?.map(|i| i.cmd.unwrap()).collect();
  let (acc, _) = run_code(&cmds);
  println!("Answer 1:{}", acc);
  Ok(())
}

#[derive(Debug)]
enum Cmd {
  Nop(i32),
  Acc(i32),
  Jmp(i32),
}

#[derive(Debug, Error)]
pub enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),
}

#[test]
fn test_run_code() {
  let inst = vec![
    Cmd::Nop(0),
    Cmd::Acc(1),
    Cmd::Jmp(4),
    Cmd::Acc(3),
    Cmd::Jmp(-3),
    Cmd::Acc(-99),
    Cmd::Acc(1),
    Cmd::Jmp(-4),
    Cmd::Acc(6),
  ];
  let (acc, _) = run_code(&inst);
  println!("{}", acc);
}
