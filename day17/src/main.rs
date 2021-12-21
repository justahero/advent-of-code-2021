use std::collections::HashSet;

use regex::Regex;

#[derive(Debug)]
struct Rect {
    pub top: i32,
    pub left: i32,
    pub bottom: i32,
    pub right: i32,
}

impl From<&str> for Rect {
    fn from(line: &str) -> Self {
        let regex = Regex::new(r"^target area: x=(-?\d*)..(-?\d*), y=(-?\d*)..(-?\d*)$").unwrap();
        let cap = regex.captures(line).unwrap();

        let left: i32 = cap[1].parse().unwrap();
        let right: i32 = cap[2].parse().unwrap();
        let top: i32 = cap[4].parse().unwrap();
        let bottom: i32 = cap[3].parse().unwrap();

        Rect {
            top,
            left,
            bottom,
            right,
        }
    }
}

/// Finds the highest possible y height value
/// Calculates the highest y mark directly
fn find_highest_y(input: &str) -> i32 {
    let rect = Rect::from(input);
    let vel = (rect.bottom + 1).abs();
    (vel * (vel + 1)) / 2
}

fn hits_target(vel_x: i32, vel_y: i32, rect: &Rect) -> bool {
    let mut vel_x = vel_x;
    let mut vel_y = vel_y;
    let mut x = 0;
    let mut y = 0;

    loop {
        x += vel_x;
        y += vel_y;

        if vel_x < 0 {
            vel_x += 1;
        } else if vel_x > 0 {
            vel_x -= 1;
        }

        vel_y -= 1;

        if rect.left <= x && x <= rect.right && rect.bottom <= y && y <= rect.top {
            return true;
        }

        if x > rect.right || y < rect.bottom {
            break;
        }
    }
    false
}

/// Finds all velocities that hit the target area
fn find_all_velocities(input: &str) -> i32 {
    let rect = Rect::from(input);
    println!("find_all_velocities: {:?}", rect);

    // get all possible x values, some small optimisation
    let mut result: HashSet<i32> = HashSet::new();
    for vel in 0..=rect.right {
        let mut vel_x = vel;
        let mut x = 0;
        loop {
            x += vel_x;
            vel_x = std::cmp::max(0, vel_x - 1);

            if rect.left <= x && x <= rect.right {
                result.insert(vel);
            }

            if vel_x == 0 {
                break;
            }
        }
    }

    // get all possible velocities
    let mut solutions = 0;
    for vel_x in result.into_iter() {
        let max_vel = (rect.bottom + 1).abs();
        for vel_y in rect.bottom..=max_vel {
            if hits_target(vel_x, vel_y, &rect) {
                solutions += 1;
            }
        }
    }

    solutions
}

fn main() {
    let input = "target area: x=60..94, y=-171..-136";

    let y = find_highest_y(input);
    dbg!(y);

    let count = find_all_velocities(input);
    dbg!(count);
}

#[cfg(test)]
mod tests {
    use crate::{find_all_velocities, find_highest_y};

    #[test]
    fn test_find_highest_y() {
        let input = "target area: x=20..30, y=-10..-5";
        assert_eq!(45, find_highest_y(input));
    }

    #[test]
    fn test_find_all_initial_velocities() {
        let input = "target area: x=20..30, y=-10..-5";
        assert_eq!(112, find_all_velocities(input));
    }
}
