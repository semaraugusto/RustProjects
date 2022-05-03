use std::cmp::Ordering;

#[derive(Debug)]
struct Target {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Target {
    fn new() -> Target {
        let string = include_str!("../../test.in")
            .trim()
            .split_terminator(':')
            .nth(1)
            .unwrap()
            .trim()
            .split_terminator(',')
            .map(|val| val.trim())
            .collect::<Vec<&str>>();

        let x: Vec<i32> = string[0][2..]
            .split_terminator("..")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let y: Vec<i32> = string[1][2..]
            .split_terminator("..")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        Target {
            min_x: *x.iter().min().unwrap(),
            max_x: *x.iter().max().unwrap(),
            min_y: *y.iter().min().unwrap(),
            max_y: *y.iter().max().unwrap(),
        }
    }
}

fn main() {
    let target = Target::new();
    // let possible_xs = 1..=target.max_x;
    let possible_ys = target.min_y..-target.min_y;
    let vel_y = possible_ys.max().unwrap();
    let result = vel_y * (vel_y + 1) / 2;
    println!("{:?}", result)
}
