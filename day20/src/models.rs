use std::{collections::HashSet, hash::Hash, hash::Hasher};

#[derive(Debug)]
pub struct Data {
    pub algo: [bool; 512],
    pub lit_pixels: HashSet<Pixel>,
}

#[derive(Debug)]
pub struct Pixel {
    pub x: i16,
    pub y: i16,
}

impl PartialEq for Pixel {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Pixel {}

impl Hash for Pixel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

#[derive(Debug)]
pub struct Range {
    pub min_pixel: Pixel,
    pub max_pixel: Pixel,
}

impl Range {
    pub fn in_range(&self, pixel: &Pixel) -> bool {
        // X
        pixel.x >= self.min_pixel.x && pixel.x <= self.max_pixel.x 
        // Y
        && pixel.y >= self.min_pixel.y && pixel.y <= self.max_pixel.y
    }
}
