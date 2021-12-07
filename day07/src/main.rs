use itertools::Itertools;

fn find_linear_pos(positions: &[u32]) -> (u32, u32) {
    let (&min, &max) = positions.iter().minmax().into_option().unwrap();

    let mut min_pos = 0_u32;
    let mut min_fuel = u32::MAX;

    for pos in min..=max {
        // calculate the distances of each crab submarine to position
        let fuel = positions.iter().map(|&crab| (crab as i32 - pos as i32).abs() as u32).sum::<u32>();
        if fuel < min_fuel {
            min_pos = pos;
            min_fuel = fuel;
        }
    }

    (min_pos, min_fuel)
}

fn find_expensive_pos(positions: &[u32]) -> (u32, u32) {
    let (&min, &max) = positions.iter().minmax().into_option().unwrap();

    let mut min_pos = 0_u32;
    let mut min_fuel = u32::MAX;

    for pos in min..=max {
        let fuel = positions
            .iter()
            .map(|&crab| {
                let diff = (crab as i32 - pos as i32).abs() as u32;
                ((diff + 1) * diff) / 2
            })
            .sum::<u32>();

        if fuel < min_fuel {
            min_pos = pos;
            min_fuel = fuel;
        }
    }

    (min_pos, min_fuel)
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .split(',')
        .map(str::trim)
        .filter_map(|val| val.parse::<u32>().ok())
        .collect_vec()
}

fn main() {
    let positions = parse_input(include_str!("input.txt"));

    let (pos, fuel) = find_linear_pos(&positions);
    dbg!(pos, fuel);

    let (pos, fuel) = find_expensive_pos(&positions);
    dbg!(pos, fuel);
}

#[cfg(test)]
mod tests {
    use crate::{find_expensive_pos, find_linear_pos};

    const INPUT: [u32; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    #[test]
    fn test_find_linear_pos() {
        assert_eq!((2, 37), find_linear_pos(&INPUT));
    }

    #[test]
    fn test_find_pos_using_expensive_move() {
        assert_eq!((5, 168), find_expensive_pos(&INPUT));
    }
}
