use glm::{Mat3, mat3};

/// Create a 2D normalized matrix (so
pub fn translate(x:f32, y:f32) -> Mat3 {
    mat3(
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
          x,   y, 1.0
    )
}

pub fn scale(sx:f32, sy:f32) -> Mat3 {
    mat3(
         sx, 0.0, 0.0,
        0.0,  sy, 0.0,
        0.0, 0.0, 1.0
    )
}

pub fn to_raw(mat:&Mat3) -> &[[f32; 3]; 3] {
    unsafe {
        ((mat as * const Mat3) as * const [[f32; 3]; 3]).as_ref().unwrap()
    }
}
