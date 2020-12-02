use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Eq, PartialEq)]
struct Entry {
  lowest: usize,
  highest: usize,
  letter: char,
  password: String,
}

impl Entry {
  fn valid(&self) -> bool {
    let count = self.password.matches(self.letter).count();
    count >= self.lowest && count <= self.highest
  }

  fn valid_second(&self) -> bool {
    let first = parse_char_at(self.password.as_str(), self.lowest - 1);
    let second = parse_char_at(self.password.as_str(), self.highest - 1);

    (first == self.letter && second != self.letter)
      || (first != self.letter && second == self.letter)
  }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

pub fn password_policy() -> (i32, i32, i32) {
  let mut valid_count = 0;
  let mut valid_second_count = 0;
  let mut count = 0;
  if let Ok(lines) = read_lines("inputs/day2.txt") {
    for line in lines {
      count += 1;
      if let Ok(l) = line {
        let entry = parse_line(l.as_str());
        if entry.valid() {
          valid_count += 1
        }
        if entry.valid_second() {
          valid_second_count += 1
        }
      }
    }
  }
  (valid_count, valid_second_count, count)
}

fn parse_usize(str_val: &str) -> usize {
  match str_val.parse::<usize>() {
    Ok(value) => value,
    _ => panic!(format!("Illegal value: [{}]", str_val)),
  }
}

fn parse_char(str_val: &str) -> char {
  match str_val.chars().nth(0) {
    Some(c) => c,
    _ => panic!(format!("Illegal char: [{}]", str_val)),
  }
}

fn parse_char_at(str_val: &str, pos: usize) -> char {
  match str_val.chars().nth(pos) {
    Some(c) => c,
    _ => panic!(format!("Illegal char: [{}]", str_val)),
  }
}

fn parse_line(policy: &str) -> Entry {
  let first_part: Vec<&str> = policy.split(":").collect();
  let second_part: Vec<&str> = policy.split(" ").collect();
  let min_max: Vec<&str> = second_part[0].split("-").collect();

  Entry {
    lowest: parse_usize(min_max[0]),
    highest: parse_usize(min_max[1]),
    letter: parse_char(second_part[1]),
    password: String::from(first_part[1].trim()),
  }
}

#[test]
fn parse_line_expected() {
  assert_eq!(
    parse_line("1-3 a: abcde"),
    Entry {
      lowest: 1,
      highest: 3,
      letter: 'a',
      password: String::from("abcde")
    }
  );
  assert_eq!(
    parse_line("1-3 b: cdefg"),
    Entry {
      lowest: 1,
      highest: 3,
      letter: 'b',
      password: String::from("cdefg")
    }
  );
  assert_eq!(
    parse_line("2-9 c: ccccccccc"),
    Entry {
      lowest: 2,
      highest: 9,
      letter: 'c',
      password: String::from("ccccccccc")
    }
  );
}

#[test]
fn validates_expected() {
  let entry1 = Entry {
    lowest: 1,
    highest: 3,
    letter: 'a',
    password: String::from("abcde"),
  };
  assert_eq!(entry1.valid(), true);
  let entry2 = Entry {
    lowest: 1,
    highest: 3,
    letter: 'b',
    password: String::from("cdefg"),
  };
  assert_eq!(entry2.valid(), false);
  let entry3 = Entry {
    lowest: 2,
    highest: 9,
    letter: 'c',
    password: String::from("ccccccccc"),
  };
  assert_eq!(entry3.valid(), true);
}

#[test]
fn validate_second_expected() {
  let entry1 = Entry {
    lowest: 1,
    highest: 3,
    letter: 'a',
    password: String::from("abcde"),
  };
  assert_eq!(entry1.valid_second(), true);
  let entry2 = Entry {
    lowest: 1,
    highest: 3,
    letter: 'b',
    password: String::from("cdefg"),
  };
  assert_eq!(entry2.valid_second(), false);
  let entry3 = Entry {
    lowest: 2,
    highest: 9,
    letter: 'c',
    password: String::from("ccccccccc"),
  };
  assert_eq!(entry3.valid_second(), false);
}
