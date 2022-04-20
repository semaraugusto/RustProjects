// struct Command {
//     direction: String,
//     steps: u64,
// }
//
// impl Command {
//     fn from_str(s: &str) {
//         let iter = s.split_whitespace();
//         let dir = iter.next();
//         let steps = iter.next();
//     }
// }
fn main() {
    let file = include_str!("../test.in");
    let (x, y, _) = file.lines().map(|l| l.split_once(" ").unwrap()).fold(
        (0, 0, 0),
        |(x_acc, y_acc, aim), (command, step)| match (command, step.parse::<i64>().unwrap()) {
            ("forward", step) => (x_acc + step, y_acc + aim * step, aim),
            ("down", step) => (x_acc, y_acc, aim + step),
            ("up", step) => (x_acc, y_acc, aim - step),
            _ => unreachable!(),
        },
    );
    println!("val {:?}", x * y);
}
