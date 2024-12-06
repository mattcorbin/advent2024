use std::collections::HashSet;
use std::fs;
use rayon::prelude::*;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Cell {
    Empty,
    Occupied,
    Guard,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Cell::Empty,
            '#' => Cell::Occupied,
            '^' => Cell::Guard,
            _ => panic!("Unknown cell type {}", c),
        }
    }
}

#[derive(Clone, Debug)]
struct Grid {
    cells: Vec<Vec<Cell>>,
    starting_position: (usize, usize),
}

impl Grid {
    fn can_move(&self, current_location: (usize, usize), direction: Direction) -> bool {
        match direction {
            Direction::North => {
                if current_location.1 == 0 {
                    false
                } else if self.cells[current_location.1 - 1][current_location.0] == Cell::Occupied {
                    false
                } else {
                    true
                }
            }
            Direction::East => {
                if current_location.0 == self.cells[current_location.1].len() - 1 {
                    false
                } else if self.cells[current_location.1][current_location.0 + 1] == Cell::Occupied {
                    false
                } else {
                    true
                }
            }
            Direction::South => {
                if current_location.1 == self.cells.len() - 1 {
                    false
                } else if self.cells[current_location.1 + 1][current_location.0] == Cell::Occupied {
                    false
                } else {
                    true
                }
            }
            Direction::West => {
                if current_location.0 == 0 {
                    false
                } else if self.cells[current_location.1][current_location.0 - 1] == Cell::Occupied {
                    false
                } else {
                    true
                }
            }
        }
    }

    fn go(&self, current_location: (usize, usize), direction: Direction) -> (usize, usize) {
        match direction {
            Direction::North => (current_location.0, current_location.1 - 1),
            Direction::East => (current_location.0 + 1, current_location.1),
            Direction::South => (current_location.0, current_location.1 + 1),
            Direction::West => (current_location.0 - 1, current_location.1),
        }
    }

    fn leaves(&self, current_location: (usize, usize), direction: Direction) -> bool {
        match direction {
            Direction::North => {
                if current_location.1 == 0 {
                    true
                } else {
                    false
                }
            }
            Direction::East => {
                if current_location.0 == self.cells[current_location.1].len() - 1 {
                    true
                } else {
                    false
                }
            }
            Direction::South => {
                if current_location.1 == self.cells.len() - 1 {
                    true
                } else {
                    false
                }
            }
            Direction::West => {
                if current_location.0 == 0 {
                    true
                } else {
                    false
                }
            }
        }
    }

    fn execute(&self) -> usize {
        let mut current_direction = Direction::North;
        let mut current_position = self.starting_position;
        let mut visited = HashSet::new();
        visited.insert(current_position);
        while !self.can_move(current_position, current_direction) {
            current_direction = current_direction.turn_right();
        }
        while !self.leaves(current_position, current_direction) {
            while !self.can_move(current_position, current_direction) {
                current_direction = current_direction.turn_right();
            }
            current_position = self.go(current_position, current_direction);
            visited.insert(current_position);
        }
        visited.len()
    }

    fn is_loop(&self) -> bool {
        let mut current_direction = Direction::North;
        let mut current_position = self.starting_position;
        let max_steps: usize = 100000;
        let mut steps = 0;
        while !self.can_move(current_position, current_direction) {
            current_direction = current_direction.turn_right();
        }
        while !self.leaves(current_position, current_direction) && steps < 100000 {
            while !self.can_move(current_position, current_direction) {
                current_direction = current_direction.turn_right();
            }
            current_position = self.go(current_position, current_direction);
            steps += 1;
        }
        steps == max_steps
    }
}

impl From<&str> for Grid {
    fn from(input: &str) -> Grid {
        let mut cells = Vec::new();
        let mut starting_position = (0, 0);
        for line in input.lines() {
            let mut row = Vec::new();
            for char in line.chars() {
                row.push(Cell::from(char));
            }
            cells.push(row);
        }
        'outer: for y in 0..cells.len() {
            for x in 0..cells[y].len() {
                if cells[y][x] == Cell::Guard {
                    starting_position = (x, y);
                    break 'outer;
                }
            }
        }
        Grid {
            cells,
            starting_position,
        }
    }
}

fn part1(input: &str) -> usize {
    let grid = Grid::from(input);
    grid.execute()
}

fn part2(input: &str) -> usize {
    let grid = Grid::from(input);
    let mut grids = Vec::new();
    for y in 0..grid.cells.len() {
        for x in 0..grid.cells[y].len() {
            if grid.cells[y][x] == Cell::Empty {
                let mut new_grid = grid.clone();
                new_grid.cells[y][x] = Cell::Occupied;
                grids.push(new_grid);
            }
        }
    }
    grids.into_par_iter().map(|gr| gr.is_loop()).filter(|res| *res == true).count()
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
        assert_eq!(41, part1(&input));
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        assert_eq!(6, part2(&input));
    }
}
