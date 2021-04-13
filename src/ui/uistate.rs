use std::collections::{HashMap, HashSet};
use crate::{
    canvas::{self, Canvas},
    ui::selection as sel,
    keyboard::CharKeyMod,
    bitmap2d::BitMap2D,
};

const GSIZE : f32 = 0.3;

pub struct UiState {
    pub filename:Option<String>,
    pub palette:HashMap<CharKeyMod, (u8, u8, u8, u8)>,
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
        let (icw, ich) = self.canvas.size();             // canvas size in pixels
        let (chunkw, chunkh) = self.chunk_size;          // chunk size in pixels
        let (nbcw, nbch) = (icw / chunkw, ich / chunkh); // number of full chunks
        let (cw, ch) = (icw as f32, ich as f32);         // size of canvas as float
        if self.exploded {
            let mut ret = Vec::new();
            for i in 0..=nbcw {
                for j in 0..=nbch {
                    // gap between chunks
                    let (gx, gy) = (i as f32 * GSIZE, j as f32 * GSIZE);

                    // topleft vertex of chunk
                    let (bx, by) = ((i * chunkw) as f32, (j * chunkh) as f32);

                    // bottomright vertex of chunk
                    let (ex, ey) = ((bx + chunkw as f32).min(cw), (by + chunkh as f32).min(ch));

                    // topleft and bottomright texture coordinates for chunk
                    let (tbx, tby) = (bx / cw, by / ch);
                    let (tex, tey) = (ex / cw, ey / ch);

                    ret.extend_from_slice(&[
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
                Vertex { pos:VertexPosition::new([ cw,0.0]), texPos:TexPosition::new([1.0,0.0]) },
                Vertex { pos:VertexPosition::new([ cw, ch]), texPos:TexPosition::new([1.0,1.0]) },
                Vertex { pos:VertexPosition::new([ cw, ch]), texPos:TexPosition::new([1.0,1.0]) },
                Vertex { pos:VertexPosition::new([0.0, ch]), texPos:TexPosition::new([0.0,1.0]) },
                Vertex { pos:VertexPosition::new([0.0,0.0]), texPos:TexPosition::new([0.0,0.0]) },
            ]
        }
    }

    pub fn render_selection(&self, selection:&HashSet<(usize, usize)>) -> Vec<sel::Vertex> {
        use sel::*;
        let mut ret = Vec::new();
        for (x, y) in selection.iter().copied() {
            let (ix, iy) = (x as isize, y as isize);

            let (itx, ity) = 
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

            let cs = 64.0;   // cell size in pixels
            let ats = 256.0; // total size of the map in pixels

            let ts = cs / ats; // normalized size of a cell

            // normalized topleft texture coordinate
            let (tx, ty) = (cs*(itx as f32 )/ ats, cs*(ity as f32) / ats);

            // compute the color we are on
            let (r, g, b, _) = self.canvas.get_pixel_color(x, y);
            let scol = [r, g, b];


            // if exploded, we need to add the explosion gap to the cell coordinates
            let (gx, gy) = if self.exploded {
                ((x / self.chunk_size.0) as f32, (y / self.chunk_size.1) as f32)
            } else {
                (0.0, 0.0)
            };

            let (px, py) = (x as f32 + gx * GSIZE, y as f32 + gy * GSIZE);

            if !(itx == 0 && ity == 0) {
                // send the cell's vertice (two triangles for a square)
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
                (x1..=x2)
                    .into_iter()
                    .flat_map(|x| {
                        (y1..=y2)
                            .into_iter()
                            .map(move |y| (x, y))
                    })
                    .for_each(|(x, y)| { set.set_bit(x, y); });
            },
            VisualType::Circle => {
                let dx = x2 as isize - x1 as isize;
                let dy = y2 as isize - y1 as isize;
                let mx = x1 as isize + (dx / 2);
                let my = y1 as isize + (dy / 2);
                let d = dx.min(dy);

                if d % 2 == 1 {
                    even_radius_andres(set, (mx, my), (d as f32 + 1.0) / 2.0);
                } else {
                    odd_radius_andres(set, (mx, my), d / 2);
                }
            },
        }
    }
}

fn odd_radius_andres<T:BitMap2D>(set:&mut T, (x0, y0):(isize, isize), r:isize) {
    let mut x = 0;
    let mut y = r;
    let mut d = r - 1;

    while x <= y {

        set.set_bit((x0+x) as usize, (y0+y) as usize);
        set.set_bit((x0+y) as usize, (y0+x) as usize);
        set.set_bit((x0-x) as usize, (y0+y) as usize);
        set.set_bit((x0-y) as usize, (y0+x) as usize);
        set.set_bit((x0-x) as usize, (y0-y) as usize);
        set.set_bit((x0-y) as usize, (y0-x) as usize);
        set.set_bit((x0+x) as usize, (y0-y) as usize);
        set.set_bit((x0+y) as usize, (y0-x) as usize);

        if d >= 2 * x {
            d = d - 2*x - 1;
            x = x + 1;
        } else if d <= 2 * (r - y) {
            d = d + 2*y - 1;
            y = y - 1;
        } else {
            d = d + 2 * (y-x-1);
            y = y - 1;
            x = x + 1;
        }
    }
}

fn even_radius_andres<T:BitMap2D>(set:&mut T, (x0, y0):(isize, isize), r:f32) {
    let mut x = 1;
    let mut y = r as isize;

    let test = |x, y| {
        let d = (x*x + y*y) as f32;
        (r - 0.5).powi(2) < d && d < (r + 0.5).powi(2)
    };

    while x <= y {

        set.set_bit((x0+x  ) as usize, (y0+y  ) as usize);
        set.set_bit((x0+y  ) as usize, (y0+x  ) as usize);
        set.set_bit((x0-x+1) as usize, (y0+y  ) as usize);
        set.set_bit((x0-y+1) as usize, (y0+x  ) as usize);
        set.set_bit((x0-x+1) as usize, (y0-y+1) as usize);
        set.set_bit((x0-y+1) as usize, (y0-x+1) as usize);
        set.set_bit((x0+x  ) as usize, (y0-y+1) as usize);
        set.set_bit((x0+y  ) as usize, (y0-x+1) as usize);

        if test(x + 1, y) {
            x = x + 1;
        } else if test(x, y - 1) {
            y = y - 1;
        } else {
            y = y - 1;
            x = x + 1;
        }
    }
}
