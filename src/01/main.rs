use std::fs;

fn main() {
    let file_path = "/Users/timgohlisch/Documents/Advent_of_Code/_2022/src/01/input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Couldn't read input.txt! :(");

    let mut callories_vector = get_calories_vector(contents);
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

    callories_vector
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}