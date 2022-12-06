use std::collections::HashSet;

fn num_unique_elements(queue: &Vec<char>) -> usize {
    let mut set = HashSet::new();
    for elem in queue.iter() {
        set.insert(elem);
    }
    set.len()
}

fn solve(input: &str, num_unique_chars: usize) -> usize {
    let mut result_idx = 0;
    let mut marker: Vec<char> = vec![];
    for (i, c) in input.chars().enumerate() {
        if marker.len() == num_unique_chars {
            let unique = num_unique_elements(&marker);
            if unique == num_unique_chars {
                result_idx = i;
                break;
            }
            marker.remove(0);
        }
        marker.push(c);
    }
    println!("{}", result_idx);

    result_idx
}

fn part1(input: &str) -> usize {
    let num_unique_chars = 4;
    solve(input, num_unique_chars)
}

fn part2(input: &str) -> usize {
    let num_unique_chars = 14;
    solve(input, num_unique_chars)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let result = part1(input);
        assert_eq!(result, 7);

        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let result = part1(input);
        assert_eq!(result, 5);

        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let result = part1(input);
        assert_eq!(result, 6);

        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let result = part1(input);
        assert_eq!(result, 10);

        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let result = part1(input);
        assert_eq!(result, 11);
    }
    #[test]
    fn test_part1_2() {
        let input = include_str!("../input.txt");
        let result = part1(input);
        assert_eq!(result, 1134);
    }
    #[test]
    fn test_part2_1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let result = part2(input);
        assert_eq!(result, 19);

        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let result = part2(input);
        assert_eq!(result, 23);

        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let result = part2(input);
        assert_eq!(result, 23);

        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let result = part2(input);
        assert_eq!(result, 29);

        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let result = part2(input);
        assert_eq!(result, 26);
    }
    #[test]
    fn test_part2_2() {
        let input = include_str!("../input.txt");
        let result = part2(input);
        assert_eq!(result, 2263);
    }
}
