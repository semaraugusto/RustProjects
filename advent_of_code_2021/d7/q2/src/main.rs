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
                        .map(|val| (0..(crab - val).abs() + 1).sum())
                        .collect::<Vec<i32>>(),
                )
                .map(|(acc, val)| acc + val)
                .collect::<Vec<i32>>()
        });
    // println!("{:?}", result);
    println!("{:?}", result.iter().min().unwrap());
}
