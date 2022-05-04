use std::cmp::Ordering;

#[derive(Debug)]
struct Target {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

enum ShotState {
    Traveling,
    Hit,
    Overshot,
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

    fn hit(&self, x: i32, y: i32) -> ShotState {
        if x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y {
            return ShotState::Hit;
        } else if x > self.max_x || y < self.min_y {
            return ShotState::Overshot;
        }

        ShotState::Traveling
    }
}

struct Point {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Point {
    fn new(x_velocity: i32, y_velocity: i32) -> Point {
        let velocity = (x_velocity, y_velocity);
        let position = (0, 0);

        Point { position, velocity }
    }

    fn step(&mut self) -> (i32, i32) {
        let next_x = self.position.0 + self.velocity.0;
        let next_y = self.position.1 + self.velocity.1;
        match self.velocity.0.cmp(&0) {
            Ordering::Greater => self.velocity.0 -= 1,
            Ordering::Less => self.velocity.0 += 1,
            Ordering::Equal => (),
        }
        self.velocity.1 -= 1;

        self.position = (next_x, next_y);
        self.position
    }

    fn hits_target(&mut self, target: &Target) -> bool {
        let (x, y) = self.position;
        match target.hit(x, y) {
            ShotState::Traveling => (),
            ShotState::Hit => return true,
            ShotState::Overshot => return false,
        }

        loop {
            let (x, y) = self.step();
            match target.hit(x, y) {
                ShotState::Traveling => continue,
                ShotState::Hit => return true,
                ShotState::Overshot => return false,
            }
        }
    }
}

fn main() {
    let target = Target::new();
    let possible_xs = 1..=target.max_x;
    let mut count = 0;
    for vel_x in possible_xs {
        let possible_ys = target.min_y..-target.min_y;
        for vel_y in possible_ys {
            let mut point = Point::new(vel_x, vel_y);
            let point_hits = point.hits_target(&target);
            count += point_hits as i32;
        }
    }
    println!("{:?}", count)
}
