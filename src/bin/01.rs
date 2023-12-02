advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split('\n').collect::<Vec<&str>>();
    let mut ans = 0;

    for l in lines.iter().take(lines.len() - 1) {
        let first_digit = l.chars().find(|c| c.is_ascii_digit()).unwrap();
        let last_digit = l.chars().rev().find(|c| c.is_ascii_digit()).unwrap();

        ans += first_digit.to_digit(10).unwrap() * 10 + last_digit.to_digit(10).unwrap();
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.split('\n').collect::<Vec<&str>>();
    let mut ans = 0;

    for l in lines.iter().take(lines.len() - 1) {
        ans += parser::parse(l);
    }

    Some(ans)
}

mod parser {
    pub fn parse(input: &str) -> u32 {
        let bytes = input.as_bytes();
        let nums = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        let mut first = u32::MAX;
        let mut last = u32::MAX;

        for i in 0..bytes.len() {
            let digit = if bytes[i].is_ascii_digit() {
                (bytes[i] as char).to_digit(10)
            } else {
                nums.iter()
                    .position(|&s| bytes[i..].starts_with(s.as_bytes()))
                    .map(|i| i as u32 + 1)
            };

            if let Some(digit) = digit {
                if first == u32::MAX {
                    first = digit;
                }

                last = digit;
            }
        }

        first * 10 + last
    }
}

#[cfg(not(feature = "test_lib"))]
#[cfg(test)]
mod tests {
    use super::*;
    advent_of_code::part!();

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_example(DAY, PART_ONE));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_example(DAY, PART_TWO));
        assert_eq!(result, Some(281));
    }

    macro_rules! test_parser {
        ($(($lit:literal, $expected:expr)),*) => {
            $(
                assert_eq!(parser::parse($lit), $expected);
            )*
        };
    }

    #[test]
    fn test_parser() {
        test_parser!(
            ("onetwo3aanvabvjadbv9", 19),
            ("abvhvatwo1", 21),
            ("1", 11),
            ("1onetwo", 12),
            ("three12", 32),
            ("21nine", 29)
        );
    }
}
