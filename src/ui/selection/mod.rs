use luminance_derive::{Semantics, Vertex, UniformInterface};
use luminance::{
    texture::Dim2,
    pixel::NormUnsigned,
    pipeline::BoundTexture,
    shader::program::Uniform,
    linear::M33,
};

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
