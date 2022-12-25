pub fn print_matrix<T: std::fmt::Debug>(matrix: &[Vec<T>]) {
    for row in matrix.iter() {
        println!("{:?}", row);
    }
    println!();
}

#[derive(Debug, Clone)]
pub enum Orientation {
    Right,
    Down,
    Left,
    Up,
}

pub fn is_empty(chr: char) -> bool {
    chr == '.' || chr == '>' || chr == 'v' || chr == '<' || chr == '^'
}

const ORIENTATIONS: [Orientation; 4] = [
    Orientation::Right,
    Orientation::Down,
    Orientation::Left,
    Orientation::Up,
];
pub fn get_starting_position(grid: &[Vec<char>]) -> (usize, usize) {
    // println!("Starting matrix:");
    // print_matrix(grid);

    let top_row = grid[0].clone();
    for (j, char) in top_row.iter().enumerate() {
        if *char != '~' {
            return (0, j);
        }
    }
    panic!("Couldn't find starting position");
}
fn tokenize_instructions(instr: &str) -> Vec<String> {
    let mut number_str = "".to_string();
    let mut tokens = vec![];
    for char in instr.chars() {
        if char.to_digit(10).is_some() {
            let prev = number_str.clone();
            number_str = prev + &char.to_string()
        } else {
            tokens.push(number_str.clone());
            number_str = "".to_string();
            tokens.push(char.to_string());
        }
    }
    tokens
}

pub fn loop_around(grid: &[Vec<char>], i: usize, j: usize, ori: Orientation) -> usize {
    match ori {
        Orientation::Right => {
            for idx in 0..grid[0].len() {
                if grid[i][idx] != '~' {
                    return idx;
                }
            }
        }
        Orientation::Down => {
            // println!("looping around column: {j}");
            for idx in 0..grid.len() {
                if grid[idx][j] != '~' {
                    return idx;
                }
            }
        }
        Orientation::Left => {
            for idx in (0..grid[0].len()).rev() {
                if grid[i][idx] != '~' {
                    return idx;
                }
            }
        }
        Orientation::Up => {
            for idx in (0..grid.len()).rev() {
                if grid[idx][j] != '~' {
                    return idx;
                }
            }
        }
    }

    panic!("could not find any . characters in loop around");
}

pub fn part1(input: &str) -> usize {
    let (map, instr) = input.split_once("\n\n").unwrap();
    let map: Vec<&str> = map.split('\n').collect();
    println!("MAP SHAPE: {} {}", map.len(), map[0].len());
    let mut num_col = 0;
    for line in map.iter() {
        if num_col < line.len() {
            num_col = line.len();
        }
        // println!("{}", line);
    }
    // println!("instr: {:?}", instr);
    // let num_cols =
    let mut grid = vec![vec!['~'; num_col]; map.len()];
    for (i, line) in map.iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char != ' ' {
                grid[i][j] = char;
            }
        }
    }
    // print_matrix(&grid);
    let mut pos = get_starting_position(&grid);
    let mut orientation_idx = 0;
    let mut orientation = ORIENTATIONS[orientation_idx].clone();
    let instructions = tokenize_instructions(instr);
    println!("starting: {:?}, looking to {:?}", pos, orientation);
    for token in instructions.iter() {
        match token.as_str() {
            "R" => {
                println!("Instruction: {:?}", token);
                orientation_idx = (orientation_idx + 1) % 4;
                orientation = ORIENTATIONS[orientation_idx].clone();
                println!("position: {:?}, looking to {:?}", pos, orientation);
                if grid[pos.0][pos.1] == '~' {
                    panic!("invalid transition has happened");
                }
                match orientation {
                    Orientation::Right => grid[pos.0][pos.1] = '>',
                    Orientation::Down => grid[pos.0][pos.1] = 'v',
                    Orientation::Left => grid[pos.0][pos.1] = '<',
                    Orientation::Up => grid[pos.0][pos.1] = '^',
                }
            }
            "L" => {
                println!("Instruction: {:?}", token);
                let temp = orientation_idx as i32 - 1i32;
                if temp < 0 {
                    orientation_idx = (4 + temp) as usize;
                } else {
                    orientation_idx = temp as usize;
                }
                orientation = ORIENTATIONS[orientation_idx].clone();
                println!("position: {:?}, looking to {:?}", pos, orientation);
                if grid[pos.0][pos.1] == '~' {
                    panic!("invalid transition has happened. Pos: {:?}", pos);
                }
                match orientation {
                    Orientation::Right => grid[pos.0][pos.1] = '>',
                    Orientation::Down => grid[pos.0][pos.1] = 'v',
                    Orientation::Left => grid[pos.0][pos.1] = '<',
                    Orientation::Up => grid[pos.0][pos.1] = '^',
                }
            }
            num => {
                if num != "\n" {
                    let mut distance = num.parse::<usize>().unwrap();
                    match orientation {
                        Orientation::Right => {
                            println!(
                                "at: ({pos:?}). Walking {distance} Right. Curr {}",
                                grid[pos.0][pos.1]
                            );
                            grid[pos.0][pos.1] = '>';
                            let mut end_idx = None;
                            let mut peek_idx = pos.1 + 1;
                            while distance > 0 {
                                // println!("at: ({:?}). Walking {distance} Right. Curr {}", (pos.0, peek_idx), grid[pos.0][peek_idx]);
                                if peek_idx >= grid[0].len() || grid[pos.0][peek_idx] == '~' {
                                    peek_idx =
                                        loop_around(&grid, pos.0, peek_idx, Orientation::Right);
                                }
                                if grid[pos.0][peek_idx] == '#' {
                                    break;
                                }
                                // println!("at2: ({:?}). Walking {distance} Right. Curr {}", (pos.0, peek_idx), grid[pos.0][peek_idx]);
                                end_idx = Some(peek_idx);
                                grid[pos.0][peek_idx] = '>';
                                peek_idx += 1;
                                distance -= 1;
                            }
                            match end_idx {
                                Some(idx) => pos.1 = idx,
                                None => (),
                            }
                            // pos = (pos.0, end_idx);
                        }
                        Orientation::Down => {
                            println!(
                                "at: ({pos:?}). Walking {distance} Down. Curr {}",
                                grid[pos.0][pos.1]
                            );
                            grid[pos.0][pos.1] = 'v';
                            let mut end_idx = None;
                            let mut peek_idx = pos.0 + 1;
                            while distance > 0 {
                                if peek_idx >= grid.len() || grid[peek_idx][pos.1] == '~' {
                                    peek_idx =
                                        loop_around(&grid, peek_idx, pos.1, Orientation::Down);
                                }
                                if grid[peek_idx][pos.1] == '#' {
                                    break;
                                }
                                end_idx = Some(peek_idx);
                                grid[peek_idx][pos.1] = 'v';
                                peek_idx += 1;
                                distance -= 1;
                            }
                            match end_idx {
                                Some(idx) => pos.0 = idx,
                                None => (),
                            }
                        }
                        Orientation::Left => {
                            println!(
                                "at: ({pos:?}). Walking {distance} Left. Curr {}",
                                grid[pos.0][pos.1]
                            );
                            grid[pos.0][pos.1] = '<';
                            let mut end_idx = None;
                            let mut peek_idx: i32 = pos.1 as i32 - 1;
                            while distance > 0 {
                                // println!("at: ({} {}). Walking {distance} Left. Curr {}", pos.0, peek_idx, grid[pos.0][peek_idx as usize]);
                                if peek_idx < 0 || grid[pos.0][peek_idx as usize] == '~' {
                                    peek_idx = loop_around(
                                        &grid,
                                        pos.0,
                                        peek_idx as usize,
                                        Orientation::Left,
                                    ) as i32;
                                }
                                if grid[pos.0][peek_idx as usize] == '#' {
                                    break;
                                }
                                end_idx = Some(peek_idx);
                                grid[pos.0][peek_idx as usize] = '<';
                                peek_idx -= 1;
                                distance -= 1;
                            }
                            // pos = (pos.0, end_idx as usize);
                            match end_idx {
                                Some(idx) => pos.1 = idx as usize,
                                None => (),
                            }
                        }
                        Orientation::Up => {
                            println!(
                                "at: ({pos:?}). Walking {distance} Up. Curr {}",
                                grid[pos.0][pos.1]
                            );
                            grid[pos.0][pos.1] = '^';
                            let mut end_idx = None;
                            let mut peek_idx: i32 = pos.0 as i32 - 1;
                            while distance > 0 {
                                if peek_idx < 0 || grid[peek_idx as usize][pos.1] == '~' {
                                    peek_idx = loop_around(
                                        &grid,
                                        peek_idx as usize,
                                        pos.1,
                                        Orientation::Up,
                                    ) as i32;
                                }
                                if grid[peek_idx as usize][pos.1] == '#' {
                                    break;
                                }
                                end_idx = Some(peek_idx);
                                grid[peek_idx as usize][pos.1] = '^';
                                peek_idx -= 1;
                                distance -= 1;
                            }
                            // pos = (end_idx as usize, pos.1);
                            match end_idx {
                                Some(idx) => pos.0 = idx as usize,
                                None => (),
                            }
                        }
                    }
                }
            }
        }
    }
    // print_matrix(&grid);

    println!("ending position: {:?}", pos);
    let facing = match orientation {
        Orientation::Right => 0,
        Orientation::Down => 1,
        Orientation::Left => 2,
        Orientation::Up => 3,
    };
    (1000 * (pos.0 + 1)) + (4 * (pos.1 + 1)) + facing
}

pub fn scan_faces(grid: &[Vec<char>]) -> u32 {
    println!("scan faces");
    println!("shape {} {}", grid.len(), grid[0].len());
    let max_width = grid
        .iter()
        .fold(0, |acc, row| if acc > row.len() { acc } else { row.len() });
    let temp = vec![vec!['~'; max_width]; grid.len()];
    print_matrix(&temp);
    0
}

pub fn part2(input: &str) -> usize {
    let (map, instr) = input.split_once("\n\n").unwrap();
    let map: Vec<&str> = map.split('\n').collect();
    println!("MAP SHAPE: {} {}", map.len(), map[0].len());
    let mut num_col = 0;
    for line in map.iter() {
        if num_col < line.len() {
            num_col = line.len();
        }
        // println!("{}", line);
    }
    // println!("instr: {:?}", instr);
    // let num_cols =
    let mut grid = vec![vec!['~'; num_col]; map.len()];
    for (i, line) in map.iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char != ' ' {
                grid[i][j] = char;
            }
        }
    }
    // print_matrix(&grid);
    let mut pos = get_starting_position(&grid);
    let mut orientation_idx = 0;
    let mut orientation = ORIENTATIONS[orientation_idx].clone();
    let instructions = tokenize_instructions(instr);
    println!("starting: {:?}, looking to {:?}", pos, orientation);
    let scan = scan_faces(&grid);
    print_matrix(&grid);

    println!("scan: {:?}", scan);
    // for token in instructions.iter() {
    //     match token.as_str() {
    //         "R" => {
    //             println!("Instruction: {:?}", token);
    //             orientation_idx = (orientation_idx + 1) % 4;
    //             orientation = ORIENTATIONS[orientation_idx].clone();
    //             println!("position: {:?}, looking to {:?}", pos, orientation);
    //             if grid[pos.0][pos.1] == '~' {
    //                 panic!("invalid transition has happened");
    //             }
    //             match orientation {
    //                 Orientation::Right => grid[pos.0][pos.1] = '>',
    //                 Orientation::Down => grid[pos.0][pos.1] = 'v',
    //                 Orientation::Left => grid[pos.0][pos.1] = '<',
    //                 Orientation::Up => grid[pos.0][pos.1] = '^',
    //             }
    //         }
    //         "L" => {
    //             println!("Instruction: {:?}", token);
    //             let temp = orientation_idx as i32 - 1i32;
    //             if temp < 0 {
    //                 orientation_idx = (4 + temp) as usize;
    //             } else {
    //                 orientation_idx = temp as usize;
    //             }
    //             orientation = ORIENTATIONS[orientation_idx].clone();
    //             println!("position: {:?}, looking to {:?}", pos, orientation);
    //             if grid[pos.0][pos.1] == '~' {
    //                 panic!("invalid transition has happened. Pos: {:?}", pos);
    //             }
    //             match orientation {
    //                 Orientation::Right => grid[pos.0][pos.1] = '>',
    //                 Orientation::Down => grid[pos.0][pos.1] = 'v',
    //                 Orientation::Left => grid[pos.0][pos.1] = '<',
    //                 Orientation::Up => grid[pos.0][pos.1] = '^',
    //             }
    //         }
    //         num => {
    //             if num != "\n" {
    //                 let mut distance = num.parse::<usize>().unwrap();
    //                 match orientation {
    //                     Orientation::Right => {
    //                         let face = get_current_face(&grid, &pos);
    //                         println!(
    //                             "at: ({pos:?}). Walking {distance} Right. at Face {face} Curr {}",
    //                             grid[pos.0][pos.1]
    //                         );
    //                         grid[pos.0][pos.1] = '>';
    //                         let mut end_idx = None;
    //                         let mut peek_idx = pos.1 + 1;
    //                         while distance > 0 {
    //                             // println!("at: ({:?}). Walking {distance} Right. Curr {}", (pos.0, peek_idx), grid[pos.0][peek_idx]);
    //                             if peek_idx >= grid[0].len() || grid[pos.0][peek_idx] == '~' {
    //                                 peek_idx =
    //                                     loop_around(&grid, pos.0, peek_idx, Orientation::Right);
    //                             }
    //                             if grid[pos.0][peek_idx] == '#' {
    //                                 break;
    //                             }
    //                             // println!("at2: ({:?}). Walking {distance} Right. Curr {}", (pos.0, peek_idx), grid[pos.0][peek_idx]);
    //                             end_idx = Some(peek_idx);
    //                             grid[pos.0][peek_idx] = '>';
    //                             peek_idx += 1;
    //                             distance -= 1;
    //                         }
    //                         if let Some(idx) = end_idx {
    //                             pos.1 = idx as usize;
    //                         }
    //                     }
    //                     Orientation::Down => {
    //                         println!(
    //                             "at: ({pos:?}). Walking {distance} Down. Curr {}",
    //                             grid[pos.0][pos.1]
    //                         );
    //                         grid[pos.0][pos.1] = 'v';
    //                         let mut end_idx = None;
    //                         let mut peek_idx = pos.0 + 1;
    //                         while distance > 0 {
    //                             if peek_idx >= grid.len() || grid[peek_idx][pos.1] == '~' {
    //                                 peek_idx =
    //                                     loop_around(&grid, peek_idx, pos.1, Orientation::Down);
    //                             }
    //                             if grid[peek_idx][pos.1] == '#' {
    //                                 break;
    //                             }
    //                             end_idx = Some(peek_idx);
    //                             grid[peek_idx][pos.1] = 'v';
    //                             peek_idx += 1;
    //                             distance -= 1;
    //                         }
    //                         if let Some(idx) = end_idx {
    //                             pos.0 = idx as usize;
    //                         }
    //                     }
    //                     Orientation::Left => {
    //                         println!(
    //                             "at: ({pos:?}). Walking {distance} Left. Curr {}",
    //                             grid[pos.0][pos.1]
    //                         );
    //                         grid[pos.0][pos.1] = '<';
    //                         let mut end_idx = None;
    //                         let mut peek_idx: i32 = pos.1 as i32 - 1;
    //                         while distance > 0 {
    //                             // println!("at: ({} {}). Walking {distance} Left. Curr {}", pos.0, peek_idx, grid[pos.0][peek_idx as usize]);
    //                             if peek_idx < 0 || grid[pos.0][peek_idx as usize] == '~' {
    //                                 peek_idx = loop_around(
    //                                     &grid,
    //                                     pos.0,
    //                                     peek_idx as usize,
    //                                     Orientation::Left,
    //                                 ) as i32;
    //                             }
    //                             if grid[pos.0][peek_idx as usize] == '#' {
    //                                 break;
    //                             }
    //                             end_idx = Some(peek_idx);
    //                             grid[pos.0][peek_idx as usize] = '<';
    //                             peek_idx -= 1;
    //                             distance -= 1;
    //                         }
    //                         // pos = (pos.0, end_idx as usize);
    //                         if let Some(idx) = end_idx {
    //                             pos.1 = idx as usize;
    //                         }
    //                     }
    //                     Orientation::Up => {
    //                         println!(
    //                             "at: ({pos:?}). Walking {distance} Up. Curr {}",
    //                             grid[pos.0][pos.1]
    //                         );
    //                         grid[pos.0][pos.1] = '^';
    //                         let mut end_idx = None;
    //                         let mut peek_idx: i32 = pos.0 as i32 - 1;
    //                         while distance > 0 {
    //                             if peek_idx < 0 || grid[peek_idx as usize][pos.1] == '~' {
    //                                 peek_idx = loop_around(
    //                                     &grid,
    //                                     peek_idx as usize,
    //                                     pos.1,
    //                                     Orientation::Up,
    //                                 ) as i32;
    //                             }
    //                             if grid[peek_idx as usize][pos.1] == '#' {
    //                                 break;
    //                             }
    //                             end_idx = Some(peek_idx);
    //                             grid[peek_idx as usize][pos.1] = '^';
    //                             peek_idx -= 1;
    //                             distance -= 1;
    //                         }
    //                         // pos = (end_idx as usize, pos.1);
    //                         if let Some(idx) = end_idx {
    //                             pos.0 = idx as usize;
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    // print_matrix(&grid);

    println!("ending position: {:?}", pos);
    let facing = match orientation {
        Orientation::Right => 0,
        Orientation::Down => 1,
        Orientation::Left => 2,
        Orientation::Up => 3,
    };
    (1000 * (pos.0 + 1)) + (4 * (pos.1 + 1)) + facing
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        let input = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";
        let result = part1(input);
        assert_eq!(result, 6032);
    }
    #[test]
    fn test_part1_2() {
        let input = include_str!("../input.txt");
        let result = part1(input);
        assert_eq!(result, 103224);
    }

    #[test]
    fn test_part2_1() {
        let input = "        ...#
            .#..
            #...
            ....
    ...#.......#
    ........#...
    ..#....#....
    ..........#.
            ...#....
            .....#..
            .#......
            ......#.

    10R5L5R10L4R5L5";
        let result = part2(input);
        assert_eq!(result, 5031);
    }
}
