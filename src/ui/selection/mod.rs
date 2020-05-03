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
    pub pos: SelPos,
    pub texPos: SelTexPos,
    #[vertex(normalized="true")]
    pub onColor: SelOnColor,
}

#[derive(UniformInterface)]
pub struct ShaderInterface {
    #[uniform]
    tex: Uniform<& 'static BoundTexture<'static, Dim2, NormUnsigned>>,
    #[uniform]
    view: Uniform<M33>,
}
