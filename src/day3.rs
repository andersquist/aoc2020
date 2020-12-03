use super::utils::read_lines;
use std::path::Path;

fn split_to_vec(row: String) -> Vec<char> {
  row.chars().collect::<Vec<char>>()
}

fn read_from_file<P>(filename: P) -> Vec<Vec<char>>
where
  P: AsRef<Path>,
{
  let mut tree_vec: Vec<Vec<char>> = Vec::new();
  if let Ok(lines) = read_lines(filename) {
    for line in lines {
      if let Ok(row) = line {
        tree_vec.push(split_to_vec(row))
      }
    }
  }
  tree_vec
}

fn count_trees_in_vec(tree_vec: &Vec<Vec<char>>, slope_r: usize, slope_d: usize) -> i32 {
  let mut x = 0;
  let mut count = 0;
  for y in (0..tree_vec.len()).step_by(slope_d) {
    if x >= tree_vec[y].len() {
      x = x - tree_vec[y].len();
    }
    match tree_vec[y][x] {
      '#' => count += 1,
      _ => (),
    }
    x += slope_r;
  }

  count
}

fn count_trees_all_slopes_in_vec(tree_vec: &Vec<Vec<char>>) -> Vec<i32> {
  let mut tree_counts: Vec<i32> = Vec::new();
  let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

  for (slope_r, slope_d) in slopes {
    tree_counts.push(count_trees_in_vec(&tree_vec, slope_r, slope_d));
  }
  tree_counts
}

pub fn count_trees() -> i32 {
  let tree_vec = read_from_file("inputs/day3.txt");
  count_trees_in_vec(&tree_vec, 3, 1)
}

pub fn count_trees_all_slopes() -> i64 {
  let tree_vec = read_from_file("inputs/day3.txt");
  count_trees_all_slopes_in_vec(&tree_vec)
    .iter()
    .map(|&v| v as i64)
    .product::<i64>()
}

#[test]
fn count_trees_as_expected() {
  let tree_vec = read_from_file("inputs/day3_test.txt");
  assert_eq!(count_trees_in_vec(&tree_vec, 3, 1), 7)
}

#[test]
fn count_trees_all_slopes_as_expectes() {
  let tree_vec = read_from_file("inputs/day3_test.txt");
  assert_eq!(
    count_trees_all_slopes_in_vec(&tree_vec),
    vec![2, 7, 3, 4, 2]
  )
}
