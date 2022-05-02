fn hex_to_bitvec(string: &'static str) -> Vec<bool> {
    string
        .trim()
        .chars()
        .flat_map(|c| {
            let n = c.to_digit(16).unwrap();
            (0..4).rev().map(move |i| (n & (1 << i)) != 0)
        })
        .collect()
}
fn bitvec_to_num(msg_bits: &[bool]) -> usize {
    msg_bits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &b)| (b as usize) << i)
        .sum()
}

fn parse_literal_value(bits: &mut (impl Iterator<Item = bool> + std::fmt::Debug)) -> usize {
    let mut numb_bitvec: Vec<bool> = Vec::new();
    // let mut num_end_idx: usize = 0;
    loop {
        let chunk = bits.take(5).collect::<Vec<bool>>();
        // num_end_idx += 5;
        numb_bitvec.extend(&chunk[1..]);
        // println!("{i}: chunk: {chunk:?}");
        if !chunk[0] {
            break;
        }
    }

    bitvec_to_num(&numb_bitvec)
}

fn parse(bits: &mut std::vec::IntoIter<bool>) -> usize {
    let mut result = bitvec_to_num(&bits.take(3).collect::<Vec<bool>>());
    let type_id = bitvec_to_num(&bits.take(3).collect::<Vec<bool>>());

    if type_id == 4 {
        let value = parse_literal_value(bits);
        return value;
    }

    let length_type_id = bits.next().unwrap();
    let mut num_subpackets = usize::MAX;
    let mut num_bits = usize::MAX;
    if length_type_id {
        num_subpackets = bitvec_to_num(&bits.take(11).collect::<Vec<bool>>());
    } else {
        num_bits = bitvec_to_num(&bits.take(15).collect::<Vec<bool>>());
    }
    println!(
        "packet_type {result} type_id {type_id} curr_msg: {:?} numb_packets {num_subpackets} num_bits: {num_bits}",
        bits.len()
    );
    let bits_remaining = bits.len();
    let mut packets: Vec<usize> = Vec::new();
    while bits_remaining - bits.len() < num_bits && packets.len() < num_subpackets {
        // let value = parse(bits);
        packets.push(parse(bits));
        result += packets.last().unwrap();
    }

    let result = match type_id {
        0 => packets.iter().sum::<usize>(),
        1 => packets.iter().product::<usize>(),
        2 => *packets.iter().min().unwrap(),
        3 => *packets.iter().max().unwrap(),
        5 => (packets[0] > packets[1]) as usize,
        6 => (packets[0] < packets[1]) as usize,
        7 => (packets[0] == packets[1]) as usize,
        _ => unreachable!(),
    };
    println!("packets {:?}", packets);

    result
    // max_iter
}

fn main() {
    let msg_bits = hex_to_bitvec(include_str!("../../test.in"));

    let res = parse(&mut msg_bits.into_iter());
    println!("{:?}", res);
}
