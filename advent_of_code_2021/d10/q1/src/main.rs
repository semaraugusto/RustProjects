use std::collections::HashMap;

type Stack<T> = Vec<T>;

fn main() {
    let forward = ['(', '[', '{', '<'];
    // let forward = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    // let backward = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
    let scores = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let matches = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    let navigation = include_str!("../../test.in")
        .trim()
        .lines()
        .fold(0, |acc, line| {
            let mut stack: Stack<char> = Stack::new();
            let mut result = 0;
            line.chars().for_each(|c| {
                // if *forward.get(&c).unwrap_or(&0) > 0 {
                if forward.contains(&c) {
                    stack.push(c);
                } else {
                    let val = stack.pop().unwrap();
                    if *matches.get(&c).unwrap() != val {
                        result = *scores.get(&c).unwrap();
                        println!("invalid: {:?}", result);
                        // result
                    }
                }
            });
            result + acc
        });
    println!("nav: {:?}", navigation);
}
