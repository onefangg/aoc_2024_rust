use std::fs;
use std::path::Path;

pub fn solve_part_a_and_b(input_str: &str) -> (usize, usize) {
    let content = fs::read_to_string(Path::new(input_str));
    (0, 0)
}

#[cfg(test)]
mod day9_tests {
    use super::*;

    #[test]
    pub fn test_with_sample_data() {
        let sln = solve_part_a_and_b("./inputs/day9_sample.txt");
        assert_eq!(sln.0, 0);
        assert_eq!(sln.1, 0);
    }

    #[test]
    pub fn test_with_actual_data() {
        let sln = solve_part_a_and_b("./inputs/day9.txt");
        assert_eq!(sln.0, 0);
        assert_eq!(sln.1, 0);
    }
}
