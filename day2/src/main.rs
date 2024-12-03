use std::fs;

fn all_same_direction(levels: &Vec<isize>) -> bool {
    levels.windows(2).map(|x| x[0] - x[1]).all(|x| x > 0)
        || levels.windows(2).map(|x| x[0] - x[1]).all(|x| x < 0)
}

fn step_within(levels: &Vec<isize>) -> bool {
    levels
        .windows(2)
        .map(|x| (x[0] - x[1]).abs())
        .all(|x| (1 <= x) && (x <= 3))
}

fn part1(input: &str) -> usize {
    let mut total = 0;
    for line in input.lines() {
        let levels: Vec<isize> = line
            .split(" ")
            .map(|x| x.parse::<isize>().unwrap())
            .collect();
        if all_same_direction(&levels) && step_within(&levels) {
            total += 1;
        }
    }
    total
}

fn part2(input: &str) -> usize {
    let mut total = 0;
    for line in input.lines() {
        let levels: Vec<isize> = line
            .split(" ")
            .map(|x| x.parse::<isize>().unwrap())
            .collect();
        if all_same_direction(&levels) && step_within(&levels) {
            total += 1;
        } else if (0..levels.len())
            .map(|x| {
                let mut local = levels.clone();
                local.remove(x);
                local
            })
            .any(|x| all_same_direction(&x) && step_within(&x))
        {
            total += 1;
        }
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
        assert_eq!(2, part1(&input));
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        assert_eq!(4, part2(&input));
    }
}
