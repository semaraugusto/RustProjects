fn main() {
    let mut fishes: Vec<i32> = include_str!("../../sample.in")
        .split(',')
        .map(|val| val.trim().parse::<i32>().unwrap())
        .collect();

    (0..80).for_each(|_| {
        let new_fishes = fishes.iter().filter(|&x| *x == 0).count();
        fishes
            .iter_mut()
            .for_each(|fish| if *fish == 0 { *fish = 6 } else { *fish -= 1 });

        (0..new_fishes).for_each(|_| fishes.push(8));
    });
    println!("{:?}", fishes.len());
}
