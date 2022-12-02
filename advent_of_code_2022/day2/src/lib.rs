#[derive(Clone, Debug)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl Rps {
    fn new(item: &str) -> Self {
        println!("Input: {}", item);
        match item {
            "A" | "X" => Rps::Rock,
            "B" | "Y" => Rps::Paper,
            "C" | "Z" => Rps::Scissors,
            _ => panic!("Invalid item"),
        }
    }
    fn get_round_winner<'a>(&'a self, other: &'a Rps) -> Winner {
        match (self, other) {
            (Rps::Rock, Rps::Paper) => Winner::Player(Player::Two),
            (Rps::Rock, Rps::Scissors) => Winner::Player(Player::One),
            (Rps::Paper, Rps::Rock) => Winner::Player(Player::One),
            (Rps::Paper, Rps::Scissors) => Winner::Player(Player::Two),
            (Rps::Scissors, Rps::Rock) => Winner::Player(Player::Two),
            (Rps::Scissors, Rps::Paper) => Winner::Player(Player::One),
            _ => Winner::Draw,
        }
    }
    fn win_against(expected_move: Rps) -> Rps {
        match expected_move {
            Rps::Rock => Rps::Paper,
            Rps::Scissors => Rps::Rock,
            Rps::Paper => Rps::Scissors,
        }
    }
    fn draw_against(expected_move: Rps) -> Rps {
        match expected_move {
            Rps::Rock => Rps::Rock,
            Rps::Scissors => Rps::Scissors,
            Rps::Paper => Rps::Paper,
        }
    }
    fn lose_against(expected_move: Rps) -> Rps {
        match expected_move {
            Rps::Rock => Rps::Scissors,
            Rps::Scissors => Rps::Paper,
            Rps::Paper => Rps::Rock,
        }
    }

    fn get_move<'a>(expected_move: Rps, action: &'a Action) -> Self {
        match action {
            Action::Win => Rps::win_against(expected_move),
            Action::Lose => Rps::lose_against(expected_move),
            Action::Draw => Rps::draw_against(expected_move),
        }
    }
    fn to_score(&self) -> u32 {
        match self {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        }
    }
}

#[derive(Clone, Debug)]
enum Action {
    Win,
    Lose,
    Draw,
}

impl Action {
    fn new(item: &str) -> Self {
        match item {
            "X" => Action::Lose,
            "Y" => Action::Draw,
            "Z" => Action::Win,
            _ => panic!("Invalid item"),
        }
    }
}

#[derive(Debug)]
enum Player {
    One,
    Two,
}

#[derive(Debug)]
enum Winner {
    Player(Player),
    Draw,
}

#[derive(Debug)]
struct Round {
    player1: Rps,
    player2: Rps,
    winner: Winner,
}
impl Round {
    fn new_part1(single_round_input: &str) -> Self {
        let mut iter = single_round_input.split_whitespace();
        let player1 = Rps::new(iter.next().unwrap());
        let player2 = Rps::new(iter.next().unwrap());
        let winner = player1.get_round_winner(&player2);
        Self {
            player1,
            player2,
            winner,
        }
    }
    fn new_part2(single_round_input: &str) -> Self {
        let mut iter = single_round_input.split_whitespace();
        let player1 = Rps::new(iter.next().unwrap());
        let action = Action::new(iter.next().unwrap());
        println!("Action: {:?}", action);
        let player2 = Rps::get_move(player1.clone(), &action);
        let winner = match action {
            Action::Win => Winner::Player(Player::Two),
            Action::Draw => Winner::Draw,
            Action::Lose => Winner::Player(Player::One),
        };
        Self {
            player1,
            player2,
            winner,
        }
    }

    fn get_result(&self) -> (u32, u32) {
        match &self.winner {
            Winner::Draw => {
                let mut player1_score = 3;
                let mut player2_score = 3;
                player1_score += self.player1.to_score();
                player2_score += self.player2.to_score();
                (player1_score, player2_score)
            }
            Winner::Player(player) => {
                let mut player1_score = 0;
                let mut player2_score = 0;
                match player {
                    Player::One => player1_score += 6,
                    Player::Two => player2_score += 6,
                }
                player1_score += self.player1.to_score();
                player2_score += self.player2.to_score();
                (player1_score, player2_score)
            }
        }
    }
    fn get_result_part2(&self) -> (u32, u32) {
        match &self.winner {
            Winner::Draw => {
                let mut player1_score = 3;
                let mut player2_score = 3;
                player1_score += self.player1.to_score();
                player2_score += self.player2.to_score();
                (player1_score, player2_score)
            }
            Winner::Player(player) => {
                let mut player1_score = 0;
                let mut player2_score = 0;
                match player {
                    Player::One => player1_score += 6,
                    Player::Two => player2_score += 6,
                }
                player1_score += self.player1.to_score();
                player2_score += self.player2.to_score();
                (player1_score, player2_score)
            }
        }
    }
}

fn solve_part1(input: &str) -> u32 {
    let mut i = 0;
    let mut sum = 0;
    input.split_terminator('\n').for_each(|line| {
        let round = Round::new_part1(line);
        let p2_score = round.get_result().1;
        sum += p2_score;
        println!("score {:?}", p2_score);
        i += 1;
    });
    sum
}

fn solve_part2(input: &str) -> u32 {
    let mut i = 0;
    let mut sum = 0;
    input.split_terminator('\n').for_each(|line| {
        let round = Round::new_part2(line);
        println!("round {:?}", round);
        let p2_score = round.get_result().1;
        sum += p2_score;
        println!("score {:?}", p2_score);
        i += 1;
    });
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_1() {
        let input = "A Y
B X
C Z";
        let actual = solve_part1(input);
        assert_eq!(actual, 15);
    }
    #[test]
    fn test_part1_2() {
        let input = include_str!("../input.txt");
        let actual = solve_part1(input);
        assert_eq!(actual, 13675);
    }
    #[test]
    fn test_part2_1() {
        let input = "A Y
    B X
    C Z";
        let actual = solve_part2(input);
        assert_eq!(actual, 12);
    }
    #[test]
    fn test_part2_2() {
        let input = include_str!("../input.txt");
        let actual = solve_part2(input);
        assert_eq!(actual, 14184);
    }
}
