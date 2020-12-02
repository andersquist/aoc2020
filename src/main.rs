mod day2;

use std::env::args;

use day2::password_policy;

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
        _ => panic!("No such day!"),
    }
}
