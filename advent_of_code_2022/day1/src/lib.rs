fn solve_first(input: &str) -> i64 {
    println!("Hello, world!");
    let result = input
        .split_terminator("\n\n")
        .fold(-1, |mut max_cal, elf_calories| {
            let total_calories = elf_calories
                .split_terminator('\n')
                .fold(0, |acc, calorie| acc + calorie.parse::<i64>().unwrap());
            if max_cal < total_calories {
                max_cal = total_calories;
            }
            max_cal
        });
    println!("{:?}", result);

    result
}
fn solve_second(input: &str) -> i64 {
    println!("Hello, world!");
    let mut result: Vec<i64> = input
        .split_terminator("\n\n")
        .map(|elf_calories| {
            elf_calories
                .split_terminator('\n')
                .fold(0, |acc, calorie| acc + calorie.parse::<i64>().unwrap())
        })
        .collect::<Vec<i64>>();

    result.sort_unstable();
    result.reverse();

    println!("{:?}", result);

    result.into_iter().take(3).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let actual = solve_first(input);
        let expected = 24000;
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_first2() {
        let input = include_str!("../challenge.txt");
        let actual = solve_first(input);
        let expected = 69693;
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_second() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let actual = solve_second(input);
        let expected = [24000, 11000, 10000].iter().sum();
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_second2() {
        let input = include_str!("../challenge.txt");
        let actual = solve_second(input);
        let expected = [69693, 66757, 64495].iter().sum();
        assert_eq!(actual, expected);
    }
}
