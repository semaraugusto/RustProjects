use std::collections::HashMap;

fn inc_counts(counts: &mut HashMap<char, u32>, character: char) {
    match counts.get_mut(&character) {
        Some(value) => *value += 1,
        None => {
            counts.insert(character, 1);
        }
    }
}

fn main() {
    let (initial_string, str_grammar) = include_str!("../../test.in").split_once("\n\n").unwrap();
    let mut string = initial_string.chars().collect::<Vec<char>>();
    let mut grammar: HashMap<(char, char), char> = HashMap::new();
    for line in str_grammar.trim().lines() {
        // let split = line
        //     .split(" -> ")
        //     .collect::<Vec<&'static str>>()
        //     .into_iter()
        //     .take(2);
        // println!("split {:?}", split);
        let mut iter = line.split(" -> ");
        // let pair = iter.next().unwrap();
        let mut pair = iter.next().unwrap().chars().take(2);
        let c1 = pair.next().unwrap();
        let c2 = pair.next().unwrap();
        let character = iter.next().unwrap().chars().next().unwrap();
        grammar.insert((c1, c2), character);
    }
    // println!("grammar {:?}", grammar);
    // println!("string {:?}", string);
    let mut counts: HashMap<char, u32> = HashMap::new();
    string.iter().for_each(|c| match counts.get_mut(c) {
        Some(value) => *value += 1,
        None => {
            counts.insert(*c, 1);
        }
    });

    for _step in 1..=40 {
        let mut result_string: Vec<char> = Vec::new();
        string.windows(2).for_each(|p| {
            // println!("window {:?}", p);
            let x = p[0];
            let y = p[1];
            result_string.push(x);
            match grammar.get(&(x, y)) {
                Some(character) => {
                    result_string.push(*character);
                    inc_counts(&mut counts, *character);
                }
                None => (),
            }
        });
        result_string.push(*string.last().unwrap());
        string = result_string;
        println!("_iter {}: len: {}", _step, string.len(),);
    }
    let max_count = counts.values().max().unwrap();
    let min_count = counts.values().min().unwrap();
    println!("COUNTS: {:?}", counts);
    println!("RESULT: {:?}", max_count - min_count);
}
