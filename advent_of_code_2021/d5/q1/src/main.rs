fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[derive(Clone, Debug)]
struct Line {
    start: (i32, i32),
    end: (i32, i32),
    is_vertical: bool,
}

impl Line {
    fn new(line_str: String) -> Option<Line> {
        if line_str.is_empty() {
            return None;
        }
        let _coords: Vec<(i32, i32)> = vec![(0, 0), (1, 1)];
        let coords = line_str
            .replace(" ", "")
            .split("->")
            .map(|coords_str| {
                let out = coords_str
                    .split(',')
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                out
            })
            .collect::<Vec<_>>();

        let start = (coords[0][0], coords[0][1]);
        let end = (coords[1][0], coords[1][1]);
        if start.0 == end.0 {
            return Some(Line {
                start: (coords[0][0], coords[0][1]),
                end: (coords[1][0], coords[1][1]),
                is_vertical: true,
            });
        } else if start.1 == end.1 {
            return Some(Line {
                start: (coords[0][0], coords[0][1]),
                end: (coords[1][0], coords[1][1]),
                is_vertical: false,
            });
        }
        None
    }
}
fn get_board_size(lines: &[Line]) -> (i32, i32) {
    let mut max_x: i32 = 0;
    let mut max_y: i32 = 0;
    for line in lines {
        let x = std::cmp::max(line.start.0, line.end.0);
        if x > max_x {
            max_x = x;
        }
        let y = std::cmp::max(line.start.1, line.end.1);
        if y > max_y {
            max_y = y;
        }
    }
    (max_x + 2, max_y + 2)
}
fn draw_vertical_line(mut board: Vec<Vec<i32>>, line: &Line) -> Vec<Vec<i32>> {
    let min = std::cmp::min(line.start.1, line.end.1);
    let max = std::cmp::max(line.start.1, line.end.1);
    let x = line.start.0;
    // println!("Drawing vertical line");
    // println!("line: {:?}", line);
    // print_board(&board);
    // println!("max_val: {} min_val: {}", min, max);
    assert!(x == line.end.0);
    for idx in min..(max + 1) {
        board[x as usize][idx as usize] += 1;
    }
    board
}

fn draw_horizontal_line(mut board: Vec<Vec<i32>>, line: &Line) -> Vec<Vec<i32>> {
    let min = std::cmp::min(line.start.0, line.end.0);
    let max = std::cmp::max(line.start.0, line.end.0);
    let y = line.start.1;
    // println!("Drawing horizontal line");
    // println!("line: {:?}", line);
    // print_board(&board);
    assert!(y == line.end.1);
    println!("max_val: {} min_val: {}", min, max);
    for idx in min..(max + 1) {
        board[idx as usize][y as usize] += 1;
    }
    board
}

fn draw_line(mut board: Vec<Vec<i32>>, line: &Line) -> Vec<Vec<i32>> {
    if line.is_vertical {
        board = draw_vertical_line(board, line)
    } else {
        board = draw_horizontal_line(board, line)
    }
    board
}
fn print_board(board: &Vec<Vec<i32>>) {
    let transposed = transpose(board.to_vec());
    for row in transposed.iter() {
        println!("{:?}", row)
    }
}
fn main() {
    let lines: Vec<Line> = include_str!("../../test.in")
        .split('\n')
        .map(|line| Line::new(line.to_string()))
        .flatten()
        .collect();

    let (max_x, max_y) = get_board_size(&lines);
    let mut board: Vec<Vec<i32>> = vec![vec![0; max_x as usize]; max_y as usize];
    for line in &lines {
        board = draw_line(board, line);
    }
    println!("min_coords: {:?}", max_x);
    println!("max_coords: {:?}", max_y);
    println!("{:?}", lines);
    print_board(&board);
    let res = board.iter().fold(0, |outer_acc, row| {
        outer_acc
            + row
                .iter()
                .fold(0, |inner_acc, val| inner_acc + (*val >= 2) as i32)
    });
    println!("{:?}", res);
}
