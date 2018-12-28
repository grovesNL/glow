use super::*;

use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlRenderingContext, WebGlShader};

#[derive(Debug)]
enum RawRenderingContext {
    WebGl1(WebGlRenderingContext),
    WebGl2(WebGl2RenderingContext),
}

#[derive(Debug)]
pub struct WebRenderingContext {
    raw: RawRenderingContext,
}

impl WebRenderingContext {
    pub fn from_webgl1_context(context: WebGlRenderingContext) -> Self {
        WebRenderingContext {
            raw: RawRenderingContext::WebGl1(context),
        }
    }

    pub fn from_webgl2_context(context: WebGl2RenderingContext) -> Self {
        WebRenderingContext {
            raw: RawRenderingContext::WebGl2(context),
        }
    }
}

#[derive(Debug)]
pub struct WebShader {
    raw: WebGlShader,
}

#[derive(Debug)]
pub struct WebProgram {
    raw: WebGlProgram,
}

impl RenderingContext for WebRenderingContext {
    type Shader = WebShader;
    type Program = WebProgram;

    fn create_shader(&self, shader_type: ShaderType) -> Result<Self::Shader, String> {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.create_shader(shader_type as u32),
            RawRenderingContext::WebGl2(ref gl) => gl.create_shader(shader_type as u32),
        }.map(|raw| WebShader { raw })
            .ok_or_else(|| String::from("Unable to create shader object"))
    }

    fn shader_source(&self, shader: &Self::Shader, source: &str) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.shader_source(&shader.raw, source),
            RawRenderingContext::WebGl2(ref gl) => gl.shader_source(&shader.raw, source),
        }
    }

    fn compile_shader(&self, shader: &Self::Shader) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.compile_shader(&shader.raw),
            RawRenderingContext::WebGl2(ref gl) => gl.compile_shader(&shader.raw),
        }
    }

    fn get_shader_compile_status(&self, shader: &Self::Shader) -> bool {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.get_shader_parameter(&shader.raw, COMPILE_STATUS)
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.get_shader_parameter(&shader.raw, COMPILE_STATUS)
            }
        }.as_bool()
            .unwrap_or(false)
    }

    fn get_shader_info_log(&self, shader: &Self::Shader) -> String {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.get_shader_info_log(&shader.raw),
            RawRenderingContext::WebGl2(ref gl) => gl.get_shader_info_log(&shader.raw),
        }.unwrap_or_else(|| String::from(""))
    }

    fn create_program(&self) -> Result<Self::Program, String> {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.create_program(),
            RawRenderingContext::WebGl2(ref gl) => gl.create_program(),
        }.map(|raw| WebProgram { raw })
            .ok_or_else(|| String::from("Unable to create program object"))
    }

    fn attach_shader(&self, program: &Self::Program, shader: &Self::Shader) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.attach_shader(&program.raw, &shader.raw),
            RawRenderingContext::WebGl2(ref gl) => gl.attach_shader(&program.raw, &shader.raw),
        }
    }

    fn link_program(&self, program: &Self::Program) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.link_program(&program.raw),
            RawRenderingContext::WebGl2(ref gl) => gl.link_program(&program.raw),
        }
    }

    fn get_program_link_status(&self, program: &Self::Program) -> bool {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.get_program_parameter(&program.raw, LINK_STATUS)
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.get_program_parameter(&program.raw, LINK_STATUS)
            }
        }.as_bool()
            .unwrap_or(false)
    }

    fn get_program_info_log(&self, program: &Self::Program) -> String {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.get_program_info_log(&program.raw),
            RawRenderingContext::WebGl2(ref gl) => gl.get_program_info_log(&program.raw),
        }.unwrap_or_else(|| String::from(""))
    }
}
