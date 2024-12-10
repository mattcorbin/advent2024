use std::fs;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct File {
    id: usize,
    len: usize,
}

impl File {
    fn new(id: usize, len: usize) -> Self {
        Self { id, len }
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct DiskMap {
    files: Vec<File>,
    map: Vec<Option<usize>>,
}

impl From<&str> for DiskMap {
    fn from(s: &str) -> Self {
        let mut file_id = 0;
        let mut is_file = true;
        let mut files = Vec::new();
        let mut map = Vec::new();
        for c in s.chars() {
            let number = c.to_digit(10).unwrap() as usize;
            if is_file {
                files.push(File::new(file_id, number));
                for _ in 0..number {
                    map.push(Some(file_id));
                }
                file_id += 1;
                is_file = false;
            } else {
                for _ in 0..number {
                    map.push(None);
                }
                is_file = true;
            }
        }
        DiskMap { files, map }
    }
}

impl DiskMap {
    fn defrag(&self) -> DiskMap {
        let mut new_map = self.map.clone();
        'outer: for i in (0..new_map.len()).rev() {
            if let Some(&item) = new_map.get(i) {
                if new_map[..i].iter().any(|x| x.is_none()) {
                    for j in 0..i {
                        if new_map[j].is_none() {
                            new_map[j] = item;
                            new_map[i] = None;
                            continue 'outer;
                        }
                    }
                }
            }
        }
        DiskMap {
            files: self.files.clone(),
            map: new_map,
        }
    }

    fn defrag_contiguous(&self) -> DiskMap {
        let mut new_map = self.map.clone();
        'outer: for file in self.files.iter().rev() {
            let len = file.len;
            let mut current_len = 0;
            for i in 0..new_map.len() {
                if new_map[i].is_some() {
                    current_len = 0;
                } else {
                    current_len += 1;
                    if current_len >= len {
                        let old_loc = new_map.iter().position(|x| x.is_some() && x.unwrap() == file.id).unwrap();
                        let new_loc = i-current_len+1;
                        if old_loc > new_loc {
                            new_map.splice(old_loc..old_loc+len, vec![None; len]);
                            new_map.splice(new_loc..new_loc+len, vec![Some(file.id); len]);
                        }
                        continue 'outer;
                    }
                }
            }
        }
        DiskMap {
            files: self.files.clone(),
            map: new_map,
        }
    }

    fn checksum(&self) -> usize {
        let mut sum = 0;
        for (idx, &item) in self.map.iter().enumerate() {
            if let Some(id) = item {
                sum += idx * id;
            }
        }
        sum
    }
}

fn part1(input: &str) -> usize {
    let disk_map = DiskMap::from(input);
    let disk_map = disk_map.defrag();
    disk_map.checksum()
}

fn part2(input: &str) -> usize {
    let disk_map = DiskMap::from(input);
    let disk_map = disk_map.defrag_contiguous();
    disk_map.checksum()
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
        assert_eq!(1928, part1(&input));
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        assert_eq!(2858, part2(&input));
    }
}
