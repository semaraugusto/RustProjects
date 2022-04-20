#![deny(clippy::infinite_iter)]

extern crate queues;
use queues::*;

fn main() {
    let mut map: Vec<Vec<i32>> = include_str!("../../test.in")
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();

    let mut counter = 0;

    for round in 0.. {
        let count_zeros = map.iter().fold(0, |outer_acc, row| {
            outer_acc
                + row
                    .iter()
                    .fold(0, |inner_acc, val| inner_acc + (*val == 0) as i32)
        });

        if count_zeros >= 100 {
            counter = round;
            break;
        } else {
            run_round(&mut map);
        }
    }

    println!("result: {:?}", counter);
}

fn valid_transition(map: &[Vec<i32>], next_pos: (i32, i32)) -> bool {
    let height = map.len();
    let width = map[0].len();
    !out_of_bounds(next_pos.0, next_pos.1, height, width)
}

fn out_of_bounds(y: i32, x: i32, height: usize, width: usize) -> bool {
    y < 0 || y >= height as i32 || x < 0 || x >= width as i32
}

fn run_round(map: &mut [Vec<i32>]) {
    let mut queue: Queue<(usize, usize)> = Queue::new();

    map.iter_mut()
        .for_each(|row| row.iter_mut().for_each(|val| *val += 1));

    for (i, row) in map.iter_mut().enumerate() {
        for (j, val) in row.iter_mut().enumerate() {
            if *val > 9 {
                queue.add((i, j)).unwrap();
            }
        }
    }

    let mut visited: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];

    recursive_flashing(&mut visited, map, &mut 0, &mut queue);
}

fn recursive_flashing(
    visited: &mut [Vec<bool>],
    map: &mut [Vec<i32>],
    counter: &mut i32,
    q: &mut Queue<(usize, usize)>,
) -> i32 {
    while q.size() != 0 {
        let last_elem = q.peek().unwrap();
        let (i, j) = last_elem;
        map[i][j] = 0;
        q.remove().unwrap();
        if !visited[i][j] {
            let directions: [(i32, i32); 8] = [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ];

            directions.iter().for_each(|dir| {
                let y = i as i32 + dir.0;
                let x = j as i32 + dir.1;
                if valid_transition(map, (y, x)) {
                    if map[y as usize][x as usize] != 0 {
                        map[y as usize][x as usize] += 1;
                    }

                    if map[y as usize][x as usize] > 9 {
                        q.add((y as usize, x as usize)).unwrap();
                    }
                }
            });

            visited[i][j] = true;
            *counter += 1;
            recursive_flashing(visited, map, counter, q);
        }
    }
    *counter
}
