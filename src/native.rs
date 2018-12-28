use super::*;

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

#[derive(Debug)]
pub struct NativeShader {
    raw: native_gl::types::GLuint,
}

#[derive(Debug)]
pub struct NativeProgram {
    raw: native_gl::types::GLuint,
}

impl RenderingContext for NativeRenderingContext {
    type Shader = NativeShader;
    type Program = NativeProgram;

    fn create_shader(&self, shader_type: ShaderType) -> Result<Self::Shader, String> {
        let gl = &self.raw;
        Ok(unsafe {
            NativeShader {
                raw: gl.CreateShader(shader_type as u32),
            }
        })
    }

    fn shader_source(&self, shader: &Self::Shader, source: &str) {
        let gl = &self.raw;
        unsafe {
            gl.ShaderSource(
                shader.raw,
                1,
                &(source.as_ptr() as *const native_gl::types::GLchar),
                &(source.len() as native_gl::types::GLint),
            );
        }
    }

    fn compile_shader(&self, shader: &Self::Shader) {
        let gl = &self.raw;
        unsafe {
            gl.CompileShader(shader.raw);
        }
    }

    fn get_shader_compile_status(&self, shader: &Self::Shader) -> bool {
        let gl = &self.raw;
        let mut status = 0;
        unsafe {
            gl.GetShaderiv(shader.raw, COMPILE_STATUS, &mut status);
        }
        1 == status
    }

    fn get_shader_info_log(&self, shader: &Self::Shader) -> String {
        let gl = &self.raw;
        let mut length = 0;
        unsafe {
            gl.GetShaderiv(shader.raw, INFO_LOG_LENGTH, &mut length);
        }
        if length > 0 {
            let mut log = String::with_capacity(length as usize);
            log.extend(std::iter::repeat('\0').take(length as usize));
            unsafe {
                gl.GetShaderInfoLog(
                    shader.raw,
                    length,
                    &mut length,
                    (&log[..]).as_ptr() as *mut native_gl::types::GLchar,
                );
            }
            log.truncate(length as usize);
            log
        } else {
            String::from("")
        }
    }

    fn create_program(&self) -> Result<Self::Program, String> {
        let gl = &self.raw;
        Ok(unsafe {
            NativeProgram {
                raw: gl.CreateProgram(),
            }
        })
    }

    fn attach_shader(&self, program: &Self::Program, shader: &Self::Shader) {
        let gl = &self.raw;
        unsafe {
            gl.AttachShader(program.raw, shader.raw);
        }
    }

    fn link_program(&self, program: &Self::Program) {
        let gl = &self.raw;
        unsafe {
            gl.LinkProgram(program.raw);
        }
    }

    fn get_program_link_status(&self, program: &Self::Program) -> bool {
        let gl = &self.raw;
        let mut status = 0;
        unsafe {
            gl.GetProgramiv(program.raw, LINK_STATUS, &mut status);
        }
        1 == status
    }

    fn get_program_info_log(&self, program: &Self::Program) -> String {
        let gl = &self.raw;
        let mut length = 0;
        unsafe {
            gl.GetProgramiv(program.raw, INFO_LOG_LENGTH, &mut length);
        }
        if length > 0 {
            let mut log = String::with_capacity(length as usize);
            log.extend(std::iter::repeat('\0').take(length as usize));
            unsafe {
                gl.GetProgramInfoLog(
                    program.raw,
                    length,
                    &mut length,
                    (&log[..]).as_ptr() as *mut native_gl::types::GLchar,
                );
            }
            log.truncate(length as usize);
            log
        } else {
            String::from("")
        }
    }
}
