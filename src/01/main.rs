mod util;

use std::{fs};

use util::{highest_value, sum_of_top_3};

fn main() {
    let file_path = "src/01/input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Couldn't read input.txt! :(");

    let mut callories_vector = get_calories_vector(contents);

    match highest_value(&mut callories_vector) {
        Some(highest_value) => println!("Highest calories: {}", highest_value),
        None => println!("The input didn't contain any integers!")
    }

    match sum_of_top_3(&mut callories_vector) {
        Some(highest_value) => println!("Sum of top 3 calories: {}", highest_value),
        None => println!("The input didn't contain atleast 3 integers!")
    }
}

fn get_calories_vector (input: String) -> Vec<i32> {
    let mut callories_vector: Vec<i32> = vec![];
    let mut current_callories = 0;

    for line in input.lines() {
        let calories = line.parse::<i32>();
        if calories.is_ok() {
            current_callories = current_callories + calories.unwrap();
        } else {
            callories_vector.push(current_callories);
            current_callories = 0;
        }
    }
    callories_vector.push(current_callories);

    callories_vector
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn get_calories_vector_sums_up_single_meals() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

        let calories_vector = get_calories_vector(input.to_string());

        assert_eq!(calories_vector.len(), 5);
        assert_eq!(calories_vector[0], 6000);
        assert_eq!(calories_vector[1], 4000);
        assert_eq!(calories_vector[2], 11000);
        assert_eq!(calories_vector[3], 24000);
        assert_eq!(calories_vector[4], 10000);
    }
}