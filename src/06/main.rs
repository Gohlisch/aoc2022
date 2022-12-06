use std::fs;

fn main() {
    let file_path = "src/06/input.txt";
    let input = fs::read_to_string(file_path)
        .expect("Couldn't read input.txt! :(");
        
    println!("First marker appears at {}", four_prior_are_different(input.as_str()));
}

fn four_prior_are_different(input: &str) -> usize {
    assert!(input.len() > 4);
    for current in 4..input.len() {
        let four_prior = &input[current-4..current];
        if !contains_duplicates(four_prior) {
            return current;
        }
    }

    panic!("No marker found!")
}

fn contains_duplicates(s: &str) -> bool {
    for c1 in s.chars() {
        let occurences = s.chars()
                                    .filter(|c2| c2 == &c1)
                                    .count();
        if occurences > 1 {
            return true;
        }
    }

    return false;
}

#[cfg(test)]
mod tests {
    use crate::{contains_duplicates, four_prior_are_different};

    #[test]
    fn it_works() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";

        let result = four_prior_are_different(input);

        assert_eq!(result, 5);
    }

    #[test]
    fn does_contain_duplicates() {
        let with_duplicates = ["aasd", "sffa", "faee", "ardr", "gasg"];

        for string_with_duplicate_chars in with_duplicates {
            assert!(contains_duplicates(string_with_duplicate_chars));
        }
    }

    #[test]
    fn does_not_contain_duplicates() {
        let with_duplicates = ["afsd", "shfa", "faje", "ardk", "gasd"];

        for string_with_duplicate_chars in with_duplicates {
            assert!(!contains_duplicates(string_with_duplicate_chars));
        }
    }
}