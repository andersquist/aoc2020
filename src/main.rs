mod day2;
mod day3;
mod day4;
mod day5;
mod utils;

use std::env::args;

use day2::password_policy;
use day3::{count_trees, count_trees_all_slopes};
use day4::{count_valid_passports, count_valid_passports_values};
use day5::{day_5_puzzle_1, day_5_puzzle_2};

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
        "4" => {
            count_valid_passports();
            count_valid_passports_values();
        }
        "5" => {
            day_5_puzzle_1();
            day_5_puzzle_2();
        }
        _ => panic!("No such day!"),
    }
}
