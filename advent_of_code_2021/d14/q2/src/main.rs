use std::collections::HashMap;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn inc_counts(counts: &mut HashMap<char, u32>, character: char) {
    match counts.get_mut(&character) {
        Some(value) => *value += 1,
        None => {
            counts.insert(character, 1);
        }
    }
}

const MAX_ROUNDS: u32 = 39;

fn count(string: Vec<char>, grammar: HashMap<(char, char), char>) -> u32 {
    let mut counts: HashMap<char, u32> = HashMap::new();
    let count: u32 = 0;
    let mut result: Vec<char> = Vec::new();
    result.push(string[0]);
    let mut tuple: [char; 3] = [string[0], string[1], string[2]];
    let mut curr_idx = 2;
    let mut is_left = true;
    string
        .windows(2)
        .for_each(|pair| match grammar.get(&(pair[0], pair[1])) {
            Some(character) => {
                let tuple: [char; 3] = [pair[0], *character, pair[1]];
                println!("TUPLE calling: {:?}", tuple);
                recursive_count(&mut counts, &tuple, &mut result, &grammar, 0, is_left);
            }
            None => (),
        });
    // loop {
    //     // for tuple in string.windows(2) {
    //     // result.push(tuple[0]);
    //     println!("tuple: {:?} result: {:?}", tuple, result);
    //     recursive_count(&mut counts, &tuple, &mut result, &grammar, 1, count);
    //     tuple = [*result.last().unwrap(), string[curr_idx]];
    //     curr_idx += 1;
    //     if curr_idx == string.len() {
    //         println!("BREAKING: {:?} result: {:?}", tuple, result);
    //         break;
    //     }
    // }
    // println!("RESULT: {:?}", counts);
    let max_count = counts.values().max().unwrap();
    let min_count = counts.values().min().unwrap();
    println!("COUNTS: {:?}", counts);
    println!("RESULT: {:?}", max_count - min_count);
    0
}
fn recursive_count(
    counts: &mut HashMap<char, u32>,
    tuple: &[char],
    result: &mut Vec<char>,
    grammar: &HashMap<(char, char), char>,
    n_rounds: u32,
    is_left: bool,
) {
    if n_rounds == MAX_ROUNDS {
        if is_left {
            inc_counts(counts, tuple[1]);
            inc_counts(counts, tuple[2]);
            // result.push(tuple[1]);
            // result.push(tuple[2]);
        } else {
            inc_counts(counts, tuple[0]);
            inc_counts(counts, tuple[1]);
            // result.push(tuple[0]);
            // result.push(tuple[1]);
        }
        println!(
            "NODE: n_rounds {:?}, tuple: {:?} result: {:?}",
            n_rounds, tuple, counts
        );
        return;
    }
    let left = (tuple[0], tuple[1]);
    let right = (tuple[1], tuple[2]);

    // println!(
    //     "LEFT n_rounds {:?}, tuple: {:?}, result: {:?}",
    //     n_rounds, tuple, result
    // );
    match grammar.get(&left) {
        Some(character) => {
            let new_tuple: [char; 3] = [left.0, *character, left.1];
            recursive_count(counts, &new_tuple, result, grammar, n_rounds + 1, true);
        }
        None => (),
    }
    // println!(
    //     "RIGHT n_rounds {:?}, tuple: {:?}, result: {:?}",
    //     n_rounds, tuple, result
    // );
    match grammar.get(&right) {
        Some(character) => {
            let new_tuple: [char; 3] = [right.0, *character, right.1];
            recursive_count(counts, &new_tuple, result, grammar, n_rounds + 1, true);
        }
        None => (),
    }
}

fn main() {
    let (initial_string, str_grammar) = include_str!("../../sample.in").split_once("\n\n").unwrap();
    let string = initial_string.chars().collect::<Vec<char>>();
    let mut grammar: HashMap<(char, char), char> = HashMap::new();
    for line in str_grammar.trim().lines() {
        let mut iter = line.split(" -> ");
        let mut pair = iter.next().unwrap().chars().take(2);
        let c1 = pair.next().unwrap();
        let c2 = pair.next().unwrap();
        let character = iter.next().unwrap().chars().next().unwrap();
        grammar.insert((c1, c2), character);
    }
    // println!("grammar {:?}", grammar);
    // println!("string {:?}", string);
    let mut counts: HashMap<char, u32> = HashMap::new();
    // string.iter().for_each(|c| match counts.get_mut(c) {
    //     Some(value) => *value += 1,
    //     None => {
    //         counts.insert(*c, 1);
    //     }
    // });

    let mut curr_pair = (string[0], string[1]);
    let mut next_idx = 2;
    let mut n_applied = 0;
    let max_n_applied = 3;
    // println!("RESULT: {:?}", max_count - min_count);
    count(string, grammar);
    // loop {
    //     inc_counts(&mut counts, curr_pair.0);
    //     inc_counts(&mut counts, curr_pair.1);
    //     println!(
    //         "Curr_pair: {:?}, get: {:?}",
    //         curr_pair,
    //         grammar.get(&curr_pair)
    //     );
    //     match grammar.get(&curr_pair) {
    //         Some(character) => {
    //             if n_applied == max_n_applied {
    //                 if next_idx == string.len() {
    //                     break;
    //                 }
    //                 curr_pair = (curr_pair.1, string[next_idx]);
    //                 next_idx += 1;
    //                 continue;
    //             }
    //             n_applied += 1;
    //             curr_pair = (*character, curr_pair.1);
    //             inc_counts(&mut counts, *character);
    //             continue;
    //         }
    //         None => {
    //             println!("MOVED 1 CHARACTER. {}: len: {}", next_idx, string.len(),);
    //             if next_idx == string.len() {
    //                 break;
    //             }
    //             curr_pair = (curr_pair.1, string[next_idx]);
    //             next_idx += 1;
    //         }
    //     };
    // }
}

// fn count(string: Vec<char>, grammar: HashMap<(char, char), char>) -> u32 {
//     let mut counts: HashMap<char, u32> = HashMap::new();
//     let count: u32 = 0;
//     let mut result: Vec<char> = Vec::new();
//     let mut pair: [char; 2] = [string[0], string[1]];
//     let mut curr_idx = 2;
//     println!("curr_idx: {} string_len: {}", curr_idx, string.len());
//
//     loop {
//         // for pair in string.windows(2) {
//         result.push(pair[0]);
//         println!("pair: {:?} result: {:?}", pair, result);
//         let res = recursive_count(&mut counts, &pair, &mut result, &grammar, 1, count);
//         if res != '-' {
//             result.push(res);
//             pair = [res, pair[1]];
//         } else {
//             // println!("ELSE pair: {:?} res: {} result: {:?}", pair, res, result);
//             if curr_idx == string.len() {
//                 println!("BREAKING: {:?} result: {:?}", pair, result);
//                 break;
//             }
//             pair = [pair[1], string[curr_idx]];
//             curr_idx += 1;
//         }
//         // if res == '-' {
//         //     println!("result: {:?}", result);
//         //     break;
//         // }
//         // break;
//         // let res = recursive_count(&mut counts, &pair, &mut result, &grammar, 1, count);
//         // if res != '-' {
//         //     result.push(res);
//         // }
//         // break;
//         // result.push(pair[1]);
//         // result = Vec::new();
//         // break;
//         // res = recursive_count(counts, pair, &mut result, grammar, 0, count);
//     }
//     result.push(pair[1]);
//     println!("RESULT: {:?}", result);
//     // let max_count = counts.values().max().unwrap();
//     // let min_count = counts.values().min().unwrap();
//     // println!("COUNTS: {:?}", counts);
//     // println!("RESULT: {:?}", max_count - min_count);
//     0
// }
// fn recursive_count(
//     counts: &mut HashMap<char, u32>,
//     pair: &[char],
//     result: &mut Vec<char>,
//     grammar: &HashMap<(char, char), char>,
//     n_rounds: u32,
//     count: u32,
// ) -> char {
//     println!("in recursive::: pair: {:?} result: {:?}", pair, result);
//     let mut last_char: char = '-';
//     match grammar.get(&(pair[0], pair[1])) {
//         Some(character) => {
//             if n_rounds == MAX_ROUNDS {
//                 // println!("MAX ROUNDS: |{}|", *character);
//                 last_char = *character;
//                 // return *character;
//                 // result.push(*character);
//                 // // pair = character pair.1 character
//                 // let new_pair = vec![*character, pair[1]];
//                 // recursive_count(counts, &new_pair, result, grammar, 0, count);
//             } else {
//                 // pair = pair.0 character
//                 // println!("else: |{}|", *character);
//                 let new_pair = vec![pair[0], *character];
//                 recursive_count(counts, &new_pair, result, grammar, n_rounds + 1, count);
//                 result.push(*character);
//
//                 // let new_pair = vec![*character, pair[1]];
//                 // recursive_count(counts, &new_pair, result, grammar, n_rounds + 1, count);
//                 // result.push(*character);
//                 // *character
//             }
//         }
//         None => (),
//     }
//
//     last_char
// }
//
//
//
// fn count(string: Vec<char>, grammar: HashMap<(char, char), char>) -> u32 {
//     let mut counts: HashMap<char, u32> = HashMap::new();
//     let count: u32 = 0;
//     let mut result: Vec<char> = Vec::new();
//     let mut tuple: [char; 2] = [string[0], string[1]];
//     let mut curr_idx = 2;
//     println!("curr_idx: {} string_len: {}", curr_idx, string.len());
//
//     loop {
//         // for tuple in string.windows(2) {
//         // result.push(tuple[0]);
//         println!("tuple: {:?} result: {:?}", tuple, result);
//         recursive_count(&mut counts, &tuple, &mut result, &grammar, 1, count);
//         tuple = [*result.last().unwrap(), string[curr_idx]];
//         curr_idx += 1;
//         if curr_idx == string.len() {
//             println!("BREAKING: {:?} result: {:?}", tuple, result);
//             break;
//         }
//         // let res = recursive_count(&mut counts, &tuple, &mut result, &grammar, 1, count);
//         // if res != '-' {
//         //     // result.push(res);
//         //     tuple = [res, tuple[1]];
//         // } else {
//         //     // println!("ELSE tuple: {:?} res: {} result: {:?}", tuple, res, result);
//         //     if curr_idx == string.len() {
//         //         println!("BREAKING: {:?} result: {:?}", tuple, result);
//         //         break;
//         //     }
//         // }
//     }
//     // loop {
//     //     // for tuple in string.windows(2) {
//     //     result.push(tuple[0]);
//     //     println!("tuple: {:?} result: {:?}", tuple, result);
//     //     let res = recursive_count(&mut counts, &tuple, &mut result, &grammar, 1, count);
//     //     if res != '-' {
//     //         result.push(res);
//     //         tuple = [res, tuple[1]];
//     //     } else {
//     //         // println!("ELSE tuple: {:?} res: {} result: {:?}", tuple, res, result);
//     //         if curr_idx == string.len() {
//     //             println!("BREAKING: {:?} result: {:?}", tuple, result);
//     //             break;
//     //         }
//     //         tuple = [tuple[1], string[curr_idx]];
//     //         curr_idx += 1;
//     //     }
//     // }
//     // result.push(tuple[1]);
//     println!("RESULT: {:?}", result);
//     // let max_count = counts.values().max().unwrap();
//     // let min_count = counts.values().min().unwrap();
//     // println!("COUNTS: {:?}", counts);
//     // println!("RESULT: {:?}", max_count - min_count);
//     0
// }
// fn recursive_count(
//     counts: &mut HashMap<char, u32>,
//     tuple: &[char],
//     result: &mut Vec<char>,
//     grammar: &HashMap<(char, char), char>,
//     n_rounds: u32,
//     count: u32,
// ) {
//     let mut last_char: char = '-';
//     match grammar.get(&(tuple[0], tuple[2])) {
//         Some(character) => {
//             if n_rounds == MAX_ROUNDS {
//                 // last_char = *character;
//                 result.push(tuple[0]);
//                 result.push(tuple[1]);
//             } else {
//                 let new_tuple = vec![tuple[0], *character, tuple[1]];
//                 println!(
//                     "in recursive first_call::: tuple: {:?} result: {:?}",
//                     tuple, result
//                 );
//                 recursive_count(counts, &new_tuple, result, grammar, n_rounds + 1, count);
//                 // result.push(*character);
//
//                 let new_tuple = vec![*character, tuple[1]];
//                 println!(
//                     "in recursive second_call::: tuple: {:?} result: {:?}",
//                     tuple, result
//                 );
//                 recursive_count(counts, &new_tuple, result, grammar, n_rounds + 1, count);
//                 // result.push(*character);
//                 println!("after recursive::: tuple: {:?} result: {:?}", tuple, result);
//             }
//         }
//         None => last_char = tuple[0],
//     }
// }
