use glm::{Mat3, mat3, GenMat};

/// Builds a translation matrix `glm` matrix.
pub fn translate(x:f32, y:f32) -> Mat3 {
    mat3(
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
          x,   y, 1.0
    )
}

/// Builds a scaling `glm` matrix.
pub fn scale(sx:f32, sy:f32) -> Mat3 {
    mat3(
         sx, 0.0, 0.0,
        0.0,  sy, 0.0,
        0.0, 0.0, 1.0
    )
}

/// Type-coercion for `glm` to `luminance` matrix representation.
pub fn to_raw(mut mat:Mat3) -> [[f32; 3]; 3] {
    mat = mat.transpose();
    unsafe {
        *((&mat as * const Mat3) as * const [[f32; 3]; 3]).as_ref().unwrap()
    }
}
