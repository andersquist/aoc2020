use super::utils::read_lines;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

lazy_static! {
  static ref BAG_RE: Regex = Regex::new(r"^(\d+) (.+) bags?").unwrap();
}

#[derive(Debug, Default, Eq, PartialEq)]
struct BagRule {
  bag: String,
  rules: HashMap<String, u32>,
}

impl FromStr for BagRule {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut rules = HashMap::new();
    let mut pair = s.split("bags contain");
    let bag = pair
      .next()
      .ok_or_else(|| format!("bag rule incorrect: {}", s))?;
    let bags = pair
      .next()
      .ok_or_else(|| format!("bag rule incorrect: {}", s))?;

    if pair.next().is_some() {
      Err(format!("field invalid {}", s))?;
    }

    for bag_rule in bags.trim().split(",") {
      if bag_rule == "no other bags." {
        break;
      }
      let caps = BAG_RE
        .captures(bag_rule.trim())
        .ok_or_else(|| format!("No regex match for [{}]", bag_rule))?;

      let amount_str = String::from(
        caps
          .get(1)
          .ok_or_else(|| format!("No amount for [{}]", bag_rule))?
          .as_str(),
      );
      let bag_color = String::from(
        caps
          .get(2)
          .ok_or_else(|| format!("No color for [{}]", bag_rule))?
          .as_str(),
      );
      let amount = amount_str.parse::<u32>().map_err(|e| e.to_string())?;
      rules.insert(bag_color, amount);
    }

    Ok(BagRule {
      bag: bag.trim().to_string(),
      rules,
    })
  }
}

fn count_rec(
  bag: &str,
  rules: &HashMap<String, HashMap<String, u32>>,
  counted: &mut HashSet<String>,
) -> usize {
  let mut count = 0;
  let mut count_set = counted;
  for (bag_name, _) in rules
    .into_iter()
    .filter(|(_k, v)| v.contains_key(&bag.to_string()))
  {
    if !count_set.contains(bag_name) {
      count += 1;
      count_set.insert(bag_name.to_string());
    }
    count += count_rec(bag_name, rules, &mut count_set);
  }
  count
}

fn count_rec_total(
  bag: &str,
  rules: &HashMap<String, HashMap<String, u32>>,
  init_count: usize,
) -> usize {
  let mut count = init_count;
  let nested_rules = rules.get(&bag.to_string()).unwrap();
  for (k, v) in nested_rules.iter() {
    let mut sub_count = count_rec_total(k, rules, 1);
    if sub_count == 0 {
      sub_count = 1;
    }
    count += *v as usize * sub_count;
  }
  count
}

#[test]
fn parse_to_hash() {
  let mut rules: HashMap<String, HashMap<String, u32>> = HashMap::new();
  if let Ok(lines) = read_lines("inputs/day7_test.txt") {
    for line in lines {
      if let Ok(l) = line {
        let rule = BagRule::from_str(l.trim()).unwrap();
        rules.insert(rule.bag, rule.rules);
      }
    }
  }
  let mut counted = HashSet::new();
  let test = count_rec("shiny gold", &rules, &mut counted);
  println!("Count: {}", test);
}

pub fn day_7_puzzle_2() {
  let mut rules: HashMap<String, HashMap<String, u32>> = HashMap::new();
  if let Ok(lines) = read_lines("inputs/day7.txt") {
    for line in lines {
      if let Ok(l) = line {
        let rule = BagRule::from_str(l.trim()).unwrap();
        rules.insert(rule.bag, rule.rules);
      }
    }
  }
  let test = count_rec_total("shiny gold", &rules, 0);
  println!("Count: {}", test);
}

pub fn day_7_puzzle_1() {
  let mut rules: HashMap<String, HashMap<String, u32>> = HashMap::new();
  if let Ok(lines) = read_lines("inputs/day7.txt") {
    for line in lines {
      if let Ok(l) = line {
        let rule = BagRule::from_str(l.trim()).unwrap();
        rules.insert(rule.bag, rule.rules);
      }
    }
  }
  let mut counted = HashSet::new();
  let test = count_rec("shiny gold", &rules, &mut counted);
  println!("Count: {}", test);
}

#[test]
fn from_str_test_1() {
  let rule =
    BagRule::from_str("dark orange bags contain 3 bright white bags, 4 muted yellow bags.")
      .unwrap();
  println!("{:?}", rule);
}

#[test]
fn from_str_test_2() {
  let rule = BagRule::from_str("faded blue bags contain no other bags.").unwrap();
  println!("{:?}", rule);
}
