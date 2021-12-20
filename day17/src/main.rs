use regex::Regex;

#[derive(Debug)]
struct Rect {
    pub top: i32,
    pub left: i32,
    pub bottom: i32,
    pub right: i32,
}

impl Rect {
    /// Returns true if the given coordinates are inside the Rect
    pub fn contains(&self, x: i32, y: i32) -> bool {
        self.left <= x && x <= self.right && self.bottom <= y && y <= self.top
    }
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
fn find_highest_y(input: &str) -> i32 {
    let rect = Rect::from(input);

    println!("RECT: {:?}", rect);
    let mut v = 0;

    let mut vel = 'outer: loop {
        let mut y = 0;
        let mut vel_y = v;
        println!("V: {}", v);
        'inner: loop {
            y += vel_y;
            vel_y -= 1;

            println!("  y: {}, vel: {}", y, vel_y);
            if rect.bottom <= y && y <= rect.top {
                break 'inner;
            } else if y < rect.bottom {
                break 'outer v - 1;
            }
        }

        v += 1;
    };

    println!("RECT: {:?}", rect);
    let mut highest_y = 0;
    let mut y = 0;
    loop {
        y += vel;
        highest_y = std::cmp::max(y, highest_y);
        vel -= 1;
        if vel < 0 {
            break;
        }
    }
    highest_y
}

fn main() {
    let input = "target area: x=60..94, y=-171..-136";

    let y = find_highest_y(input);
    dbg!(y);
}

#[cfg(test)]
mod tests {
    use crate::find_highest_y;

    #[test]
    fn test_find_highest_y() {
        let input = "target area: x=20..30, y=-10..-5";
        assert_eq!(45, find_highest_y(input));
    }
}
