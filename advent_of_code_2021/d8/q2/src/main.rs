use itertools::Itertools;
use std::collections::HashMap;

const LETTER_WIRE_COUNTS: [i32; 10] = [6, 2, 5, 5, 4, 5, 6, 3, 7, 6];

#[derive(Clone, Debug)]
struct Entry {
    segments: Vec<Wire>,
    output: Vec<Wire>,
    one: Wire,
    four: Wire,
    seven: Wire,
    eight: Wire,
}

impl Entry {
    fn new(line_str: &str) -> Entry {
        let mut iter = line_str.split('|');
        let segments = iter
            .next()
            .unwrap()
            .split_whitespace()
            .map(Wire::new)
            .collect::<Vec<Wire>>();

        let output = iter
            .next()
            .unwrap()
            .split_whitespace()
            .map(Wire::new)
            .collect::<Vec<Wire>>();

        let one: Wire = segments
            .iter()
            .find(|wire| wire.letter == Some('1'))
            .unwrap()
            .clone();

        let four: Wire = segments
            .iter()
            .find(|wire| wire.letter == Some('4'))
            .unwrap()
            .clone();

        let seven: Wire = segments
            .iter()
            .find(|wire| wire.letter == Some('7'))
            .unwrap()
            .clone();

        let eight: Wire = segments
            .iter()
            .find(|wire| wire.letter == Some('8'))
            .unwrap()
            .clone();

        Entry {
            segments,
            output,
            one,
            four,
            seven,
            eight,
        }
    }

    fn get_solution(&self) -> Vec<&str> {
        self.segments
            .iter()
            .fold(vec![""; 10], |mut solution, wire| {
                match wire.signal.len() {
                    2 => solution[1] = &wire.signal,
                    3 => solution[7] = &wire.signal,
                    4 => solution[4] = &wire.signal,
                    7 => solution[8] = &wire.signal,
                    5 => {
                        let matches_one = &wire
                            .signal
                            .chars()
                            .filter(|chr| self.one.signal.contains(*chr))
                            .count();
                        let matches_four = &wire
                            .signal
                            .chars()
                            .filter(|chr| self.four.signal.contains(*chr))
                            .count();
                        let matches_seven = &wire
                            .signal
                            .chars()
                            .filter(|chr| self.seven.signal.contains(*chr))
                            .count();

                        match (matches_one, matches_four, matches_seven) {
                            (1, 2, 2) => solution[2] = &wire.signal,
                            (1, 3, 2) => solution[5] = &wire.signal,
                            (2, _, 3) => solution[3] = &wire.signal,
                            (_, _, _) => unreachable!(),
                        }
                    }
                    6 => {
                        let matches_one = &wire
                            .signal
                            .chars()
                            .filter(|chr| self.one.signal.contains(*chr))
                            .count();
                        let matches_four = &wire
                            .signal
                            .chars()
                            .filter(|chr| self.four.signal.contains(*chr))
                            .count();
                        match (matches_one, matches_four) {
                            (1, _) => solution[6] = &wire.signal,
                            (2, 3) => solution[0] = &wire.signal,
                            (2, 4) => solution[9] = &wire.signal,
                            (_, _) => unreachable!(),
                        }
                    }
                    _ => unreachable!(),
                }
                solution
            })
    }
    fn decode(&self, solutions: Vec<&str>) -> u32 {
        println!(
            "solutions: {:?}",
            solutions
                .iter()
                .map(|sol| sol.chars().sorted())
                .collect::<Vec<_>>()
        );
        let decoded_vals = self
            .output
            .iter()
            .map(|out_wire| {
                println!("out_wire: {:?}", out_wire.signal.chars().sorted());
                solutions
                    .iter()
                    .filter(|solution| {
                        println!("solution: {:?}", solution.len());
                        println!("wire: {:?}", out_wire.len);
                        solution.len() == out_wire.len
                    })
                    .sorted()
                    .position(|solution| {
                        solution
                            .chars()
                            .sorted()
                            .zip(out_wire.signal.chars().sorted())
                            .all(|(a, b)| a == b)
                    })
                    .unwrap() as u32
            })
            .collect::<Vec<u32>>();

        println!("decoded: {:?}", decoded_vals);
        decoded_vals
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, val)| acc + 10_u32.pow(i as u32) * val)
    }
}

fn is_three(guess: &Wire, one_wire: &Wire) -> bool {
    if guess.signal.len() != 5 {
        return false;
    }
    let mut matches = 0;
    for one_sig in one_wire.signal.chars() {
        let sig_match = guess
            .signal
            .chars()
            .find_map(|guess_sig| Some(guess_sig == one_sig));
        match sig_match {
            Some(_) => matches += 1,
            None => break,
        }
    }
    matches == 2
}

fn is_five(guess: &Wire, one_wire: &Wire) -> bool {
    if guess.signal.len() != 5 {
        return false;
    }

    let mut matches = 0;
    for one_sig in one_wire.signal.chars() {
        let sig_match = guess
            .signal
            .chars()
            .find_map(|guess_sig| Some(guess_sig == one_sig));
        match sig_match {
            Some(_) => matches += 1,
            None => break,
        }
    }
    matches == 2
}

#[derive(Clone, Debug)]
struct Wire {
    signal: String,
    letter: Option<char>,
    len: usize,
}

impl Wire {
    fn new(signal: &str) -> Wire {
        let len = signal.len();
        let letter: Option<char>;
        if len == 2 {
            letter = Some('1');
        } else if len == 3 {
            letter = Some('7');
        } else if len == 4 {
            letter = Some('4');
        } else if len == 7 {
            letter = Some('8');
        } else {
            letter = None;
        }

        Wire {
            signal: signal.to_string(),
            letter,
            len,
        }
    }
}

fn main() {
    let mut entries: Vec<Entry> = include_str!("../../sample.in")
        .lines()
        .map(Entry::new)
        .collect();
    // println!("entries: {:?}", entries);
    for entry in entries {
        // println!("Entry: {:?}", entry);
        let solution = entry.get_solution();
        // println!("{:?}", solution);
        let decoded = entry.decode(solution);
        println!("{:?}", decoded);
    }
}

fn get_result(wires: Vec<String>, output: Vec<String>) -> i32 {
    let mut table: HashMap<usize, Vec<String>> = HashMap::new();
    wires.iter().for_each(|wire_str| {
        table
            .entry(wire_str.len() - 1)
            .and_modify(|ls| ls.push(wire_str.to_string()))
            .or_insert_with(|| vec![wire_str.to_string()]);
    });
    println!("table: {:?}", table);
    1
}
