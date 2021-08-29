use std::collections::{HashMap, HashSet};
#[macro_use]
extern crate lazy_static;

fn json_tokens(data: &String, stack: &mut Vec<char>) -> Result<String, String> {
    lazy_static! {
        static ref STARTERS: HashSet<char> = ['{', '[', '('].iter().cloned().collect();
        static ref FINISHERS: HashSet<char> = ['}', ']', ')'].iter().cloned().collect();
        static ref FINISHERS_STARTERS_MAP: HashMap<char, char> =
            [('}', '{'), (']', '['), (')', '(')]
                .iter()
                .cloned()
                .collect();
        static ref SPECIALS: HashSet<char> = ['{', '[', '(', '}', ']', ')', '\"', '\\']
            .iter()
            .cloned()
            .collect();
    };

    static MIRRORS: [char; 2] = ['\"', '\''];
    let mut escapee: bool = false;

    for (cnt, letter) in data.chars().enumerate() {
        if !SPECIALS.contains(&letter) || escapee {
            // Check should save performance if we have few specials and long strings
            // also lets us ignore whitespaces and newlines
            escapee = false;
            continue;
        }
        if letter == '\\' {
            escapee = true;
            continue;
        }

        if MIRRORS.contains(&letter) {
            if stack.ends_with(&[letter]) {
                stack.pop();

            } else {
                stack.push(letter);
            }

        } else if STARTERS.contains(&letter) {
            stack.push(letter);

        } else {
            // We found escapes, mirrors and starters, only one left is finishers.
            let &starter = FINISHERS_STARTERS_MAP.get(&letter).unwrap();
            if stack.ends_with(&[starter]) {
                stack.pop();
            } else {
                if !stack.ends_with(&['\"']) {
                    return Err("Bad json".to_string());
                }
            }
        }

        if stack.is_empty() {
            return Ok(data[0..cnt + 1].to_string());
        }
    }
    if !stack.is_empty() {
        println!("{:?}", stack);
        panic!("Json syntax error");
    }

    return Ok(data.to_string());
}

fn main() {
    let mut stack: Vec<char> = std::vec::Vec::new();
    let test_data: String = String::from("{\"{{var}}}}\": 123}");
    for _ in 1..10 {
        json_tokens(&test_data, &mut stack);
    }
}
