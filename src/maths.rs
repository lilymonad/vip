pub fn mul_m3(m1:&[[f32;3];3], m2:&[[f32;3];3]) -> [[f32;3];3] {
    let mut ret = [[0.0;3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                ret[i][j] += m1[i][k] * m2[k][j];
            }
        }
    }

    ret
}

pub fn translate(x:f32, y:f32) -> [[f32;3];3] {
    [
        [1.0, 0.0,   x],
        [0.0, 1.0,   y],
        [0.0, 0.0, 1.0],
    ]
}

pub fn scale(sx:f32, sy:f32) -> [[f32;3];3] {
    [
        [ sx, 0.0, 0.0],
        [0.0,  sy, 0.0],
        [0.0, 0.0, 1.0],
    ]
}


