fn main() {
    let mut fishes: [usize; 9] =
        include_str!("../../test.in")
            .split(',')
            .fold([0; 9], |mut fish_counter, val| {
                fish_counter[val.trim().parse::<usize>().unwrap()] += 1;
                fish_counter
            });

    println!("{:?}", fishes);

    (0..256).for_each(|_| {
        let new_fishes = fishes[0];
        for idx in 1..fishes.len() {
            fishes[idx - 1] = fishes[idx];
        }
        fishes[6] += new_fishes;
        fishes[8] = new_fishes;
    });
    println!("{:?}", fishes.iter().sum::<usize>());
}
