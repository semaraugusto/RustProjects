fn main() {
    let file = include_str!("../sample.in");
    let depths: Vec<i32> = file
        .split_terminator('\n')
        .map(|item| item.replace(" ", "").parse::<i32>().unwrap())
        .collect();

    println!("depths: {:?}", depths);
    let mut prev1 = &depths[1];
    let mut prev2 = &depths[0];
    let mut count = 0;
    let mut prev_sum = 2147483647;
    let mut curr_sum;
    for curr in depths.iter().skip(2) {
        curr_sum = prev1 + prev2 + curr;
        if curr_sum > prev_sum {
            count += 1;
        }
        prev2 = prev1;
        prev1 = curr;
        prev_sum = curr_sum;
    }
    println!("count: {}", count);
}
