#[derive(Debug)]
struct Point {
    y: usize,
    x: usize,
}

const OCCUPIED: char = '█';
const EMPTY: char = ' ';

impl Point {
    fn new(line: &str) -> Point {
        let (x, y): (&str, &str) = line.split_once(',').unwrap();
        let y = y.parse::<usize>().unwrap();
        let x = x.parse::<usize>().unwrap();
        Point { y, x }
    }
}

#[derive(Debug)]
struct Instruction {
    orientation: &'static str,
    position: usize,
}

impl Instruction {
    fn new(line: &'static str) -> Instruction {
        let instr = line
            .split_whitespace()
            .nth(2)
            .unwrap()
            .split_terminator('=')
            .collect::<Vec<&'static str>>();

        Instruction {
            orientation: instr[0],
            position: instr[1].parse::<usize>().unwrap(),
        }
    }
}

fn parse_input() -> (Vec<Point>, Vec<Instruction>) {
    let (str_points, instructions) = include_str!("../../test.in").split_once("\n\n").unwrap();

    let mut points: Vec<Point> = Vec::new();

    for str_point in str_points.lines() {
        points.push(Point::new(str_point));
    }
    let instructions: Vec<Instruction> = instructions.lines().map(Instruction::new).collect();

    (points, instructions)
}

fn run_instruction(paper: &[Vec<char>], instr: &Instruction) -> Vec<Vec<char>> {
    let mut output: Vec<Vec<char>>;
    if instr.orientation == "y" {
        // UP
        output = paper[..instr.position][..].to_vec();

        for (idx, out_row) in output.iter_mut().enumerate() {
            let offset = (instr.position as i64 * 2 - idx as i64).abs() as usize;
            if offset >= paper.len() {
                continue;
            }
            for (jdx, out_val) in out_row.iter_mut().enumerate() {
                let paper_val = paper[offset][jdx];
                if paper_val == OCCUPIED {
                    *out_val = OCCUPIED;
                }
            }
        }
    } else {
        // LEFT
        output = paper
            .iter()
            .map(|row| row[..instr.position].to_vec())
            .collect::<Vec<Vec<char>>>();

        for (idx, out_row) in output.iter_mut().enumerate() {
            for (jdx, out_val) in out_row.iter_mut().enumerate() {
                let mut offset = instr.position as i64 * 2 - jdx as i64;
                while offset as usize > paper[0].len() {
                    offset -= 1;
                }
                let paper_val = paper[idx][offset as usize];
                if paper_val == OCCUPIED {
                    *out_val = OCCUPIED;
                }
            }
        }
    }
    output
}

fn main() {
    let (points, instructions) = parse_input();
    let max_x = points.iter().map(|point| point.x).max().unwrap() + 1;
    let max_y = points.iter().map(|point| point.y).max().unwrap() + 1;

    let mut paper = vec![vec![EMPTY; max_x]; max_y];

    for point in &points {
        paper[point.y][point.x] = OCCUPIED;
    }

    for instr in instructions {
        paper = run_instruction(&paper, &instr);
    }
    print_matrix(&paper, None);
}

fn print_matrix(mat: &Vec<Vec<char>>, instr: Option<&Instruction>) {
    for m in mat.iter() {
        // println!("{} {}", i, m);
        m.iter().for_each(|x| print!("{}", x));
        println!();
    }
}
