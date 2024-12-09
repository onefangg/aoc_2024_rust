use itertools::Itertools;
use std::fs;
use std::path::Path;

pub fn solve_part_a_and_b(input_str: &str) -> (u64, usize) {
    let content = fs::read_to_string(Path::new(input_str)).unwrap();
    let mut create_disk_map = Vec::<isize>::new();

    content.chars().enumerate().for_each(|(idx, x)| {
        let freq = x.to_digit(10).unwrap() as usize;
        let ele = match idx % 2 == 0 {
            true => idx.div_ceil(2) as isize,
            false => -1,
        };
        (0..freq).for_each(|_| create_disk_map.push(ele));
    });

    let mut currIdx = create_disk_map.len() - 1;
    while currIdx > 0 {
        let ele = create_disk_map[currIdx];
        let first_free_idx = match create_disk_map.iter().position(|&x| x == -1) {
            Some(idx) => idx,
            None => create_disk_map.len(),
        };
        if first_free_idx >= currIdx {
            break;
        }
        create_disk_map.remove(currIdx);
        create_disk_map.push(-1);
        create_disk_map.remove(first_free_idx);
        create_disk_map.insert(first_free_idx, ele);
        currIdx -= 1;
    }

    (calculate_checksum(&create_disk_map), 0)
}

pub fn calculate_checksum(arr: &Vec<isize>) -> u64 {
    arr.iter()
        .enumerate()
        .filter(|(_, &e)| e != -1)
        .map(|(i, &e)| (i as u64 * e as u64))
        .sum()
}

#[cfg(test)]
mod day9_tests {
    use super::*;

    #[test]
    pub fn test_with_sample_data() {
        let sln = solve_part_a_and_b("./inputs/day9_sample.txt");
        assert_eq!(sln.0, 1928);
        assert_eq!(sln.1, 0);
    }

    #[test]
    pub fn test_with_actual_data() {
        let sln = solve_part_a_and_b("./inputs/day9.txt");
        // need to revisit why part 1 is SO SLOW
        assert_eq!(sln.0, 6211348208140);
        assert_eq!(sln.1, 0);
    }
}
