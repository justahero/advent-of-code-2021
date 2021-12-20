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
fn find_highest_y(input: &str) -> i32 {
    let rect = Rect::from(input);
    println!("RECT: {:?}", rect);

    // Try to find direct calculation

    // Note it's possible to find a start velocity that misses the target, but a higher velocity might fit it again

    let mut start_v = 0;
    let vel = 'outer: loop {
        let mut y = 0;
        let mut vel_y = start_v;

        println!("Round vel: {}", vel_y);

        'inner: loop {
            y += vel_y;
            vel_y -= 1;

            // println!("  y: {}, vel: {}, top: {}, bottom: {}", y, vel_y, rect.top, rect.bottom);

            if rect.bottom <= y && y <= rect.top {
                // hit the target, exit the loop
                break 'inner;
            } else if y < rect.bottom {
                // we missed the target height, use the previous velocity
                break 'outer start_v - 1;
            }
        }

        // hit the mark, increase the velocity
        start_v += 1;
    };

    // calculate the highest y mark directly
    (vel * (vel + 1)) / 2
}

fn main() {
    let input = "target area: x=60..94, y=-171..-136";

    let y = find_highest_y(input);
    dbg!(y);
    // 820 is too low
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
