use std::{cell::Cell, path::Path, fs, collections::{HashMap, BTreeMap}};
use rusttype::{Font, Point, Scale};
use luminance::{
    texture::{Texture, GenMipmaps, Dim2, Sampler},
    pixel::{NormUnsigned, NormRGB8UI, NormR8UI},
    context::GraphicsContext,
    pipeline::BoundTexture,
    shader::program::Uniform,
    linear::M33,
};

use luminance_derive::{Semantics, Vertex, UniformInterface};
#[derive(Clone, Copy, Debug, Eq, PartialEq, Semantics)]
pub enum Semantics {
    #[sem(name="pos", repr="[f32;2]", wrapper="VP")]
    Position,
    #[sem(name="texPos", repr="[f32;2]", wrapper="TP")]
    TexPos,
//        #[sem(name="color", repr="[u8;3]", wrapper="VC")]
//        Color,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Vertex)]
#[vertex(sem = "Semantics")]
pub struct Vertex {
    pub pos: VP,
    pub texPos: TP,
//        #[vertex(normalized="true")]
//        pub rgb: VC,
}

#[derive(UniformInterface)]
pub struct ShaderInterface {
    #[uniform]
    tex: Uniform<& 'static BoundTexture<'static, Dim2, NormUnsigned>>,
    #[uniform]
    view: Uniform<M33>,
}

pub fn render_text<'a, S>(font: &Font<'a>, text:S, raster: &Texture<Dim2, NormRGB8UI>)
    -> Option<()>
    where
        S : AsRef<str>,
{
    let text : &str = text.as_ref();
    let mut pos = Point { x:0f32, y:0f32 };

    for c in text.chars() {
        let glyph = font
            .glyph(c)
            .scaled(Scale::uniform(128f32))
            .positioned(pos);

        let bounds = glyph.pixel_bounding_box()?;
        let (w, h) = (bounds.width() as u32, bounds.height() as u32);
        let num_pixels = (w * h) as usize;
        let mut map : Vec<(u8,u8,u8)> = Vec::with_capacity(num_pixels);
        map.resize(num_pixels, (0,0,0));

        glyph.draw(|x, y, v| {
            let v = (v * 255f32) as u8;
            map[(y*w + x) as usize] = (v,v,v);
        });

        raster.upload_part(GenMipmaps::No
                           , [pos.x as u32, pos.y as u32]
                           , [bounds.width() as u32, bounds.height() as u32], &map).ok()?;

        pos.x += bounds.width() as f32;
    }

    Some(())
}

pub struct GlyphRect {
    pub topleft: (f32, f32),
    pub size: (f32, f32),
    pub y_offset: f32,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct FontID(usize);

impl Into<usize> for FontID {
    fn into(self) -> usize { self.0 }
}

pub struct TextRenderer {
    pub atlas: Texture<Dim2, NormR8UI>,
    pub glyphs: BTreeMap<(char, FontID), GlyphRect>,

    text_cache: Cell<HashMap<(String, FontID), Vec<Vertex>>>,
}

impl TextRenderer {

    pub fn render_text<S:AsRef<str>>(&self, text:S, mut pos:(f32, f32), id:FontID) -> Vec<Vertex> {
        let [aw, ah] = self.atlas.size();
        text.as_ref()
            .chars()
            .map(|c| {
                let rect = self.glyphs.get(&(c, id));

                rect.map(|rect| {
                    let scale = 2.0;

                    let (x,y) = rect.topleft;
                    let (w,h) = rect.size;
                    let (sx,sy) = pos;
                    let (sw,sh) = (w * aw as f32 * scale, h * ah as f32 * scale);
                    pos = (sx + sw, sy);
                    let sy = sy + rect.y_offset * 32.0 * scale;
                    vec![
                        Vertex {
                            pos: VP::new([sx, sy]),
                            texPos: TP::new([x, y]),
                        },
                        Vertex {
                            pos: VP::new([sx, sy + sh]),
                            texPos: TP::new([x, y+h]),
                        },
                        Vertex {
                            pos: VP::new([sx + sw, sy + sh]),
                            texPos: TP::new([x+w, y+h]),
                        },
                        Vertex {
                            pos: VP::new([sx + sw, sy + sh]),
                            texPos: TP::new([x+w, y+h]),
                        },
                        Vertex {
                            pos: VP::new([sx + sw, sy]),
                            texPos: TP::new([x+w, y]),
                        },
                        Vertex {
                            pos: VP::new([sx, sy]),
                            texPos: TP::new([x, y]),
                        }
                    ]
                })
                .unwrap_or_else(|| {
                    if c == ' ' {
                        pos.0 += 10.0;
                    }
                    vec![]
                })
            })
            .flatten()
            .collect()
    }

    pub fn render_text_cached<'a, S:AsRef<str>>(& 'a self, text:S, pos:(f32,f32), id:FontID) -> & 'a [Vertex] {
        let map = unsafe { self.text_cache.as_ptr().as_mut().unwrap() };
        map.entry((text.as_ref().to_string(), id))
            .or_insert_with(|| {
                self.render_text(text, pos, id)
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
        let chars : Vec<(usize, char)> = (33..127u8).map(|n| n as char).enumerate().collect();

        let res = self.resolution;
        let nb_chars = chars.len() as u32;
        let mut min_s = 512;
        while (min_s * min_s) / (res * res) < nb_chars {
            min_s *= 2
        }


        let nf = self.fonts.len() as u32;
        let (aw, ah) = (min_s, min_s * nf);
        let atlas : Texture<Dim2, NormR8UI> = Texture::new(ctx
                                                             , [aw, ah]
                                                             , 0, sampler).ok()?;
        let mut glyphs = BTreeMap::new();

        let glyphs_per_row = min_s / res;


        for (fi, content) in self.fonts.iter().enumerate() {
            let fy = min_s * fi as u32;

            let f = Font::from_bytes(&content).ok()?;
            for (ci, c) in &chars {
                let x = ci % glyphs_per_row as usize;
                let y = ci / glyphs_per_row as usize;

                let glyph = f
                    .glyph(*c)
                    .scaled(Scale::uniform(self.resolution as f32))
                    .positioned(Point { x:x as f32, y:y as f32 });
                let bb = glyph.pixel_bounding_box()?;
                let (w, h) = (bb.width() as u32, bb.height() as u32);
                let num_pixels = (w * h) as usize;
                let mut map : Vec<u8> = Vec::with_capacity(num_pixels);
                map.resize(num_pixels, 0);

                glyph.draw(|x, y, v| {
                    let v = (v * 255f32) as u8;
                    map[(y*w + x) as usize] = v;
                });

                let (gx, gy) = (res * x as u32, fy + res * y as u32);
                atlas.upload_part(GenMipmaps::No
                   , [gx, gy]
                   , [bb.width() as u32, bb.height() as u32], &map).ok()?;

                println!("min of '{}' is {:?}", c, bb.min);
                glyphs.insert((*c, FontID(fi)), GlyphRect {
                    topleft: (gx as f32 / aw as f32, gy as f32 / ah as f32),
                    size: (w as f32 / aw as f32, h as f32 / ah as f32),
                    y_offset: (bb.min.y as f32) / (res as f32),
                });
            }
        }

        Some(TextRenderer {
            atlas,
            glyphs,
            text_cache: Cell::new(HashMap::new()),
        })
    }
}
