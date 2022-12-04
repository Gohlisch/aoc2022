mod range;

use std::{fs};
use range::Range;
use regex::Regex;

fn main() {
    let file_path = "src/04/input.txt";
    let input = fs::read_to_string(file_path)
        .expect("Couldn't read input.txt! :(");

    let ranges_containing_each_other = input.lines()
        .map(str_to_ranges)
        .filter(|option| option.is_some())
        .map(|option| option.unwrap())
        .filter(|ranges| ranges.0.includes(&ranges.1) || ranges.1.includes(&ranges.0))
        .count();

    println!("{} ranges contain each other.", ranges_containing_each_other);
}

fn str_to_ranges(str: &str) -> Option<(Range, Range)> {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let captures = re.captures(str)?;
    unsafe {
        let a = captures.get(1)?.as_str().parse().unwrap_unchecked();
        let b = captures.get(2)?.as_str().parse().unwrap_unchecked();
        let c = captures.get(3)?.as_str().parse().unwrap_unchecked();
        let d = captures.get(4)?.as_str().parse().unwrap_unchecked();
        
        Some((Range(a ,b), Range(c, d)))
    }
}

#[cfg(test)]
mod tests {
    use crate::str_to_ranges;

    #[test]
    fn parses_input_line() {
        let line = "2-4,6-8";

        let ranges = str_to_ranges(line).unwrap();
        assert_eq!(ranges.0.0, 2);
        assert_eq!(ranges.0.1, 4);
        assert_eq!(ranges.1.0, 6);
        assert_eq!(ranges.1.1, 8);
    }
}