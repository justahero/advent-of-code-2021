use itertools::Itertools;

#[derive(Debug)]
struct Image {
    pub pixels: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl Image {
    pub fn new(width: usize, height: usize, pixels: Vec<u8>) -> Self {
        Self { width, height, pixels }
    }
}

#[derive(Debug)]
struct ImageEnhancer {
    pub lookup: Vec<u8>,
    pub image: Image,
}

fn parse_input(input: &str) -> ImageEnhancer {
    fn convert(c: char) -> u8 {
        if c == '#' {
            1
        } else {
            0
        }
    }

    let (algorithm, image) = input.split_once("\n\n").unwrap();
    let lookup = algorithm
        .chars()
        .map(convert)
        .collect_vec();

    let lines = image.lines().collect_vec();
    let height = lines.len();
    let width = lines[0].len();

    let image = lines
        .iter()
        .flat_map(|&line| line.chars().map(convert))
        .collect_vec();

    ImageEnhancer {
        lookup,
        image: Image::new(width, height, image),
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    #[test]
    fn test_parse_input() {
        let enhancer = parse_input(include_str!("example.txt"));
        assert_eq!(512, enhancer.lookup.len());
        assert_eq!(5, enhancer.width);
        assert_eq!(5, enhancer.height);
        assert_eq!(25, enhancer.image.len());
    }
}
