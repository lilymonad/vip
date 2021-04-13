mod shader;

pub use shader::*;

/// This structure represent a VIPix canvas:
/// - Its size in pixels (Width, Height).
/// - Its data (a big array of Width x Height pixels).
pub struct Canvas {
    pub size : (usize, usize),
    pub data : Vec<(u8, u8, u8, u8)>,
}

impl Canvas {
    pub fn new(x:usize, y:usize) -> Self {
        Self {
            size: (x, y),
            data: vec![(0, 0, 0, 255); x * y],
        }
    }

    pub fn set_pixel_color(&mut self, x:usize, y:usize, rgba:(u8, u8, u8, u8)) {
        let (w, h) = self.size;
        let id = y * w + x;

        assert!(id < w*h);

        self.data[id] = rgba;
    }

    pub fn get_pixel_color(&self, x:usize, y:usize) -> (u8, u8, u8, u8) {
        let (w, h) = self.size;
        let id = y * w + x;

        assert!(id < w*h);

        self.data[id]
    }

    pub fn size(&self) -> (usize, usize) {
        self.size
    }

    pub fn data_raw(&self) -> &[u8] {
        unsafe {
            let slice : &[(u8, u8, u8, u8)] = self.data.as_ref();
            let ptr = slice.as_ptr() as *const u8;
            core::slice::from_raw_parts(ptr, slice.len() * 4)
        }
    }

    pub fn width(&self) -> usize {
        self.size.0
    }

    pub fn height(&self) -> usize {
        self.size.1
    }
}

impl AsRef<[(u8, u8, u8, u8)]> for Canvas {
    fn as_ref(&self) -> &[(u8, u8, u8, u8)] {
        &self.data
    }
}

impl std::ops::Deref for Canvas {
    type Target = [(u8, u8, u8, u8)];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
