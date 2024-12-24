advent_of_code::solution!(2);

fn parse_input(input: &str) -> Vec<Vec<u128>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect()
        })
        .collect()
}

fn is_safe(nums: &[u128]) -> bool {
    if nums.len() < 2 {
        return false;
    }

    let increasing = nums[0] < nums[1];
    nums.windows(2).all(|window| {
        let diff = window[1] as i128 - window[0] as i128;
        if diff.abs() > 3 {
            return false;
        }
        if increasing {
            return diff > 0;
        }
        return diff < 0;
    })
}

fn part_one(input: &str) -> Option<u128> {
    Some(
        parse_input(input)
            .iter()
            .filter(|nums| is_safe(nums))
            .count() as u128
    )
}

fn part_two(input: &str) -> Option<u128> {
   Some(parse_input(input).iter().filter(|nums| {
    if is_safe(nums) { return true; }
    for i in 0..nums.len() {
        let mut nums_without_i = nums.to_vec();
        nums_without_i.remove(i);
        if is_safe(&nums_without_i) {
            return true;
        }
    }
    return false;
   }).count() as u128)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2))
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4))
    }
}
