use std::{fs};

fn main() {
    let file_path = "src/01/input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Couldn't read input.txt! :(");

    let mut callories_vector = get_calories_vector(contents);

    match highest_callories(&mut callories_vector) {
        Some(highest_callories) => println!("Highest calories: {}", highest_callories),
        None => println!("The input didn't contain any integers!")
    }

    match sum_of_top_3_callories(&mut callories_vector) {
        Some(highest_callories) => println!("Sum of top 3 calories: {}", highest_callories),
        None => println!("The input didn't contain atleast 3 integers!")
    }
}

fn highest_callories(callories_vector: &mut Vec<i32>) -> Option<i32> {
    callories_vector.sort_unstable_by(|a, b| b.cmp(a));
    callories_vector.get(0).cloned()
}

fn sum_of_top_3_callories(callories_vector: &mut Vec<i32>) -> Option<i32> {
    if callories_vector.len() < 3 {
        return None;
    }

    callories_vector.sort_unstable_by(|a, b| b.cmp(a));
    Option::from(callories_vector[0]+callories_vector[1]+callories_vector[2])
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

    #[test]
    fn highest_callories_gets_highest_value() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        let mut calories_vector = get_calories_vector(input.to_string());

        let result = highest_callories(&mut calories_vector);

        assert_eq!(result.unwrap(), 24000);
    }

    #[test]
    fn highest_callories_returns_none_if_vector_is_empty() {
        let mut calories_vector = vec![];

        let result = highest_callories(&mut calories_vector);

        assert!(result.is_none());
    }

    #[test]
    fn sum_of_top_3_callories_sums_up_top_3_values() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        let mut calories_vector = get_calories_vector(input.to_string());

        let result = sum_of_top_3_callories(&mut calories_vector);

        assert_eq!(result.unwrap(), 45000);
    }


    #[test]
    fn sum_of_top_3_callories_returns_none_if_vector_is_bellow_len_3() {
        let mut calories_vector = vec![42, 1337];

        let result = sum_of_top_3_callories(&mut calories_vector);

        assert!(result.is_none());
    }
}