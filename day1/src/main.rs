use std::fs;
use itertools::izip;
use regex:: Regex;

fn build_lists(input: &str) -> (Vec<isize>, Vec<isize>) {
    let mut lefts: Vec<isize> = Vec::new();
    let mut rights: Vec<isize> = Vec::new();
    let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        lefts.push(caps[1].parse().unwrap());
        rights.push(caps[2].parse().unwrap());
    }
    (lefts, rights)
}

fn part1(input: &str) -> usize {
    let (mut lefts, mut rights) = build_lists(input);
    let mut total = 0;
    lefts.sort();
    rights.sort();
    for (left, right) in izip!(&lefts, &rights) {
        total += left.abs_diff(*right)
    }
    total
}

fn part2(input: &str) -> usize {
    let (lefts, rights) = build_lists(input);
    let mut total = 0;
    for item in lefts {
        total += item as usize * rights.iter().filter(|x| **x == item).count()
    }
    total
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        assert_eq!(11, part1(&input));
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        assert_eq!(31, part2(&input));
    }
}
