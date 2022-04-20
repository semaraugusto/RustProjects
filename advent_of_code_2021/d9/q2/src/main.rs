extern crate queues;
use queues::*;

fn main() {
    let map: Vec<Vec<i32>> = include_str!("../../test.in")
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();

    let directions: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let height = map.len();
    let width = map[0].len();

    let mut basins: Vec<(usize, usize)> = Vec::new();
    for (i, row) in map.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            let is_smallest = directions.iter().fold(true, |is_smallest, dir| {
                let y = i as i32 + dir.0;
                let x = j as i32 + dir.1;
                if out_of_bounds(y, x, height, width) {
                    is_smallest
                } else {
                    is_smallest && *val < map[y as usize][x as usize]
                }
            });
            // println!("smallest: {:?}", is_smallest);
            if is_smallest {
                basins.push((i, j))
            }
        }
    }
    // println!("basins: {:?}", basins);
    let mut result: Vec<i32> = basins
        .iter()
        .map(|(i, j)| bfsearch(&map, *i, *j))
        .collect::<Vec<i32>>();

    result.sort_unstable();

    // println!("result: {:?}", result);
    result = result.into_iter().rev().take(3).collect();
    println!("result: {:?}", result.iter().product::<i32>());
}

fn valid_transition(map: &[Vec<i32>], next_pos: (i32, i32)) -> bool {
    let height = map.len();
    let width = map[0].len();
    !out_of_bounds(next_pos.0, next_pos.1, height, width)
        && map[next_pos.0 as usize][next_pos.1 as usize] != 9
}

fn out_of_bounds(y: i32, x: i32, height: usize, width: usize) -> bool {
    y < 0 || y >= height as i32 || x < 0 || x >= width as i32
}

fn bfsearch(map: &[Vec<i32>], i: usize, j: usize) -> i32 {
    let mut visited: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    let mut queue: Queue<(usize, usize)> = queue![(i, j)];

    let result = recursive_bfsearch(&mut visited, map, &mut queue);

    result.iter().fold(0, |outer_acc, row| {
        outer_acc + row.iter().fold(0, |inner_acc, b| inner_acc + *b as i32)
    })
}

fn recursive_bfsearch(
    visited: &mut [Vec<bool>],
    map: &[Vec<i32>],
    q: &mut Queue<(usize, usize)>,
) -> Vec<Vec<bool>> {
    while q.size() != 0 {
        let last_elem = q.peek().unwrap();
        let (i, j) = last_elem;
        // println!("le: {:?}", last_elem);
        q.remove().unwrap();
        // println!("i: {:?}", i);
        // println!("j: {:?}", j);
        if visited[i][j] {
            return visited.to_vec();
        }
        visited[i][j] = true;
        let directions: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        directions.iter().for_each(|dir| {
            let y = i as i32 + dir.0;
            let x = j as i32 + dir.1;
            if valid_transition(map, (y, x)) && map[y as usize][x as usize] > map[i][j] {
                q.add((y as usize, x as usize)).unwrap();
            }
        });
        recursive_bfsearch(visited, map, q);
    }
    visited.to_vec()
}
