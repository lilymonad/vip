use luminance_derive::{Semantics, Vertex, UniformInterface};
use luminance::{
    texture::{Texture, GenMipmaps, Dim2, Sampler},
    pixel::{NormUnsigned, NormRGBA8UI},
    context::GraphicsContext,
    pipeline::BoundTexture,
    shader::program::Uniform,
    linear::M33,
};
use std::collections::HashSet;
use crate::canvas::Canvas;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Semantics)]
pub enum Semantics {
    #[sem(name="pos", repr="[f32;2]", wrapper="SelPos")]
    Position,
    #[sem(name="texPos", repr="[f32;2]", wrapper="SelTexPos")]
    Tex,
    #[sem(name="onColor", repr="[u8;3]", wrapper="SelOnColor")]
    Color,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Vertex)]
#[vertex(sem = "Semantics")]
pub struct Vertex {
    pos: SelPos,
    texPos: SelTexPos,
    #[vertex(normalized="true")]
    onColor: SelOnColor,
}

#[derive(UniformInterface)]
pub struct ShaderInterface {
    #[uniform]
    tex: Uniform<& 'static BoundTexture<'static, Dim2, NormUnsigned>>,
    #[uniform]
    view: Uniform<M33>,
}

pub fn vertice_from_selection(selection:&HashSet<(usize,usize)>, canvas:&Canvas) -> Vec<Vertex> {
    let mut ret = Vec::new();
    for (x, y) in selection {
        let (ix, iy) = (*x as isize, *y as isize);

        let (tx, ty) = 
        [   ( 0, -1, (0, 2)),
            ( 1, -1, (0, 0)),
            ( 1,  0, (2, 0)),
            ( 1,  1, (0, 0)),
            ( 0,  1, (0, 1)),
            (-1,  1, (0, 0)),
            (-1,  0, (1, 0)),
            (-1, -1, (0, 0)) ]
                .iter()
                .map(|(dx, dy, weight)| {
                    let pt = (ix.wrapping_add(*dx) as usize, iy.wrapping_add(*dy) as usize);
                    if !selection.contains(&pt) {
                        *weight
                    } else {
                        (0, 0)
                    }
                })
                .fold((0, 0), |(x1,y1), (x2,y2)| (x1+x2, y1+y2));

        let cs = 64.0;
        let ats = 256.0;

        let ts = cs / ats;
        let (tx, ty) = (cs*(tx as f32 )/ ats, cs*(ty as f32) / ats);

        let (r, g, b) = canvas.get_pixel_color(*x, *y);
        let scol = [r, g, b];

        let (px, py) = (*x as f32, *y as f32);
        ret.extend_from_slice(&[
            Vertex {
                pos: SelPos::new([px, py]),
                texPos: SelTexPos::new([tx, ty]),
                onColor: SelOnColor::new(scol.clone()),
            },
            Vertex {
                pos: SelPos::new([(px + 1.0), py]),
                texPos: SelTexPos::new([tx + ts, ty]),
                onColor: SelOnColor::new(scol.clone()),
            },
            Vertex {
                pos: SelPos::new([(px + 1.0), (py + 1.0)]),
                texPos: SelTexPos::new([tx + ts, ty + ts]),
                onColor: SelOnColor::new(scol.clone()),
            },
            Vertex {
                pos: SelPos::new([(px + 1.0), (py + 1.0)]),
                texPos: SelTexPos::new([tx + ts, ty + ts]),
                onColor: SelOnColor::new(scol.clone()),
            },
            Vertex {
                pos: SelPos::new([px, (py + 1.0)]),
                texPos: SelTexPos::new([tx, ty + ts]),
                onColor: SelOnColor::new(scol.clone()),
            },
            Vertex {
                pos: SelPos::new([px, py]),
                texPos: SelTexPos::new([tx, ty]),
                onColor: SelOnColor::new(scol.clone()),
            },
        ]);
    }

    ret
}
