pub fn solve_part_a(input_path: &str) -> usize {}

pub fn solve_part_b(input_path: &str) -> usize {}

#[cfg(test)]
mod day3_tests {
    use super::*;
    #[test]
    pub fn test_with_sample_data() {
        assert_eq!(solve_part_a("./inputs/day3_sample.txt"), 0);
        assert_eq!(solve_part_b("./inputs/day3_sample.txt"), 0);
    }

    #[test]
    pub fn test_with_actual_data() {
        assert_eq!(solve_part_a("./inputs/day3.txt"), 0);
        assert_eq!(solve_part_b("./inputs/day3.txt"), 0);
    }
}
