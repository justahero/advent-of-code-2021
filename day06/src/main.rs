struct Population {
    pub list: [u64; 9],
}

impl Population {
    pub fn new() -> Self {
        Self { list: [0; 9] }
    }

    /// Inserts a new fish with the number of days until a new fish is creted
    pub fn insert_fish(&mut self, fish: u64) {
        self.list[fish as usize] += 1;
    }

    /// Advances the population by `n` days
    ///
    /// * the timer of all fish decreases by 1
    /// * fish in bucket [0] reset to [6], also create a new fish with internal time of [8]
    pub fn advance(&self, days: u64) -> u64 {
        let mut buckets = self.list.clone();
        for _ in 0..days {
            buckets.rotate_left(1);
            buckets[6] += buckets[8];
        }
        buckets.iter().sum()
    }
}

/// Parses the input
fn parse_input(input: &str) -> Population {
    input
        .split(',')
        .map(str::trim)
        .filter_map(|val| val.parse::<u64>().ok())
        .fold(Population::new(), |mut population, fish| {
            population.insert_fish(fish);
            population
        })
}

fn main() {
    let population = parse_input(include_str!("input.txt"));
    dbg!(population.advance(80));
    dbg!(population.advance(256));
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn first_part_count_population() {
        let population = parse_input(INPUT);
        assert_eq!(26, population.advance(18));
        assert_eq!(5934, population.advance(80));
    }

    #[test]
    fn second_part_count_population() {
        let population = parse_input(INPUT);
        assert_eq!(26984457539, population.advance(256));
    }
}
