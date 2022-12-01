use std::fs;

fn main() {
    let file_path = "src/01/input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Couldn't read input.txt! :(");

    let mut callories_vector = get_calories_vector(contents);
    println!("Highest calories: {}", callories_vector.len());
    callories_vector.sort();
    println!("Highest calories: {}", callories_vector[callories_vector.len()-1]);
    println!("Top 3 calories: {}", callories_vector[callories_vector.len()-1]+callories_vector[callories_vector.len()-2]+callories_vector[callories_vector.len()-3]);
}

fn get_calories_vector (mut input: String) -> Vec<i32> {
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
    use crate::get_calories_vector;

    #[test]
    fn sums_up_single_meals() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

        let calories_vector = get_calories_vector(input.to_string());

        assert_eq!(calories_vector.len(), 5);
    }
}