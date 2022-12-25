use std::collections::HashMap;
use std::collections::VecDeque;

fn print_matrix<T: std::fmt::Debug>(matrix: &Vec<Vec<T>>) {
    for row in matrix.iter() {
        println!("{:?}", row);
    }
}
const INF: i32 = i32::MAX;
const DIR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
fn get_starting_pos(heights: &Vec<Vec<i32>>) -> (i32, i32) {
    let mut start = (0, 0);
    for (i, row) in heights.iter().enumerate() {
        match row.iter().position(|&x| x == -1) {
            Some(j) => start = (i as i32, j as i32),
            None => continue,
        }
    }
    start
}
fn get_ending_pos(heights: &Vec<Vec<i32>>) -> (i32, i32) {
    let mut end = (0, 0);
    for (i, row) in heights.iter().enumerate() {
        match row.iter().position(|&x| x == INF) {
            Some(j) => end = (i as i32, j as i32),
            None => continue,
        }
    }
    end
}
fn get_surrounding(
    heights: &Vec<Vec<i32>>,
    visited: &Vec<Vec<bool>>,
    position: (i32, i32),
) -> Vec<(i32, i32)> {
    let max_y = heights.len() - 1;
    let max_x = heights[0].len() - 1;
    DIR.iter()
        .map(|(y, x)| (position.0 as i32 + y, position.1 as i32 + x))
        .filter(|(y, x)| *y >= 0 && *x >= 0 && *y <= max_y as i32 && *x <= max_x as i32)
        .filter(|(y, x)| {
            (heights[*y as usize][*x as usize] - heights[position.0 as usize][position.1 as usize])
                < 2
        })
        // .filter(|(y, x)| visited[*y as usize][*x as usize] == false)
        .collect()
}

fn shortest_climb(heights: &Vec<Vec<i32>>, start_pos: (i32, i32), end_pos: (i32, i32)) -> i32 {
    let mut visited = vec![vec![false; heights[0].len()]; heights.len()];
    let mut to_visit = VecDeque::new();
    to_visit.push_back(start_pos);
    let mut shortest: Vec<Vec<i32>> = vec![vec![INF; heights[0].len()]; heights.len()];
    shortest[start_pos.0 as usize][start_pos.1 as usize] = 0;
    let mut prev_pos = start_pos;

    while let Some(position) = to_visit.pop_back() {
        visited[position.0 as usize][position.1 as usize] = true;
        let candidates = get_surrounding(heights, &visited, position);
        for cand in candidates.iter() {
            if shortest[cand.0 as usize][cand.1 as usize]
                > shortest[position.0 as usize][position.1 as usize] + 1
            {
                shortest[cand.0 as usize][cand.1 as usize] =
                    shortest[position.0 as usize][position.1 as usize] + 1;
                to_visit.push_back(*cand)
            }
        }
    }
    assert!(visited[end_pos.0 as usize][end_pos.1 as usize]);
    assert!(shortest[end_pos.0 as usize][end_pos.1 as usize] != INF);
    shortest[end_pos.0 as usize][end_pos.1 as usize] as i32
}

fn part1(input: &str) -> i32 {
    let mut heights: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'S' => -1,
                    'E' => INF,
                    c => c as i32 - 'a' as i32,
                })
                .collect()
        })
        .collect();

    let end_pos = get_ending_pos(&heights);
    let start_pos = get_starting_pos(&heights);
    heights[end_pos.0 as usize][end_pos.1 as usize] = 'z' as i32 - 'a' as i32;
    heights[start_pos.0 as usize][start_pos.1 as usize] = 0;
    // println!("{:?}", heights);
    print_matrix(&heights);
    shortest_climb(&heights, start_pos, end_pos)
}

fn get_all_starting_positions(heights: &Vec<Vec<i32>>) -> Vec<(i32, i32)> {
    let mut positions = vec![];
    for (i, row) in heights.iter().enumerate() {
        match row.iter().position(|&x| x == 0) {
            Some(j) => positions.push((i as i32, j as i32)),
            None => continue,
        }
    }
    positions
}
fn part2(input: &str) -> i32 {
    let mut heights: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'S' => 0,
                    'E' => INF,
                    c => c as i32 - 'a' as i32,
                })
                .collect()
        })
        .collect();

    let end_pos = get_ending_pos(&heights);
    heights[end_pos.0 as usize][end_pos.1 as usize] = 'z' as i32 - 'a' as i32;

    let all_starts = get_all_starting_positions(&heights);
    let mut shortest = INF;
    for start_pos in all_starts.iter() {
        let actual = shortest_climb(&heights, *start_pos, end_pos);
        if actual < shortest {
            shortest = actual;
        }
    }
    // println!("{:?}", heights);
    print_matrix(&heights);
    shortest
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    // #[ignore]
    fn part1_1() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        let result = part1(input);
        assert_eq!(result, 31);
    }
    #[test]
    // #[ignore]
    fn part1_2() {
        let input = include_str!("../input.txt");
        let result = part1(input);
        println!("i32::MAX: {:?}", i32::MAX);
        assert_eq!(result, 468);
    }
    #[test]
    // #[ignore]
    fn part2_1() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        let result = part2(input);
        assert_eq!(result, 29);
    }
    #[test]
    // #[ignore]
    fn part2_2() {
        let input = include_str!("../input.txt");
        let result = part2(input);
        assert_eq!(result, 29);
    }
}
