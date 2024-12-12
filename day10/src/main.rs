use pathfinding::prelude::yen;
use std::collections::HashSet;
use std::fs;

struct Topology {
    map: Vec<Vec<usize>>,
}

impl Topology {
    fn neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let a = x as isize;
        let b = y as isize;
        let ret = vec![(a - 1, b), (a, b - 1), (a + 1, b), (a, b + 1)];
        ret.into_iter()
            .filter(|&(x, y)| {
                x >= 0
                    && y >= 0
                    && y < self.map.len() as isize
                    && x < self.map[y as usize].len() as isize
            })
            .map(|(x, y)| (x as usize, y as usize))
            .collect()
    }

    fn successors(&self, loc: (usize, usize)) -> Vec<((usize, usize), usize)> {
        let x = loc.0;
        let y = loc.1;
        let current = self.map[y][x];
        let mut ret = Vec::new();
        for (x2, y2) in self.neighbours(x, y) {
            if self.map[y2][x2] == current + 1 {
                ret.push(((x2, y2), 1));
            }
        }
        ret
    }

    fn trails(&self, x: usize, y: usize) -> Vec<Vec<(usize, usize)>> {
        yen(
            &(x, y),
            |x| self.successors(*x),
            |&(x, y)| self.map[y][x] == 9,
            100000,
        )
        .into_iter()
        .map(|x| x.0)
        .collect()
    }

    fn score(&self, x: usize, y: usize) -> usize {
        if self.map[y][x] != 0 {
            return 0;
        }
        let mut set = HashSet::new();
        let paths = self.trails(x, y);
        for path in paths {
            set.insert(path.last().unwrap().clone());
        }
        set.len()
    }

    fn score_all(&self) -> usize {
        let mut sum = 0;
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                sum += self.score(x, y);
            }
        }
        sum
    }

    fn rate(&self, x: usize, y: usize) -> usize {
        if self.map[y][x] != 0 {
            return 0;
        }
        let paths = self.trails(x, y);
        paths.len()
    }

    fn rate_all(&self) -> usize {
        let mut sum = 0;
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                sum += self.rate(x, y);
            }
        }
        sum
    }
}

impl From<&str> for Topology {
    fn from(value: &str) -> Self {
        let mut map = Vec::new();
        for line in value.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c.to_digit(10).unwrap() as usize);
            }
            map.push(row);
        }
        Topology { map }
    }
}

fn part1(input: &str) -> usize {
    let topology = Topology::from(input);
    topology.score_all()
}

fn part2(input: &str) -> usize {
    let topology = Topology::from(input);
    topology.rate_all()
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
        assert_eq!(36, part1(&input));
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        assert_eq!(81, part2(&input));
    }
}
