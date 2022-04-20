fn main() {
    let crabs: Vec<i32> = include_str!("../../test.in")
        .split(',')
        .map(|val| val.trim().parse::<i32>().unwrap())
        .collect();

    let max_val = crabs.iter().max().unwrap();
    // println!("{:?}", crabs);
    // println!("{}", max_val);

    let result = crabs
        .iter()
        .fold(vec![0; *max_val as usize], |counter, crab| {
            counter
                .iter()
                .zip(
                    (0..*max_val)
                        .map(|val| (crab - val).abs())
                        .collect::<Vec<i32>>(),
                )
                .map(|(x, y)| x + y)
                .collect::<Vec<i32>>()
        });
    println!("{:?}", result.iter().min().unwrap());
}
