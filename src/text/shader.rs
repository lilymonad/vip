use luminance::{
    texture::Dim2,
    pixel::NormUnsigned,
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
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Vertex)]
#[vertex(sem = "Semantics")]
pub struct Vertex {
    pub pos: VP,
    pub texPos: TP,
}

#[derive(UniformInterface)]
pub struct ShaderInterface {
    #[uniform]
    tex: Uniform<& 'static BoundTexture<'static, Dim2, NormUnsigned>>,
    #[uniform]
    view: Uniform<M33>,
}
