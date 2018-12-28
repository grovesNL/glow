#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub use self::native::*;

#[cfg(target_arch = "wasm32")]
mod web;
#[cfg(target_arch = "wasm32")]
pub use self::web::*;

/// The shader type.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ShaderType {
    Fragment = 0x8B30,
    Vertex = 0x8B31,
    Geometry = 0x8DD9,
    TessEvaluation = 0x8E87,
    TessControl = 0x8E88,
    Compute = 0x91B9,
}

pub(crate) const COMPILE_STATUS: u32 = 0x8B81;
pub(crate) const INFO_LOG_LENGTH: u32 = 0x8B84;
pub(crate) const LINK_STATUS: u32 = 0x8B82;

pub trait RenderingContext {
    type Shader: std::fmt::Debug;
    type Program: std::fmt::Debug;

    fn create_shader(&self, shader_type: ShaderType) -> Result<Self::Shader, String>;

    fn shader_source(&self, shader: &Self::Shader, source: &str);

    fn compile_shader(&self, shader: &Self::Shader);

    fn get_shader_compile_status(&self, shader: &Self::Shader) -> bool;

    fn get_shader_info_log(&self, shader: &Self::Shader) -> String;

    fn create_program(&self) -> Result<Self::Program, String>;

    fn attach_shader(&self, program: &Self::Program, shader: &Self::Shader);

    fn link_program(&self, program: &Self::Program);

    fn get_program_link_status(&self, program: &Self::Program) -> bool;

    fn get_program_info_log(&self, program: &Self::Program) -> String;
}
