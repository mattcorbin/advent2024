use std::cmp::Ordering;
use std::fs;

#[derive(Copy, Clone)]
struct Rule {
    first: usize,
    second: usize
}

impl Rule {
    fn valid(&self, pages: &Vec<usize>) -> bool {
        let mut order = Vec::new();
        for page in pages {
            if self.first == *page {
                order.push(*page);
            } else if self.second == *page {
                order.push(*page);
            }
        }
        order.len() < 2 || order == vec![self.first, self.second]
    }
}


impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        let mut splits = value.split("|");
        Rule{
            first: splits.next().unwrap().parse().unwrap(),
            second: splits.next().unwrap().parse().unwrap(),
        }
    }
}

fn sort_rule(a: usize, b: usize, rules: &Vec<Rule>) -> Ordering {
    let applicable_rule = rules.iter().filter(|rule| (rule.first == a && rule.second == b) || (rule.first == b && rule.second == a)).cloned().collect::<Vec<Rule>>();
    if applicable_rule.len() > 1 {
        panic!("wtf");
    } else if applicable_rule.len() == 0 {
        return Ordering::Equal
    }
    if let Some(rule) = applicable_rule.first() {
        if a == rule.first {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    } else {
        Ordering::Equal
    }
}

#[derive(Clone)]
struct Pages {
    numbers: Vec<usize>,
}

impl Pages {
    fn valid(&self, rules: &Vec<Rule>) -> bool {
        for rule in rules {
            if !rule.valid(&self.numbers) {
                return false
            }
        }
        true
    }

    fn middle(&self) -> usize {
        self.numbers[self.numbers.len() / 2]
    }

    fn fix_order(&self, rules: &Vec<Rule>) -> Self {
        let mut new_numbers = self.numbers.clone();
        new_numbers.sort_by(|a, b| sort_rule(*a, *b, rules));
        Pages {
            numbers: new_numbers,
        }
    }
}

impl From<&str> for Pages {
    fn from(value: &str) -> Self {
        Pages {
            numbers: value.split(",").map(|x| x.parse().unwrap()).collect::<Vec<usize>>()
        }
    }
}

fn part1(input: &str) -> usize {
    let mut total = 0;
    let mut rules = Vec::new();
    let mut pages_lists = Vec::new();
    for line in input.lines() {
        if line.contains("|") {
            rules.push(Rule::from(line));
        } else if line.contains(",") {
            pages_lists.push(Pages::from(line))
        }
    }
    for pages in pages_lists {
        if pages.valid(&rules) {
            total += pages.middle();
        }
    }
    total
}

fn part2(input: &str) -> usize {
    let mut rules = Vec::new();
    let mut pages_lists = Vec::new();
    for line in input.lines() {
        if line.contains("|") {
            rules.push(Rule::from(line));
        } else if line.contains(",") {
            pages_lists.push(Pages::from(line))
        }
    }
    let mut pages_to_reorder = Vec::new();
    for pages in pages_lists {
        if !pages.valid(&rules) {
            pages_to_reorder.push(pages.clone());
        }
    }
    pages_to_reorder.into_iter().map(|p| p.fix_order(&rules)).map(|p| p.middle()).sum()
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
        assert_eq!(143, part1(&input));
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        assert_eq!(123, part2(&input));
    }
}
