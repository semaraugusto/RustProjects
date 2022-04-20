fn main() {
    let map: Vec<Vec<u32>> = include_str!("../../test.in")
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u32)
                .collect()
        })
        .collect();

    let directions: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let height = map.len() as i32 - 1;
    let width = map[0].len() as i32 - 1;

    let mut result = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            let is_smallest = directions.iter().fold(true, |is_smallest, dir| {
                let y = i as i32 + dir.0;
                let x = j as i32 + dir.1;
                if y < 0 || y > height || x < 0 || x > width {
                    is_smallest
                } else {
                    is_smallest && *val < map[y as usize][x as usize]
                }
            });
            println!("smallest: {:?}", is_smallest);
            if is_smallest {
                result += 1 + val;
            }
        }
    }
    println!("map: {:?}", map);
    println!("result: {:?}", result);
}
