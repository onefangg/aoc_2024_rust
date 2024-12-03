use std::fs;
use std::path::Path;

pub fn solve_part_a(input_path: &str) -> u32 {
    let content = fs::read_to_string(&Path::new(input_path)).unwrap();
    0
}

pub fn solve_part_b(input_path: &str) -> u32 {
    let content = fs::read_to_string(&Path::new(input_path)).unwrap();
    0
}

#[cfg(test)]
mod day3_tests {
    use super::*;
    #[test]
    pub fn test_with_sample_data() {
        assert_eq!(solve_part_a("./inputs/day4a_sample.txt"), 0);
        assert_eq!(solve_part_b("./inputs/day4b_sample.txt"), 0);
    }

    #[test]
    pub fn test_with_actual_data() {
        assert_eq!(solve_part_a("./inputs/day4.txt"), 0);
        assert_eq!(solve_part_b("./inputs/day4.txt"), 0);
    }
}
