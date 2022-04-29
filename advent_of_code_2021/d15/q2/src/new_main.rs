use priority_queue::PriorityQueue;
use std::collections::HashSet;
use std::fmt::Debug;

fn print_matrix<T: Debug>(matrix: &[Vec<T>]) {
    println!("Start matrix");
    matrix.iter().for_each(|row| {
        println!("{:?}", row);
    });
    println!("End matrix");
}

fn valid_move(dir: &(u32, u32), pos: &(u32, u32), bounds: &(u32, u32)) -> bool {
    dir.0 + pos.0 < bounds.0 && dir.1 + pos.1 < bounds.1
}

fn get_neighbours(pos: (u32, u32), matrix: &[Vec<u32>]) -> Vec<(u32, u32)> {
    let max_rows = matrix.len() as u32;
    let max_cols = matrix[0].len() as u32;
    let bounds = (max_rows, max_cols);
    let mut neighbours = Vec::new();
    let directions = [(-1, -1)];

    directions.iter().for_each(|dir| {
        if valid_move(dir, &pos, &bounds) {
            neighbours.push((pos.0 + dir.0, pos.1 + dir.1));
        }
    });
    neighbours
}

const INF: u32 = 200;

fn main() {
    let matrix = include_str!("../../sample.in")
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    print_matrix(&matrix);
    const START_POSITION: (u32, u32) = (0, 0);
    let end_position: (u32, u32) = (matrix.len() as u32 - 1, matrix[0].len() as u32 - 1);
    println!("starting at {:?}", START_POSITION);
    println!("ending at {:?}", end_position);

    let mut distances = vec![vec![INF; matrix[0].len()]; matrix.len()];
    distances[0][0] = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            neighbours = get_neighbours(&distances);
            if distances[i][j] == INF {}
        }
    }
    // print_matrix(&distances);
}
