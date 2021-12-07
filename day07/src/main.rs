use itertools::Itertools;

fn linear_fuel(crab: u32, pos: u32) -> u32 {
    (crab as i32 - pos as i32).abs() as u32
}

fn expensive_fuel(crab: u32, pos: u32) -> u32 {
    let diff = (crab as i32 - pos as i32).abs() as u32;
    ((diff + 1) * diff) / 2
}

fn find_optimal_pos<F>(positions: &[u32], fuel_fn: F) -> (u32, u32)
where
    F: Fn(u32, u32) -> u32,
{
    let (&min, &max) = positions.iter().minmax().into_option().unwrap();
    (min..=max)
        .map(|pos| {
            let fuel = positions
                .iter()
                .map(|&crab| fuel_fn(crab, pos))
                .sum::<u32>();
            (pos, fuel)
        })
        .min_by_key(|(_, fuel)| *fuel)
        .unwrap()
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

    let (pos, fuel) = find_optimal_pos(&positions, linear_fuel);
    dbg!(pos, fuel);

    let (pos, fuel) = find_optimal_pos(&positions, expensive_fuel);
    dbg!(pos, fuel);
}

#[cfg(test)]
mod tests {
    use crate::{expensive_fuel, find_optimal_pos, linear_fuel};

    const INPUT: [u32; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    #[test]
    fn test_find_linear_pos() {
        assert_eq!((2, 37), find_optimal_pos(&INPUT, linear_fuel));
    }

    #[test]
    fn test_find_pos_using_expensive_move() {
        assert_eq!((5, 168), find_optimal_pos(&INPUT, expensive_fuel));
    }
}
