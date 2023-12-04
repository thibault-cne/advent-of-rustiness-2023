advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let n = lines.len();

    let sum = lines
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.char_indices()
                .filter(|&(_, c)| !c.is_ascii_digit() && c != '.')
                .map(move |(x, _)| (x, y))
        })
        .map(|(x, y)| {
            let numbers: Vec<_> =
                ((if y == 0 { y } else { y - 1 })..=(if y == n - 1 { y } else { y + 1 }))
                    .flat_map(|y| {
                        let line = lines[y];

                        let start = line[..=(if x == 0 { x } else { x - 1 })]
                            .chars()
                            .rev()
                            .take_while(|c| c.is_ascii_digit())
                            .count();
                        let end = line[(if x == line.len() - 1 { x } else { x + 1 })..]
                            .chars()
                            .take_while(|c| c.is_ascii_digit())
                            .count();

                        line[(x - start)..=(x + end)]
                            .split(|c: char| !c.is_ascii_digit())
                            .filter_map(|n| n.parse::<u32>().ok())
                    })
                    .collect();

            numbers.iter().sum::<u32>()
        })
        .sum::<u32>();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let n = lines.len();

    let sum = lines
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.char_indices()
                .filter(|&(_, c)| (c == '*'))
                .map(move |(x, _)| (x, y))
        })
        .filter_map(|(x, y)| {
            let numbers: Vec<_> =
                ((if y == 0 { y } else { y - 1 })..=(if y == n - 1 { y } else { y + 1 }))
                    .flat_map(|y| {
                        let line = lines[y];

                        let start = line[..=(if x == 0 { x } else { x - 1 })]
                            .chars()
                            .rev()
                            .take_while(|c| c.is_ascii_digit())
                            .count();
                        let end = line[(if x == line.len() - 1 { x } else { x + 1 })..]
                            .chars()
                            .take_while(|c| c.is_ascii_digit())
                            .count();

                        line[(x - start)..=(x + end)]
                            .split(|c: char| !c.is_ascii_digit())
                            .filter_map(|n| n.parse::<u32>().ok())
                    })
                    .collect();

            (numbers.len() == 2).then(|| numbers.iter().product::<u32>())
        })
        .sum::<u32>();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    advent_of_code::part!();

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_example(DAY, PART_ONE));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_example(DAY, PART_TWO));
        assert_eq!(result, Some(467835));
    }
}
