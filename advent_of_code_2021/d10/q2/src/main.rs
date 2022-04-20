use std::collections::HashMap;

type Stack<T> = Vec<T>;

fn main() {
    let forward = ['(', '[', '{', '<'];
    // let forward = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    // let backward = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
    let scores = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    let fmatches = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    let bmatches = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let mut result: Vec<i64> = include_str!("../../test.in")
        .trim()
        .lines()
        .filter_map(|line| {
            let mut stack: Stack<char> = Stack::new();
            let valid = line.chars().fold(true, |is_valid, c| {
                // if *forward.get(&c).unwrap_or(&0) > 0 {
                let mut line_valid = true;
                if forward.contains(&c) {
                    stack.push(c);
                } else {
                    let val = stack.pop().unwrap();
                    if *fmatches.get(&c).unwrap() != val {
                        line_valid = false;
                    }
                }
                is_valid && line_valid
            });
            let mut result: i64 = 0;
            if valid {
                println!("valid: {:?}", valid);
                while !stack.is_empty() {
                    let val = stack.pop().unwrap();
                    println!("result: {:?}", val);
                    let corresponding = *bmatches.get(&val).unwrap();
                    result = result * 5 + scores.get(&corresponding).unwrap();
                }
                println!("result: {:?}", result);
                Some(result)
            } else {
                None
            }
        })
        .collect::<Vec<i64>>();

    result.sort_unstable();
    let mid = result.len() / 2;

    println!("list: {:?}", result);
    println!("result: {:?}", result[mid]);
}
