use luminance_derive::{Semantics, Vertex};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Semantics)]
pub enum Semantics {
    #[sem(name="pos", repr="[f32;2]", wrapper="VertexPosition")]
    Position,
    #[sem(name="color", repr="[f32;3]", wrapper="Color")]
    Color,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Vertex)]
#[vertex(sem = "Semantics")]
pub struct Vertex {
    pub pos: VertexPosition,
    pub color: Color,
}

pub fn render_background((w, h): (f32, f32)) -> Vec<Vertex> {
    const pwstep : f32 = 32.0;
    const phstep : f32 = 32.0;

    let wdiv : usize = ((w + (pwstep * 0.5)) / pwstep).ceil() as usize;
    let hdiv : usize = ((h + (phstep * 0.5)) / phstep).ceil() as usize;

    let wstep : f32 = 2.0 / wdiv as f32;
    let hstep : f32 = 2.0 / hdiv as f32;

    let mut ret = Vec::with_capacity(wdiv * hdiv * 6);

    for i in 0..wdiv {
        for j in 0..hdiv {
            let (x, y) = (i as f32, j as f32);
            let (top, left, bottom, right) = (
                -1.0 + y * hstep,
                -1.0 + x * wstep,
                -1.0 + (y + 1.0) * hstep,
                -1.0 + (x + 1.0) * wstep,
            );

            let (first, second) = if j % 2 == 0 {
                (Color::new([0.5,0.5,0.5]), Color::new([0.8,0.8,0.8]))
            } else {
                (Color::new([0.8,0.8,0.8]), Color::new([0.5,0.5,0.5]))
            };

            // start with a left-corner triangle
            let arr = if (i+j) % 2 == 0 {
                [
                    Vertex { pos:VertexPosition::new([left,bottom]), color:first },
                    Vertex { pos:VertexPosition::new([left,top]), color:first },
                    Vertex { pos:VertexPosition::new([right,top]), color:first },

                    Vertex { pos:VertexPosition::new([right,top]), color:second },
                    Vertex { pos:VertexPosition::new([right,bottom]), color:second },
                    Vertex { pos:VertexPosition::new([left,bottom]), color:second },
                ]
            // start with a right-corner triangle
            } else {
                [
                    Vertex { pos:VertexPosition::new([left,top]), color:second },
                    Vertex { pos:VertexPosition::new([left,bottom]), color:second },
                    Vertex { pos:VertexPosition::new([right,bottom]), color:second },

                    Vertex { pos:VertexPosition::new([right,bottom]), color:first },
                    Vertex { pos:VertexPosition::new([right,top]), color:first },
                    Vertex { pos:VertexPosition::new([left,top]), color:first },
                ]
            };

            ret.extend_from_slice(&arr);
        }
    }

    ret
}
