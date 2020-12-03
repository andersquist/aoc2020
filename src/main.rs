mod day2;
mod day3;
mod utils;

use std::env::args;

use day2::password_policy;
use day3::{count_trees, count_trees_all_slopes};

fn main() {
    let day = match args().nth(1) {
        Some(input) => input,
        None => "2".to_string(),
    };

    match day.as_str() {
        "2" => {
            let (valid, valid_second, count) = password_policy();
            println!("{} of {} passwords was valid.", valid, count);
            println!(
                "{} of {} passwords was valid according updated policy.",
                valid_second, count
            );
        }
        "3" => {
            println!("Puzzle 1: {} trees were encountered", count_trees());
            println!(
                "Puzzle 2: {} product of all trees encounterd",
                count_trees_all_slopes()
            );
        }
        _ => panic!("No such day!"),
    }
}
