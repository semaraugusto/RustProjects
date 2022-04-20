fn main() {
    let output: Vec<Vec<String>> = include_str!("../../test.in")
        .lines()
        .into_iter()
        .map(|line| {
            line.split('|')
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(String::from)
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    let lens = [2, 4, 3, 7]; // 1, 4, 7, 8

    let result = lens.iter().fold(0, |outer_acc, num_signals| {
        outer_acc
            + output.iter().fold(0, |inner_acc, output_list| {
                inner_acc
                    + output_list
                        .iter()
                        .filter(|signal_str| signal_str.len() == *num_signals)
                        .count()
            })
    });
    println!("{:?}", result);
}
