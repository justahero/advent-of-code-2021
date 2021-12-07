use itertools::Itertools;

/// TODO replace `i32` with `u32`
fn find_horizontal_pos(positions: &[i32]) -> (i32, i32) {
    let (&min, &max) = positions.iter().minmax().into_option().unwrap();

    let mut min_pos = 0_i32;
    let mut min_fuel = i32::MAX;

    for pos in min..=max {
        println!("POS: {}", pos);

        // calculate the distances of each crab submarine to position
        let fuel = positions.iter().map(|crab| (crab - pos).abs()).sum::<i32>();
        if fuel < min_fuel {
            min_pos = pos;
            min_fuel = fuel;
        }
    }

    (min_pos, min_fuel)
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(str::trim)
        .filter_map(|val| val.parse::<i32>().ok())
        .collect_vec()
}

fn main() {
    let positions = parse_input(include_str!("input.txt"));
    let (pos, fuel) = find_horizontal_pos(&positions);

    dbg!(pos, fuel);
}

#[cfg(test)]
mod tests {
    use crate::find_horizontal_pos;

    const INPUT: [i32; 10] = [16,1,2,0,4,2,7,1,2,14];

    #[test]
    fn find_optimal_horizontal_pos() {
        assert_eq!((2, 37), find_horizontal_pos(&INPUT));
    }
}
