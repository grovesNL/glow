#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub use self::native::*;

#[cfg(target_arch = "wasm32")]
mod web;
#[cfg(target_arch = "wasm32")]
pub use self::web::*;

/// The type of the shader.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ShaderType {
    Fragment = 0x8B30,
    Vertex = 0x8B31,
    Geometry = 0x8DD9,
    TessEvaluation = 0x8E87,
    TessControl = 0x8E88,
    Compute = 0x91B9,
}

/// A buffer binding target.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BufferBindingTarget {
    /// Vertex attributes.
    ArrayBuffer = 0x8892,
    /// Atomic counter storage.
    AtomicCounterBuffer = 0x92C0,
    /// Buffer copy source.
    CopyReadBuffer = 0x8F36,
    /// Buffer copy destination.
    CopyWriteBuffer = 0x8F37,
    /// Indirect compute dispatch commands.
    DispatchIndirectBuffer = 0x90EE,
    /// Indirect command arguments.
    DrawIndirect = 0x8F3F,
    /// Vertex array indices.
    ElementArray = 0x8893,
    /// Pixel read target.
    PixelPack = 0x88EB,
    /// Texture data source.
    PixelUnpack = 0x88EC,
    /// Query result buffer.
    Query = 0x9192,
    /// Read-write storage for shaders.
    ShaderStorage = 0x90D2,
    /// Texture data buffer.
    Texture = 0x8C2A,
    /// Transform feedback buffer.
    TransformFeedback = 0x8C8E,
    /// Uniform block storage.
    Uniform = 0x8A11,
}

/// The kind of primitive to render.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PrimitiveMode {
    Points = 0x0000,
    LineStrip = 0x0003,
    LineLoop = 0x0002,
    Lines = 0x0001,
    LineStripAdjacency = 0x000B,
    LinesAdjacency = 0x000A,
    TriangleStrip = 0x0005,
    TriangleFan = 0x0006,
    Triangles = 0x0004,
    TriangleStripAdjacency = 0x000D,
    TrianglesAdjacency = 0x000C,
    Patches = 0x000E,
}

pub(crate) const COMPILE_STATUS: u32 = 0x8B81;
pub(crate) const INFO_LOG_LENGTH: u32 = 0x8B84;
pub(crate) const LINK_STATUS: u32 = 0x8B82;

pub trait RenderingContext {
    type Shader: Copy
        + Clone
        + std::fmt::Debug
        + Eq
        + std::hash::Hash
        + Ord
        + PartialEq
        + PartialOrd;
    type Program: Copy
        + Clone
        + std::fmt::Debug
        + Eq
        + std::hash::Hash
        + Ord
        + PartialEq
        + PartialOrd;
    type Buffer: Copy
        + Clone
        + std::fmt::Debug
        + Eq
        + std::hash::Hash
        + Ord
        + PartialEq
        + PartialOrd;

    unsafe fn create_shader(&self, shader_type: ShaderType) -> Result<Self::Shader, String>;

    unsafe fn shader_source(&self, shader: Self::Shader, source: &str);

    unsafe fn compile_shader(&self, shader: Self::Shader);

    unsafe fn get_shader_compile_status(&self, shader: Self::Shader) -> bool;

    unsafe fn get_shader_info_log(&self, shader: Self::Shader) -> String;

    unsafe fn create_program(&self) -> Result<Self::Program, String>;

    unsafe fn attach_shader(&self, program: Self::Program, shader: Self::Shader);

    unsafe fn link_program(&self, program: Self::Program);

    unsafe fn get_program_link_status(&self, program: Self::Program) -> bool;

    unsafe fn get_program_info_log(&self, program: Self::Program) -> String;

    unsafe fn use_program(&self, program: Option<Self::Program>);

    unsafe fn create_buffer(&self) -> Result<Self::Buffer, String>;

    unsafe fn bind_buffer(&self, target: BufferBindingTarget, buffer: Option<Self::Buffer>);

    unsafe fn draw_arrays(&self, mode: PrimitiveMode, first: i32, count: i32);
}
