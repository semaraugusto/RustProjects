use std::collections::HashSet;

fn solve_part1(input: &str) -> u32 {
    let mut result: u32 = 0;
    input.split_terminator('\n').for_each(|sack| {
        let split_point = sack.len() / 2;
        let left = &sack[..split_point];
        let right = &sack[split_point..];
        let mut common = None;
        for l in left.chars() {
            for r in right.chars() {
                if l == r {
                    common = Some(r);
                    break;
                }
            }
        }

        match common {
            Some(x) => result += get_priority(x),
            None => unreachable!("nice input my guy"),
        };
        println!("{:?}", result);
    });
    result
}

fn get_priority(c: char) -> u32 {
    if c.is_uppercase() {
        c as u32 - 'A' as u32 + 27
    } else {
        c as u32 - 'a' as u32 + 1
    }
}

fn make_set(input: &str) -> HashSet<char> {
    let mut set = HashSet::new();
    for c in input.chars() {
        let entry = set.insert(c);
    }
    set
}

fn solve_part2(input: &str) -> u32 {
    let mut results = vec![];
    for group in input
        .split_terminator('\n')
        .collect::<Vec<&str>>()
        .chunks(3)
    {
        let group0 = make_set(group[0]);
        let group1 = make_set(group[1]);
        let group2 = make_set(group[2]);
        for elem in group0.iter() {
            if group1.contains(elem) && group2.contains(elem) {
                let result = get_priority(*elem);
                results.push(result);
            }
        }
    }
    println!("results = {:?}", results.iter().sum::<u32>());
    results.iter().sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let result = solve_part1(input);
        assert_eq!(result, 157);
    }
    #[test]
    fn test_part1_2() {
        let input = include_str!("../input.txt");
        let result = solve_part1(input);
        assert_eq!(result, 7581);
    }
    #[test]
    fn test_part2_1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let result = solve_part2(input);
        assert_eq!(result, 70);
    }
    #[test]
    fn test_part2_2() {
        let input = include_str!("../input.txt");
        let result = solve_part2(input);
        assert_eq!(result, 2525);
    }
}
