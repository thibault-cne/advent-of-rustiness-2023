advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split('\n').collect::<Vec<&str>>();
    let mut ans = 0;

    for l in lines.iter().take(lines.len() - 1) {
        let first_digit = l.chars().find(|c| c.is_ascii_digit()).unwrap();
        let last_digit = l
            .chars()
            .rev()
            .find(|c| c.is_ascii_digit())
            .unwrap();

        ans += first_digit.to_digit(10).unwrap() * 10 + last_digit.to_digit(10).unwrap();
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.split('\n').collect::<Vec<&str>>();
    let mut ans = 0;
    let numbers = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    for l in lines.iter().take(lines.len() - 1) {
        let mut first_digit = None;
        let mut last_digit = None;

        for (i, c) in l.char_indices() {
            if first_digit.is_none() {
                first_digit = if c.is_ascii_digit() {
                    Some(c.to_digit(10).unwrap())
                } else {
                    numbers
                        .iter()
                        .find(|(s, _)| l[i..].starts_with(s))
                        .map(|(_, n)| *n)
                };
            }

            if last_digit.is_none() {
                let index = l.len() - i - 1;
                let c = l.as_bytes()[index] as char;
                last_digit = if c.is_ascii_digit() {
                    Some(c.to_digit(10).unwrap())
                } else {
                    numbers
                        .iter()
                        .find(|(s, _)| l[..=index].ends_with(s))
                        .map(|(_, n)| *n)
                };
            }

            if first_digit.is_some() && last_digit.is_some() {
                break;
            }
        }

        ans += first_digit.unwrap() * 10 + last_digit.unwrap();
    }

    Some(ans)
}

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
}
