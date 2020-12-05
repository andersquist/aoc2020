use super::utils::parse_newline_sep;
use lazy_static::lazy_static;
use regex::Regex;
use std::path::Path;
use std::str::FromStr;

#[derive(parse_display::FromStr, Debug)]
enum Height {
  #[display("{0}cm")]
  Cm(u32),
  #[display("{0}in")]
  In(u32),
}

#[derive(Debug, Default)]
struct Passport {
  byr: Option<u32>,
  iyr: Option<u32>,
  eyr: Option<u32>,
  hgt_set: bool,
  hgt: Option<Height>,
  hcl: Option<String>,
  ecl: Option<String>,
  pid: Option<String>,
}

impl Passport {
  fn is_valid(&self) -> bool {
    self.byr.is_some()
      && self.iyr.is_some()
      && self.eyr.is_some()
      && self.hgt_set
      && self.hcl.is_some()
      && self.ecl.is_some()
      && self.pid.is_some()
  }

  fn is_valid_values_opt(&self) -> Option<bool> {
    lazy_static! {
      static ref HAIR_COLOR_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
      static ref EYE_COLOR_RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
      static ref PASSPORT_ID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
    }
    let valid = (1920..=2002).contains(&self.byr?)
      && (2010..=2020).contains(&self.iyr?)
      && (2020..=2030).contains(&self.eyr?)
      && match self.hgt.as_ref()? {
        Height::Cm(cm) => (150..=193).contains(cm),
        Height::In(inch) => (59..=76).contains(inch),
      }
      && HAIR_COLOR_RE.is_match(self.hcl.as_ref()?)
      && EYE_COLOR_RE.is_match(self.ecl.as_ref()?)
      && PASSPORT_ID_RE.is_match(self.pid.as_ref()?);
    Some(valid)
  }

  fn is_valid_values(&self) -> bool {
    self.is_valid_values_opt().unwrap_or_default()
  }
}

impl FromStr for Passport {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut passport = Passport::default();

    for field in s.split_whitespace() {
      let mut pair = field.split(":");
      let key = pair
        .next()
        .ok_or_else(|| format!("field incorrect {}", field))?;
      let value = pair
        .next()
        .ok_or_else(|| format!("field incorrect {}", field))?;
      if pair.next().is_some() {
        Err(format!("field invalid {}", field))?;
      }

      match key {
        "byr" => passport.byr = Some(value.parse::<u32>().map_err(|e| e.to_string())?),
        "iyr" => passport.iyr = Some(value.parse::<u32>().map_err(|e| e.to_string())?),
        "eyr" => passport.eyr = Some(value.parse::<u32>().map_err(|e| e.to_string())?),
        "hgt" => {
          passport.hgt_set = true;
          passport.hgt = value.parse().ok();
        }
        "hcl" => passport.hcl = Some(value.to_string()),
        "ecl" => passport.ecl = Some(value.to_string()),
        "pid" => passport.pid = Some(value.to_string()),
        _ => {
          // Skip the rest
        }
      }
    }
    Ok(passport)
  }
}

pub fn count_valid_passports() {
  let p = Path::new("inputs/day4.txt");
  if let Ok(passports) = parse_newline_sep::<Passport>(p) {
    println!(
      "Puzzle 1: {} valid passports",
      passports.filter(|passport| passport.is_valid()).count()
    );
  }
}

pub fn count_valid_passports_values() {
  let p = Path::new("inputs/day4.txt");
  if let Ok(passports) = parse_newline_sep::<Passport>(p) {
    println!(
      "Puzzle 2: {} valid passports",
      passports
        .filter(|passport| passport.is_valid_values())
        .count()
    );
  }
}

#[test]
fn creates_a_valid_passport_1() {
  let passport = Passport::from_str(
    "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm",
  )
  .unwrap();
  assert_eq!(passport.is_valid(), true);
}

#[test]
fn creates_an_invalid_passport_2() {
  let passport = Passport::from_str(
    "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929",
  )
  .unwrap();
  assert_eq!(passport.is_valid(), false);
}

#[test]
fn creates_a_valid_passport_3() {
  let passport = Passport::from_str(
    "hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm",
  )
  .unwrap();
  assert_eq!(passport.is_valid(), true);
}

#[test]
fn creates_an_invalid_passport_4() {
  let passport = Passport::from_str(
    "hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in",
  )
  .unwrap();
  assert_eq!(passport.is_valid(), false);
}

// #[test]
// fn count_valid_passports_expect() {
//   assert_eq!(read_from_file("inputs/day4_test.txt"), 2);
// }
