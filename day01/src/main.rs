use itertools::Itertools;

fn parse(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(str::parse::<i32>)
        .filter_map(Result::ok)
        .collect()
}

pub fn count_single(depths: &[i32]) -> usize {
    depths
        .iter()
        .tuple_windows()
        .into_iter()
        .filter(|(x, y)| x < y)
        .count()
}

fn main() {
    let depths = parse(include_str!("input.txt"));
    let count = count_single(&depths);
    dbg!(count);
}

#[cfg(test)]
mod tests {
    use crate::count_single;

    #[test]
    fn test_count_increases() {
        let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(7, count_single(&depths));
    }
}
