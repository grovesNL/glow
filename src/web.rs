use super::*;

use std::cell::RefCell;
use slotmap::{new_key_type, SecondaryMap, SlotMap};
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlRenderingContext, WebGlShader};

#[derive(Debug)]
enum RawRenderingContext {
    WebGl1(WebGlRenderingContext),
    WebGl2(WebGl2RenderingContext),
}

// Workaround for stable Rust
// See https://github.com/orlp/slotmap/blob/b5df4ac7ee8aa795668bf79ebf8929d2f39bec8e/src/lib.rs#L198
type SlotMapWithoutCopy<K, V> = (SlotMap<K, ()>, SecondaryMap<K, V>);

type TrackedResource<K: slotmap::Key, V> = RefCell<SlotMapWithoutCopy<K, V>>;

fn trackedResource<K: slotmap::Key, V>() -> TrackedResource<K, V> {
    RefCell::new((SlotMap::with_key(), SecondaryMap::new()))
}

#[derive(Debug)]
pub struct WebRenderingContext {
    raw: RawRenderingContext,
    shaders: TrackedResource<WebShaderKey, WebGlShader>,
    programs: TrackedResource<WebProgramKey, WebGlProgram>,
}

impl WebRenderingContext {
    pub fn from_webgl1_context(context: WebGlRenderingContext) -> Self {
        WebRenderingContext {
            raw: RawRenderingContext::WebGl1(context),
            shaders: trackedResource(),
            programs: trackedResource(),
        }
    }

    pub fn from_webgl2_context(context: WebGl2RenderingContext) -> Self {
        WebRenderingContext {
            raw: RawRenderingContext::WebGl2(context),
            shaders: trackedResource(),
            programs: trackedResource(),
        }
    }
}

new_key_type! { pub struct WebShaderKey; }
new_key_type! { pub struct WebProgramKey; }

impl RenderingContext for WebRenderingContext {
    type Shader = WebShaderKey;
    type Program = WebProgramKey;

    fn create_shader(&self, shader_type: ShaderType) -> Result<Self::Shader, String> {
        let raw_shader = match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.create_shader(shader_type as u32),
            RawRenderingContext::WebGl2(ref gl) => gl.create_shader(shader_type as u32),
        };

        match raw_shader {
            Some(s) => {
                let key = self.shaders.borrow_mut().0.insert(());
                self.shaders.borrow_mut().1.insert(key, s);
                Ok(key)
            }
            None => Err(String::from("Unable to create shader object")),
        }
    }

    fn shader_source(&self, shader: Self::Shader, source: &str) {
        let shaders = self.shaders.borrow();
        let raw_shader = shaders.1.get(shader).expect("Invalid shader");
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.shader_source(raw_shader, source),
            RawRenderingContext::WebGl2(ref gl) => gl.shader_source(raw_shader, source),
        }
    }

    fn compile_shader(&self, shader: Self::Shader) {
        let shaders = self.shaders.borrow();
        let raw_shader = shaders.1.get(shader).expect("Invalid shader");
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.compile_shader(raw_shader),
            RawRenderingContext::WebGl2(ref gl) => gl.compile_shader(raw_shader),
        }
    }

    fn get_shader_compile_status(&self, shader: Self::Shader) -> bool {
        let shaders = self.shaders.borrow();
        let raw_shader = shaders.1.get(shader).expect("Invalid shader");
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.get_shader_parameter(raw_shader, COMPILE_STATUS)
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.get_shader_parameter(raw_shader, COMPILE_STATUS)
            }
        }.as_bool()
            .unwrap_or(false)
    }

    fn get_shader_info_log(&self, shader: Self::Shader) -> String {
        let shaders = self.shaders.borrow();
        let raw_shader = shaders.1.get(shader).expect("Invalid shader");
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.get_shader_info_log(raw_shader),
            RawRenderingContext::WebGl2(ref gl) => gl.get_shader_info_log(raw_shader),
        }.unwrap_or_else(|| String::from(""))
    }

    fn create_program(&self) -> Result<Self::Program, String> {
        let shaders = self.shaders.borrow();
        let raw_program = match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.create_program(),
            RawRenderingContext::WebGl2(ref gl) => gl.create_program(),
        };

        match raw_program {
            Some(p) => {
                let key = self.programs.borrow_mut().0.insert(());
                self.programs.borrow_mut().1.insert(key, p);
                Ok(key)
            }
            None => Err(String::from("Unable to create program object")),
        }
    }

    fn attach_shader(&self, program: Self::Program, shader: Self::Shader) {
        let programs = self.programs.borrow();
        let shaders = self.shaders.borrow();
        let raw_program = programs.1.get(program).expect("Invalid program");
        let raw_shader = shaders.1.get(shader).expect("Invalid shader");
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.attach_shader(raw_program, raw_shader),
            RawRenderingContext::WebGl2(ref gl) => gl.attach_shader(raw_program, raw_shader),
        }
    }

    fn link_program(&self, program: Self::Program) {
        let programs = self.programs.borrow();
        let raw_program = programs.1.get(program).expect("Invalid program");
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.link_program(raw_program),
            RawRenderingContext::WebGl2(ref gl) => gl.link_program(raw_program),
        }
    }

    fn get_program_link_status(&self, program: Self::Program) -> bool {
        let programs = self.programs.borrow();
        let raw_program = programs.1.get(program).expect("Invalid program");
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.get_program_parameter(raw_program, LINK_STATUS)
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.get_program_parameter(raw_program, LINK_STATUS)
            }
        }.as_bool()
            .unwrap_or(false)
    }

    fn get_program_info_log(&self, program: Self::Program) -> String {
        let programs = self.programs.borrow();
        let raw_program = programs.1.get(program).expect("Invalid program");
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.get_program_info_log(raw_program),
            RawRenderingContext::WebGl2(ref gl) => gl.get_program_info_log(raw_program),
        }.unwrap_or_else(|| String::from(""))
    }
}
