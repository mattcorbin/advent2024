use rayon::prelude::*;
use std::fs;

#[derive(Clone, Debug)]
struct Equation {
    left: usize,
    right: Vec<usize>,
}

impl From<&str> for Equation {
    fn from(input: &str) -> Equation {
        let mut splits = input.split(": ");
        let left = splits.next().unwrap().parse().unwrap();
        let splits = splits.next().unwrap().split(" ");
        let right = splits.map(|item| item.parse().unwrap()).collect();
        Equation { left, right }
    }
}

impl Equation {
    fn test_all(
        &self,
        max: usize,
        current: usize,
        items: &Vec<usize>,
        operators: &[fn(usize, usize) -> usize],
    ) -> Vec<usize> {
        if current > max {
            return vec![];
        }
        if items.len() > 0 {
            let mut results = Vec::new();
            let mut local = items.clone();
            let item = local.remove(0);
            for op in operators {
                results.extend(self.test_all(max, op(current, item), &local, operators));
            }
            results
        } else {
            vec![current]
        }
    }

    fn valid(&self, operators: &[fn(usize, usize) -> usize]) -> bool {
        let mut local = self.right.clone();
        let start = local.remove(0);
        self.test_all(self.left, start, &local, operators)
            .contains(&self.left)
    }
}

fn add(a: usize, b: usize) -> usize {
    a + b
}

fn mult(a: usize, b: usize) -> usize {
    a * b
}

fn part1(input: &str) -> usize {
    input
        .par_lines()
        .map(|line| Equation::from(line))
        .filter(|e| e.valid(&[add, mult]))
        .map(|r| r.left)
        .sum()
}

fn concat(a: usize, b: usize) -> usize {
    (a.to_string() + &*b.to_string()).parse().unwrap()
}

fn part2(input: &str) -> usize {
    input
        .par_lines()
        .map(|line| Equation::from(line))
        .filter(|e| e.valid(&[add, mult, concat]))
        .map(|r| r.left)
        .sum()
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
        assert_eq!(3749, part1(&input));
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        assert_eq!(11387, part2(&input));
    }
}
