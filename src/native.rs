use super::*;

use std::sync::Arc;

mod native_gl {
    include!(concat!(env!("OUT_DIR"), "/opengl_bindings.rs"));
}

pub struct NativeRenderingContext {
    raw: native_gl::Gl,
}

impl NativeRenderingContext {
    pub fn from_glutin_window(window: &glutin::GlWindow) -> Self {
        use glutin::GlContext;
        let raw = native_gl::Gl::load_with(|s| window.get_proc_address(s) as *const _);
        NativeRenderingContext { raw }
    }
}

impl std::fmt::Debug for NativeRenderingContext {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // TODO
        write!(f, "TODO")
    }
}

impl RenderingContext for NativeRenderingContext {
    type Shader = native_gl::types::GLuint;
    type Program = native_gl::types::GLuint;
    type Buffer = native_gl::types::GLuint;
    type VertexArray = native_gl::types::GLuint;

    unsafe fn create_shader(&self, shader_type: ShaderType) -> Result<Self::Shader, String> {
        let gl = &self.raw;
        Ok(gl.CreateShader(shader_type as u32))
    }

    unsafe fn delete_shader(&self, shader: Self::Shader) {
        let gl = &self.raw;
        gl.DeleteShader(shader);
    }

    unsafe fn shader_source(&self, shader: Self::Shader, source: &str) {
        let gl = &self.raw;
        gl.ShaderSource(
            shader,
            1,
            &(source.as_ptr() as *const native_gl::types::GLchar),
            &(source.len() as native_gl::types::GLint),
        );
    }

    unsafe fn compile_shader(&self, shader: Self::Shader) {
        let gl = &self.raw;
        gl.CompileShader(shader);
    }

    unsafe fn get_shader_compile_status(&self, shader: Self::Shader) -> bool {
        let gl = &self.raw;
        let mut status = 0;
        gl.GetShaderiv(shader, COMPILE_STATUS, &mut status);
        1 == status
    }

    unsafe fn get_shader_info_log(&self, shader: Self::Shader) -> String {
        let gl = &self.raw;
        let mut length = 0;
        gl.GetShaderiv(shader, INFO_LOG_LENGTH, &mut length);
        if length > 0 {
            let mut log = String::with_capacity(length as usize);
            log.extend(std::iter::repeat('\0').take(length as usize));
            gl.GetShaderInfoLog(
                shader,
                length,
                &mut length,
                (&log[..]).as_ptr() as *mut native_gl::types::GLchar,
            );
            log.truncate(length as usize);
            log
        } else {
            String::from("")
        }
    }

    unsafe fn create_program(&self) -> Result<Self::Program, String> {
        let gl = &self.raw;
        Ok(gl.CreateProgram())
    }

    unsafe fn delete_program(&self, program: Self::Program) {
        let gl = &self.raw;
        gl.DeleteProgram(program);
    }

    unsafe fn attach_shader(&self, program: Self::Program, shader: Self::Shader) {
        let gl = &self.raw;
        gl.AttachShader(program, shader);
    }

    unsafe fn detach_shader(&self, program: Self::Program, shader: Self::Shader) {
        let gl = &self.raw;
        gl.DetachShader(program, shader);
    }

    unsafe fn link_program(&self, program: Self::Program) {
        let gl = &self.raw;
        gl.LinkProgram(program);
    }

    unsafe fn get_program_link_status(&self, program: Self::Program) -> bool {
        let gl = &self.raw;
        let mut status = 0;
        gl.GetProgramiv(program, LINK_STATUS, &mut status);
        1 == status
    }

    unsafe fn get_program_info_log(&self, program: Self::Program) -> String {
        let gl = &self.raw;
        let mut length = 0;
        gl.GetProgramiv(program, INFO_LOG_LENGTH, &mut length);
        if length > 0 {
            let mut log = String::with_capacity(length as usize);
            log.extend(std::iter::repeat('\0').take(length as usize));
            gl.GetProgramInfoLog(
                program,
                length,
                &mut length,
                (&log[..]).as_ptr() as *mut native_gl::types::GLchar,
            );
            log.truncate(length as usize);
            log
        } else {
            String::from("")
        }
    }

    unsafe fn use_program(&self, program: Option<Self::Program>) {
        let gl = &self.raw;
        gl.UseProgram(program.unwrap_or(0));
    }

    unsafe fn create_buffer(&self) -> Result<Self::Buffer, String> {
        let gl = &self.raw;
        let mut buffer = 0;
        gl.GenBuffers(1, &mut buffer);
        Ok(buffer)
    }

    unsafe fn bind_buffer(&self, target: BufferBindingTarget, buffer: Option<Self::Buffer>) {
        let gl = &self.raw;
        gl.BindBuffer(target as u32, buffer.unwrap_or(0));
    }

    unsafe fn draw_arrays(&self, mode: PrimitiveMode, first: i32, count: i32) {
        let gl = &self.raw;
        gl.DrawArrays(mode as u32, first, count);
    }

    unsafe fn create_vertex_array(&self) -> Result<Self::VertexArray, String> {
        let gl = &self.raw;
        let mut vertex_array = 0;
        gl.GenVertexArrays(1, &mut vertex_array);
        Ok(vertex_array)
    }

    unsafe fn delete_vertex_array(&self, vertex_array: Self::VertexArray) {
        let gl = &self.raw;
        gl.DeleteVertexArrays(1, &vertex_array);
    }

    unsafe fn bind_vertex_array(&self, vertex_array: Option<Self::VertexArray>) {
        let gl = &self.raw;
        gl.BindVertexArray(vertex_array.unwrap_or(0));
    }

    unsafe fn clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        let gl = &self.raw;
        gl.ClearColor(red, green, blue, alpha);
    }

    unsafe fn clear(&self, mask: ClearMask) {
        let gl = &self.raw;
        gl.Clear(mask.bits());
    }
}

pub struct NativeRenderLoop {
    window: Arc<glutin::GlWindow>,
}

impl NativeRenderLoop {
    pub fn from_glutin_window(window: Arc<glutin::GlWindow>) -> Self {
        NativeRenderLoop { window }
    }
}

impl RenderLoop for NativeRenderLoop {
    type Window = Arc<glutin::GlWindow>;

    fn run<F: FnMut(&mut bool) + 'static>(&self, mut callback: F) {
        let mut running = true;
        while running {
            callback(&mut running);
            (*self.window).swap_buffers().unwrap();
        }
    }
}
