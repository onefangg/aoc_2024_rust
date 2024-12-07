use itertools::Itertools;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub fn find_coord(grid_data: &Vec<Vec<char>>, find: char) -> (usize, usize) {
    for (r, row) in grid_data.iter().enumerate() {
        for (c, &col) in row.iter().enumerate() {
            if col == find {
                return (r, c);
            }
        }
    }
    // shouldn't happen
    (0, 0)
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum GuardDirection {
    Up,
    Down,
    Left,
    Right,
}

impl GuardDirection {
    pub fn turn(self) -> GuardDirection {
        match self {
            GuardDirection::Up => GuardDirection::Right,
            GuardDirection::Right => GuardDirection::Down,
            GuardDirection::Down => GuardDirection::Left,
            GuardDirection::Left => GuardDirection::Up,
        }
    }
}

struct Guard {
    row_idx: usize,
    col_idx: usize,
    direction: GuardDirection,
    visited: HashSet<(usize, usize)>,
}

impl Guard {
    pub fn move_one_space(
        &mut self,
        grid: &Vec<Vec<char>>,
        oob_col: usize,
        oob_row: usize,
    ) -> bool {
        let mut is_turn = false;
        match &self.direction {
            GuardDirection::Up => {
                if self.row_idx == 0 {
                    return false;
                }
                if grid[self.row_idx - 1][self.col_idx] != '#' {
                    self.row_idx -= 1;
                } else {
                    is_turn = true;
                }
            }
            GuardDirection::Down => {
                if self.row_idx == (oob_row - 1) {
                    return false;
                }
                if grid[self.row_idx + 1][self.col_idx] != '#' {
                    self.row_idx += 1;
                } else {
                    is_turn = true;
                }
            }
            GuardDirection::Left => {
                if self.col_idx == 0 {
                    return false;
                }
                if grid[self.row_idx][self.col_idx - 1] != '#' {
                    self.col_idx -= 1;
                } else {
                    is_turn = true;
                }
            }
            GuardDirection::Right => {
                if self.col_idx == (oob_col - 1) {
                    return false;
                }
                if grid[self.row_idx][self.col_idx + 1] != '#' {
                    self.col_idx += 1;
                } else {
                    is_turn = true;
                }
            }
        }
        if is_turn {
            self.direction = self.direction.clone().turn();
        } else {
            self.visited.insert((self.row_idx, self.col_idx));
        }
        true
    }
}

pub fn solve_part_a(input_path: &str) -> usize {
    let content = fs::read_to_string(&Path::new(input_path)).unwrap();
    let grid = content
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let (width, height) = (grid[0].len(), grid.len());
    let (curr_row, curr_col) = find_coord(&grid, '^');
    let mut guard = Guard {
        row_idx: curr_row,
        col_idx: curr_col,
        direction: GuardDirection::Up,
        visited: HashSet::from([(curr_row, curr_col)]),
    };
    loop {
        // println!(
        //     "guard at row {:?}, guard at col {:?}, facing direction: {:?}",
        //     guard.row_idx, guard.col_idx, guard.direction
        // );
        match guard.move_one_space(&grid, width, height) {
            true => continue,
            false => break,
        }
    }
    guard.visited.len()
}

pub fn solve_part_b(input_path: &str) -> usize {
    0
}

#[cfg(test)]
mod day6_tests {
    use super::*;

    #[test]
    pub fn test_with_sample_data() {
        assert_eq!(solve_part_a("./inputs/day6_sample.txt"), 41);
        assert_eq!(solve_part_b("./inputs/day6_sample.txt"), 0);
    }

    #[test]
    pub fn test_with_actual_data() {
        assert_eq!(solve_part_a("./inputs/day6.txt"), 5331);
        assert_eq!(solve_part_b("./inputs/day6.txt"), 0);
    }
}
