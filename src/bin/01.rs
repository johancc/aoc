use std::collections::HashMap;

advent_of_code::solution!(1);

static  SEPARATOR: &str = "   ";

fn parse_input(input: &str) -> (Vec<u128>, Vec<u128>) {
    let mut left_side = Vec::new();
    let mut right_side = Vec::new();

    input.lines()
     .filter(|l| !l.is_empty())
     .map(|l| {
        let mut split = l.split(SEPARATOR);
        let left = split.next().unwrap().parse::<u128>().unwrap();
        let right = split.next().unwrap().parse::<u128>().unwrap();
        (left, right)
     })
     .for_each(|(left, right)| {
        left_side.push(left);
        right_side.push(right);
     });

     (left_side, right_side)
}

pub fn part_one(input: &str) -> Option<u128> {
    let (mut left, mut right) = parse_input(input);
    left.sort();
    right.sort();

    let mut distance = 0;
    for (&x, &y) in left.iter().zip(right.iter()) {
        distance += x.abs_diff(y);
    }
    Some(distance)
}

pub fn part_two(input: &str) -> Option<u128> {
    let (left, right) = parse_input(input);

    let mut freq = HashMap::<u128, u128>::new();

    right.iter().for_each(|id| {
        *freq.entry(*id).or_default() += 1;
    });

    let sum = left.iter().fold(0, |acc, id|
        acc + id * freq.get(&id).unwrap_or(&0)
    );

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11))
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
