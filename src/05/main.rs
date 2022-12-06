use std::{fs};

use regex::Regex;

fn main() {
    let file_path = "src/05/input.txt";
    let input = fs::read_to_string(file_path)
        .expect("Couldn't read input.txt! :(");

    // part 1
    let mut stacks = input_to_stacks(input.as_str());
    let mut commands = input.lines()
                                                        .map(str_to_commands)
                                                        .filter(|option| option.is_some())
                                                        .map(|option| option.unwrap());
    execute_commands_on_stacks(commands, &mut stacks);
    for stack in stacks {
        print!("{}", stack[stack.len()-1]);
    }
    print!("\n");

    // part 2
    let mut stacks = input_to_stacks(input.as_str());
    let mut commands = input.lines()
                                                        .map(str_to_commands)
                                                        .filter(|option| option.is_some())
                                                        .map(|option| option.unwrap());
    execute_commands_on_stacks_retain_order(commands, &mut stacks);
    for stack in stacks {
        print!("{}", stack[stack.len()-1]);
    }
    print!("\n");
}

type Command = (usize, usize, usize);
type Identifier = char;

fn execute_commands_on_stacks(commands: impl Iterator<Item = Command>, stacks: &mut  Vec<Vec<Identifier>>) {
    for (amount, from, to) in commands {
        for _ in 0..amount  {
            let val = stacks[from-1].pop().unwrap();
            stacks[to-1].push(val);
        }
    }
}

fn execute_commands_on_stacks_retain_order(commands: impl Iterator<Item = Command>, stacks: &mut  Vec<Vec<Identifier>>) {
    for (amount, from, to) in commands {
        for i in stacks[from-1].len()-amount .. stacks[from-1].len() {
            let val = stacks[from-1][i];
            stacks[to-1].push(val);
        }

        for i in 0..amount {
            stacks[from-1].pop();
        }
    }
}

fn str_to_commands(line: &str) -> Option<Command> {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let captures = re.captures(line)?;
    unsafe {
        let a = captures.get(1)?.as_str().parse().unwrap_unchecked();
        let b = captures.get(2)?.as_str().parse().unwrap_unchecked();
        let c = captures.get(3)?.as_str().parse().unwrap_unchecked();
        
        Some((a,b,c))
    }
}

fn input_to_stacks(input: &str) -> Vec<Vec<Identifier>> {
    let stack_numeration_regex = Regex::new(r" 1 (  \d )+").unwrap();
    let mut it = input.lines().rev();
    let mut stacks: Vec<Vec<Identifier>> = vec![];

    if let Some(stack_numbers) = it.find(|x| stack_numeration_regex.is_match(x)) {
        if let Some(amound_of_stacks) = stack_numbers.chars()
                                                                .rev()
                                                                .find(|c| char_to_no(*c).is_some())
                                                                .map(char_to_no)
                                                                
                                                                
                                                                .map(|c| c.unwrap()) {
            for _ in 0 .. amound_of_stacks {
                stacks.push(vec![]);
            }

            for line in it {
                let re = Regex::new(r"(\[[A-Z]\] ?)|(    ?)").unwrap();
                for (i, cap) in re.captures_iter(line).enumerate() {
                    if let Some(c) = stack_input_to_char(&cap[0]) {
                        stacks[i].push(c);
                    } 
                }
            }
        }
    }

    stacks
}

fn char_to_no(c: char) -> Option<i32> {
    match c {
        '0'..='9' => Some((c as i32) - 48),
        _ => None
    }
}

fn stack_input_to_char(captured: &str) -> Option<char> {
    let captured = captured.trim();
    if captured.len() < 3 {
        return None;
    }

    match captured.chars().nth(1) {
        Some(variable @ 'A'..='Z') => Some(variable),
        _ => None,
    }
}



#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;

    use crate::*;

    #[test]
    fn creates_3_stacks() {
        let input = "[D]    
[N] [C]    
[Z] [M] [P]
    1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        let stacks = input_to_stacks(input);

        assert_eq!(stacks.len(), 3);
    }

    #[test]
    fn puts_chars_into_created_stacks() {
        let input = "[D]    
[N] [C]    
[Z] [M] [P]
    1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        let stacks = input_to_stacks(input);
        let first_stack = &stacks[0];

        assert_eq!(first_stack[0], 'Z');
        assert_eq!(first_stack[1], 'N');
        assert_eq!(first_stack[2], 'D');
    }

    #[test]
    fn parses_char_to_no() {
        let chars = '0' ..= '9';

        let mut numbers = chars.map(char_to_no)
                                                    .filter(|optional| optional.is_some())
                                                    .map(|optional| optional.unwrap());

        for i in 0 ..= 9 {
            assert_eq!(numbers.next().unwrap(), i);
        }
    }

    #[test]
    fn parses_capture_group_to_char() {
        let captured_group = "[Q] ";

        let c = stack_input_to_char(captured_group).unwrap();

        assert_eq!(c, 'Q');
    }

    #[test]
    fn parses_capture_group_to_none() {
        let captured_group = "    ";

        let c = stack_input_to_char(captured_group);

        assert!(c.is_none());
    }

    #[test]
    fn parses_command() {
        let input_line = "move 1 from 2 to 3";

        let command = str_to_commands(input_line).unwrap();

        assert_eq!(command.0, 1);
        assert_eq!(command.1, 2);
        assert_eq!(command.2, 3);
    }
}