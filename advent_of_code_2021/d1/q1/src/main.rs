fn main() {
    let file = include_str!("../input.in");
    let depths: Vec<i32> = file
        .split_terminator('\n')
        .map(|item| item.replace(" ", "").parse::<i32>().unwrap())
        .collect();

    println!("depths: {:?}", depths);
    let mut prev = &depths[0];
    let mut count = 0;
    for curr in depths.iter().skip(1) {
        if curr > prev {
            count += 1;
        }
        prev = curr;
    }
    println!("count: {}", count);
}
