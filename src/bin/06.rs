advent_of_code::solution!(6);

use nom::{
    bytes::complete::tag,
    character::complete::{self, digit1, line_ending, space1},
    multi::separated_list0,
    sequence::preceded,
    IResult,
};

pub fn part_one(input: &str) -> Option<u32> {
    let (_, (time, distance)) = parse(input).expect("could not parse input");

    let ans = time
        .iter()
        .zip(distance.iter())
        .map(|(&t, &d)| ((0..t).map(|h| (t - h) * h).filter(|&dist| dist > d).count() as u32))
        .product();

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, (time, distance)) = parse_two(input).expect("could not parse input");

    let ans = (0..time)
        .map(|h| (time - h) * h)
        .filter(|&dist| dist > distance)
        .count() as u32;

    Some(ans)
}

fn parse(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let time = preceded(tag("Time:"), space1);
    let (remaining, time) = preceded(time, separated_list0(space1, complete::u32))(input)?;

    let distance = preceded(line_ending, preceded(tag("Distance:"), space1));
    let (remaining, distance) =
        preceded(distance, separated_list0(space1, complete::u32))(remaining)?;

    Ok((remaining, (time, distance)))
}

fn parse_two(input: &str) -> IResult<&str, (u64, u64)> {
    let time = preceded(tag("Time:"), space1);
    let (remaining, time) = preceded(time, separated_list0(space1, digit1))(input)?;

    let distance = preceded(line_ending, preceded(tag("Distance:"), space1));
    let (remaining, distance) = preceded(distance, separated_list0(space1, digit1))(remaining)?;

    let time = time.join("").parse().expect("Invalid time number");
    let distance = distance.join("").parse().expect("Invalid time number");

    Ok((remaining, (time, distance)))
}

#[cfg(test)]
mod tests {
    use super::*;
    advent_of_code::part!();

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_example(DAY, PART_ONE));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_example(DAY, PART_TWO));
        assert_eq!(result, Some(71503));
    }
}
