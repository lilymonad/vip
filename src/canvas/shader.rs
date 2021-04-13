use luminance::{
    linear::M33,
    texture::Dim2,
    pipeline::BoundTexture,
    shader::program::Uniform,
    pixel::NormUnsigned,
};
use luminance_derive::{Semantics, Vertex, UniformInterface};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Semantics)]
pub enum Semantics {
    #[sem(name="pos", repr="[f32;2]", wrapper="VertexPosition")]
    Position,
    #[sem(name="texPos", repr="[f32;2]", wrapper="TexPosition")]
    Tex,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Vertex)]
#[vertex(sem = "Semantics")]
pub struct Vertex {
    pub pos: VertexPosition,
    pub texPos: TexPosition,
}

#[derive(UniformInterface)]
pub struct ShaderInterface {
    #[uniform]
    tex: Uniform<& 'static BoundTexture<'static, Dim2, NormUnsigned>>,
    #[uniform]
    view: Uniform<M33>,
}
