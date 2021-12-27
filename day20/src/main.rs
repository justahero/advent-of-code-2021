use std::{fmt::Display, ops::Shl};

use itertools::Itertools;

#[derive(Debug, Clone)]
struct Image {
    pub pixels: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl Image {
    const PIXELS: [(i32, i32); 9] = [(1, 1), (0, 1), (-1, 1), (1, 0), (0, 0), (-1, 0), (1, -1), (0, -1), (-1, -1)];

    pub fn new(width: usize, height: usize, pixels: Vec<u8>) -> Self {
        Self { width, height, pixels }
    }

    /// Returns the calculated index of the 3x3 pixel matrix around the given coordinates.
    pub fn index(&self, x: i32, y: i32) -> u32 {
        let mut result = 0;

        for (index, &(px, py)) in Self::PIXELS.iter().enumerate() {
            if self.get(x + px, y + py) == 1 {
                result |= 1_u32.shl(index);
            }
        }

        result
    }

    #[inline(always)]
    fn get(&self, x: i32, y: i32) -> u8 {
        if 0 <= x && x < self.width as i32 && 0 <= y && y < self.height as i32 {
            self.pixels[(y * self.width as i32 + x) as usize]
        } else {
            0
        }
    }

    pub fn count_lit(&self) -> usize {
        self.pixels.iter().filter(|&p| *p == 1).count()
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.pixels.chunks(self.width) {
            let pixels = line.iter().map(|&p| if p == 1 { '#' } else { '.' }).join("");
            writeln!(f, "{}", pixels)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
struct ImageEnhancer {
    pub lookup: Vec<u8>,
    pub image: Image,
}

impl ImageEnhancer {
    pub fn apply(&self, steps: usize) -> Image {
        (0..steps).fold(self.image.clone(), |result, _| self.enhance(&result))
    }

    fn enhance(&self, image: &Image) -> Image {
        let width = image.width + 2;
        let height = image.height + 2;
        
        let mut pixels = vec![0_u8; width * height];

        for y in 0..height as i32 {
            for x in 0..width as i32 {
                let index = image.index(x - 1, y - 1) as usize;
                pixels[(y * width as i32 + x) as usize] = self.lookup[index];
            }
        }

        Image {
            width,
            height,
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
    assert_eq!(512, lookup.len());

    let lines = image.lines().collect_vec();
    let height = lines.len();
    let width = lines[0].len();
    println!("IMAGE: {}, {}", width, height);

    let pixels = lines
        .iter()
        .flat_map(|&line| line.chars().map(convert))
        .collect_vec();

    ImageEnhancer {
        lookup,
        image: Image::new(width, height, pixels),
    }
}

fn main() {
    let enhancer = parse_input(include_str!("input.txt"));

    let image = enhancer.apply(2);
    dbg!(image.count_lit());
    // 5278 too high, 4814 too low
}

#[cfg(test)]
mod tests {
    use crate::{Image, parse_input};

    #[test]
    fn test_get_image_index() {
        // TODO increase width / height, test more cases
        let image = Image::new(2, 2, vec![1, 0, 1, 1]);
        assert_eq!(0b000000000, image.index(-2, -2));
        assert_eq!(0b000000001, image.index(-1, -1));
        assert_eq!(0b000010011, image.index(0, 0));
        assert_eq!(0b000100110, image.index(1, 0));
        assert_eq!(0b010011000, image.index(0, 1));
        assert_eq!(0b100110000, image.index(1, 1));
    }

    #[test]
    fn test_parse_input() {
        let enhancer = parse_input(include_str!("example.txt"));
        assert_eq!(512, enhancer.lookup.len());
        assert_eq!(5, enhancer.image.width);
        assert_eq!(5, enhancer.image.height);
        assert_eq!(25, enhancer.image.pixels.len());
    }

    #[test]
    fn test_count_lit_pixels() {
        let enhancer = parse_input(include_str!("example.txt"));
        let image = enhancer.apply(2);
        assert_eq!(35, image.count_lit());
    }
}
