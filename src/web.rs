use super::*;

use slotmap::{new_key_type, SecondaryMap, SlotMap};
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use web_sys::{
    WebGl2RenderingContext, WebGlBuffer, WebGlFramebuffer, WebGlProgram, WebGlRenderbuffer,
    WebGlRenderingContext, WebGlSampler, WebGlShader, WebGlSync, WebGlTexture,
    WebGlVertexArrayObject,
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

    unsafe fn create_shader(&self, shader_type: ShaderType) -> Result<Self::Shader, String> {
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

    unsafe fn get_tex_image(
        &self,
        _target: TextureTarget,
        _level: i32,
        _format: TextureFormat,
        _ty: TextureType,
        _pixels: *mut std::ffi::c_void,
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

    unsafe fn bind_buffer(&self, target: BufferTarget, buffer: Option<Self::Buffer>) {
        let buffers = self.buffers.borrow();
        let raw_buffer = buffer.map(|b| buffers.1.get_unchecked(b));
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.bind_buffer(target as u32, raw_buffer),
            RawRenderingContext::WebGl2(ref gl) => gl.bind_buffer(target as u32, raw_buffer),
        }
    }

    unsafe fn bind_buffer_range(
        &self,
        _target: BufferTarget,
        _index: u32,
        _buffer: Option<Self::Buffer>,
        _offset: i32,
        _size: i32,
    ) {
        // Blocked by https://github.com/rustwasm/wasm-bindgen/issues/1038
        panic!("Bind buffer range is not supported yet");
    }

    unsafe fn bind_framebuffer(
        &self,
        target: FramebufferTarget,
        framebuffer: Option<Self::Framebuffer>,
    ) {
        let framebuffers = self.framebuffers.borrow();
        let raw_framebuffer = framebuffer.map(|f| framebuffers.1.get_unchecked(f));
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.bind_framebuffer(target as u32, raw_framebuffer)
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.bind_framebuffer(target as u32, raw_framebuffer)
            }
        }
    }

    unsafe fn create_vertex_array(&self) -> Result<Self::VertexArray, String> {
        let raw_vertex_array = match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => {
                panic!("Vertex array objects are not supported");
            }
            // TODO: Extension
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
                    panic!("Vertex array objects are not supported");
                }
                // TODO: Extension
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
                panic!("Vertex array objects are not supported");
            }
            // TODO: Extension
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

    unsafe fn clear(&self, mask: ClearMask) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.clear(mask.bits()),
            RawRenderingContext::WebGl2(ref gl) => gl.clear(mask.bits()),
        }
    }

    unsafe fn patch_parameter_i32(&self, _parameter: PatchParameterI32, _value: i32) {
        panic!("Patch parameter is not supported");
    }

    unsafe fn pixel_store_i32(&self, parameter: PixelStoreParameterI32, value: i32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.pixel_storei(parameter as u32, value),
            RawRenderingContext::WebGl2(ref gl) => gl.pixel_storei(parameter as u32, value),
        }
    }

    unsafe fn pixel_store_bool(&self, parameter: PixelStoreParameterBool, value: bool) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.pixel_storei(parameter as u32, value as i32),
            RawRenderingContext::WebGl2(ref gl) => gl.pixel_storei(parameter as u32, value as i32),
        }
    }

    unsafe fn clear_buffer_i32_slice(
        &self,
        target: FramebufferBufferTarget,
        draw_buffer: u32,
        values: &mut [i32],
    ) {
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => {
                panic!("Clear buffer with `i32` slice is not supported")
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.clear_bufferiv_with_i32_array(target as u32, draw_buffer as i32, values)
            }
        }
    }

    unsafe fn clear_buffer_u32_slice(
        &self,
        target: FramebufferBufferTarget,
        draw_buffer: u32,
        values: &mut [u32],
    ) {
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => {
                panic!("Clear buffer with `u32` slice is not supported")
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.clear_bufferuiv_with_u32_array(target as u32, draw_buffer as i32, values)
            }
        }
    }

    unsafe fn clear_buffer_f32_slice(
        &self,
        target: FramebufferBufferTarget,
        draw_buffer: u32,
        values: &mut [f32],
    ) {
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => {
                panic!("Clear buffer with `f32` slice is not supported")
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.clear_bufferfv_with_f32_array(target as u32, draw_buffer as i32, values)
            }
        }
    }

    unsafe fn clear_buffer_depth_stencil(
        &self,
        target: FramebufferBufferTarget,
        draw_buffer: u32,
        depth: f32,
        stencil: i32,
    ) {
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => {
                panic!("Clear buffer depth stencil is not supported")
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.clear_bufferfi(target as u32, draw_buffer as i32, depth, stencil)
            }
        }
    }

    unsafe fn copy_buffer_sub_data(
        &self,
        src_target: BufferTarget,
        dst_target: BufferTarget,
        src_offset: i32,
        dst_offset: i32,
        size: i32,
    ) {
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => panic!("Copy buffer subdata is not supported"),
            RawRenderingContext::WebGl2(ref gl) => gl
                .copy_buffer_sub_data_with_i32_and_i32_and_i32(
                    src_target as u32,
                    dst_target as u32,
                    src_offset,
                    dst_offset,
                    size,
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

    unsafe fn disable(&self, parameter: Parameter) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.disable(parameter as u32),
            RawRenderingContext::WebGl2(ref gl) => gl.disable(parameter as u32),
        }
    }

    unsafe fn disable_draw_buffer(&self, _parameter: Parameter, _draw_buffer: u32) {
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

    unsafe fn draw_arrays(&self, mode: PrimitiveMode, first: i32, count: i32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.draw_arrays(mode as u32, first, count),
            RawRenderingContext::WebGl2(ref gl) => gl.draw_arrays(mode as u32, first, count),
        }
    }

    unsafe fn draw_arrays_instanced(
        &self,
        mode: PrimitiveMode,
        first: i32,
        count: i32,
        instance_count: i32,
    ) {
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
        _mode: PrimitiveMode,
        _first: i32,
        _count: i32,
        _instance_count: i32,
        _base_instance: u32,
    ) {
        panic!("Draw arrays instanced base instance is not supported");
    }

    unsafe fn draw_buffer(&self, _draw_buffer: u32) {
        // Blocked by https://github.com/rustwasm/wasm-bindgen/issues/1038
        panic!("Draw buffer is not supported yet");
    }

    unsafe fn draw_buffers(&self, _buffers: &[u32]) {
        // Blocked by https://github.com/rustwasm/wasm-bindgen/issues/1038
        panic!("Draw buffers is not supported yet");
    }

    unsafe fn draw_elements(
        &self,
        mode: PrimitiveMode,
        count: i32,
        element_type: ElementType,
        offset: i32,
    ) {
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
        _mode: PrimitiveMode,
        _count: i32,
        _element_type: ElementType,
        _offset: i32,
        _base_vertex: i32,
    ) {
        panic!("Draw elements base vertex is not supported");
    }

    unsafe fn draw_elements_instanced(
        &self,
        mode: PrimitiveMode,
        count: i32,
        element_type: ElementType,
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
        _mode: PrimitiveMode,
        _count: i32,
        _element_type: ElementType,
        _offset: i32,
        _instance_count: i32,
        _base_vertex: i32,
    ) {
        panic!("Draw elements instanced base vertex is not supported");
    }

    unsafe fn draw_elements_instanced_base_vertex_base_instance(
        &self,
        _mode: PrimitiveMode,
        _count: i32,
        _element_type: ElementType,
        _offset: i32,
        _instance_count: i32,
        _base_vertex: i32,
        _base_instance: u32,
    ) {
        panic!("Draw elements instanced base vertex base instance is not supported");
    }

    unsafe fn enable(&self, parameter: Parameter) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.enable(parameter as u32),
            RawRenderingContext::WebGl2(ref gl) => gl.enable(parameter as u32),
        }
    }

    unsafe fn enable_draw_buffer(&self, _parameter: Parameter, _draw_buffer: u32) {
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
        target: FramebufferTarget,
        attachment: u32,
        renderbuffer_target: RenderbufferTarget,
        renderbuffer: Option<Self::Renderbuffer>,
    ) {
        let renderbuffers = self.renderbuffers.borrow();
        let raw_renderbuffer = renderbuffer.map(|r| renderbuffers.1.get_unchecked(r));
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => {
                panic!("Framebuffer renderbuffer is not supported");
            }
            RawRenderingContext::WebGl2(ref gl) => gl.framebuffer_renderbuffer(
                target as u32,
                attachment,
                renderbuffer_target as u32,
                raw_renderbuffer,
            ),
        }
    }

    unsafe fn framebuffer_texture(
        &self,
        _target: FramebufferTarget,
        _attachment: u32,
        _texture: Option<Self::Texture>,
        _level: i32,
    ) {
        panic!("Framebuffer texture is not supported");
    }

    unsafe fn framebuffer_texture_layer(
        &self,
        target: FramebufferTarget,
        attachment: u32,
        texture: Option<Self::Texture>,
        level: i32,
        layer: i32,
    ) {
        let textures = self.textures.borrow();
        let raw_texture = texture.map(|t| textures.1.get_unchecked(t));
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => {
                panic!("Framebuffer texture layer is not supported")
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.framebuffer_texture_layer(target as u32, attachment, raw_texture, level, layer)
            }
        }
    }

    unsafe fn front_face(&self, value: FrontFace) {
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

    unsafe fn cull_face(&self, value: Face) {
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

    unsafe fn polygon_offset(&self, factor: f32, units: f32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.polygon_offset(factor, units),
            RawRenderingContext::WebGl2(ref gl) => gl.polygon_offset(factor, units),
        }
    }

    unsafe fn polygon_mode(&self, _face: PolygonFace, _mode: PolygonMode) {
        panic!("Polygon mode is not supported");
    }

    unsafe fn finish(&self) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.finish(),
            RawRenderingContext::WebGl2(ref gl) => gl.finish(),
        }
    }

    unsafe fn bind_texture(&self, target: TextureTarget, texture: Option<Self::Texture>) {
        let textures = self.textures.borrow();
        let raw_texture = texture.map(|t| textures.1.get_unchecked(t));
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.bind_texture(target.0, raw_texture),
            RawRenderingContext::WebGl2(ref gl) => gl.bind_texture(target.0, raw_texture),
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

    unsafe fn fence_sync(
        &self,
        condition: FenceSyncCondition,
        flags: FenceSyncFlags,
    ) -> Result<Self::Fence, String> {
        let raw_fence = match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => panic!("Fences are not supported"), // TODO: Extension
            RawRenderingContext::WebGl2(ref gl) => gl.fence_sync(condition as u32, flags.bits()),
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

    unsafe fn tex_parameter_f32(
        &self,
        target: TextureTarget,
        parameter: TextureParameter,
        value: f32,
    ) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.tex_parameterf(target.0, parameter as u32, value)
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.tex_parameterf(target.0, parameter as u32, value)
            }
        }
    }

    unsafe fn tex_parameter_i32(
        &self,
        target: TextureTarget,
        parameter: TextureParameter,
        value: i32,
    ) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.tex_parameteri(target.0, parameter as u32, value)
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.tex_parameteri(target.0, parameter as u32, value)
            }
        }
    }

    unsafe fn tex_parameter_f32_slice(
        &self,
        _target: TextureTarget,
        _parameter: TextureParameter,
        _values: &[f32],
    ) {
        // Blocked by https://github.com/rustwasm/wasm-bindgen/issues/1038
        panic!("Texture parameters for `&[f32]` are not supported yet");
    }

    unsafe fn tex_parameter_i32_slice(
        &self,
        _target: TextureTarget,
        _parameter: TextureParameter,
        _values: &[i32],
    ) {
        panic!("Texture parameters for `&[i32]` are not supported yet");
    }

    unsafe fn tex_sub_image_2d_u8_slice(
        &self,
        target: TextureTarget,
        level: i32,
        x_offset: i32,
        y_offset: i32,
        width: i32,
        height: i32,
        format: TextureFormat,
        ty: TextureType,
        pixels: Option<&mut [u8]>,
    ) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_u8_array(
                    target.0, level, x_offset, y_offset, width, height, format.0, ty.0, pixels,
                )
                .unwrap(); // TODO: Handle return value?
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_u8_array(
                    target.0, level, x_offset, y_offset, width, height, format.0, ty.0, pixels,
                )
                .unwrap(); // TODO: Handle return value?
            }
        }
    }

    unsafe fn tex_sub_image_2d_pixel_buffer_offset(
        &self,
        target: TextureTarget,
        level: i32,
        x_offset: i32,
        y_offset: i32,
        width: i32,
        height: i32,
        format: TextureFormat,
        ty: TextureType,
        pixel_buffer_offset: i32,
    ) {
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => {
                panic!("Sub image 2D pixel buffer offset is not supported")
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_i32(
                    target.0,
                    level,
                    x_offset,
                    y_offset,
                    width,
                    height,
                    format.0,
                    ty.0,
                    pixel_buffer_offset,
                )
                .unwrap(); // TODO: Handle return value?
            }
        }
    }

    unsafe fn depth_func(&self, func: Func) {
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

    unsafe fn vertex_attrib_pointer_f32(
        &self,
        index: u32,
        size: i32,
        data_type: VertexDataType,
        normalized: bool,
        stride: i32,
        offset: i32,
    ) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.vertex_attrib_pointer_with_i32(
                index,
                size,
                data_type.0,
                normalized,
                stride,
                offset,
            ),
            RawRenderingContext::WebGl2(ref gl) => gl.vertex_attrib_pointer_with_i32(
                index,
                size,
                data_type.0,
                normalized,
                stride,
                offset,
            ),
        }
    }

    unsafe fn vertex_attrib_pointer_i32(
        &self,
        index: u32,
        size: i32,
        data_type: VertexDataType,
        stride: i32,
        offset: i32,
    ) {
        match self.raw {
            RawRenderingContext::WebGl1(ref _gl) => {
                panic!("Integer vertex attrib pointer is not supported")
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.vertex_attrib_i_pointer_with_i32(index, size, data_type.0, stride, offset)
            }
        }
    }

    unsafe fn vertex_attrib_pointer_f64(
        &self,
        _index: u32,
        _size: i32,
        _data_type: VertexDataType,
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

    unsafe fn blend_func(&self, src: BlendFactor, dst: BlendFactor) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.blend_func(src as u32, dst as u32),
            RawRenderingContext::WebGl2(ref gl) => gl.blend_func(src as u32, dst as u32),
        }
    }

    unsafe fn blend_func_draw_buffer(
        &self,
        _draw_buffer: u32,
        _src: BlendFactor,
        _dst: BlendFactor,
    ) {
        panic!("Draw buffer blend func is not supported");
    }

    unsafe fn blend_func_separate(
        &self,
        src_rgb: BlendFactor,
        dst_rgb: BlendFactor,
        src_alpha: BlendFactor,
        dst_alpha: BlendFactor,
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
        _src_rgb: BlendFactor,
        _dst_rgb: BlendFactor,
        _src_alpha: BlendFactor,
        _dst_alpha: BlendFactor,
    ) {
        panic!("Draw buffer blend func separate is not supported");
    }

    unsafe fn blend_equation(&self, mode: BlendMode) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.blend_equation(mode as u32),
            RawRenderingContext::WebGl2(ref gl) => gl.blend_equation(mode as u32),
        }
    }

    unsafe fn blend_equation_draw_buffer(&self, _draw_buffer: u32, _mode: BlendMode) {
        panic!("Draw buffer blend equation is not supported");
    }

    unsafe fn blend_equation_separate(&self, mode_rgb: BlendMode, mode_alpha: BlendMode) {
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
        _mode_rgb: BlendMode,
        _mode_alpha: BlendMode,
    ) {
        panic!("Draw buffer blend equation separate is not supported");
    }

    unsafe fn stencil_func(&self, func: Func, reference: i32, mask: u32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.stencil_func(func as u32, reference, mask),
            RawRenderingContext::WebGl2(ref gl) => gl.stencil_func(func as u32, reference, mask),
        }
    }

    unsafe fn stencil_func_separate(&self, face: Face, func: Func, reference: i32, mask: u32) {
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

    unsafe fn stencil_mask_separate(&self, face: Face, mask: u32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.stencil_mask_separate(face as u32, mask),
            RawRenderingContext::WebGl2(ref gl) => gl.stencil_mask_separate(face as u32, mask),
        }
    }

    unsafe fn stencil_op(&self, stencil_fail: StencilOp, depth_fail: StencilOp, pass: StencilOp) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.stencil_op(stencil_fail as u32, depth_fail as u32, pass as u32)
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.stencil_op(stencil_fail as u32, depth_fail as u32, pass as u32)
            }
        }
    }

    unsafe fn stencil_op_separate(
        &self,
        face: Face,
        stencil_fail: StencilOp,
        depth_fail: StencilOp,
        pass: StencilOp,
    ) {
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
