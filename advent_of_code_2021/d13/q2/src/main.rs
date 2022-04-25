#[derive(Debug)]
struct Point {
    y: usize,
    x: usize,
}

const OCCUPIED: char = '#';
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
    // println!("INSTRUCTION {:?}", instr);
    let mut output: Vec<Vec<char>>;
    if instr.orientation == "y" {
        // UP
        output = paper[..instr.position][..].to_vec();

        // println!("start output");
        for (out_row, paper_row) in output
            .iter_mut()
            .zip(paper[instr.position + 1..].iter().rev())
        {
            for (out_val, paper_val) in out_row.iter_mut().zip(paper_row.iter()) {
                if *paper_val == OCCUPIED {
                    *out_val = OCCUPIED;
                }
                // todo!();
            }
        }
    } else {
        // LEFT
        output = paper
            .iter()
            .map(|row| row[..instr.position].to_vec())
            .collect::<Vec<Vec<char>>>();

        for (out_row, paper_row) in output.iter_mut().zip(paper.iter()) {
            for (out_val, paper_val) in out_row.iter_mut().zip(paper_row.iter().rev()) {
                if *paper_val == OCCUPIED {
                    *out_val = OCCUPIED;
                }
                // todo!();
            }
        }
        // println!("end output");
    }
    output
}

fn main() {
    let (points, instructions) = parse_input();
    let max_x = points.iter().map(|point| point.x).max().unwrap() + 1;
    let max_y = points.iter().map(|point| point.y).max().unwrap() + 1;
    let bounds = Point { x: max_x, y: max_y };

    let mut paper = vec![vec![EMPTY; max_x]; max_y];

    for point in &points {
        paper[point.y][point.x] = OCCUPIED;
    }
    let result = paper.iter().fold(0, |outer_acc, row| {
        outer_acc
            + row
                .iter()
                .fold(0, |inner_acc, val| inner_acc + (*val == OCCUPIED) as u32)
    });
    println!("init value {}", result);

    println!("START (y, x) ({}, {})", paper.len(), paper[0].len());
    // print_matrix(&paper, None);

    for instr in instructions {
        paper = run_instruction(&paper, &instr);
        // print_matrix(&paper, Some(&instr));
        // break;
        // println!();
    }
    // println!("END (y, x) ({}, {})", paper.len(), paper[0].len());
    println!("END (y, x) ({}, {})", paper.len(), paper[0].len());
    let result = paper.iter().fold(0, |outer_acc, row| {
        outer_acc
            + row
                .iter()
                .fold(0, |inner_acc, val| inner_acc + (*val == OCCUPIED) as u32)
    });
    println!("RESULT {}", result);
    print_matrix(&paper, None);
    // println!("END (y, x) ({}, {})", paper.len(), paper[0].len());
    // println!("(max_y, max_x) ({:?}, {:?})", max_y, max_x);
    // println!("points {:?}", points);
    // println!("instructions {:?}", instructions);
}

fn print_matrix(mat: &Vec<Vec<char>>, instr: Option<&Instruction>) {
    println!("instruction {:?}", instr);
    for (i, m) in mat.iter().enumerate() {
        // println!("{} {}", i, m);
        m.iter().for_each(|x| print!("{} ", x));
        println!();
    }
}
