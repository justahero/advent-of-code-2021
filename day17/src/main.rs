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

/// Calculates the parabola of the start y velocity, if it hits the target area return true, otherwise false
fn parabola(start_velocity: i32, rect: &Rect) -> bool {
    let mut y = 0;
    let mut vel = start_velocity;
    loop {
        y += vel;
        vel -= 1;
        println!("  y: {}, vel: {}", y, vel);

        if rect.bottom <= y && y <= rect.top {
            return true;
        } else if y < rect.bottom {
            break;
        }
    }
    false
}

/// Finds the highest possible y height value
fn find_highest_y(input: &str) -> i32 {
    let rect = Rect::from(input);
    println!("RECT: {:?}", rect);

    // calculate the highest y mark directly
    let vel = (rect.bottom + 1).abs();
    (vel * (vel + 1)) / 2
}

/// Finds all velocities that hit the target area
fn find_all_velocities(input: &str) -> i32 {
    let rect = Rect::from(input);
    println!("RECT: {:?}", rect);

    0
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
