use super::utils::parse_singleline_sep;
use lazy_static::lazy_static;
use regex::Regex;
use std::path::Path;
use std::str::FromStr;

lazy_static! {
  static ref SEAT_RE: Regex = Regex::new(r"^([F|B]{7})([L|R]{3})$").unwrap();
}

#[derive(Debug, Default, Eq, PartialEq)]
struct Seat {
  row: u32,
  column: u32,
  id: u32,
}

impl FromStr for Seat {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut seat = Seat::default();
    let caps = SEAT_RE
      .captures(s.trim())
      .ok_or_else(|| format!("No regex match for [{}]", s))?;

    let front_back = String::from(
      caps
        .get(1)
        .ok_or_else(|| format!("No front/back for [{}]", s))?
        .as_str(),
    )
    .replace("F", "0")
    .replace("B", "1");
    let left_right = String::from(
      caps
        .get(2)
        .ok_or_else(|| format!("No left/right for [{}]", s))?
        .as_str(),
    )
    .replace("L", "0")
    .replace("R", "1");
    seat.row = u32::from_str_radix(front_back.as_str(), 2).unwrap_or_default();
    seat.column = u32::from_str_radix(left_right.as_str(), 2).unwrap_or_default();
    seat.id = seat.row * 8 + seat.column;
    Ok(seat)
  }
}

pub fn day_5_puzzle_1() {
  let p = Path::new("inputs/day5.txt");
  if let Ok(seats) = parse_singleline_sep::<Seat>(p) {
    println!(
      "Puzzle 1: max id is {}",
      seats.map(|s| s.id).max().unwrap_or_default()
    );
  }
}

pub fn day_5_puzzle_2() {
  let p = Path::new("inputs/day5.txt");
  if let Ok(seats) = parse_singleline_sep::<Seat>(p) {
    let seat_ids: Vec<u32> = seats.map(|s| s.id).map(|v| v).collect();
    let first = seat_ids.iter().min().unwrap_or_else(|| &0);
    let last = seat_ids.iter().max().unwrap_or_else(|| &0);
    let mut our_seat: u32 = 0;
    for v in *first..=*last {
      if !seat_ids.contains(&v) {
        our_seat = v;
      }
    }
    println!("Puzzle 2:{}", our_seat);
  }
}

#[test]
fn test_seat_from_str1() {
  let seat = Seat::from_str("BFFFBBFRRR").unwrap();
  println!("{:?}", seat);
  assert_eq!(
    seat,
    Seat {
      row: 70,
      column: 7,
      id: 567
    }
  );
}

#[test]
fn test_seat_from_str2() {
  let seat = Seat::from_str("FFFBBBFRRR").unwrap();
  println!("{:?}", seat);
  assert_eq!(
    seat,
    Seat {
      row: 14,
      column: 7,
      id: 119
    }
  );
}

#[test]
fn test_seat_from_str3() {
  let seat = Seat::from_str("BBFFBBFRLL").unwrap();
  println!("{:?}", seat);
  assert_eq!(
    seat,
    Seat {
      row: 102,
      column: 4,
      id: 820
    }
  );
}
