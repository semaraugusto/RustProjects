struct Snafu {
    value: i64,
    str: String,
}

impl Snafu {
    fn new(str: &str) -> Self {
        let mut result = 0;
        for (i, chr) in str.chars().rev().enumerate() {
            result += 5i64.pow(i as u32)
                * match chr {
                    '1' => 1i64,
                    '2' => 2i64,
                    '0' => 0i64,
                    '-' => -1i64,
                    '=' => -2i64,
                    _ => panic!("Invalid character"),
                }
        }
        Self {
            value: result,
            str: str.to_string(),
        }
    }
    fn from(value: i64) -> Self {
        let mut remainder = value;
        let mut out = String::default();
        while remainder > 0 {
            let glyph = match remainder % 5 {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => '=',
                4 => '-',
                _ => unreachable!(),
            };
            out.push(glyph);
            remainder += 2;
            remainder /= 5;
        }
        Self {
            value,
            str: out.chars().rev().collect::<String>(),
        }
    }
}

pub fn part1(input: &str) -> String {
    let sum = input
        .lines()
        .fold(0, |acc, str| Snafu::new(str).value + acc);
    println!("part1 {}", sum);

    Snafu::from(sum).str
    // result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snafu_to_dec() {
        let expected = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 15, 20, 2022, 12345, 314159265, 1747, 906, 198, 11, 201,
            31, 1257, 32, 353, 107, 7, 3, 37,
        ];
        let snafu = [
            "1",
            "2",
            "1=",
            "1-",
            "10",
            "11",
            "12",
            "2=",
            "2-",
            "20",
            "1=0",
            "1-0",
            "1=11-2",
            "1-0---0",
            "1121-1110-1=0",
            "1=-0-2",
            "12111",
            "2=0=",
            "21",
            "2=01",
            "111",
            "20012",
            "112",
            "1=-1=",
            "1-12",
            "12",
            "1=",
            "122",
        ];

        for (expct, snf) in expected.iter().zip(snafu.iter()) {
            assert_eq!(Snafu::new(snf).value, *expct);
        }
    }
    #[test]
    fn test_dec_to_snafu() {
        let snafu = [
            12345, 2022, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 15, 20, 2022, 12345, 314159265, 1747, 906,
            198, 11, 201, 31, 1257, 32, 353, 107, 7, 3, 37,
        ];
        let expected = [
            "1-0---0",
            "1=11-2",
            "1",
            "2",
            "1=",
            "1-",
            "10",
            "11",
            "12",
            "2=",
            "2-",
            "20",
            "1=0",
            "1-0",
            "1=11-2",
            "1-0---0",
            "1121-1110-1=0",
            "1=-0-2",
            "12111",
            "2=0=",
            "21",
            "2=01",
            "111",
            "20012",
            "112",
            "1=-1=",
            "1-12",
            "12",
            "1=",
            "122",
        ];

        for (expct, snf) in expected.iter().zip(snafu.iter()) {
            assert_eq!(Snafu::from(*snf).str, *expct);
        }
    }

    #[test]
    fn test_part1_1() {
        let input = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

        let result = part1(input);
        assert_eq!(result, "2=-1=0");
    }
    #[test]
    fn test_part1_2() {
        let input = include_str!("../input.txt");
        let result = part1(input);
        assert_eq!(result, "2-=102--02--=1-12=22");
    }
}
