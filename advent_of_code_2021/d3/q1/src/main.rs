fn main() {
    let val: Vec<_> = include_str!("../test.in")
        .lines()
        .map(String::from)
        .collect();

    let x = adds_val(val);

    let max_len = val.iter().map(|v| v.len()).max().unwrap();
    let majority: u32 = (val.len() / 2).try_into().unwrap();

    println!("max_len {}", max_len);
    let mut counts = vec![0; max_len as usize];

    let mut gamma = 0;
    let mut epsilon = 0;
    for (idx, count) in counts.iter_mut().enumerate().take(max_len) {
        for v in val.iter() {
            let bool_val = v.chars().nth(idx).unwrap().to_digit(2).unwrap();
            *count += bool_val;
        }
        if *count > majority {
            gamma += 1 << (max_len - idx - 1);
        } else {
            epsilon += 1 << (max_len - idx - 1);
        }
    }
    println!("{:?}", gamma);
    println!("{:?}", epsilon);
    println!("{:?}", gamma * epsilon);
}

fn adds_val(val: Vec<_>) -> i32 {
    return 0;
}
