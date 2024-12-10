use std::fs;
use std::path::Path;
use itertools::Itertools;

pub fn solve(input_str: &str) -> usize {
    let content = fs::read_to_string(Path::new(input_str)).unwrap();
    let grid = content.lines()
        .map(|l| l.chars().map(|x| x.to_digit(10).unwrap()).collect_vec())
        .collect_vec();
    let (width, height) = (grid[0].len(), grid.len());
    let mut trailheads = Vec::<(usize, usize)>::new();
    let _ = (0..height).map(|r| (0..width).for_each(|c|
        if grid[r][c] == 0 {
            trailheads.push((r, c));
        }
    ));

    let mut score = 0;
    for (trail_row, trail_col) in trailheads {
        let n = get_valid_neighbours(&grid, trail_col, trail_col);
        dbg!(n);
    }
    0
}

pub fn get_valid_neighbours(grid: &Vec<Vec<(u32)>>, curr_row: usize, curr_col: usize) -> Vec<(usize, usize)> {
    let curr_element = grid[curr_row][curr_col];
    let adjacent = vec![
        grid.get(curr_row-1).ok_or(10).into_iter().get(curr_col).ok() as usize,
        grid.get(curr_row+1).ok_or(10).into_iter().get(curr_col).ok() as usize,
        grid.get(curr_row).ok_or(10).into_iter().get(curr_col-1).ok() as usize,
        grid.get(curr_row).ok_or(10).into_iter().get(curr_col+1).ok() as usize,
    ]
        .enumerate()
        .map(|(_, x)| { x- curr_element })
        .filter(|_,e| e.abs() == 1).map(|(i,x)| i).collect_vec::<usize>();
    adjacent
}


#[cfg(test)]
mod day10_tests {
    use super::*;

    #[test]
    pub fn solve_with_sample_input() {
        assert_eq!(solve("./inputs/day10_sample.txt"), 36);
    }
    #[test]
    pub fn solve_with_actual_input() {
        assert_eq!(solve("./inputs/day10.txt"), 0);
    }
}