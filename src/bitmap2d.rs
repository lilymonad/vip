use std::collections::HashSet;
use image::{
    GrayImage,
    Luma,
};

/// This trait is mainly used to generalize selection masks when doing masked operations on image.
pub trait BitMap2D {
    fn set_bit(&mut self, x:usize, y:usize);
    fn clear_bit(&mut self, x:usize, y:usize);
    fn get_bit(&self, x:usize, y:usize) -> bool;
}

impl BitMap2D for HashSet<(usize, usize)> {
    fn set_bit(&mut self, x:usize, y:usize) {
        self.insert((x, y));
    }

    fn clear_bit(&mut self, x:usize, y:usize) {
        self.remove(&(x, y));
    }

    fn get_bit(&self, x:usize, y:usize) -> bool {
        self.contains(&(x, y))
    }
}

impl BitMap2D for GrayImage {
    fn set_bit(&mut self, x:usize, y:usize) {
        self.put_pixel(x as u32, y as u32, Luma([1u8]));
    }

    fn clear_bit(&mut self, x:usize, y:usize) {
        self.put_pixel(x as u32, y as u32, Luma([0u8]));
    }

    fn get_bit(&self, x:usize, y:usize) -> bool {
        self.get_pixel(x as u32, y as u32)[0] == 1u8
    }
}
