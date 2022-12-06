use std::fs;

fn main() {
    let file_path = "src/06/input.txt";
    let input = fs::read_to_string(file_path)
        .expect("Couldn't read input.txt! :(");
        
        println!("First start-of-packet-marker appears at {}", n_prior_are_different(input.as_str(), 4));
        
        println!("First start-of-message-marker appears at {}", n_prior_are_different(input.as_str(), 14));
}

fn n_prior_are_different(input: &str, n: usize) -> usize {
    assert!(input.len() > n);
    for current in n .. input.len() {
        let n_prior = &input[current-n..current];
        if !contains_duplicates(n_prior) {
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
    false
}

#[cfg(test)]
mod tests {
    use crate::{contains_duplicates, n_prior_are_different};

    #[test]
    fn four_prior_are_different() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";

        let result = n_prior_are_different(input, 4);

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