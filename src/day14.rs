use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct Robot {
    pos_x: i64,
    pos_y: i64,
    vel_x: i64,
    vel_y: i64,
}

impl Robot {
    pub fn move_by_seconds(&self, width: i64, height: i64, seconds: usize) -> Robot {
        let mut new_x = self.pos_x + self.vel_x * seconds as i64;
        let mut new_y = self.pos_y + self.vel_y * seconds as i64;

        while (new_x < 0 || new_x >= width) {
            let dir = if new_x >= width { -1 } else { 1 };
            new_x += dir * width;
        }
        while (new_y < 0 || new_y >= height) {
            let dir = if new_y >= height { -1 } else { 1 };
            new_y += dir * height;
        }

        Robot {
            pos_x: new_x,
            pos_y: new_y,
            vel_x: self.vel_x,
            vel_y: self.vel_y,
        }
    }
}

pub fn get_safety_ratio(input_robots: &Vec<Robot>, width: i64, height: i64) -> u64 {
    let (mid_x, mid_y) = (width / 2, height / 2);
    let mut quads: [u64; 4] = [0; 4];
    for robot in input_robots {
        if (robot.pos_x < mid_x) && (robot.pos_y < mid_y) {
            quads[0] += 1;
        } else if (robot.pos_x > mid_x) && (robot.pos_y < mid_y) {
            quads[1] += 1;
        } else if (robot.pos_x < mid_x) && (robot.pos_y > mid_y) {
            quads[2] += 1;
        } else if (robot.pos_x > mid_x) && (robot.pos_y > mid_y) {
            quads[3] += 1;
        }
    }

    quads.iter().product()
}

pub fn solve(input_str: &str, width: i64, height: i64) -> u64 {
    let content = fs::read_to_string(Path::new(input_str)).unwrap();
    let init_robots = content
        .lines()
        .map(|l| {
            let (str_x, str_y, str_v_x, str_v_y) = l
                .split(|y: char| !y.is_ascii_digit() && y != '-')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap();

            Robot {
                pos_x: str_x,
                pos_y: str_y,
                vel_x: str_v_x,
                vel_y: str_v_y,
            }
        })
        .collect_vec();
    let shift_robots = init_robots
        .into_iter()
        .map(|x| x.move_by_seconds(width, height, 100))
        .collect_vec();
    get_safety_ratio(&shift_robots, width, height)
}

#[cfg(test)]
mod day14_tests {
    use super::*;
    #[test]
    pub fn test_with_sample_data() {
        assert_eq!(solve("./inputs/day14_sample.txt", 11, 7), 12);
    }
    #[test]
    pub fn test_with_actual_data() {
        assert_eq!(solve("./inputs/day14.txt", 101, 103), 228690000);
    }
}
