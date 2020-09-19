mod shader;

use std::fs::File;
use std::{cell::Cell, path::Path, fs, collections::{HashMap, BTreeMap}};
use rusttype::{Font, Point, Scale};

use ttf_parser::{Font as TTFFont};
use msdfgen::{FontExt, Bitmap, RGB, Range, EDGE_THRESHOLD, OVERLAP_SUPPORT};

use luminance::{
    texture::{Texture, GenMipmaps, Sampler, Dim2},
    pixel::{NormR8UI, NormRGB8UI},
    context::GraphicsContext,
};

pub use shader::*;

#[derive(Clone, Copy)]
pub struct GlyphRect {
    pub atlas_coord: (f32, f32),
    pub atlas_size: (f32, f32),
    pub offset: msdfgen::Bounds<f32>,
    pub scale: f32,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct FontID(usize);

impl Into<usize> for FontID {
    fn into(self) -> usize { self.0 }
}

pub struct FontInfo {
    toppest: f32,
    lowest: f32,
    glyphs: [Option<GlyphRect>; 256],
}

pub struct TextRenderer {
    pub atlas: Texture<Dim2, NormRGB8UI>,
    pub fonts: BTreeMap<FontID, FontInfo>,
    pub resolution: f32,

    text_cache: Cell<HashMap<(String, FontID), Vec<Vertex>>>,
}

pub enum HAlign {
    Left(usize),
    Center,
    Right(usize),
}

pub enum VAlign {
    Top(usize),
    Center,
    Bottom(usize),
}

type Alignment = (HAlign, VAlign);

impl TextRenderer {

    pub fn render_text<S:AsRef<str>>(&self, text:S, (ha, va):Alignment, (screenw, screenh):(f32, f32), id:FontID, size:f32)
        -> Vec<Vertex>
    {
        let scale = size / self.resolution;
        let font = self.fonts.get(&id).unwrap();
        let text : Vec<Option<GlyphRect>> = text.as_ref().chars().map(|c| font.glyphs[c as usize].clone()).collect();

        let mut text_width = 0.0;

        for c in &text {
            if let Some(rect) = c {
                text_width += size * (rect.offset.right - rect.offset.left) * rect.scale;
            } else {
                text_width += self.resolution * 0.5 * scale;
            }
        }

        let toppest = font.toppest * size;
        let bottomest = font.lowest * size;

        let mut sx = match ha {
            HAlign::Left(offset) => offset as f32,
            HAlign::Center => (screenw - text_width) * 0.5,
            HAlign::Right(offset) => screenw - offset as f32 - text_width,
        };

        let sy = match va {
            VAlign::Top(offset) => offset as f32 + toppest - bottomest,
            VAlign::Center => (screenh - size) * 0.5,
            VAlign::Bottom(offset) => screenh - offset as f32 + bottomest,
        };


        text
            .into_iter()
            .map(|rect| {
                rect.map(|rect| {
                    let (x,y) = rect.atlas_coord; // topleft coords in atlas
                    let (w,h) = rect.atlas_size;  // rect size of glyph in atlas
                    let (top, left, bottom, right) = (
                        size * rect.offset.top * rect.scale,
                        size * rect.offset.left * rect.scale,
                        size * rect.offset.bottom * rect.scale,
                        size * rect.offset.right * rect.scale,
                    );
                    sx += right - left;
                    vec![
                        Vertex {
                            pos: VP::new([sx + left, sy + top]),
                            texPos: TP::new([x, y]),
                        },
                        Vertex {
                            pos: VP::new([sx + left, sy + bottom]),
                            texPos: TP::new([x, y+h]),
                        },
                        Vertex {
                            pos: VP::new([sx + right, sy + bottom]),
                            texPos: TP::new([x+w, y+h]),
                        },
                        Vertex {
                            pos: VP::new([sx + right, sy + bottom]),
                            texPos: TP::new([x+w, y+h]),
                        },
                        Vertex {
                            pos: VP::new([sx + right, sy + top]),
                            texPos: TP::new([x+w, y]),
                        },
                        Vertex {
                            pos: VP::new([sx + left, sy + top]),
                            texPos: TP::new([x, y]),
                        }
                    ]
                })
                .unwrap_or_else(|| {
                    sx += self.resolution * 0.5 * scale;
                    vec![]
                })
            })
            .flatten()
            .collect()
    }

    pub fn render_text_cached<'a, S:AsRef<str>>(& 'a self, text:S, pos:Alignment, id:FontID) -> & 'a [Vertex] {
        let map = unsafe { self.text_cache.as_ptr().as_mut().unwrap() };
        map.entry((text.as_ref().to_string(), id))
            .or_insert_with(|| {
                self.render_text(text, pos, (800.0, 600.0), id, 64.0)
            })
    }
}


pub struct TextRendererBuilder {
    fonts: Vec<Vec<u8>>,
    resolution: u32,
}

impl TextRendererBuilder {
    pub fn for_resolution(resolution:u32) -> Self {
        Self {
            fonts: Vec::new(),
            resolution
        }
    }

    pub fn add_font<P:AsRef<Path>>(&mut self, file:P) -> Option<FontID> {
        let ret = FontID(self.fonts.len());
        let content : Vec<u8> = fs::read(file).ok()?;

        self.fonts.push(content);

        Some(ret)
    }

    pub fn build<C:GraphicsContext>(&self, ctx: &mut C, sampler: Sampler) -> Option<TextRenderer> {
        let chars : Vec<(usize, char)> = (33..255u8).map(|n| n as char).enumerate().collect();

        let res = self.resolution;
        let nb_chars = chars.len() as u32;
        let mut min_s = 512;
        while (min_s * min_s) / (res * res) < nb_chars {
            min_s *= 2
        }


        let nf = self.fonts.len() as u32;
        let (aw, ah) = (min_s, min_s * nf);
        let atlas : Texture<Dim2, NormRGB8UI> = Texture::new(ctx
                                                             , [aw, ah]
                                                             , 0, sampler).ok()?;
        let mut fonts = BTreeMap::new();

        let glyphs_per_row = min_s / res;

        //let mut map : Vec<u8> = Vec::with_capacity((res * res) as usize);
        let mut map = Bitmap::new(res, res);

        for (fi, content) in self.fonts.iter().enumerate() {
            let mut original_size = None;
            let fy = min_s * fi as u32;

            let f = TTFFont::from_data(&content, 0)?;//Font::from_bytes(&content).ok()?;
            let toppest = f.ascender() as f32 / f.height() as f32;
            let lowest  = f.descender() as f32 / f.height() as f32;
            let mut glyphs = [None; 256];

            for (ci, c) in &chars {
                let x = ci % glyphs_per_row as usize;
                let y = ci / glyphs_per_row as usize;

                let (w, h) = (res, res);
                let glyph = match f.glyph_index(*c) {
                    Some(id) => id,
                    None => continue,
                };
                let mut shape = match f.glyph_shape(glyph) {
                    Some(shape) => shape,
                    None => continue,
                };

                let mut bounds = shape.get_bounds();
                let framing = bounds.autoframe(res, res, Range::Px(4.0), None).unwrap();

                let origin = *original_size.get_or_insert(framing.range);

                shape.edge_coloring_simple(3.0, 0);
                shape.generate_msdf(&mut map, &framing, EDGE_THRESHOLD, OVERLAP_SUPPORT);

                std::mem::swap(&mut bounds.bottom, &mut bounds.top);

                map.flip_y();
                let mapu8 : Bitmap<RGB<u8>> = map.convert();

                let (top, left, bottom, right) = (
                    (bounds.top + framing.translate.y) * framing.scale.y,
                    (bounds.left + framing.translate.x) * framing.scale.x,
                    (bounds.bottom + framing.translate.y) * framing.scale.y,
                    (bounds.right + framing.translate.x) * framing.scale.x,
                );

                let (gx, gy) = (res * x as u32, fy + res * y as u32);
                atlas.upload_part_raw(GenMipmaps::No
                   , [gx, gy]
                   , [w, h], mapu8.raw_pixels()).ok().unwrap();

                let bounds = msdfgen::Bounds {
                    bottom: -bounds.top as f32 / res as f32 * framing.scale.y as f32,
                    top: -bounds.bottom as f32 / res as f32 * framing.scale.y as f32,
                    left: bounds.left as f32 / res as f32 * framing.scale.x as f32,
                    right: bounds.right as f32 / res as f32 * framing.scale.x as f32,
                };

                glyphs[*c as usize] = Some(GlyphRect {
                    atlas_coord: ((gx as f32 + left as f32) / aw as f32, (gy as f32 + top as f32) / ah as f32),
                    atlas_size: ((right - left) as f32 / aw as f32, (bottom - top) as f32 / ah as f32),
                    offset: bounds,
                    scale: (framing.range / origin) as f32,
                });
            }

            fonts.insert(FontID(fi), FontInfo { toppest, lowest, glyphs });
        }

        let tex = atlas.get_raw_texels();
        image::save_buffer("font_atlas.png", &tex, aw, ah, image::ColorType::Rgb8).unwrap();

        Some(TextRenderer {
            atlas,
            fonts,
            resolution: res as f32,
            text_cache: Cell::new(HashMap::new()),
        })
    }
}
