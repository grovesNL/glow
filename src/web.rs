use super::*;

use slotmap::{new_key_type, SecondaryMap, SlotMap};
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use web_sys::{
    WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlSampler,
    WebGlShader, WebGlSync, WebGlTexture, WebGlVertexArrayObject,
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

impl super::Context for Context {
    type Shader = WebShaderKey;
    type Program = WebProgramKey;
    type Buffer = WebBufferKey;
    type VertexArray = WebVertexArrayKey;
    type Texture = WebTextureKey;
    type Sampler = WebSamplerKey;
    type Fence = WebFenceKey;

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

    unsafe fn bind_buffer(&self, target: BufferBindingTarget, buffer: Option<Self::Buffer>) {
        let buffers = self.buffers.borrow();
        let raw_buffer = buffer.map(|b| buffers.1.get_unchecked(b));
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.bind_buffer(target as u32, raw_buffer),
            RawRenderingContext::WebGl2(ref gl) => gl.bind_buffer(target as u32, raw_buffer),
        }
    }

    unsafe fn draw_arrays(&self, mode: PrimitiveMode, first: i32, count: i32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.draw_arrays(mode as u32, first, count),
            RawRenderingContext::WebGl2(ref gl) => gl.draw_arrays(mode as u32, first, count),
        }
    }

    unsafe fn draw_buffer(&self, _buffer: u32) {
        // Blocked by https://github.com/rustwasm/wasm-bindgen/issues/1038
        panic!("Draw buffer is not supported yet");
    }

    unsafe fn draw_buffers(&self, _buffers: &[u32]) {
        // Blocked by https://github.com/rustwasm/wasm-bindgen/issues/1038
        panic!("Draw buffers is not supported yet");
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

    unsafe fn pixel_store_i32(&self, parameter: PixelStoreI32Parameter, value: i32) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.pixel_storei(parameter as u32, value),
            RawRenderingContext::WebGl2(ref gl) => gl.pixel_storei(parameter as u32, value),
        }
    }

    unsafe fn pixel_store_bool(&self, parameter: PixelStoreBoolParameter, value: bool) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.pixel_storei(parameter as u32, value as i32),
            RawRenderingContext::WebGl2(ref gl) => gl.pixel_storei(parameter as u32, value as i32),
        }
    }

    unsafe fn enable(&self, parameter: Parameter) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.enable(parameter as u32),
            RawRenderingContext::WebGl2(ref gl) => gl.enable(parameter as u32),
        }
    }

    unsafe fn enable_i(&self, _parameter: Parameter, _buffer: u32) {
        panic!("Draw buffer enable is not supported");
    }

    unsafe fn disable(&self, parameter: Parameter) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.disable(parameter as u32),
            RawRenderingContext::WebGl2(ref gl) => gl.disable(parameter as u32),
        }
    }

    unsafe fn disable_i(&self, _parameter: Parameter, _buffer: u32) {
        panic!("Draw buffer disable is not supported");
    }

    unsafe fn front_face(&self, value: FrontFace) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.front_face(value as u32),
            RawRenderingContext::WebGl2(ref gl) => gl.front_face(value as u32),
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

    unsafe fn color_mask_i(
        &self,
        _buffer: u32,
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

    unsafe fn bind_texture(&self, target: TextureBindingTarget, texture: Option<Self::Texture>) {
        let textures = self.textures.borrow();
        let raw_texture = texture.map(|t| textures.1.get_unchecked(t));
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.bind_texture(target as u32, raw_texture),
            RawRenderingContext::WebGl2(ref gl) => gl.bind_texture(target as u32, raw_texture),
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
        target: TextureBindingTarget,
        parameter: TextureParameter,
        value: f32,
    ) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.tex_parameterf(target as u32, parameter as u32, value)
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.tex_parameterf(target as u32, parameter as u32, value)
            }
        }
    }

    unsafe fn tex_parameter_i32(
        &self,
        target: TextureBindingTarget,
        parameter: TextureParameter,
        value: i32,
    ) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => {
                gl.tex_parameteri(target as u32, parameter as u32, value)
            }
            RawRenderingContext::WebGl2(ref gl) => {
                gl.tex_parameteri(target as u32, parameter as u32, value)
            }
        }
    }

    unsafe fn tex_parameter_f32_slice(
        &self,
        _target: TextureBindingTarget,
        _parameter: TextureParameter,
        value: &[f32],
    ) {
        // Blocked by https://github.com/rustwasm/wasm-bindgen/issues/1038
        panic!("Texture parameters for `&[f32]` are not supported yet");
    }

    unsafe fn tex_parameter_i32_slice(
        &self,
        _target: TextureBindingTarget,
        _parameter: TextureParameter,
        value: &[i32],
    ) {
        panic!("Texture parameters for `&[i32]` are not supported yet");
    }

    unsafe fn depth_func(&self, func: Func) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.depth_func(func as u32),
            RawRenderingContext::WebGl2(ref gl) => gl.depth_func(func as u32),
        }
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

    unsafe fn blend_func(&self, src: BlendFactor, dst: BlendFactor) {
        match self.raw {
            RawRenderingContext::WebGl1(ref gl) => gl.blend_func(src as u32, dst as u32),
            RawRenderingContext::WebGl2(ref gl) => gl.blend_func(src as u32, dst as u32),
        }
    }

    unsafe fn blend_func_i(&self, _buffer: u32, _src: BlendFactor, _dst: BlendFactor) {
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

    unsafe fn blend_func_separate_i(
        &self,
        _buffer: u32,
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

    unsafe fn blend_equation_i(&self, _buffer: u32, _mode: BlendMode) {
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

    unsafe fn blend_equation_separate_i(
        &self,
        _buffer: u32,
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
