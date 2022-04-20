fn main() {
    let val: Vec<_> = include_str!("../test.in")
        .lines()
        .map(String::from)
        .collect();

    let max_len = val.iter().map(|v| v.len()).max().unwrap();

    let oxygen = get_oxygen(&val, max_len);
    let co2 = get_co2(&val, max_len);

    println!("{:?}", oxygen);
    println!("{:?}", co2);
    println!("{:?}", oxygen * co2);
}

fn get_oxygen(measures: &[String], max_len: usize) -> i32 {
    get_measure(measures.to_vec(), max_len, |a, b| a >= b)
}
fn get_co2(measures: &[String], max_len: usize) -> i32 {
    get_measure(measures.to_vec(), max_len, |a, b| a < b)
}

fn get_measure(measures: Vec<String>, max_len: usize, fn_comp: fn(u32, u32) -> bool) -> i32 {
    let mut counts = vec![0; max_len];
    let mut measurement = 0;
    let mut measures = measures;
    let mut majority: u32 = match measures.len() % 2 == 0 {
        true => (measures.len() / 2) as u32 + 1,
        false => (measures.len() / 2) as u32,
    };
    for (idx, count) in counts.iter_mut().enumerate().take(max_len) {
        for v in measures.iter() {
            let bool_val = v.chars().nth(idx).unwrap().to_digit(2).unwrap();
            *count += bool_val;
        }
        measures = measures
            .iter()
            .filter(|str_val| {
                str_val.chars().nth(idx).unwrap().to_digit(2).unwrap()
                    == (fn_comp(*count, majority) as u32)
            })
            .map(|v| v.to_string())
            .collect();

        if measures.len() == 1 {
            measurement = i32::from_str_radix(&measures[0], 2).unwrap();
        }
        majority = match measures.len() % 2 == 1 {
            true => (measures.len() / 2) as u32 + 1,
            false => (measures.len() / 2) as u32,
        };
    }
    measurement
}
