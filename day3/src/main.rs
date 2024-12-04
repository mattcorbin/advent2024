use std::fs;
use regex::Regex;

struct Mult {
    l: usize,
    r: usize
}

impl Mult {
    fn execute(&self) -> usize {
        self.l * self.r
    }
}

impl From<&str> for Mult {
    fn from(value: &str) -> Self {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let caps = re.captures(value).unwrap();
        Mult{
            l: caps[1].parse().unwrap(),
            r: caps[2].parse().unwrap()
        }
    }
}

fn part1(input: &str) -> usize {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let mults = re.find_iter(input).map(|m| Mult::from(m.as_str())).collect::<Vec<Mult>>();
    mults.iter().map(|m| m.execute()).sum()
}

fn part2(input: &str) -> usize {
    let re = Regex::new(r"(?<all>(?<op>mul|do|don't)\((\d+,\d+)*\))").unwrap();
    let mut mults = Vec::new();
    let mut running = true;
    for token in re.captures_iter(input) {
        match token.name("op").unwrap().as_str() {
            "mul" => {
                if running {
                    mults.push(Mult::from(token.name("all").unwrap().as_str()))
                }
            }
            "do" => {
                if token.name("all").unwrap().as_str() == "do()" {
                    running = true
                }
            }
            "don't" => {
                if token.name("all").unwrap().as_str() == "don't()" {
                    running = false
                }
            }
            _ => panic!("at the disco!")
        }
    }
    mults.iter().map(|m| m.execute()).sum()
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
        assert_eq!(161, part1(&input));
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        assert_eq!(48, part2(&input));
    }
}
