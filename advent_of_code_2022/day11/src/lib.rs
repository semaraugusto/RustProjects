#[derive(Debug)]
pub enum Operator {
    AddSelf,
    MulSelf,
    Add(usize),
    Mul(usize),
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    op: Option<Operator>,
    divisible_by: usize,
    dest: (usize, usize),
    num_inspections: usize,
}

impl Monkey {
    fn new(monkey_input: &str) -> Self {
        let mut iter = monkey_input.lines();
        println!("{:?}", iter.next());
        let items = iter
            .next()
            .unwrap()
            .trim()
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|s| {
                println!("s: -{}-", s.trim());
                let num = s.trim().parse::<u64>().unwrap();
                num
            })
            .collect();
        let operation = iter.next().unwrap();
        let op: Option<Operator>;
        match operation {
            op_str if op_str.contains("Operation: ") => {
                let op_str: Vec<&str> = op_str
                    .split_once("= ")
                    .unwrap()
                    .1
                    .split_whitespace()
                    .collect();
                match (op_str[0], op_str[1], op_str[2]) {
                    ("old", "*", "old") => op = Some(Operator::MulSelf),
                    ("old", "+", "old") => op = Some(Operator::AddSelf),
                    ("old", "*", number) => {
                        op = Some(Operator::Mul(number.trim().parse::<usize>().unwrap()))
                    }
                    ("old", "+", number) => {
                        op = Some(Operator::Add(number.trim().parse::<usize>().unwrap()))
                    }
                    _ => panic!("Unknown operation"),
                }
            }
            _ => panic!("Invalid operation"),
        }

        let divisible_by = iter
            .next()
            .unwrap()
            .split_once("by ")
            .unwrap()
            .1
            .trim()
            .parse::<usize>()
            .unwrap();

        let first = iter
            .next()
            .unwrap()
            .split_once("monkey ")
            .unwrap()
            .1
            .trim()
            .parse::<usize>()
            .unwrap();
        let second = iter
            .next()
            .unwrap()
            .split_once("monkey ")
            .unwrap()
            .1
            .trim()
            .parse::<usize>()
            .unwrap();
        Self {
            items,
            op,
            divisible_by,
            dest: (first, second),
            num_inspections: 0,
        }
    }

    fn inspect(&mut self, idx: usize) {
        let item = self.items[idx];
        match self.op {
            Some(Operator::AddSelf) => self.items[idx] += item,
            Some(Operator::MulSelf) => self.items[idx] *= item,
            Some(Operator::Add(num)) => self.items[idx] += num as u64,
            Some(Operator::Mul(num)) => self.items[idx] *= num as u64,
            None => (),
        }
        self.items[idx] /= 3;

        match self.items[idx] % self.divisible_by as u64 {
            0 => (),
            _ => {
                self.items[idx] +=
                    self.divisible_by as u64 - self.items[idx] % self.divisible_by as u64
            }
        }
    }
}

impl std::fmt::Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Monkey: items: {:?}, op: {:?}, div_by: {}, dest: {:?} num_inspections: {}",
            self.items, self.op, self.divisible_by, self.dest, self.num_inspections
        )
    }
}

const TOTAL_ROUNDS: usize = 20;
fn part1(input: &str) -> u64 {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::new).collect();

    for round in 0..TOTAL_ROUNDS {
        for monkey_idx in 0..monkeys.len() {
            for item_idx in 0..monkeys[monkey_idx].items.len() {
                monkeys[monkey_idx].num_inspections += 1;
                match monkeys[monkey_idx].op {
                    Some(Operator::AddSelf) => monkeys[monkey_idx].items[item_idx] *= 2,
                    Some(Operator::MulSelf) => {
                        monkeys[monkey_idx].items[item_idx] *= monkeys[monkey_idx].items[item_idx]
                    }
                    Some(Operator::Add(num)) => monkeys[monkey_idx].items[item_idx] += num as u64,
                    Some(Operator::Mul(num)) => monkeys[monkey_idx].items[item_idx] *= num as u64,
                    None => (),
                }
                monkeys[monkey_idx].items[item_idx] /= 3;
            }
            while monkeys[monkey_idx].items.len() > 0 {
                let test = monkeys[monkey_idx].items[0] % monkeys[monkey_idx].divisible_by as u64;
                match test {
                    0 => {
                        let destination = monkeys[monkey_idx].dest.0;
                        let item = monkeys[monkey_idx].items[0];
                        monkeys[monkey_idx].items.swap_remove(0);
                        monkeys[destination].items.push(item);
                    }
                    _ => {
                        let destination = monkeys[monkey_idx].dest.1;
                        let item = monkeys[monkey_idx].items[0];
                        monkeys[monkey_idx].items.swap_remove(0);
                        monkeys[destination].items.push(item);
                    }
                }
            }
        }
        println!("ROUND: {}", round);
        for (i, monkey) in monkeys.iter().enumerate() {
            println!("monkey {}: {}", i, monkey);
        }
    }
    let top_two = find_top_two(&monkeys);
    top_two[0] as u64 * top_two[1] as u64
}
fn find_top_two(monkeys: &[Monkey]) -> Vec<usize> {
    use std::collections::BinaryHeap;
    let scores: Vec<usize> = monkeys
        .iter()
        .map(|monkey| monkey.num_inspections)
        .collect();
    let mut heap = scores.iter().copied().collect::<BinaryHeap<usize>>();
    let mut top_two = Vec::new();
    for _ in 0..2 {
        if let Some(v) = heap.pop() {
            top_two.push(v);
        }
    }
    top_two
}

const TOTAL_ROUNDS_PART2: usize = 10_000;
fn part2(input: &str, total_rounds: usize) -> u64 {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::new).collect();

    let mmc = monkeys
        .iter()
        .fold(1, |acc, monkey| acc * monkey.divisible_by);
    println!("mmc: {}", mmc);
    for round in 0..TOTAL_ROUNDS_PART2 {
        for monkey_idx in 0..monkeys.len() {
            for item_idx in 0..monkeys[monkey_idx].items.len() {
                monkeys[monkey_idx].num_inspections += 1;
                match monkeys[monkey_idx].op {
                    Some(Operator::AddSelf) => monkeys[monkey_idx].items[item_idx] *= 2,
                    Some(Operator::MulSelf) => {
                        monkeys[monkey_idx].items[item_idx] *= monkeys[monkey_idx].items[item_idx]
                    }
                    Some(Operator::Add(num)) => monkeys[monkey_idx].items[item_idx] += num as u64,
                    Some(Operator::Mul(num)) => monkeys[monkey_idx].items[item_idx] *= num as u64,
                    None => (),
                }
            }
            while !monkeys[monkey_idx].items.is_empty() {
                let test = monkeys[monkey_idx].items[0] % monkeys[monkey_idx].divisible_by as u64;
                match test {
                    0 => {
                        let destination = monkeys[monkey_idx].dest.0;
                        let item = monkeys[monkey_idx].items[0];
                        monkeys[monkey_idx].items.swap_remove(0);
                        monkeys[destination].items.push(item % mmc as u64);
                    }
                    _ => {
                        let destination = monkeys[monkey_idx].dest.1;
                        let item = monkeys[monkey_idx].items[0];
                        monkeys[monkey_idx].items.swap_remove(0);
                        monkeys[destination].items.push(item % mmc as u64);
                    }
                }
            }
        }
        println!("ROUND: {}", round);
        for (i, monkey) in monkeys.iter().enumerate() {
            println!("monkey {}: {}", i, monkey);
        }
    }
    let top_two = find_top_two(&monkeys);
    top_two[0] as u64 * top_two[1] as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        let input = include_str!("../toy.txt");
        let result = part1(input);
        assert_eq!(result, 10605);
    }
    #[test]
    fn test_part1_2() {
        let input = include_str!("../input.txt");
        let result = part1(input);
        assert_eq!(result, 51075);
    }
    #[test]
    fn test_part2_1() {
        let input = include_str!("../toy.txt");
        let result = part2(input);
        assert_eq!(result, 2713310158);
    }
    #[test]
    fn test_part2_2() {
        let input = include_str!("../input.txt");
        let result = part2(input);
        assert_eq!(result, 11741456163);
    }
}
