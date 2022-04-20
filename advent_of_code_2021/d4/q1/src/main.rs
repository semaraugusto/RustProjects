use std::cmp;

#[derive(Debug)]
struct Board {
    numbers: Vec<Vec<i32>>,
    hits: Vec<Vec<bool>>,
}

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
fn check_rows(hits: &Vec<Vec<bool>>) -> bool {
    let rows_max = hits.iter().fold(0, |max_val, line_vec| {
        cmp::max(
            max_val,
            line_vec
                .iter()
                .fold(0, |acc, hit_bool| acc + *hit_bool as i32),
        )
    });
    // println!("rows_max: {:?}", rows_max);
    rows_max == 5
}

impl Board {
    fn new(board: String) -> Board {
        let hits = vec![vec![false; 5]; 5];
        let mut numbers: Vec<Vec<_>> = board
            .split('\n')
            .map(|line| {
                line.split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect()
            })
            .collect();

        if numbers[numbers.len() - 1].is_empty() {
            numbers.pop();
        }
        println!("numbers {:?}", numbers);
        Board { numbers, hits }
    }
    fn bingo(&self) -> bool {
        let bingoed_lines: bool = check_rows(&(self.hits));
        if bingoed_lines {
            return bingoed_lines;
        }
        // println!("hits: {:?}", self.hits);
        let transposed = transpose((self.hits).clone());
        // println!("transposed: {:?}", transposed);

        let bingoed_columns: bool = check_rows(&transposed);
        bingoed_columns
    }
    fn get_score(&self, last_draw: &i32) -> i32 {
        let out = self
            .numbers
            .iter()
            .enumerate()
            .fold(0, |outer_acc, (idx, row)| {
                outer_acc
                    + row
                        .iter()
                        .enumerate()
                        .filter(|(j, _)| !self.hits[idx][*j])
                        .fold(0, |inner_acc, (_, val)| {
                            println!("{}", val);
                            inner_acc + val
                        })
            });
        println!("last draw: {} mul: {}", last_draw, out);
        last_draw * out
    }
    fn add_guess(&mut self, number: &i32) {
        for (hit_row, numb_row) in self.hits.iter_mut().zip(self.numbers.iter_mut()) {
            for (hit, numb) in hit_row.iter_mut().zip(numb_row.iter()) {
                if numb == number {
                    *hit = true;
                }
            }
        }
    }
}
fn main() {
    let (numbers, str_boards) = include_str!("../test.in").split_once("\n\n").unwrap();
    let numbers: Vec<i32> = numbers.split(',').map(|x| x.parse().unwrap()).collect();

    // let mut boards: Vec<Board> = Vec::new();
    let mut boards: Vec<Board> = str_boards
        .split("\n\n")
        .map(|str_board| Board::new(str_board.to_string()))
        .collect();
    // println!("boards {:?}", boards);
    let mut end = false;
    for numb in &numbers {
        if end {
            break;
        }

        println!("Drawn: {}", numb);
        for board in boards.iter_mut() {
            board.add_guess(numb);
            if board.bingo() {
                println!("you WON: score {}", board.get_score(numb));
                end = true;
                break;
            }
        }
    }
}
