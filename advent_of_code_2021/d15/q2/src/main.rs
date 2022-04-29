use priority_queue::PriorityQueue;
use std::collections::HashSet;
use std::fmt::Debug;

fn print_matrix<T: Debug>(matrix: &[Vec<T>]) {
    matrix.iter().for_each(|row| {
        println!("{:?}", row);
    });
}

fn valid_move(next_pos: &(i32, i32), bounds: &(i32, i32)) -> bool {
    next_pos.0 < bounds.0 && next_pos.0 >= 0 && (next_pos.1 < bounds.1 && next_pos.1 >= 0)
}

const INF: i32 = i32::MAX;
const MULT: usize = 5;

fn extend_tile(tile: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![0u32; tile.len() * MULT]; tile.len() * MULT];

    for y in 0..MULT {
        for x in 0..MULT {
            for i in 0..tile.len() {
                for j in 0..tile[0].len() {
                    let k = y * tile.len() + i;
                    let l = x * tile[0].len() + j;

                    matrix[k][l] = tile[i][j] + (y + x) as u32;
                    if matrix[k][l] > 9 {
                        matrix[k][l] -= 9;
                    }
                }
            }
        }
    }

    matrix
}

fn main() {
    let tile = include_str!("../../test.in")
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u32)
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let matrix = extend_tile(tile);

    let mut distances = vec![vec![INF; matrix[0].len()]; matrix.len()];
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let max_rows = matrix.len() as i32;
    let max_cols = matrix[0].len() as i32;
    let bounds = (max_rows, max_cols);
    // distances[0][0] = 0;
    let mut pq: PriorityQueue<(i32, i32), i32> = PriorityQueue::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    distances[0][0] = 0;
    pq.push((0, 0), 0);

    while let Some(item) = pq.pop() {
        let (pos, weight) = item;
        visited.insert(pos);
        directions.iter().for_each(|dir| {
            let next_i = (dir.0 + pos.0) as i32;
            let next_j = (dir.1 + pos.1) as i32;
            let next_pos = (next_i, next_j);

            if !visited.contains(&next_pos) && valid_move(&next_pos, &bounds) {
                let next_i = next_i as usize;
                let next_j = next_j as usize;

                let candidate_dist = -weight + matrix[next_i][next_j] as i32;

                if candidate_dist < distances[next_i][next_j] {
                    distances[next_i][next_j] = candidate_dist;
                    pq.push_increase(next_pos, -candidate_dist);
                }
            }
        });
    }
    println!(
        "result {}",
        distances[distances.len() - 1][distances[0].len() - 1]
    );
}
