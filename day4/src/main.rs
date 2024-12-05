use crate::Direction::*;
use std::fs;

#[derive(Copy, Clone)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

fn next(grid: &Vec<Vec<char>>, x: usize, y: usize, direction: Direction) -> Option<(usize, usize)> {
    match direction {
        Up => {
            if y != 0 {
                return Some((x, y - 1));
            }
        }
        Down => {
            if y != grid.len() - 1 {
                return Some((x, y + 1));
            }
        }
        Left => {
            if x != 0 {
                return Some((x - 1, y));
            }
        }
        Right => {
            if x != grid[y].len() - 1 {
                return Some((x + 1, y));
            }
        }
        UpRight => {
            if y != 0 && x != grid[y].len() - 1 {
                return Some((x + 1, y - 1));
            }
        }
        DownRight => {
            if y != grid.len() - 1 && x != grid[y].len() - 1 {
                return Some((x + 1, y + 1));
            }
        }
        DownLeft => {
            if y != grid.len() - 1 && x != 0 {
                return Some((x - 1, y + 1));
            }
        }
        UpLeft => {
            if y != 0 && x != 0 {
                return Some((x - 1, y - 1));
            }
        }
    }
    None
}

fn follow_direction(grid: &Vec<Vec<char>>, x: usize, y: usize, direction: Direction) -> usize {
    if let Some(point) = next(&grid, x, y, direction) {
        if grid[point.1][point.0] == 'M' {
            if let Some(point) = next(&grid, point.0, point.1, direction) {
                if grid[point.1][point.0] == 'A' {
                    if let Some(point) = next(&grid, point.0, point.1, direction) {
                        if grid[point.1][point.0] == 'S' {
                            return 1;
                        }
                    }
                }
            }
        }
    }
    0
}

fn part1(input: &str) -> usize {
    let mut total = 0;
    let mut grid = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().collect::<Vec<char>>());
    }
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'X' {
                for direction in [Up, Down, Left, Right, UpRight, UpLeft, DownRight, DownLeft] {
                    total += follow_direction(&grid, x, y, direction);
                }
            }
        }
    }
    total
}

fn near_edge(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    x == 0 || y == 0 || x == grid[y].len() - 1 || y == grid.len() - 1
}

fn xmas(grid: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    if near_edge(grid, x, y) {
        return 0;
    }
    let valid_xmases = [
        ['M', 'M', 'S', 'S'],
        ['S', 'M', 'M', 'S'],
        ['S', 'S', 'M', 'M'],
        ['M', 'S', 'S', 'M'],
    ];
    let test_xmas = [grid[y-1][x-1], grid[y-1][x+1], grid[y+1][x+1], grid[y+1][x-1]];
    if valid_xmases.contains(&test_xmas) {
        return 1;
    }
    0
}

fn part2(input: &str) -> usize {
    let mut total = 0;
    let mut grid = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().collect::<Vec<char>>());
    }
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'A' {
                total += xmas(&grid, x, y);
            }
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
        assert_eq!(18, part1(&input));
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        assert_eq!(9, part2(&input));
    }
}
