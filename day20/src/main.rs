use std::ops::{Index, Neg, Shl};

use itertools::Itertools;

#[derive(Debug)]
struct Image {
    pub pixels: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl Image {
    const PIXELS: [(i32, i32); 9] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (0, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];
    
    pub fn new(width: usize, height: usize, pixels: Vec<u8>) -> Self {
        Self { width, height, pixels }
    }

    /// Returns the calculated index of the 3x3 pixel matrix around the given coordinates.
    pub fn index(&self, x: i32, y: i32) -> u32 {
        let mut result = 0;

        for (index, &(px, py)) in Self::PIXELS.iter().enumerate() {
            let (px, py) = (px + x, py + y);
            if 0 <= px && px < self.width as i32 && 0 <= py && py < self.height as i32 {
                if self.pixels[(py * self.width as i32 + px) as usize] == 1 {
                    result |= 1_u32.shl(8 - index);
                }
            }
        }

        result
    }
}

#[derive(Debug)]
struct ImageEnhancer {
    pub lookup: Vec<u8>,
    pub image: Image,
}

impl ImageEnhancer {
    pub fn apply(&self) -> Image {
        let mut pixels = Vec::new();

        for y in -1..self.image.height as i32 + 1 {
            for x in -1..self.image.width as i32 + 1 {
                let binary = self.image.index(x, y);
            }
        }

        Image {
            width: self.image.width + 2,
            height: self.image.height + 2,
            pixels,
        }
    }
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
    use crate::{Image, parse_input};

    #[test]
    fn test_get_image_index() {
        let image = Image::new(2, 2, vec![1, 0, 0, 1]);
        assert_eq!(0b000000000, image.index(-2, -2));
        assert_eq!(0b000000001, image.index(-1, -1));
        assert_eq!(0b000010001, image.index(0, 0));
        assert_eq!(0b010001000, image.index(0, 1));
    }

    #[test]
    fn test_parse_input() {
        let enhancer = parse_input(include_str!("example.txt"));
        assert_eq!(512, enhancer.lookup.len());
        assert_eq!(5, enhancer.image.width);
        assert_eq!(5, enhancer.image.height);
        assert_eq!(25, enhancer.image.pixels.len());
    }
}
