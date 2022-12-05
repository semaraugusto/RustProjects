fn solve_part1(input: &str) -> String {
    let (stacks_in, moves_in) = input.split_once("\n\n").unwrap();
    let mut stacks = stacks_in.split("\n").fold(Vec::new(), |mut acc, stack_in| {
        acc.push(
            stack_in
                .split_whitespace()
                .fold(Vec::new(), |mut single_stack, item| {
                    single_stack.push(item);
                    single_stack
                }),
        );
        acc
    });
    moves_in.split('\n').for_each(|single_move| {
        let move_vec = single_move.split_whitespace().collect::<Vec<&str>>();
        if move_vec.is_empty() {
            return;
        }
        let mut move_iter = move_vec.iter();
        let count = move_iter.nth(1).unwrap().parse::<u32>().unwrap();
        let from = move_iter.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let to = move_iter.nth(1).unwrap().parse::<usize>().unwrap() - 1;

        for _ in 0..count {
            let item = stacks[from].pop().unwrap();
            stacks[to].push(item);
        }
    });
    stacks
        .iter()
        .fold(String::new(), |result: String, stack| match stack.last() {
            Some(item) => result + item,
            None => result,
        })
}

fn solve_part2(input: &str) -> String {
    let (stacks_in, moves_in) = input.split_once("\n\n").unwrap();
    let mut stacks = stacks_in.split("\n").fold(Vec::new(), |mut acc, stack_in| {
        acc.push(
            stack_in
                .split_whitespace()
                .fold(Vec::new(), |mut single_stack, item| {
                    single_stack.push(item);
                    single_stack
                }),
        );
        acc
    });
    moves_in.split('\n').for_each(|single_move| {
        let move_vec = single_move.split_whitespace().collect::<Vec<&str>>();
        if move_vec.is_empty() {
            return;
        }
        let mut move_iter = move_vec.iter();
        let count = move_iter.nth(1).unwrap().parse::<u32>().unwrap();
        let from = move_iter.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let to = move_iter.nth(1).unwrap().parse::<usize>().unwrap() - 1;

        let mut temp = vec![];
        for _ in 0..count {
            let item = stacks[from].pop().unwrap();
            temp.push(item);
        }
        for item in temp.iter().rev() {
            stacks[to].push(item);
        }
    });
    stacks
        .iter()
        .fold(String::new(), |result: String, stack| match stack.last() {
            Some(item) => result + item,
            None => result,
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_1() {
        let input = "Z N 
M C D 
P

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let result = solve_part1(input);
        assert_eq!(result, "CMZ");
    }
    #[test]
    fn test_part1_2() {
        let input = include_str!("../input.txt");
        let result = solve_part1(input);
        assert_eq!(result, "FWSHSPJWM");
    }
    #[test]
    fn test_part2_1() {
        let input = "Z N 
M C D 
P

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let result = solve_part2(input);
        assert_eq!(result, "MCD");
    }
    #[test]
    fn test_part2_2() {
        let input = include_str!("../input.txt");
        let result = solve_part2(input);
        assert_eq!(result, "PWPWHGFZS");
    }
}
