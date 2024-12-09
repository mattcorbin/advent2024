use std::cmp::{max, min};
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashSet;
use std::fs;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Map {
    hwidth: usize,
    vwidth: usize,
    grid: Vec<Vec<char>>,
    antenna: HashSet<char>,
    antenna_locs: Vec<(usize, usize)>,
}

impl Map {
    fn dist(a: (usize, usize), b: (usize, usize)) -> usize {
        ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()) as usize
    }

    fn points(&self, a: (usize, usize), b: (usize, usize)) -> Vec<(isize, isize)> {
        let x0 = a.0 as isize;
        let x1 = b.0 as isize;
        let y0 = a.1 as isize;
        let y1 = b.1 as isize;
        let x_dist = (x0 - x1).abs();
        let y_dist = (y0 - y1).abs();
        vec![
            (x0 + x_dist, y0 + y_dist),
            (x0 - x_dist, y0 + y_dist),
            (x0 + x_dist, y0 - y_dist),
            (x0 - x_dist, y0 - y_dist),
            (x1 + x_dist, y1 + y_dist),
            (x1 - x_dist, y1 + y_dist),
            (x1 + x_dist, y1 - y_dist),
            (x1 - x_dist, y1 - y_dist),
        ]
    }

    fn right_dist(dist: usize, test: (usize, usize), a: (usize, usize), b: (usize, usize)) -> bool {
        (Self::dist(a, test) == dist && Self::dist(b, test) == dist * 2)
            || (Self::dist(a, test) == dist * 2 && Self::dist(b, test) == dist)
    }

    fn in_grid(&self, x: isize, y: isize) -> bool {
        x >= 0 && y >= 0 && x < self.hwidth as isize && y < self.vwidth as isize
    }

    fn calculate_antinodes(&self, a: (usize, usize), b: (usize, usize)) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        let dist = Self::dist(a, b);
        for possibility in self.points(a, b) {
            if self.in_grid(possibility.0, possibility.1) {
                let local_x = possibility.0 as usize;
                let local_y = possibility.1 as usize;
                if Self::right_dist(dist, (local_x, local_y), a, b) {
                    res.push((local_x, local_y));
                }
            }
        }
        res
    }

    fn find_antinodes(&self, antenna: char) -> Vec<(usize, usize)> {
        let mut locs = Vec::new();
        for (y, row) in self.grid.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == antenna {
                    locs.push((x, y));
                }
            }
        }
        let mut results = Vec::new();
        for pair in locs.iter().combinations(2) {
            results.push(self.calculate_antinodes(*pair[0], *pair[1]));
        }
        results
            .into_iter()
            .flatten()
            .filter(|item| !locs.contains(item))
            .collect()
    }

    fn antinodes(&self) -> HashSet<(usize, usize)> {
        self.antenna
            .clone()
            .into_par_iter()
            .map(|antenna| self.find_antinodes(antenna))
            .flatten()
            .collect()
    }


    fn all_points(&self, a: (usize, usize), b: (usize, usize)) -> Vec<(isize, isize)> {
        let x0 = a.0 as isize;
        let x1 = b.0 as isize;
        let y0 = a.1 as isize;
        let y1 = b.1 as isize;
        let x_diff = x0 - x1;
        let y_diff = y0 - y1;
        if x_diff == 0 && y_diff == 0 {
            vec![]
        } else if x_diff == 0 {
            let mut res = Vec::new();
            let mut high_y = max(y0, y1);
            while high_y < self.vwidth as isize {
                res.push((x0, high_y));
                high_y += y_diff.abs();
            }
            let mut low_y = min(y0, y1);
            while low_y >= 0 {
                res.push((x0, low_y));
                low_y -= y_diff.abs();
            }
            res
        } else if y_diff == 0 {
            let mut res = Vec::new();
            let mut high_x = max(x0, x1);
            while high_x < self.hwidth as isize {
                res.push((high_x, y0));
                high_x += x_diff.abs();
            }
            let mut low_x = min(x0, x1);
            while low_x >= 0 {
                res.push((low_x, y0));
                low_x -= x_diff.abs();
            }
            res
        } else {
            let mut res = Vec::new();
            let x_diff = x_diff.abs();
            let y_diff = y_diff.abs();
            if y0 > y1 {
                if x0 > x1 {
                    let mut next_point = (x0 + x_diff, y0 + y_diff);
                    while self.in_grid(next_point.0, next_point.1) {
                        res.push((next_point.0, next_point.1));
                        next_point = (next_point.0 + x_diff, next_point.1 + y_diff);
                    }
                    let mut next_point = (x1 - x_diff, y1 - y_diff);
                    while self.in_grid(next_point.0, next_point.1) {
                        res.push((next_point.0, next_point.1));
                        next_point = (next_point.0 - x_diff, next_point.1 - y_diff);
                    }
                } else {
                    let mut next_point = (x0 - x_diff, y0 + y_diff);
                    while self.in_grid(next_point.0, next_point.1) {
                        res.push((next_point.0, next_point.1));
                        next_point = (next_point.0 - x_diff, next_point.1 + y_diff);
                    }
                    let mut next_point = (x1 + x_diff, y1 - y_diff);
                    while self.in_grid(next_point.0, next_point.1) {
                        res.push((next_point.0, next_point.1));
                        next_point = (next_point.0 + x_diff, next_point.1 - y_diff);
                    }
                }
            } else {
                if x0 > x1 {
                    let mut next_point = (x0 + x_diff, y0 - y_diff);
                    while self.in_grid(next_point.0, next_point.1) {
                        res.push((next_point.0, next_point.1));
                        next_point = (next_point.0 + x_diff, next_point.1 - y_diff);
                    }
                    let mut next_point = (x1 - x_diff, y1 + y_diff);
                    while self.in_grid(next_point.0, next_point.1) {
                        res.push((next_point.0, next_point.1));
                        next_point = (next_point.0 - x_diff, next_point.1 + y_diff);
                    }
                } else {
                    let mut next_point = (x0 - x_diff, y0 - y_diff);
                    while self.in_grid(next_point.0, next_point.1) {
                        res.push((next_point.0, next_point.1));
                        next_point = (next_point.0 - x_diff, next_point.1 - y_diff);
                    }
                    let mut next_point = (x1 + x_diff, y1 + y_diff);
                    while self.in_grid(next_point.0, next_point.1) {
                        res.push((next_point.0, next_point.1));
                        next_point = (next_point.0 + x_diff, next_point.1 + y_diff);
                    }
                }
            }
            res
        }
    }

    fn calculate_all_antinodes(&self, a: (usize, usize), b: (usize, usize)) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        for possibility in self.all_points(a, b) {
            if self.in_grid(possibility.0, possibility.1) {
                let local_x = possibility.0 as usize;
                let local_y = possibility.1 as usize;
                res.push((local_x, local_y));
            }
        }
        res
    }

    fn find_all_antinodes(&self, antenna: char) -> Vec<(usize, usize)> {
        let mut locs = Vec::new();
        for (y, row) in self.grid.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == antenna {
                    locs.push((x, y));
                }
            }
        }
        let mut results = Vec::new();
        for pair in locs.iter().combinations(2) {
            results.push(self.calculate_all_antinodes(*pair[0], *pair[1]));
        }
        results
            .into_iter()
            .flatten()
            .filter(|item| !locs.contains(item))
            .collect()
    }

    fn extended_antinodes(&self) -> HashSet<(usize, usize)> {
        let mut set: HashSet<(usize, usize)> = self.antenna
            .clone()
            .into_par_iter()
            .map(|antenna| self.find_all_antinodes(antenna))
            .flatten()
            .collect();
            for loc in &self.antenna_locs {
                set.insert(loc.clone());
            }
        set
    }
}

impl From<&str> for Map {
    fn from(input: &str) -> Map {
        let mut grid = Vec::new();
        let vwidth = input.lines().count();
        let hwidth = input.lines().next().unwrap().len();
        for line in input.lines() {
            grid.push(line.chars().collect::<Vec<char>>());
        }
        let mut antenna = HashSet::new();
        let mut antenna_locs = Vec::new();
        for (y, row) in grid.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c != '.' {
                    antenna.insert(*c);
                    antenna_locs.push((x, y));
                }
            }
        }
        Map {
            hwidth,
            vwidth,
            grid,
            antenna,
            antenna_locs,
        }
    }
}

fn part1(input: &str) -> usize {
    let map = Map::from(input);
    map.antinodes().len()
}

fn part2(input: &str) -> usize {
    let map = Map::from(input);
    map.extended_antinodes().len()
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
        assert_eq!(14, part1(&input));
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        assert_eq!(34, part2(&input));
    }
}
