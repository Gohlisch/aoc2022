mod compartments;

use std::fs;

use crate::compartments::Compartments;

fn main() {
    let file_path = "src/03/input.txt";
    let input = fs::read_to_string(file_path)
        .expect("Couldn't read input.txt! :(");

    let sum_of_priorities = input.lines()
        .map(Compartments::from_str)
        .map(|compartments| compartments.get_priority())
        .map(|option| option.or_else(|| Some(0)).unwrap() as i64)
        .reduce(|sum, priority| sum + priority);

    println!("Sum of priorities is {}", sum_of_priorities.unwrap());
}