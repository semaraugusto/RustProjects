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
    let file = include_str!("../sample.in");
    let (x, y) = file.lines().map(|l| l.split_once(" ").unwrap()).fold(
        (0, 0),
        |(x_acc, y_acc), (command, step)| match (command, step.parse::<i64>().unwrap()) {
            ("forward", step) => (x_acc + step, y_acc),
            ("down", step) => (x_acc, y_acc + step),
            ("up", step) => (x_acc, y_acc - step),
            _ => unreachable!(),
        },
    );
    println!("val {:?}", x * y);
}
