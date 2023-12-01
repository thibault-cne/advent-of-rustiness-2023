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
        ans += Parser::parse(l);
    }

    Some(ans)
}

struct Parser;

impl Parser {
    fn parse(input: &str) -> u32 {
        let mut iter = input.chars().peekable();
        let mut first = u32::MAX;
        let mut last = u32::MAX;

        while let Some(c) = iter.next() {
            // If c is a digit
            let digit = if c.is_ascii_digit() {
                c.to_digit(10)
            } else {
                match c {
                    'o' if try_parse(&mut iter, "one") => Some(1),
                    't' if try_parse(&mut iter, "two") => Some(2),
                    't' if try_parse(&mut iter, "three") => Some(3),
                    'f' if try_parse(&mut iter, "four") => Some(4),
                    'f' if try_parse(&mut iter, "five") => Some(5),
                    's' if try_parse(&mut iter, "six") => Some(6),
                    's' if try_parse(&mut iter, "seven") => Some(7),
                    'e' if try_parse(&mut iter, "eight") => Some(8),
                    'n' if try_parse(&mut iter, "nine") => Some(9),
                    _ => None,
                }
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

fn try_parse<I: Iterator<Item = char>>(iter: &mut std::iter::Peekable<I>, pattern: &str) -> bool {
    let mut index = 1;
    let chars = pattern.as_bytes();

    while let Some(peek) = iter.peek() {
        if index == pattern.len() || *peek != chars[index] as char {
            break;
        }

        iter.next();
        index += 1;
    }

    index == pattern.len()
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
                assert_eq!(Parser::parse($lit), $expected);
            )*
        };
    }

    #[test]
    fn test_parser() {
        test_parser!(
            ("onetwo3aanvabvjadbv9", 19),
            ("abvhvatwo1", 21),
            ("1", 11),
            ("1onetwo", 12)
        );
    }

    macro_rules! try_parse {
        ($(($lit:literal, $prefix:literal, $expected:expr, $end:literal)),*) => {
            $(
                let mut iter = $lit.chars().peekable();
                iter.next();
                assert_eq!(try_parse(&mut iter, $prefix), $expected);
                assert_eq!(&iter.collect::<String>(), $end);
            )*
        };
    }

    #[test]
    fn test_try_parse() {
        try_parse!(
            ("onetwo1", "one", true, "two1"),
            ("otwo1two", "one", false, "two1two"),
            ("three", "three", true, "")
        );
    }
}
