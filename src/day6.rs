use super::utils::parse_newline_sep;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Default, Eq, PartialEq)]
struct Form {
  positive_answers: usize,
  common_answers: usize,
}

impl FromStr for Form {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut form = Form::default();
    let mut group_answers = HashSet::new();
    let mut common_answers = HashMap::new();
    let mut groups = 0;

    for person in s.trim().split("\n") {
      groups += 1;
      for a in person.trim().chars() {
        group_answers.insert(a);
        let count = common_answers.entry(a).or_insert(0);
        *count += 1;
      }
    }
    //println!("{:?} groups:{}", common_answers, groups);
    form.positive_answers = group_answers.len();
    form.common_answers = common_answers.values().filter(|v| **v == groups).count();

    if form.common_answers == 0 {
      println!("No common!{} {:?} {:?} {}", s, form, common_answers, groups);
    }
    Ok(form)
  }
}

pub fn day_6_puzzle_1() {
  let p = Path::new("inputs/day6.txt");
  if let Ok(forms) = parse_newline_sep::<Form>(p) {
    println!(
      "Puzzle 1: {} sum of answers",
      forms.map(|f| f.positive_answers).sum::<usize>()
    );
  }
}

pub fn day_6_puzzle_2() {
  let p = Path::new("inputs/day6.txt");
  if let Ok(forms) = parse_newline_sep::<Form>(p) {
    println!(
      "Puzzle 2: {} sum of common answers",
      forms.map(|f| f.common_answers).sum::<usize>()
    );
  }
}

#[test]
fn test_form_from_str1() {
  let form = Form::from_str("abc").unwrap();
  println!("{:?}", form);
  assert_eq!(
    form,
    Form {
      positive_answers: 3,
      common_answers: 3
    }
  );
}

#[test]
fn test_form_from_str2() {
  let form = Form::from_str(
    "a
b
c",
  )
  .unwrap();
  println!("{:?}", form);
  assert_eq!(
    form,
    Form {
      positive_answers: 3,
      common_answers: 0
    }
  );
}

#[test]
fn test_form_from_str3() {
  let form = Form::from_str(
    "ab
ac",
  )
  .unwrap();
  println!("{:?}", form);
  assert_eq!(
    form,
    Form {
      positive_answers: 3,
      common_answers: 1
    }
  );
}

#[test]
fn test_form_from_str4() {
  let form = Form::from_str(
    "a
a
a
a",
  )
  .unwrap();
  println!("{:?}", form);
  assert_eq!(
    form,
    Form {
      positive_answers: 1,
      common_answers: 1
    }
  );
}

#[test]
fn test_form_from_str5() {
  let form = Form::from_str("b").unwrap();
  println!("{:?}", form);
  assert_eq!(
    form,
    Form {
      positive_answers: 1,
      common_answers: 1
    }
  );
}

#[test]
fn test_form_from_str6() {
  let form = Form::from_str(
    "mnwzxtdeh
nphdzwe
ehyxndwz
dxnwhez
dnwbhvez",
  )
  .unwrap();
  println!("{:?}", form);
  assert_eq!(
    form,
    Form {
      positive_answers: 13,
      common_answers: 6
    }
  );
}
