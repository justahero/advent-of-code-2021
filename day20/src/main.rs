use std::{collections::HashSet, fmt::Display};

use itertools::Itertools;

#[derive(Debug, Clone)]
struct Image {
    pub pixels: HashSet<(i32, i32)>,
    pub outside: u8,
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
}

impl Image {
    const PIXELS: [(i32, i32); 9] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (0, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    pub fn new(pixels: HashSet<(i32, i32)>, outside: u8) -> Self {
        let (min_x, max_x) = pixels.iter().map(|&(px, _)| px).minmax().into_option().unwrap();
        let (min_y, max_y) = pixels.iter().map(|&(_, py)| py).minmax().into_option().unwrap();

        Self {
            min_x,
            max_x,
            min_y,
            max_y,
            pixels,
            outside,
        }
    }

    /// Returns the calculated index of the 3x3 pixel matrix around the given coordinates.
    #[inline(always)]
    pub fn index(&self, x: i32, y: i32) -> u32 {
        let mut result = 0_u32;

        for (index, &(dx, dy)) in Self::PIXELS.iter().enumerate() {
            result |= (self.get(x + dx, y + dy) as u32) << (8 - index);
        }

        result
    }

    #[inline(always)]
    fn get(&self, x: i32, y: i32) -> u8 {
        if self.min_x <= x && x <= self.max_x && self.min_y <= y && y <= self.max_y {
            self.pixels.contains(&(x, y)) as u8
        } else {
            self.outside
        }
    }

    pub fn count_lit(&self) -> usize {
        self.pixels.len()
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (min_x, max_x) = self.pixels.iter().map(|&(px, _)| px).minmax().into_option().unwrap();
        let (min_y, max_y) = self.pixels.iter().map(|&(_, py)| py).minmax().into_option().unwrap();
        let pad = 3;

        for x in (min_x - pad)..=(max_x + pad) {
            for y in (min_y - pad)..=(max_y + pad) {
                let c = if self.pixels.get(&(x, y)).is_some() {
                    '#'
                } else {
                    '.'
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
struct ImageEnhancer {
    pub lookup: Vec<u8>,
}

impl ImageEnhancer {
    pub fn apply(&self, steps: usize, image: Image) -> Image {
        let mut image = image;
        for _ in 0..steps {
            image = self.enhance(&image);
        }
        image
    }

    fn enhance(&self, image: &Image) -> Image {
        let mut pixels = HashSet::new();
        let (min_x, max_x) = image.pixels.iter().map(|&(px, _)| px).minmax().into_option().unwrap();
        let (min_y, max_y) = image.pixels.iter().map(|&(_, py)| py).minmax().into_option().unwrap();
        let pad = 3;

        for x in (min_x - pad)..=(max_x + pad) {
            for y in (min_y - pad)..=(max_y + pad) {
                let index = image.index(x, y);
                if self.lookup[index as usize] == 1 {
                    pixels.insert((x, y));
                }
            }
        }

        let outside = ((image.outside == 0 && self.lookup[0] == 1)
            || (image.outside == 1 && self.lookup[511] == 1)) as u8;

        Image::new(pixels, outside)
    }
}

fn parse_input(input: &str) -> (ImageEnhancer, Image) {
    fn convert(c: char) -> u8 {
        if c == '#' {
            1
        } else {
            0
        }
    }

    let (algorithm, image) = input.split_once("\n\n").unwrap();
    let lookup = algorithm.chars().map(convert).collect_vec();
    assert_eq!(512, lookup.len());

    let lines = image.lines().collect_vec();
    let pixels = lines
        .iter()
        .enumerate()
        .fold(HashSet::new(), |mut result, (y, &line)| {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    result.insert((x as i32, y as i32));
                }
            }
            result
        });

    (ImageEnhancer { lookup }, Image::new(pixels, 0))
}

fn main() {
    let (enhancer, original_image) = parse_input(include_str!("input.txt"));

    let pixels = enhancer.apply(2, original_image.clone()).count_lit();
    dbg!(5081, pixels);

    let pixels = enhancer.apply(50, original_image).count_lit();
    dbg!(pixels);
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{parse_input, Image};

    #[test]
    fn test_get_image_index() {
        let pixels = [(0, 0), (0, 1), (1, 1)].into_iter().collect::<HashSet<_>>();
        let image = Image::new(pixels, 0);
        assert_eq!(0b000000000, image.index(-2, -2));
        assert_eq!(0b000000001, image.index(-1, -1));
        assert_eq!(0b000010011, image.index(0, 0));
        assert_eq!(0b000100110, image.index(1, 0));
        assert_eq!(0b010011000, image.index(0, 1));
        assert_eq!(0b100110000, image.index(1, 1));
    }

    #[test]
    fn test_parse_input() {
        let (enhancer, image) = parse_input(include_str!("example.txt"));
        assert_eq!(512, enhancer.lookup.len());
        assert_eq!(10, image.pixels.len());
    }

    #[test]
    fn test_count_lit_pixels() {
        let (enhancer, image) = parse_input(include_str!("example.txt"));
        let image = enhancer.apply(2, image);
        assert_eq!(35, image.count_lit());
    }

    #[test]
    fn test_example_2() {
        let (enhancer, image) = parse_input(include_str!("example2.txt"));
        let image = enhancer.apply(2, image);
        assert_eq!(5619, image.count_lit());
    }

    #[test]
    fn test_deep_enhance_example() {
        let (enhancer, image) = parse_input(include_str!("example.txt"));
        let image = enhancer.apply(50, image);
        assert_eq!(3351, image.count_lit());
    }
}
