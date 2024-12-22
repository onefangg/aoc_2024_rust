pub fn solve(input_seeds: Vec<u64>) -> u64 {
    let mut res = 0;
    for seed in input_seeds {
        let mut temp = seed;
        for _ in 0..2000 {
            temp = generate_pseudo_random(temp);
        }
        res += temp;
    }
    res
}

pub fn generate_pseudo_random(seed: u64) -> u64 {
    let a = ((seed * 64) ^ seed) % 16777216;
    let b = ((a / 32) ^ a) % 16777216;
    let c = ((b * 2048) ^ b) % 16777216;
    c
}

#[cfg(test)]
mod day22_tests {
    use super::*;
    use std::fs;
    #[test]
    pub fn test_with_sample_data() {
        assert_eq!(solve(vec![1, 10, 100, 2024]), 37327623);
    }
    #[test]
    pub fn test_with_actual_data() {
        let read_input = fs::read_to_string("./inputs/day22.txt")
            .unwrap()
            .lines()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        assert_eq!(solve(read_input), 19458130434);
    }
}
