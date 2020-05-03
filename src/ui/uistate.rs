use std::collections::{HashMap, HashSet};
use crate::{
    canvas::{self, Canvas},
    ui::selection as sel,
    keyboard::CharKeyMod,
    bitmap2d::BitMap2D,
};

pub struct UiState {
    pub palette:HashMap<CharKeyMod, (u8, u8, u8)>,
    pub must_resize:bool,
    pub scale:(f32, f32),
    pub zoom:f32,
    pub center:(f32, f32),
    pub canvas:Canvas,
    pub visual_type:VisualType,
    pub window_size:(f32, f32),
    pub selection:HashSet<(usize, usize)>,
    pub chunk_size:(usize, usize),
    pub exploded:bool,
}

impl UiState {
    pub fn render_canvas(&self) -> Vec<canvas::Vertex> {
        use canvas::*;
        let (icw, ich) = self.canvas.size();
        let (chunkw, chunkh) = self.chunk_size;
        let (nbcw, nbch) = (icw / chunkw, ich / chunkh);
        let (cw, ch) = (icw as f32, ich as f32);
        if self.exploded {
            let mut ret = Vec::new();
            for i in 0..nbcw {
                for j in 0..nbch {
                    let (gx, gy) = (i as f32 * 1.0, j as f32 * 1.0);
                    let (bx, by) = ((i * chunkw) as f32, (j * chunkh) as f32);
                    let (ex, ey) = ((bx + chunkw as f32).min(cw), (by + chunkh as f32).min(ch));

                    let (tbx, tby) = (bx / cw, by / ch);
                    let (tex, tey) = (ex / cw, ey / ch);

                    ret.append(&mut vec![
                        Vertex { pos:VertexPosition::new([bx + gx,by + gy]), texPos:TexPosition::new([tbx,tby]) },
                        Vertex { pos:VertexPosition::new([ex + gx,by + gy]), texPos:TexPosition::new([tex,tby]) },
                        Vertex { pos:VertexPosition::new([ex + gx,ey + gy]), texPos:TexPosition::new([tex,tey]) },
                        Vertex { pos:VertexPosition::new([ex + gx,ey + gy]), texPos:TexPosition::new([tex,tey]) },
                        Vertex { pos:VertexPosition::new([bx + gx,ey + gy]), texPos:TexPosition::new([tbx,tey]) },
                        Vertex { pos:VertexPosition::new([bx + gx,by + gy]), texPos:TexPosition::new([tbx,tby]) },
                    ]);
                }
            }

            ret
        } else {
            vec![
                Vertex { pos:VertexPosition::new([0.0,0.0]), texPos:TexPosition::new([0.0,0.0]) },
                Vertex { pos:VertexPosition::new([cw,0.0]), texPos:TexPosition::new([1.0,0.0]) },
                Vertex { pos:VertexPosition::new([cw,ch]), texPos:TexPosition::new([1.0,1.0]) },
                Vertex { pos:VertexPosition::new([cw,ch]), texPos:TexPosition::new([1.0,1.0]) },
                Vertex { pos:VertexPosition::new([0.0,ch]), texPos:TexPosition::new([0.0,1.0]) },
                Vertex { pos:VertexPosition::new([0.0,0.0]), texPos:TexPosition::new([0.0,0.0]) },
            ]
        }
    }

    pub fn render_selection(&self, selection:&HashSet<(usize, usize)>) -> Vec<sel::Vertex> {
        use sel::*;
        let mut ret = Vec::new();
        for (mut x, mut y) in selection.iter().copied() {
            let (ix, iy) = (x as isize, y as isize);

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

            let (r, g, b) = self.canvas.get_pixel_color(x, y);
            let scol = [r, g, b];


            if self.exploded {
                x += x / self.chunk_size.0;
                y += y / self.chunk_size.1;
            }

            let (px, py) = (x as f32, y as f32);

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
}

pub enum VisualType {
    Square,
    Circle,
}

impl VisualType {
    pub fn select_pixels<T:BitMap2D>(&self, set:&mut T, (x1, y1):(usize, usize), (x2, y2):(usize, usize)) {
        match self {
            VisualType::Square => {
                (x1..x2+1)
                    .into_iter()
                    .flat_map(|x| {
                        (y1..y2+1)
                            .into_iter()
                            .map(move |y| (x, y))
                    })
                    .for_each(|(x, y)| { set.set_bit(x, y); });
            },
            VisualType::Circle => {
                let (mx, my) = (((x1+x2) / 2) as isize, ((y1+y2) / 2) as isize);
                let w = (x2 - x1) as f32 / 2.0;
                (x1..x2+1)
                    .into_iter()
                    .flat_map(|x| {
                        (y1..y2+1)
                            .into_iter()
                            .map(move |y| (x, y))
                    })
                    .for_each(move |(x, y)| {
                        let (dx, dy) = (mx - x as isize, my - y as isize);
                        let dd = (dx*dx + dy*dy) as f32;
                        if (dd - w*w).abs() <= 2.0 {
                            set.set_bit(x, y);
                        }
                    });
            },
        }
    }
}
