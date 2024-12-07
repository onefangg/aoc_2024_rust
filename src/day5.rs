use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

pub fn solve_part_a_and_b(input_path: &str) -> usize {
    let content = fs::read_to_string(&Path::new(input_path)).unwrap();
    let parsed_content = content.split("\r\n\r\n").collect::<Vec<&str>>();
    let (rules, updates) = (parsed_content[0], parsed_content[1]);

    let rules_lookup = rules
        .lines()
        .map(|r| {
            let (l, r) = r.split_once('|').unwrap();
            (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap())
        })
        .fold(
            HashMap::<usize, HashSet<usize>>::new(),
            |mut acc, (l, r)| {
                acc.entry(l)
                    .and_modify(|e| {
                        (*e).insert(r);
                    })
                    .or_insert(HashSet::from([r]));
                acc
            },
        );

    let mut part_a = 0 as usize;
    let unordered_updates = updates
        .lines()
        .map(|line| {
            line.split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect_vec()
        })
        .for_each(|mut x| {
            if x.iter()
                .is_sorted_by(|a, b| rules_lookup.contains_key(a) && rules_lookup[a].contains(b))
            {
                part_a += x[x.len() / 2];
            } else {
            }
        });
    part_a
}

pub fn solve_part_b(input_path: &str) -> usize {
    0
}

#[cfg(test)]
mod day5_tests {
    use super::*;
    #[test]
    pub fn test_with_sample_data() {
        assert_eq!(solve_part_a_and_b("./inputs/day5_sample.txt"), 143);
        // assert_eq!(solve_part_b("./inputs/day5b_sample.txt"), 0);
    }

    #[test]
    pub fn test_with_actual_data() {
        assert_eq!(solve_part_a_and_b("./inputs/day5.txt"), 5091);
        assert_eq!(solve_part_b("./inputs/day5.txt"), 0);
    }
}
