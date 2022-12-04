#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(range_in: &str) -> Self {
        let start = range_in.split('-').next().unwrap().parse::<u64>().unwrap();
        let end = range_in.split('-').nth(1).unwrap().parse::<u64>().unwrap();

        Self { start, end }
    }

    fn is_fully_contained_in(&self, other: &Range) -> bool {
        if self.start >= other.start && self.end <= other.end {
            true
        } else {
            false
        }
    }
    fn overlaps(&self, other: &Range) -> bool {
        println!(
            "self({} {}) -> other({}, {})",
            self.start, self.end, other.start, other.end
        );
        if other.start <= self.end && other.start >= self.start {
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
struct Pair {
    left: Range,
    right: Range,
}

impl Pair {
    fn new(line: &str) -> Self {
        let mut iter = line.split(',');
        let left = Range::new(iter.next().unwrap());
        let right = Range::new(iter.next().unwrap());
        Self { left, right }
    }
    // fn is_fully_contained(&self) -> bool {
    //     self.left.contains(&self.right)
    // }
    fn is_fully_contained(&self) -> bool {
        let is_left_in_right = self.left.is_fully_contained_in(&self.right);
        let is_right_in_left = self.right.is_fully_contained_in(&self.left);
        is_left_in_right || is_right_in_left
    }
    fn overlaps(&self) -> bool {
        // self.left.overlaps(&self.right)
        let left_overlaps = self.left.overlaps(&self.right);
        let right_overlaps = self.right.overlaps(&self.left);
        left_overlaps || right_overlaps
    }
}

fn solve_part1(input: &str) -> u32 {
    let mut sum: u32 = 0;
    input.split_terminator('\n').for_each(|line| {
        let pair = Pair::new(line);
        let is_fully_contained = pair.is_fully_contained();
        println!(
            "{:?} {:?} -> fully_contained? {}",
            pair.left, pair.right, is_fully_contained
        );
        sum += is_fully_contained as u32;
    });
    sum
}

fn solve_part2(input: &str) -> u32 {
    let mut sum: u32 = 0;
    input.split_terminator('\n').for_each(|line| {
        let pair = Pair::new(line);
        let overlaps = pair.overlaps();
        // println!("{:?} {:?} -> overlaps? {}", pair.left, pair.right, overlaps);
        sum += overlaps as u32;
    });
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_1() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let result = solve_part1(input);
        assert_eq!(result, 2);
    }
    #[test]
    fn test_part1_2() {
        let input = include_str!("../input.txt");
        let result = solve_part1(input);
        assert_eq!(result, 582);
    }
    #[test]
    fn test_part2_1() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let result = solve_part2(input);
        assert_eq!(result, 4);
    }
    #[test]
    fn test_part2_2() {
        let input = include_str!("../input.txt");
        let result = solve_part2(input);
        assert_eq!(result, 893);
    }
}
