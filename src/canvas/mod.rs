mod shader;

pub use shader::{Semantics, Vertex, ShaderInterface, VertexPosition, TexPosition, VertexColor};

/// This structure represent a VIPix canvas:
/// - Its size in pixels (Width, Height).
/// - Its data (a big array of Width x Height pixels).
pub struct Canvas {
    size : (usize, usize),
    data : Vec<(u8, u8, u8)>,
}

impl Canvas {
    pub fn new(x:usize, y:usize) -> Self {
        Self {
            size: (x, y),
            data: vec![(0, 0, 0); x * y],
        }
    }

    pub fn set_pixel_color(&mut self, x:usize, y:usize, rgb:(u8, u8, u8)) {
        let (w, h) = self.size;
        let id = y * w + x;

        assert!(id < w*h);

        self.data[y * w + x] = rgb;
    }

    pub fn get_pixel_color(&self, x:usize, y:usize) -> (u8, u8, u8) {
        let (w, h) = self.size;
        let id = y * w + x;

        assert!(id < w*h);

        self.data[y * w + x]
    }

    pub fn size(&self) -> (usize, usize) {
        self.size
    }
}

impl AsRef<[(u8, u8, u8)]> for Canvas {
    fn as_ref(&self) -> &[(u8, u8, u8)] {
        &self.data
    }
}

impl std::ops::Deref for Canvas {
    type Target = [(u8, u8, u8)];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
