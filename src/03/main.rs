mod compartments;
mod groups;
pub mod util;

use std::fs;
use crate::{compartments::Compartments, groups::Group, util::next_three_as_tuple};

fn main() {
    let file_path = "src/03/input.txt";
    let input = fs::read_to_string(file_path)
        .expect("Couldn't read input.txt! :(");

    // part 1
    let sum_of_priorities = input.lines()
        .map(Compartments::from_str)
        .map(|compartments| compartments.get_priority())
        .map(|option| option.or_else(|| Some(0)).unwrap() as i64)
        .reduce(|sum, priority| sum + priority);

    println!("Sum of priorities is {}", sum_of_priorities.unwrap());

    // part 2
    let mut it = input.lines();
    let mut badge_sum: i64 = 0;

    while let Ok(tuple) = next_three_as_tuple(&mut it) {
        let group = Group::from_tuple(tuple);
        
        if let Some(badge_priority) = group.get_badge_priority() {
            badge_sum = badge_sum + badge_priority as i64;
        } else {
            panic!("Priority of a group couldn't be determined! 🤯")
        }
    }

    println!("Badgesum is {}", badge_sum);
}

