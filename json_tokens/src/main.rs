use std::collections::HashMap;

fn json_tokens(data: &String, stack: &mut Vec<char>) -> String {
    static STARTERS: [char; 3] = ['{', '[', '('];
    static FINISHERS: [char; 3] = ['}', ']', ')'];
    static MIRRORS: [char; 1] = ['\"'];
    static ESCAPES: [char; 1] = ['\\'];

    for (cnt, letter) in data.chars().enumerate() {
        if ESCAPES.contains(&letter) {
            continue;
        }
        if MIRRORS.contains(&letter) {
            // Handle them
            if stack.ends_with(&[letter]) {
                stack.pop();
            }
            else {
                stack.push(letter);
            }
        }
        else {
            if  STARTERS.contains(&letter) {
                stack.push(letter);
            }
            let indx = FINISHERS.iter().position(|&x: &char| x == letter);

            if indx.is_some() {
                let index = indx.unwrap();
                if stack.ends_with(&STARTERS[index..index+1]) {
                    stack.pop().expect("Faulty json");

                }
            }
        }

        if stack.is_empty() {
            return data[0..cnt+1].to_string();
        }
        //println!("{}", letter);
    }
    if !stack.is_empty() {
        println!("{:?}", stack);
        panic!("Json syntax error");
    }

    return data.to_string();

}

fn main() {
    let mut stack: Vec<char> = std::vec::Vec::new();
    let test_data : String = String::from("{\"{{var}}}}\": 123}");
    for i in 1..10000000 {
        json_tokens(&test_data, &mut stack);
    }

}

