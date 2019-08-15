use super::*;

use js_sys::Array;
use slotmap::{new_key_type, SecondaryMap, SlotMap};
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use web_sys::{
    WebGl2RenderingContext, WebGlBuffer, WebGlFramebuffer, WebGlProgram, WebGlRenderbuffer,
    WebGlRenderingContext, WebGlSampler, WebGlShader, WebGlSync, WebGlTexture,
    WebGlUniformLocation, WebGlVertexArrayObject,
};

#[derive(Debug)]
enum RawRenderingContext {
    WebGl1(WebGlRenderingContext),
    WebGl2(WebGl2RenderingContext),
}

// Workaround for stable Rust
// See https://github.com/orlp/slotmap/blob/b5df4ac7ee8aa795668bf79ebf8929d2f39bec8e/src/lib.rs#L198
type SlotMapWithoutCopy<K, V> = (SlotMap<K, ()>, SecondaryMap<K, V>);

type TrackedResource<K, V> = RefCell<SlotMapWithoutCopy<K, V>>;

fn tracked_resource<K: slotmap::Key, V>() -> TrackedResource<K, V> {
    RefCell::new((SlotMap::with_key(), SecondaryMap::new()))
}

#[derive(Debug)]
pub struct Context {
    raw: RawRenderingContext,
    shaders: TrackedResource<WebShaderKey, WebGlShader>,
    programs: TrackedResource<WebProgramKey, WebGlProgram>,
    buffers: TrackedResource<WebBufferKey, WebGlBuffer>,
    vertex_arrays: TrackedResource<WebVertexArrayKey, WebGlVertexArrayObject>,
    textures: TrackedResource<WebTextureKey, WebGlTexture>,
    samplers: TrackedResource<WebSamplerKey, WebGlSampler>,
    fences: TrackedResource<WebFenceKey, WebGlSync>,
    framebuffers: TrackedResource<WebFramebufferKey, WebGlFramebuffer>,
    renderbuffers: TrackedResource<WebRenderbufferKey, WebGlRenderbuffer>,
    uniform_locations: TrackedResource<WebUniformLocationKey, WebGlUniformLocation>,
}

impl Context {
    pub fn from_webgl1_context(context: WebGlRenderingContext) -> Self {
        Context {
            raw: RawRenderingContext::WebGl1(context),
            shaders: tracked_resource(),
            programs: tracked_resource(),
            buffers: tracked_resource(),
            vertex_arrays: tracked_resource(),
            textures: tracked_resource(),
            samplers: tracked_resource(),
            fences: tracked_resource(),
            framebuffers: tracked_resource(),
            renderbuffers: tracked_resource(),
            uniform_locations: tracked_resource(),
        }
    }

    pub fn from_webgl2_context(context: WebGl2RenderingContext) -> Self {
        Context {
            raw: RawRenderingContext::WebGl2(context),
            shaders: tracked_resource(),
            programs: tracked_resource(),
            buffers: tracked_resource(),
            vertex_arrays: tracked_resource(),
            textures: tracked_resource(),
            samplers: tracked_resource(),
            fences: tracked_resource(),
            framebuffers: tracked_resource(),
            renderbuffers: tracked_resource(),
            uniform_locations: tracked_resource(),
        }
    }
}

new_key_type! { pub struct WebShaderKey; }
new_key_type! { pub struct WebProgramKey; }
new_key_type! { pub struct WebBufferKey; }
new_key_type! { pub struct WebVertexArrayKey; }
new_key_type! { pub struct WebTextureKey; }
new_key_type! { pub struct WebSamplerKey; }
new_key_type! { pub struct WebFenceKey; }
new_key_type! { pub struct WebFramebufferKey; }
new_key_type! { pub struct WebRenderbufferKey; }
new_key_type! { pub struct WebUniformLocationKey; }

impl super::Context for Context {
    type Shader = WebShaderKey;
    type Program = WebProgramKey;
    type Buffer = WebBufferKey;
    type VertexArray = WebVertexArrayKey;
    type Texture = WebTextureKey;
    type Sampler = WebSamplerKey;
    type Fence = WebFenceKey;
    type Framebuffer = WebFramebufferKey;
    type Renderbuffer = WebRenderbufferKey;
    type UniformLocation = WebUniformLocationKey;

    unsafe fn create_framebuffer(&self) -> Result<Self::Framebuffer, String> {
        let raw_framebuffer = match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.create_framebuffer(),
            RawRenderingContext::WebGl2(ref gl) => gl.create_framebuffer(),
        };

        match raw_framebuffer {
            Some(s) => {
                let key = self.framebuffers.borrow_mut().0.insert(());
                self.framebuffers.borrow_mut().1.insert(key, s);
                Ok(key)
            }
            None => Err(String::from("Unable to create framebuffer object")),
        }
    }

    unsafe fn create_renderbuffer(&self) -> Result<Self::Renderbuffer, String> {
        let raw_renderbuffer = match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.create_renderbuffer(),
            RawRenderingContext::WebGl2(ref gl) => gl.create_renderbuffer(),
        };

        match raw_renderbuffer {
            Some(s) => {
                let key = self.renderbuffers.borrow_mut().0.insert(());
                self.renderbuffers.borrow_mut().1.insert(key, s);
                Ok(key)
            }
            None => Err(String::from("Unable to create renderbuffer object")),
        }
    }

    unsafe fn create_sampler(&self) -> Result<Self::Sampler, String> {
        let raw_sampler = match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => panic!("Sampler objects are not supported"),
            RawRenderingContext::WebGl2(ref gl) => gl.create_sampler(),
        };

        match raw_sampler {
            Some(s) => {
                let key = self.samplers.borrow_mut().0.insert(());
                self.samplers.borrow_mut().1.insert(key, s);
                Ok(key)
            }
            None => Err(String::from("Unable to create sampler object")),
        }
    }

    unsafe fn create_shader(&self, shader_type: u32) -> Result<Self::Shader, String> {
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

    unsafe fn create_texture(&self) -> Result<Self::Texture, String> {
        let raw_texture = match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.create_texture(),
            RawRenderingContext::WebGl2(ref gl) => gl.create_texture(),
        };

        match raw_texture {
            Some(t) => {
                let key = self.textures.borrow_mut().0.insert(());
                self.textures.borrow_mut().1.insert(key, t);
                Ok(key)
            }
            None => Err(String::from("Unable to create texture object")),
        }
    }

    unsafe fn delete_shader(&self, shader: Self::Shader) {
        let mut shaders = self.shaders.borrow_mut();
        match shaders.1.remove(shader) {
            Some(ref s) => match self.raw {
                RawRenderingContext::WebGl1(ref gl) => gl.delete_shader(Some(s)),
                RawRenderingContext::WebGl2(ref gl) => gl.delete_shader(Some(s)),
            },
            None => {}
        }
    }

    unsafe fn shader_source(&self, shader: Self::Shader, source: &str) {
        let shaders = self.shaders.borrow();
        let raw_shader = shaders.1.get_unchecked(shader);
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.shader_source(raw_shader, source),
            RawRenderingContext::WebGl2(ref gl) => gl.shader_source(raw_shader, source),
        }
    }

    unsafe fn compile_shader(&self, shader: Self::Shader) {
        let shaders = self.shaders.borrow();
        let raw_shader = shaders.1.get_unchecked(shader);
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.compile_shader(raw_shader),
            RawRenderingContext::WebGl2(ref gl) => gl.compile_shader(raw_shader),
        }
    }

    unsafe fn get_shader_compile_status(&self, shader: Self::Shader) -> bool {
        let shaders = self.shaders.borrow();
        let raw_shader = shaders.1.get_unchecked(shader);
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.get_shader_parameter(raw_shader, COMPILE_STATUS)
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.get_shader_parameter(raw_shader, COMPILE_STATUS)
            }
        }
        .as_bool()
        .unwrap_or(false)
    }

    unsafe fn get_shader_info_log(&self, shader: Self::Shader) -> String {
        let shaders = self.shaders.borrow();
        let raw_shader = shaders.1.get_unchecked(shader);
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.get_shader_info_log(raw_shader),
            RawRenderingContext::WebGl2(ref gl) => gl.get_shader_info_log(raw_shader),
        }
        .unwrap_or_else(|| String::from(""))
    }

    unsafe fn get_tex_image_u8_slice(
        &self,
        _target: u32,
        _level: i32,
        _format: u32,
        _ty: u32,
        _pixels: Option<&[u8]>,
    ) {
        panic!("Get tex image is not supported");
    }

    unsafe fn get_tex_image_pixel_buffer_offset(
        &self,
        _target: u32,
        _level: i32,
        _format: u32,
        _ty: u32,
        _pixel_buffer_offset: i32,
    ) {
        panic!("Get tex image is not supported");
    }

    unsafe fn create_program(&self) -> Result<Self::Program, String> {
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

    unsafe fn delete_program(&self, program: Self::Program) {
        let mut programs = self.programs.borrow_mut();
        match programs.1.remove(program) {
            Some(ref p) => match self.raw {
                RawRenderingContext::WebGl1(ref gl) => gl.delete_program(Some(p)),
                RawRenderingContext::WebGl2(ref gl) => gl.delete_program(Some(p)),
            },
            None => {}
        }
    }

    unsafe fn attach_shader(&self, program: Self::Program, shader: Self::Shader) {
        let programs = self.programs.borrow();
        let shaders = self.shaders.borrow();
        let raw_program = programs.1.get_unchecked(program);
        let raw_shader = shaders.1.get_unchecked(shader);
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.attach_shader(raw_program, raw_shader),
            RawRenderingContext::WebGl2(ref gl) => gl.attach_shader(raw_program, raw_shader),
        }
    }

    unsafe fn detach_shader(&self, program: Self::Program, shader: Self::Shader) {
        let programs = self.programs.borrow();
        let shaders = self.shaders.borrow();
        let raw_program = programs.1.get_unchecked(program);
        let raw_shader = shaders.1.get_unchecked(shader);
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.detach_shader(raw_program, raw_shader),
            RawRenderingContext::WebGl2(ref gl) => gl.detach_shader(raw_program, raw_shader),
        }
    }

    unsafe fn link_program(&self, program: Self::Program) {
        let programs = self.programs.borrow();
        let raw_program = programs.1.get_unchecked(program);
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.link_program(raw_program),
            RawRenderingContext::WebGl2(ref gl) => gl.link_program(raw_program),
        }
    }

    unsafe fn get_program_link_status(&self, program: Self::Program) -> bool {
        let programs = self.programs.borrow();
        let raw_program = programs.1.get_unchecked(program);
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.get_program_parameter(raw_program, LINK_STATUS)
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.get_program_parameter(raw_program, LINK_STATUS)
            }
        }
        .as_bool()
        .unwrap_or(false)
    }

    unsafe fn get_program_info_log(&self, program: Self::Program) -> String {
        let programs = self.programs.borrow();
        let raw_program = programs.1.get_unchecked(program);
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.get_program_info_log(raw_program),
            RawRenderingContext::WebGl2(ref gl) => gl.get_program_info_log(raw_program),
        }
        .unwrap_or_else(|| String::from(""))
    }

    unsafe fn get_active_uniforms(&self, program: Self::Program) -> u32 {
        let programs = self.programs.borrow();
        let raw_program = programs.1.get_unchecked(program);
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.get_program_parameter(raw_program, WebGlRenderingContext::ACTIVE_UNIFORMS)
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.get_program_parameter(raw_program, WebGl2RenderingContext::ACTIVE_UNIFORMS)
            }
        }
        .as_f64()
        .map(|v| v as u32)
        .unwrap_or(0)
    }

    unsafe fn get_active_uniform(
        &self,
        program: Self::Program,
        index: u32,
    ) -> Option<ActiveUniform> {
        let programs = self.programs.borrow();
        let raw_program = programs.1.get_unchecked(program);
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.get_active_uniform(raw_program, index)
                    .map(|au| ActiveUniform {
                        size: au.size(),
                        utype: au.type_(),
                        name: au.name(),
                    })
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.get_active_uniform(raw_program, index)
                    .map(|au| ActiveUniform {
                        size: au.size(),
                        utype: au.type_(),
                        name: au.name(),
                    })
            }
        }
    }

    unsafe fn use_program(&self, program: Option<Self::Program>) {
        let programs = self.programs.borrow();
        let raw_program = program.map(|p| programs.1.get_unchecked(p));
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.use_program(raw_program),
            RawRenderingContext::WebGl2(ref gl) => gl.use_program(raw_program),
        }
    }

    unsafe fn create_buffer(&self) -> Result<Self::Buffer, String> {
        let raw_buffer = match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.create_buffer(),
            RawRenderingContext::WebGl2(ref gl) => gl.create_buffer(),
        };

        match raw_buffer {
            Some(p) => {
                let key = self.buffers.borrow_mut().0.insert(());
                self.buffers.borrow_mut().1.insert(key, p);
                Ok(key)
            }
            None => Err(String::from("Unable to create buffer object")),
        }
    }

    unsafe fn bind_buffer(&self, target: u32, buffer: Option<Self::Buffer>) {
        let buffers = self.buffers.borrow();
        let raw_buffer = buffer.map(|b| buffers.1.get_unchecked(b));
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.bind_buffer(target, raw_buffer),
            RawRenderingContext::WebGl2(ref gl) => gl.bind_buffer(target, raw_buffer),
        }
    }

    unsafe fn bind_buffer_range(
        &self,
        target: u32,
        index: u32,
        buffer: Option<Self::Buffer>,
        offset: i32,
        size: i32,
    ) {
        let buffers = self.buffers.borrow();
        let raw_buffer = buffer.map(|b| buffers.1.get_unchecked(b));
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => {
                panic!("bind_buffer_range not supported on webgl1");
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.bind_buffer_range_with_i32_and_i32(target, index, raw_buffer, offset, size);
            }
        }
    }

    unsafe fn bind_framebuffer(&self, target: u32, framebuffer: Option<Self::Framebuffer>) {
        let framebuffers = self.framebuffers.borrow();
        let raw_framebuffer = framebuffer.map(|f| framebuffers.1.get_unchecked(f));
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.bind_framebuffer(target, raw_framebuffer),
            RawRenderingContext::WebGl2(ref gl) => gl.bind_framebuffer(target, raw_framebuffer),
        }
    }

    unsafe fn bind_renderbuffer(&self, target: u32, renderbuffer: Option<Self::Renderbuffer>) {
        let renderbuffers = self.renderbuffers.borrow();
        let raw_renderbuffer = renderbuffer.map(|r| renderbuffers.1.get_unchecked(r));
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.bind_renderbuffer(target, raw_renderbuffer),
            RawRenderingContext::WebGl2(ref gl) => gl.bind_renderbuffer(target, raw_renderbuffer),
        }
    }

    unsafe fn blit_framebuffer(
        &self,
        src_x0: i32,
        src_y0: i32,
        src_x1: i32,
        src_y1: i32,
        dst_x0: i32,
        dst_y0: i32,
        dst_x1: i32,
        dst_y1: i32,
        mask: u32,
        filter: u32,
    ) {
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => {
                panic!("framebuffer blitting usupported in webgl1")
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.blit_framebuffer(
                    src_x0, src_y0, src_x1, src_y1, dst_x0, dst_y0, dst_x1, dst_y1, mask, filter,
                );
            }
        }
    }

    unsafe fn create_vertex_array(&self) -> Result<Self::VertexArray, String> {
        let raw_vertex_array = match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => {
                panic!("Vertex array objects are not supported"); // TODO: Extension
            }
            RawRenderingContext::WebGl2(ref gl) => gl.create_vertex_array(),
        };

        match raw_vertex_array {
            Some(va) => {
                let key = self.vertex_arrays.borrow_mut().0.insert(());
                self.vertex_arrays.borrow_mut().1.insert(key, va);
                Ok(key)
            }
            None => Err(String::from("Unable to create vertex array object")),
        }
    }

    unsafe fn delete_vertex_array(&self, vertex_array: Self::VertexArray) {
        let mut vertex_arrays = self.vertex_arrays.borrow_mut();
        match vertex_arrays.1.remove(vertex_array) {
            Some(ref va) => match self.raw {
                RawRenderingContext::WebGl1(ref _gl) => {
                    panic!("Vertex array objects are not supported"); // TODO: Extension
                }
                RawRenderingContext::WebGl2(ref gl) => gl.delete_vertex_array(Some(va)),
            },
            None => {}
        }
    }

    unsafe fn bind_vertex_array(&self, vertex_array: Option<Self::VertexArray>) {
        let vertex_arrays = self.vertex_arrays.borrow();
        let raw_vertex_array = vertex_array.map(|va| vertex_arrays.1.get_unchecked(va));
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => {
                panic!("Vertex array objects are not supported"); // TODO: Extension
            }
            RawRenderingContext::WebGl2(ref gl) => gl.bind_vertex_array(raw_vertex_array),
        }
    }

    unsafe fn clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.clear_color(red, green, blue, alpha),
            RawRenderingContext::WebGl2(ref gl) => gl.clear_color(red, green, blue, alpha),
        }
    }

    unsafe fn supports_f64_precision() -> bool {
        false
    }

    unsafe fn clear_depth_f64(&self, _depth: f64) {
        panic!("64-bit float precision is not supported in WebGL");
    }

    unsafe fn clear_depth_f32(&self, depth: f32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.clear_depth(depth),
            RawRenderingContext::WebGl2(ref gl) => gl.clear_depth(depth),
        }
    }

    unsafe fn clear_stencil(&self, stencil: i32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.clear_stencil(stencil),
            RawRenderingContext::WebGl2(ref gl) => gl.clear_stencil(stencil),
        }
    }

    unsafe fn clear(&self, mask: u32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.clear(mask),
            RawRenderingContext::WebGl2(ref gl) => gl.clear(mask),
        }
    }

    unsafe fn patch_parameter_i32(&self, _parameter: u32, _value: i32) {
        panic!("Patch parameter is not supported");
    }

    unsafe fn pixel_store_i32(&self, parameter: u32, value: i32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.pixel_storei(parameter, value),
            RawRenderingContext::WebGl2(ref gl) => gl.pixel_storei(parameter, value),
        }
    }

    unsafe fn pixel_store_bool(&self, parameter: u32, value: bool) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.pixel_storei(parameter, value as i32),
            RawRenderingContext::WebGl2(ref gl) => gl.pixel_storei(parameter, value as i32),
        }
    }

    unsafe fn bind_frag_data_location(
        &self,
        _program: Self::Program,
        _color_number: u32,
        _name: &str,
    ) {
        panic!("Bind frag data location is not supported");
    }

    unsafe fn buffer_data_size(&self, target: u32, size: i32, usage: u32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.buffer_data_with_i32(target, size, usage),
            RawRenderingContext::WebGl2(ref gl) => gl.buffer_data_with_i32(target, size, usage),
        }
    }

    unsafe fn buffer_data_u8_slice(&self, target: u32, data: &[u8], usage: u32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.buffer_data_with_u8_array(target, data, usage)
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.buffer_data_with_u8_array(target, data, usage)
            }
        }
    }

    unsafe fn buffer_sub_data_u8_slice(&self, target: u32, offset: i32, src_data: &[u8]) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.buffer_sub_data_with_i32_and_u8_array(target, offset, src_data)
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.buffer_sub_data_with_i32_and_u8_array(target, offset, src_data)
            }
        }
    }

    unsafe fn get_buffer_sub_data(&self, target: u32, offset: i32, dst_data: &mut [u8]) {
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => panic!("get_buffer_sub_data not supported"),
            RawRenderingContext::WebGl2(ref gl) => {
                gl.get_buffer_sub_data_with_i32_and_u8_array(target, offset, dst_data)
            }
        }
    }

    unsafe fn buffer_storage(
        &self,
        _target: u32,
        _size: i32,
        _data: Option<&mut [u8]>,
        _flags: u32,
    ) {
        panic!("Buffer storage is not supported");
    }

    unsafe fn check_framebuffer_status(&self, target: u32) -> u32 {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.check_framebuffer_status(target),
            RawRenderingContext::WebGl2(ref gl) => gl.check_framebuffer_status(target),
        }
    }

    unsafe fn clear_buffer_i32_slice(&self, target: u32, draw_buffer: u32, values: &mut [i32]) {
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => {
                panic!("Clear buffer with `i32` slice is not supported");
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.clear_bufferiv_with_i32_array(target, draw_buffer as i32, values);
            }
        }
    }

    unsafe fn clear_buffer_u32_slice(&self, target: u32, draw_buffer: u32, values: &mut [u32]) {
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => {
                panic!("Clear buffer with `u32` slice is not supported")
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.clear_bufferuiv_with_u32_array(target, draw_buffer as i32, values)
            }
        }
    }

    unsafe fn clear_buffer_f32_slice(&self, target: u32, draw_buffer: u32, values: &mut [f32]) {
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => {
                panic!("Clear buffer with `f32` slice is not supported")
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.clear_bufferfv_with_f32_array(target, draw_buffer as i32, values)
            }
        }
    }

    unsafe fn clear_buffer_depth_stencil(
        &self,
        target: u32,
        draw_buffer: u32,
        depth: f32,
        stencil: i32,
    ) {
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => {
                panic!("Clear buffer depth stencil is not supported")
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.clear_bufferfi(target, draw_buffer as i32, depth, stencil)
            }
        }
    }

    unsafe fn client_wait_sync(&self, _fence: Self::Fence, _flags: u32, _timeout: i32) -> u32 {
        panic!("Client wait sync is not supported")
    }

    unsafe fn copy_buffer_sub_data(
        &self,
        src_target: u32,
        dst_target: u32,
        src_offset: i32,
        dst_offset: i32,
        size: i32,
    ) {
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => panic!("Copy buffer subdata is not supported"),
            RawRenderingContext::WebGl2(ref gl) => gl
                .copy_buffer_sub_data_with_i32_and_i32_and_i32(
                    src_target, dst_target, src_offset, dst_offset, size,
                ),
        }
    }

    unsafe fn delete_buffer(&self, buffer: Self::Buffer) {
        let mut buffers = self.buffers.borrow_mut();
        match buffers.1.remove(buffer) {
            Some(ref b) => match self.raw {
                RawRenderingContext::WebGl1(ref gl) => gl.delete_buffer(Some(b)),
                RawRenderingContext::WebGl2(ref gl) => gl.delete_buffer(Some(b)),
            },
            None => {}
        }
    }

    unsafe fn delete_framebuffer(&self, framebuffer: Self::Framebuffer) {
        let mut framebuffers = self.framebuffers.borrow_mut();
        match framebuffers.1.remove(framebuffer) {
            Some(ref f) => match self.raw {
                RawRenderingContext::WebGl1(ref gl) => gl.delete_framebuffer(Some(f)),
                RawRenderingContext::WebGl2(ref gl) => gl.delete_framebuffer(Some(f)),
            },
            None => {}
        }
    }

    unsafe fn delete_renderbuffer(&self, renderbuffer: Self::Renderbuffer) {
        let mut renderbuffers = self.renderbuffers.borrow_mut();
        match renderbuffers.1.remove(renderbuffer) {
            Some(ref r) => match self.raw {
                RawRenderingContext::WebGl1(ref gl) => gl.delete_renderbuffer(Some(r)),
                RawRenderingContext::WebGl2(ref gl) => gl.delete_renderbuffer(Some(r)),
            },
            None => {}
        }
    }

    unsafe fn delete_sampler(&self, sampler: Self::Sampler) {
        let mut samplers = self.samplers.borrow_mut();
        match samplers.1.remove(sampler) {
            Some(ref s) => match self.raw {
                RawRenderingContext::WebGl1(ref _gl) => panic!("Samplers are not supported"),
                RawRenderingContext::WebGl2(ref gl) => gl.delete_sampler(Some(s)),
            },
            None => {}
        }
    }

    unsafe fn delete_sync(&self, fence: Self::Fence) {
        let mut fences = self.fences.borrow_mut();
        match fences.1.remove(fence) {
            Some(ref f) => match self.raw {
                RawRenderingContext::WebGl1(ref _gl) => panic!("Fences are not supported"),
                RawRenderingContext::WebGl2(ref gl) => gl.delete_sync(Some(f)),
            },
            None => {}
        }
    }

    unsafe fn delete_texture(&self, texture: Self::Texture) {
        let mut textures = self.textures.borrow_mut();
        match textures.1.remove(texture) {
            Some(ref t) => match self.raw {
                RawRenderingContext::WebGl1(ref gl) => gl.delete_texture(Some(t)),
                RawRenderingContext::WebGl2(ref gl) => gl.delete_texture(Some(t)),
            },
            None => {}
        }
    }

    unsafe fn disable(&self, parameter: u32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.disable(parameter),
            RawRenderingContext::WebGl2(ref gl) => gl.disable(parameter),
        }
    }

    unsafe fn disable_draw_buffer(&self, _parameter: u32, _draw_buffer: u32) {
        panic!("Draw buffer disable is not supported");
    }

    unsafe fn disable_vertex_attrib_array(&self, index: u32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.disable_vertex_attrib_array(index),
            RawRenderingContext::WebGl2(ref gl) => gl.disable_vertex_attrib_array(index),
        }
    }

    unsafe fn dispatch_compute(&self, _groups_x: u32, _groups_y: u32, _groups_z: u32) {
        panic!("Dispatch compute is not supported");
    }

    unsafe fn dispatch_compute_indirect(&self, _offset: i32) {
        panic!("Dispatch compute indirect is not supported");
    }

    unsafe fn draw_arrays(&self, mode: u32, first: i32, count: i32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.draw_arrays(mode as u32, first, count),
            RawRenderingContext::WebGl2(ref gl) => gl.draw_arrays(mode as u32, first, count),
        }
    }

    unsafe fn draw_arrays_instanced(&self, mode: u32, first: i32, count: i32, instance_count: i32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => {
                panic!("Draw arrays instanced is not supported") // TODO: Extension
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.draw_arrays_instanced(mode as u32, first, count, instance_count)
            }
        }
    }

    unsafe fn draw_arrays_instanced_base_instance(
        &self,
        _mode: u32,
        _first: i32,
        _count: i32,
        _instance_count: i32,
        _base_instance: u32,
    ) {
        panic!("Draw arrays instanced base instance is not supported");
    }

    unsafe fn draw_buffer(&self, draw_buffer: u32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => {
                panic!("draw_buffer not supported on webgl1");
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.draw_buffers(&Array::of1(&draw_buffer.into()));
            }
        }
    }

    unsafe fn draw_buffers(&self, buffers: &[u32]) {
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => {
                panic!("draw_buffers not supported on webgl1");
            }
            RawRenderingContext::WebGl2(ref gl) => {
                let js_buffers = Array::new();
                for &b in buffers {
                    js_buffers.push(&b.into());
                }
                gl.draw_buffers(&js_buffers);
            }
        }
    }

    unsafe fn draw_elements(&self, mode: u32, count: i32, element_type: u32, offset: i32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.draw_elements_with_i32(mode as u32, count, element_type as u32, offset);
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.draw_elements_with_i32(mode as u32, count, element_type as u32, offset);
            }
        }
    }

    unsafe fn draw_elements_base_vertex(
        &self,
        _mode: u32,
        _count: i32,
        _element_type: u32,
        _offset: i32,
        _base_vertex: i32,
    ) {
        panic!("Draw elements base vertex is not supported");
    }

    unsafe fn draw_elements_instanced(
        &self,
        mode: u32,
        count: i32,
        element_type: u32,
        offset: i32,
        instance_count: i32,
    ) {
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => {
                panic!("Draw elements instanced is not supported") // TODO: Extension
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.draw_elements_instanced_with_i32(
                    mode as u32,
                    count,
                    element_type as u32,
                    offset,
                    instance_count,
                );
            }
        }
    }

    unsafe fn draw_elements_instanced_base_vertex(
        &self,
        _mode: u32,
        _count: i32,
        _element_type: u32,
        _offset: i32,
        _instance_count: i32,
        _base_vertex: i32,
    ) {
        panic!("Draw elements instanced base vertex is not supported");
    }

    unsafe fn draw_elements_instanced_base_vertex_base_instance(
        &self,
        _mode: u32,
        _count: i32,
        _element_type: u32,
        _offset: i32,
        _instance_count: i32,
        _base_vertex: i32,
        _base_instance: u32,
    ) {
        panic!("Draw elements instanced base vertex base instance is not supported");
    }

    unsafe fn enable(&self, parameter: u32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.enable(parameter),
            RawRenderingContext::WebGl2(ref gl) => gl.enable(parameter),
        }
    }

    unsafe fn enable_draw_buffer(&self, _parameter: u32, _draw_buffer: u32) {
        panic!("Draw buffer enable is not supported");
    }

    unsafe fn enable_vertex_attrib_array(&self, index: u32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.enable_vertex_attrib_array(index),
            RawRenderingContext::WebGl2(ref gl) => gl.enable_vertex_attrib_array(index),
        }
    }

    unsafe fn flush(&self) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.flush(),
            RawRenderingContext::WebGl2(ref gl) => gl.flush(),
        }
    }

    unsafe fn framebuffer_renderbuffer(
        &self,
        target: u32,
        attachment: u32,
        renderbuffer_target: u32,
        renderbuffer: Option<Self::Renderbuffer>,
    ) {
        let renderbuffers = self.renderbuffers.borrow();
        let raw_renderbuffer = renderbuffer.map(|r| renderbuffers.1.get_unchecked(r));
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => {
                panic!("Framebuffer renderbuffer is not supported");
            }
            RawRenderingContext::WebGl2(ref gl) => gl.framebuffer_renderbuffer(
                target,
                attachment,
                renderbuffer_target,
                raw_renderbuffer,
            ),
        }
    }

    unsafe fn framebuffer_texture(
        &self,
        _target: u32,
        _attachment: u32,
        _texture: Option<Self::Texture>,
        _level: i32,
    ) {
        panic!("Framebuffer texture is not supported");
    }

    unsafe fn framebuffer_texture_2d(
        &self,
        target: u32,
        attachment: u32,
        texture_target: u32,
        texture: Option<Self::Texture>,
        level: i32,
    ) {
        let textures = self.textures.borrow();
        let raw_texture = texture.map(|t| textures.1.get_unchecked(t));
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.framebuffer_texture_2d(target, attachment, texture_target, raw_texture, level);
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.framebuffer_texture_2d(target, attachment, texture_target, raw_texture, level);
            }
        }
    }

    unsafe fn framebuffer_texture_3d(
        &self,
        _target: u32,
        _attachment: u32,
        _texture_target: u32,
        _texture: Option<Self::Texture>,
        _level: i32,
        _layer: i32,
    ) {
        panic!("Framebuffer texture 3D is not supported");
    }

    unsafe fn framebuffer_texture_layer(
        &self,
        target: u32,
        attachment: u32,
        texture: Option<Self::Texture>,
        level: i32,
        layer: i32,
    ) {
        let textures = self.textures.borrow();
        let raw_texture = texture.map(|t| textures.1.get_unchecked(t));
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => {
                panic!("Framebuffer texture layer is not supported");
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.framebuffer_texture_layer(target, attachment, raw_texture, level, layer);
            }
        }
    }

    unsafe fn front_face(&self, value: u32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.front_face(value as u32),
            RawRenderingContext::WebGl2(ref gl) => gl.front_face(value as u32),
        }
    }

    unsafe fn get_error(&self) -> u32 {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.get_error(),
            RawRenderingContext::WebGl2(ref gl) => gl.get_error(),
        }
    }

    unsafe fn get_parameter_i32(&self, parameter: u32) -> i32 {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.get_parameter(parameter),
            RawRenderingContext::WebGl2(ref gl) => gl.get_parameter(parameter),
        }
        .unwrap()
        .as_f64()
        .map(|v| v as i32)
        // Errors will be caught by the browser or through `get_error`
        // so return a default instead
        .unwrap_or(0)
    }

    unsafe fn get_parameter_indexed_i32(&self, parameter: u32, index: u32) -> i32 {
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => {
                panic!("Get parameter indexed is not supported")
            }
            RawRenderingContext::WebGl2(ref gl) => gl.get_indexed_parameter(parameter, index),
        }
        .unwrap()
        .as_f64()
        .map(|v| v as i32)
        // Errors will be caught by the browser or through `get_error`
        // so return a default instead
        .unwrap_or(0)
    }

    unsafe fn get_parameter_indexed_string(&self, parameter: u32, index: u32) -> String {
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => {
                panic!("Get parameter indexed is not supported")
            }
            RawRenderingContext::WebGl2(ref gl) => gl.get_indexed_parameter(parameter, index),
        }
        .unwrap()
        .as_string()
        // Errors will be caught by the browser or through `get_error`
        // so return a default instead
        .unwrap_or(String::from(""))
    }

    unsafe fn get_parameter_string(&self, parameter: u32) -> String {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.get_parameter(parameter),
            RawRenderingContext::WebGl2(ref gl) => gl.get_parameter(parameter),
        }
        .unwrap()
        .as_string()
        // Errors will be caught by the browser or through `get_error`
        // so return a default instead
        .unwrap_or(String::from(""))
    }

    unsafe fn get_uniform_location(
        &self,
        program: Self::Program,
        name: &str,
    ) -> Option<Self::UniformLocation> {
        let programs = self.programs.borrow();
        let raw_program = programs.1.get_unchecked(program);
        let raw_uniform = match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.get_uniform_location(raw_program, name),
            RawRenderingContext::WebGl2(ref gl) => gl.get_uniform_location(raw_program, name),
        };
        if let Some(u) = raw_uniform {
            let mut uniform_locations = self.uniform_locations.borrow_mut();
            let key = uniform_locations.0.insert(());
            uniform_locations.1.insert(key, u);
            Some(key)
        } else {
            None
        }
    }

    unsafe fn get_attrib_location(&self, program: Self::Program, name: &str) -> i32 {
        let programs = self.programs.borrow();
        let raw_program = programs.1.get_unchecked(program);
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.get_attrib_location(raw_program, name),
            RawRenderingContext::WebGl2(ref gl) => gl.get_attrib_location(raw_program, name),
        }
    }

    unsafe fn bind_attrib_location(&self, program: Self::Program, index: u32, name: &str) {
        let programs = self.programs.borrow();
        let raw_program = programs.1.get_unchecked(program);
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.bind_attrib_location(raw_program, index, name),
            RawRenderingContext::WebGl2(ref gl) => gl.bind_attrib_location(raw_program, index, name),
        }
    }

    unsafe fn get_sync_status(&self, fence: Self::Fence) -> u32 {
        let fences = self.fences.borrow();
        let raw_fence = fences.1.get_unchecked(fence);
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => panic!("Sync is not supported"),
            RawRenderingContext::WebGl2(ref gl) => {
                gl.get_sync_parameter(raw_fence, SYNC_STATUS)
                    .as_f64()
                    .map(|v| v as u32)
                    .unwrap_or(UNSIGNALED)
            }
        }
    }

    unsafe fn is_sync(&self, fence: Self::Fence) -> bool {
        let fences = self.fences.borrow();
        let raw_fence = fences.1.get_unchecked(fence);
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => panic!("Sync is not supported"),
            RawRenderingContext::WebGl2(ref gl) => gl.is_sync(Some(raw_fence)),
        }
    }

    unsafe fn renderbuffer_storage(
        &self,
        target: u32,
        internal_format: u32,
        width: i32,
        height: i32,
    ) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.renderbuffer_storage(target, internal_format, width, height);
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.renderbuffer_storage(target, internal_format, width, height);
            }
        }
    }

    unsafe fn sampler_parameter_f32(&self, sampler: Self::Sampler, name: u32, value: f32) {
        let samplers = self.samplers.borrow();
        let raw_sampler = samplers.1.get_unchecked(sampler);
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => {
                panic!("Samper parameter for `f32` is not supported")
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.sampler_parameterf(raw_sampler, name, value);
            }
        }
    }

    unsafe fn sampler_parameter_f32_slice(
        &self,
        _sampler: Self::Sampler,
        _name: u32,
        _value: &mut [f32],
    ) {
        panic!("Sampler parameter for `f32` slice is not supported");
    }

    unsafe fn sampler_parameter_i32(&self, sampler: Self::Sampler, name: u32, value: i32) {
        let samplers = self.samplers.borrow();
        let raw_sampler = samplers.1.get_unchecked(sampler);
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => {
                panic!("Samper parameter for `i32` is not supported")
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.sampler_parameteri(raw_sampler, name, value);
            }
        }
    }

    unsafe fn tex_image_2d(
        &self,
        target: u32,
        level: i32,
        internal_format: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        ty: u32,
        pixels: Option<&[u8]>,
    ) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                // TODO: Handle return value?
                gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                    target,
                    level,
                    internal_format,
                    width,
                    height,
                    border,
                    format,
                    ty,
                    pixels,
                )
                .unwrap();
            }
            RawRenderingContext::WebGl2(ref gl) => {
                // TODO: Handle return value?
                gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                    target,
                    level,
                    internal_format,
                    width,
                    height,
                    border,
                    format,
                    ty,
                    pixels,
                )
                .unwrap();
            }
        }
    }

    unsafe fn tex_image_3d(
        &self,
        target: u32,
        level: i32,
        internal_format: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        format: u32,
        ty: u32,
        pixels: Option<&[u8]>,
    ) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                panic!("3d textures are not supported")
            }
            RawRenderingContext::WebGl2(ref gl) => {
                // TODO: Handle return value?
                gl.tex_image_3d_with_opt_u8_array(
                    target,
                    level,
                    internal_format,
                    width,
                    height,
                    depth,
                    border,
                    format,
                    ty,
                    pixels,
                )
                .unwrap();
            }
        }
    }

    unsafe fn tex_storage_2d(
        &self,
        target: u32,
        levels: i32,
        internal_format: u32,
        width: i32,
        height: i32,
    ) {
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => panic!("Tex storage 2D is not supported"),
            RawRenderingContext::WebGl2(ref gl) => {
                gl.tex_storage_2d(target, levels, internal_format, width, height);
            }
        }
    }

    unsafe fn tex_storage_3d(
        &self,
        target: u32,
        levels: i32,
        internal_format: u32,
        width: i32,
        height: i32,
        depth: i32,
    ) {
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => panic!("Tex storage 3D is not supported"),
            RawRenderingContext::WebGl2(ref gl) => {
                gl.tex_storage_3d(target, levels, internal_format, width, height, depth);
            }
        }
    }

    unsafe fn uniform_1_i32(&self, uniform_location: Option<Self::UniformLocation>, x: i32) {
        let uniform_locations = self.uniform_locations.borrow();
        let raw_uniform_location = uniform_location.map(|u| uniform_locations.1.get_unchecked(u));
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.uniform1i(raw_uniform_location, x),
            RawRenderingContext::WebGl2(ref gl) => gl.uniform1i(raw_uniform_location, x),
        }
    }

    unsafe fn uniform_2_i32(
        &self,
        uniform_location: Option<Self::UniformLocation>,
        x: i32,
        y: i32,
    ) {
        let uniform_locations = self.uniform_locations.borrow();
        let raw_uniform_location = uniform_location.map(|u| uniform_locations.1.get_unchecked(u));
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.uniform2i(raw_uniform_location, x, y),
            RawRenderingContext::WebGl2(ref gl) => gl.uniform2i(raw_uniform_location, x, y),
        }
    }

    unsafe fn uniform_3_i32(
        &self,
        uniform_location: Option<Self::UniformLocation>,
        x: i32,
        y: i32,
        z: i32,
    ) {
        let uniform_locations = self.uniform_locations.borrow();
        let raw_uniform_location = uniform_location.map(|u| uniform_locations.1.get_unchecked(u));
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.uniform3i(raw_uniform_location, x, y, z),
            RawRenderingContext::WebGl2(ref gl) => gl.uniform3i(raw_uniform_location, x, y, z),
        }
    }

    unsafe fn uniform_4_i32(
        &self,
        uniform_location: Option<Self::UniformLocation>,
        x: i32,
        y: i32,
        z: i32,
        w: i32,
    ) {
        let uniform_locations = self.uniform_locations.borrow();
        let raw_uniform_location = uniform_location.map(|u| uniform_locations.1.get_unchecked(u));
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.uniform4i(raw_uniform_location, x, y, z, w),
            RawRenderingContext::WebGl2(ref gl) => gl.uniform4i(raw_uniform_location, x, y, z, w),
        }
    }

    unsafe fn uniform_1_i32_slice(
        &self,
        uniform_location: Option<Self::UniformLocation>,
        v: &mut [i32; 1],
    ) {
        let uniform_locations = self.uniform_locations.borrow();
        let raw_uniform_location = uniform_location.map(|u| uniform_locations.1.get_unchecked(u));
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.uniform1iv_with_i32_array(raw_uniform_location, v)
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.uniform1iv_with_i32_array(raw_uniform_location, v)
            }
        }
    }

    unsafe fn uniform_2_i32_slice(
        &self,
        uniform_location: Option<Self::UniformLocation>,
        v: &mut [i32; 2],
    ) {
        let uniform_locations = self.uniform_locations.borrow();
        let raw_uniform_location = uniform_location.map(|u| uniform_locations.1.get_unchecked(u));
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.uniform2iv_with_i32_array(raw_uniform_location, v)
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.uniform2iv_with_i32_array(raw_uniform_location, v)
            }
        }
    }

    unsafe fn uniform_3_i32_slice(
        &self,
        uniform_location: Option<Self::UniformLocation>,
        v: &mut [i32; 3],
    ) {
        let uniform_locations = self.uniform_locations.borrow();
        let raw_uniform_location = uniform_location.map(|u| uniform_locations.1.get_unchecked(u));
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.uniform3iv_with_i32_array(raw_uniform_location, v)
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.uniform3iv_with_i32_array(raw_uniform_location, v)
            }
        }
    }

    unsafe fn uniform_4_i32_slice(
        &self,
        uniform_location: Option<Self::UniformLocation>,
        v: &mut [i32; 4],
    ) {
        let uniform_locations = self.uniform_locations.borrow();
        let raw_uniform_location = uniform_location.map(|u| uniform_locations.1.get_unchecked(u));
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.uniform4iv_with_i32_array(raw_uniform_location, v)
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.uniform4iv_with_i32_array(raw_uniform_location, v)
            }
        }
    }

    unsafe fn uniform_1_f32(&self, uniform_location: Option<Self::UniformLocation>, x: f32) {
        let uniform_locations = self.uniform_locations.borrow();
        let raw_uniform_location = uniform_location.map(|u| uniform_locations.1.get_unchecked(u));
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.uniform1f(raw_uniform_location, x),
            RawRenderingContext::WebGl2(ref gl) => gl.uniform1f(raw_uniform_location, x),
        }
    }

    unsafe fn uniform_2_f32(
        &self,
        uniform_location: Option<Self::UniformLocation>,
        x: f32,
        y: f32,
    ) {
        let uniform_locations = self.uniform_locations.borrow();
        let raw_uniform_location = uniform_location.map(|u| uniform_locations.1.get_unchecked(u));
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.uniform2f(raw_uniform_location, x, y),
            RawRenderingContext::WebGl2(ref gl) => gl.uniform2f(raw_uniform_location, x, y),
        }
    }

    unsafe fn uniform_3_f32(
        &self,
        uniform_location: Option<Self::UniformLocation>,
        x: f32,
        y: f32,
        z: f32,
    ) {
        let uniform_locations = self.uniform_locations.borrow();
        let raw_uniform_location = uniform_location.map(|u| uniform_locations.1.get_unchecked(u));
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.uniform3f(raw_uniform_location, x, y, z),
            RawRenderingContext::WebGl2(ref gl) => gl.uniform3f(raw_uniform_location, x, y, z),
        }
    }

    unsafe fn uniform_4_f32(
        &self,
        uniform_location: Option<Self::UniformLocation>,
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    ) {
        let uniform_locations = self.uniform_locations.borrow();
        let raw_uniform_location = uniform_location.map(|u| uniform_locations.1.get_unchecked(u));
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.uniform4f(raw_uniform_location, x, y, z, w),
            RawRenderingContext::WebGl2(ref gl) => gl.uniform4f(raw_uniform_location, x, y, z, w),
        }
    }

    unsafe fn uniform_1_f32_slice(
        &self,
        uniform_location: Option<Self::UniformLocation>,
        v: &[f32; 1],
    ) {
        let uniform_locations = self.uniform_locations.borrow();
        let raw_uniform_location = uniform_location.map(|u| uniform_locations.1.get_unchecked(u));
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.uniform1fv_with_f32_array(raw_uniform_location, v)
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.uniform1fv_with_f32_array(raw_uniform_location, v)
            }
        }
    }

    unsafe fn uniform_2_f32_slice(
        &self,
        uniform_location: Option<Self::UniformLocation>,
        v: &[f32; 2],
    ) {
        let uniform_locations = self.uniform_locations.borrow();
        let raw_uniform_location = uniform_location.map(|u| uniform_locations.1.get_unchecked(u));
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.uniform2fv_with_f32_array(raw_uniform_location, v)
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.uniform2fv_with_f32_array(raw_uniform_location, v)
            }
        }
    }

    unsafe fn uniform_3_f32_slice(
        &self,
        uniform_location: Option<Self::UniformLocation>,
        v: &[f32; 3],
    ) {
        let uniform_locations = self.uniform_locations.borrow();
        let raw_uniform_location = uniform_location.map(|u| uniform_locations.1.get_unchecked(u));
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.uniform3fv_with_f32_array(raw_uniform_location, v)
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.uniform3fv_with_f32_array(raw_uniform_location, v)
            }
        }
    }

    unsafe fn uniform_4_f32_slice(
        &self,
        uniform_location: Option<Self::UniformLocation>,
        v: &[f32; 4],
    ) {
        let uniform_locations = self.uniform_locations.borrow();
        let raw_uniform_location = uniform_location.map(|u| uniform_locations.1.get_unchecked(u));
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.uniform4fv_with_f32_array(raw_uniform_location, v)
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.uniform4fv_with_f32_array(raw_uniform_location, v)
            }
        }
    }

    unsafe fn uniform_matrix_2_f32_slice(
        &self,
        uniform_location: Option<Self::UniformLocation>,
        transpose: bool,
        v: &[f32; 4],
    ) {
        let uniform_locations = self.uniform_locations.borrow();
        let raw_uniform_location = uniform_location.map(|u| uniform_locations.1.get_unchecked(u));
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.uniform_matrix2fv_with_f32_array(raw_uniform_location, transpose, v)
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.uniform_matrix2fv_with_f32_array(raw_uniform_location, transpose, v)
            }
        }
    }

    unsafe fn uniform_matrix_3_f32_slice(
        &self,
        uniform_location: Option<Self::UniformLocation>,
        transpose: bool,
        v: &[f32; 9],
    ) {
        let uniform_locations = self.uniform_locations.borrow();
        let raw_uniform_location = uniform_location.map(|u| uniform_locations.1.get_unchecked(u));
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.uniform_matrix3fv_with_f32_array(raw_uniform_location, transpose, v)
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.uniform_matrix3fv_with_f32_array(raw_uniform_location, transpose, v)
            }
        }
    }

    unsafe fn uniform_matrix_4_f32_slice(
        &self,
        uniform_location: Option<Self::UniformLocation>,
        transpose: bool,
        v: &[f32; 16],
    ) {
        let uniform_locations = self.uniform_locations.borrow();
        let raw_uniform_location = uniform_location.map(|u| uniform_locations.1.get_unchecked(u));
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.uniform_matrix4fv_with_f32_array(raw_uniform_location, transpose, v)
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.uniform_matrix4fv_with_f32_array(raw_uniform_location, transpose, v)
            }
        }
    }

    unsafe fn unmap_buffer(&self, _target: u32) {
        panic!("Unmap buffer is not supported");
    }

    unsafe fn cull_face(&self, value: u32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.cull_face(value as u32),
            RawRenderingContext::WebGl2(ref gl) => gl.cull_face(value as u32),
        }
    }

    unsafe fn color_mask(&self, red: bool, green: bool, blue: bool, alpha: bool) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.color_mask(red, green, blue, alpha),
            RawRenderingContext::WebGl2(ref gl) => gl.color_mask(red, green, blue, alpha),
        }
    }

    unsafe fn color_mask_draw_buffer(
        &self,
        _draw_buffer: u32,
        _red: bool,
        _green: bool,
        _blue: bool,
        _alpha: bool,
    ) {
        panic!("Draw buffer color masks are not supported");
    }

    unsafe fn depth_mask(&self, value: bool) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.depth_mask(value),
            RawRenderingContext::WebGl2(ref gl) => gl.depth_mask(value),
        }
    }

    unsafe fn blend_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.blend_color(red, green, blue, alpha),
            RawRenderingContext::WebGl2(ref gl) => gl.blend_color(red, green, blue, alpha),
        }
    }

    unsafe fn line_width(&self, width: f32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.line_width(width),
            RawRenderingContext::WebGl2(ref gl) => gl.line_width(width),
        }
    }

    unsafe fn map_buffer_range(
        &self,
        _target: u32,
        _offset: i32,
        _length: i32,
        _access: u32,
    ) -> *mut u8 {
        panic!("Map buffer range is not supported")
    }

    unsafe fn flush_mapped_buffer_range(
        &self,
        _target: u32,
        _offset: i32,
        _length: i32,
    ) {
        panic!("Flush mapped buffer range is not supported")
    }

    unsafe fn invalidate_buffer_sub_data(
        &self,
        _target: u32,
        _offset: i32,
        _length: i32,
    ) {
        panic!("Invalidate buffer sub data is not supported")
    }

    unsafe fn polygon_offset(&self, factor: f32, units: f32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.polygon_offset(factor, units),
            RawRenderingContext::WebGl2(ref gl) => gl.polygon_offset(factor, units),
        }
    }

    unsafe fn polygon_mode(&self, _face: u32, mode: u32) {
        if mode != FILL {
            panic!("Polygon mode other than FILL is not supported");
        }
    }

    unsafe fn finish(&self) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.finish(),
            RawRenderingContext::WebGl2(ref gl) => gl.finish(),
        }
    }

    unsafe fn bind_texture(&self, target: u32, texture: Option<Self::Texture>) {
        let textures = self.textures.borrow();
        let raw_texture = texture.map(|t| textures.1.get_unchecked(t));
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.bind_texture(target, raw_texture),
            RawRenderingContext::WebGl2(ref gl) => gl.bind_texture(target, raw_texture),
        }
    }

    unsafe fn bind_sampler(&self, unit: u32, sampler: Option<Self::Sampler>) {
        let samplers = self.samplers.borrow();
        let raw_sampler = sampler.map(|s| samplers.1.get_unchecked(s));
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => panic!("Bind sampler is not supported"),
            RawRenderingContext::WebGl2(ref gl) => gl.bind_sampler(unit, raw_sampler),
        }
    }

    unsafe fn active_texture(&self, unit: u32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.active_texture(unit),
            RawRenderingContext::WebGl2(ref gl) => gl.active_texture(unit),
        }
    }

    unsafe fn fence_sync(&self, condition: u32, flags: u32) -> Result<Self::Fence, String> {
        let raw_fence = match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => panic!("Fences are not supported"), // TODO: Extension
            RawRenderingContext::WebGl2(ref gl) => gl.fence_sync(condition as u32, flags),
        };
        match raw_fence {
            Some(f) => {
                let key = self.fences.borrow_mut().0.insert(());
                self.fences.borrow_mut().1.insert(key, f);
                Ok(key)
            }
            None => Err(String::from("Unable to create fence object")),
        }
    }

    unsafe fn tex_parameter_f32(&self, target: u32, parameter: u32, value: f32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.tex_parameterf(target, parameter, value),
            RawRenderingContext::WebGl2(ref gl) => gl.tex_parameterf(target, parameter, value),
        }
    }

    unsafe fn tex_parameter_i32(&self, target: u32, parameter: u32, value: i32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.tex_parameteri(target, parameter, value),
            RawRenderingContext::WebGl2(ref gl) => gl.tex_parameteri(target, parameter, value),
        }
    }

    unsafe fn tex_parameter_f32_slice(&self, _target: u32, _parameter: u32, _values: &[f32]) {
        // Blocked by https://github.com/rustwasm/wasm-bindgen/issues/1038
        panic!("Texture parameters for `&[f32]` are not supported yet");
    }

    unsafe fn tex_parameter_i32_slice(&self, _target: u32, _parameter: u32, _values: &[i32]) {
        panic!("Texture parameters for `&[i32]` are not supported yet");
    }

    unsafe fn tex_sub_image_2d_u8_slice(
        &self,
        target: u32,
        level: i32,
        x_offset: i32,
        y_offset: i32,
        width: i32,
        height: i32,
        format: u32,
        ty: u32,
        pixels: Option<&[u8]>,
    ) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_u8_array(
                    target, level, x_offset, y_offset, width, height, format, ty, pixels,
                )
                .unwrap(); // TODO: Handle return value?
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_u8_array(
                    target, level, x_offset, y_offset, width, height, format, ty, pixels,
                )
                .unwrap(); // TODO: Handle return value?
            }
        }
    }

    unsafe fn tex_sub_image_2d_pixel_buffer_offset(
        &self,
        target: u32,
        level: i32,
        x_offset: i32,
        y_offset: i32,
        width: i32,
        height: i32,
        format: u32,
        ty: u32,
        pixel_buffer_offset: i32,
    ) {
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => {
                panic!("Sub image 2D pixel buffer offset is not supported")
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_i32(
                    target,
                    level,
                    x_offset,
                    y_offset,
                    width,
                    height,
                    format,
                    ty,
                    pixel_buffer_offset,
                )
                .unwrap(); // TODO: Handle return value?
            }
        }
    }

    unsafe fn tex_sub_image_3d_u8_slice(
        &self,
        target: u32,
        level: i32,
        x_offset: i32,
        y_offset: i32,
        z_offset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        ty: u32,
        pixels: Option<&[u8]>,
    ) {
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => {
                panic!("Sub image 3D is not supported")
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.tex_sub_image_3d_with_opt_u8_array(
                    target,
                    level,
                    x_offset,
                    y_offset,
                    z_offset,
                    width,
                    height,
                    depth,
                    format,
                    ty,
                    pixels,
                )
                .unwrap(); // TODO: Handle return value?
            }
        }
    }

    unsafe fn tex_sub_image_3d_pixel_buffer_offset(
        &self,
        target: u32,
        level: i32,
        x_offset: i32,
        y_offset: i32,
        z_offset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        ty: u32,
        pixel_buffer_offset: i32,
    ) {
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => {
                panic!("Sub image 3D pixel buffer offset is not supported")
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.tex_sub_image_3d_with_i32(
                    target,
                    level,
                    x_offset,
                    y_offset,
                    z_offset,
                    width,
                    height,
                    depth,
                    format,
                    ty,
                    pixel_buffer_offset,
                )
                .unwrap(); // TODO: Handle return value?
            }
        }
    }

    unsafe fn depth_func(&self, func: u32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.depth_func(func as u32),
            RawRenderingContext::WebGl2(ref gl) => gl.depth_func(func as u32),
        }
    }

    unsafe fn depth_range_f32(&self, near: f32, far: f32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.depth_range(near, far),
            RawRenderingContext::WebGl2(ref gl) => gl.depth_range(near, far),
        }
    }

    unsafe fn depth_range_f64(&self, _near: f64, _far: f64) {
        panic!("Depth range with 64-bit float values is not supported");
    }

    unsafe fn depth_range_f64_slice(&self, _first: u32, _count: i32, _values: &[[f64; 2]]) {
        panic!("Depth range with 64-bit float slices is not supported");
    }

    unsafe fn scissor(&self, x: i32, y: i32, width: i32, height: i32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.scissor(x, y, width, height),
            RawRenderingContext::WebGl2(ref gl) => gl.scissor(x, y, width, height),
        }
    }

    unsafe fn scissor_slice(&self, _first: u32, _count: i32, _scissors: &[[i32; 4]]) {
        panic!("Scissor slice is not supported");
    }

    unsafe fn vertex_attrib_divisor(&self, index: u32, divisor: u32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => {
                panic!("Vertex attrib divisor is not supported"); // TODO: Extension
            }
            RawRenderingContext::WebGl2(ref gl) => gl.vertex_attrib_divisor(index, divisor),
        }
    }

    unsafe fn vertex_attrib_pointer_f32(
        &self,
        index: u32,
        size: i32,
        data_type: u32,
        normalized: bool,
        stride: i32,
        offset: i32,
    ) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl
                .vertex_attrib_pointer_with_i32(index, size, data_type, normalized, stride, offset),
            RawRenderingContext::WebGl2(ref gl) => gl
                .vertex_attrib_pointer_with_i32(index, size, data_type, normalized, stride, offset),
        }
    }

    unsafe fn vertex_attrib_pointer_i32(
        &self,
        index: u32,
        size: i32,
        data_type: u32,
        stride: i32,
        offset: i32,
    ) {
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => {
                panic!("Integer vertex attrib pointer is not supported")
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.vertex_attrib_i_pointer_with_i32(index, size, data_type, stride, offset)
            }
        }
    }

    unsafe fn vertex_attrib_pointer_f64(
        &self,
        _index: u32,
        _size: i32,
        _data_type: u32,
        _stride: i32,
        _offset: i32,
    ) {
        panic!("64-bit float precision is not supported in WebGL");
    }

    unsafe fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.viewport(x, y, width, height),
            RawRenderingContext::WebGl2(ref gl) => gl.viewport(x, y, width, height),
        }
    }

    unsafe fn viewport_f32_slice(&self, _first: u32, _count: i32, _values: &[[f32; 4]]) {
        panic!("Viewport `f32` slice is not supported");
    }

    unsafe fn blend_func(&self, src: u32, dst: u32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.blend_func(src as u32, dst as u32),
            RawRenderingContext::WebGl2(ref gl) => gl.blend_func(src as u32, dst as u32),
        }
    }

    unsafe fn blend_func_draw_buffer(&self, _draw_buffer: u32, _src: u32, _dst: u32) {
        panic!("Draw buffer blend func is not supported");
    }

    unsafe fn blend_func_separate(
        &self,
        src_rgb: u32,
        dst_rgb: u32,
        src_alpha: u32,
        dst_alpha: u32,
    ) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.blend_func_separate(
                src_rgb as u32,
                dst_rgb as u32,
                src_alpha as u32,
                dst_alpha as u32,
            ),
            RawRenderingContext::WebGl2(ref gl) => gl.blend_func_separate(
                src_rgb as u32,
                dst_rgb as u32,
                src_alpha as u32,
                dst_alpha as u32,
            ),
        }
    }

    unsafe fn blend_func_separate_draw_buffer(
        &self,
        _draw_buffer: u32,
        _src_rgb: u32,
        _dst_rgb: u32,
        _src_alpha: u32,
        _dst_alpha: u32,
    ) {
        panic!("Draw buffer blend func separate is not supported");
    }

    unsafe fn blend_equation(&self, mode: u32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.blend_equation(mode as u32),
            RawRenderingContext::WebGl2(ref gl) => gl.blend_equation(mode as u32),
        }
    }

    unsafe fn blend_equation_draw_buffer(&self, _draw_buffer: u32, _mode: u32) {
        panic!("Draw buffer blend equation is not supported");
    }

    unsafe fn blend_equation_separate(&self, mode_rgb: u32, mode_alpha: u32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.blend_equation_separate(mode_rgb as u32, mode_alpha as u32)
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.blend_equation_separate(mode_rgb as u32, mode_alpha as u32)
            }
        }
    }

    unsafe fn blend_equation_separate_draw_buffer(
        &self,
        _draw_buffer: u32,
        _mode_rgb: u32,
        _mode_alpha: u32,
    ) {
        panic!("Draw buffer blend equation separate is not supported");
    }

    unsafe fn stencil_func(&self, func: u32, reference: i32, mask: u32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.stencil_func(func as u32, reference, mask),
            RawRenderingContext::WebGl2(ref gl) => gl.stencil_func(func as u32, reference, mask),
        }
    }

    unsafe fn stencil_func_separate(&self, face: u32, func: u32, reference: i32, mask: u32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.stencil_func_separate(face as u32, func as u32, reference, mask)
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.stencil_func_separate(face as u32, func as u32, reference, mask)
            }
        }
    }

    unsafe fn stencil_mask(&self, mask: u32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.stencil_mask(mask),
            RawRenderingContext::WebGl2(ref gl) => gl.stencil_mask(mask),
        }
    }

    unsafe fn stencil_mask_separate(&self, face: u32, mask: u32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.stencil_mask_separate(face as u32, mask),
            RawRenderingContext::WebGl2(ref gl) => gl.stencil_mask_separate(face as u32, mask),
        }
    }

    unsafe fn stencil_op(&self, stencil_fail: u32, depth_fail: u32, pass: u32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.stencil_op(stencil_fail as u32, depth_fail as u32, pass as u32)
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.stencil_op(stencil_fail as u32, depth_fail as u32, pass as u32)
            }
        }
    }

    unsafe fn stencil_op_separate(&self, face: u32, stencil_fail: u32, depth_fail: u32, pass: u32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.stencil_op_separate(
                face as u32,
                stencil_fail as u32,
                depth_fail as u32,
                pass as u32,
            ),
            RawRenderingContext::WebGl2(ref gl) => gl.stencil_op_separate(
                face as u32,
                stencil_fail as u32,
                depth_fail as u32,
                pass as u32,
            ),
        }
    }

    unsafe fn debug_message_control(
        &self,
        _source: u32,
        _msg_type: u32,
        _severity: u32,
        _ids: &[u32],
        _enabled: bool,
    ) {
        panic!("WebGL does not support the KHR_debug extension.")
    }

    unsafe fn debug_message_insert<S>(
        &self,
        _source: u32,
        _msg_type: u32,
        _id: u32,
        _severity: u32,
        _msg: S,
    ) where
        S: AsRef<str>
    {
        panic!("WebGL does not support the KHR_debug extension.")
    }

    unsafe fn debug_message_callback<F>(&self, _callback: F)
    where
        F: FnMut(u32, u32, u32, u32, &str)
    {
        panic!("WebGL does not support the KHR_debug extension.")
    }

    unsafe fn get_debug_message_log(&self, _count: u32) -> Vec<DebugMessageLogEntry> {
        panic!("WebGL does not support the KHR_debug extension.")
    }

    unsafe fn push_debug_group<S>(
        &self,
        _source: u32,
        _id: u32,
        _message: S
    ) where
        S: AsRef<str>
    {
        panic!("WebGL does not support the KHR_debug extension.")
    }

    unsafe fn pop_debug_group(&self) {
        panic!("WebGL does not support the KHR_debug extension.")
    }

    unsafe fn object_label<S>(&self, _identifier: u32, _name: u32, _label: Option<S>)
    where
        S: AsRef<str>
    {
        panic!("WebGL does not support the KHR_debug extension.")
    }

    unsafe fn get_object_label(&self, _identifier: u32, _name: u32) -> String {
        panic!("WebGL does not support the KHR_debug extension.")
    }

    unsafe fn object_ptr_label<S>(&self, sync: Self::Fence, _label: Option<S>) where
        S: AsRef<str>
    {
        panic!("WebGL does not support the KHR_debug extension.")
    }

    unsafe fn get_object_ptr_label(&self, sync: Self::Fence) -> String {
        panic!("WebGL does not support the KHR_debug extension.")
    }

    unsafe fn get_uniform_block_index(&self, program: Self::Program, name: &str) -> Option<u32> {
        let programs = self.programs.borrow();
        let raw_program = programs.1.get_unchecked(program);
        let index = match self.raw {
            RawRenderingContext::WebGl1(ref gl) => panic!("Uniform blocks are not supported"),
            RawRenderingContext::WebGl2(ref gl) => gl.get_uniform_block_index(raw_program, name),
        };
        if index == INVALID_INDEX {
            None
        } else {
            Some(index)
        }
    }

    unsafe fn uniform_block_binding(&self, program: Self::Program, index: u32, binding: u32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => panic!("Uniform buffer bindings are not supported"),
            RawRenderingContext::WebGl2(ref gl) => {
                let programs = self.programs.borrow();
                let raw_program = programs.1.get_unchecked(program);
                gl.uniform_block_binding(raw_program, index, binding);
            }
        }
    }

    unsafe fn get_shader_storage_block_index(&self, _program: Self::Program, _name: &str) -> Option<u32> {
        panic!("Shader Storage Buffers are not supported by webgl")
    }

    unsafe fn shader_storage_block_binding(&self, _program: Self::Program, _index: u32, binding: u32) {
        panic!("Shader Storage Buffers are not supported by webgl")
    }
}

pub struct RenderLoop;

impl RenderLoop {
    pub fn from_request_animation_frame() -> Self {
        RenderLoop
    }
}

impl super::RenderLoop for RenderLoop {
    type Window = ();

    fn run<F: FnMut(&mut bool) + 'static>(&self, mut callback: F) {
        fn request_animation_frame(f: &Closure<FnMut()>) {
            use wasm_bindgen::JsCast;
            web_sys::window()
                .unwrap()
                .request_animation_frame(f.as_ref().unchecked_ref())
                .unwrap();
        }

        let mut running = true;
        let f = std::rc::Rc::new(std::cell::RefCell::new(None));
        let g = f.clone();
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            callback(&mut running);
            if !running {
                let _ = f.borrow_mut().take();
                return;
            }
            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<FnMut()>));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }
}
