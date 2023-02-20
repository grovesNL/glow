use super::*;
use crate::{gl46gles32 as gl, version::Version};
use std::{
    collections::HashSet,
    ffi::{c_void, CStr, CString},
    num::NonZeroU32,
    os::raw::c_char,
};

#[derive(Default)]
struct Constants {
    max_label_length: i32,
}

pub struct Context {
    extensions: HashSet<String>,
    constants: Constants,
    version: Version,
    not_loaded: HashSet<&'static str>,
}

impl Context {
    fn is_loaded(&self, name: &'static str) -> bool {
        !self.not_loaded.contains(name)
    }

    pub unsafe fn from_loader_function_cstr<F>(loader_function: F) -> Self
    where
        F: Fn(&CStr) -> *const c_void,
    {
        let mut not_loaded = HashSet::new();

        let load_fn = &|p: *const u8| {
            let c_str = std::ffi::CStr::from_ptr(p as *const c_char);
            loader_function(c_str) as *const c_void
        };

        match gl::load_gl_functions(load_fn) {
            Ok(_) => {}
            Err(e) => {
                for s in e {
                    not_loaded.insert(s);
                }
            }
        };

        // Retrieve and parse `GL_VERSION`
        let raw_string = gl::glGetString(VERSION);

        if raw_string.is_null() {
            panic!("Reading GL_VERSION failed. Make sure there is a valid GL context currently active.")
        }

        let raw_version = std::ffi::CStr::from_ptr(raw_string as *const c_char)
            .to_str()
            .unwrap()
            .to_owned();
        let version = Version::parse(&raw_version).unwrap();

        // Setup extensions and constants after the context has been built
        let mut context = Self {
            extensions: HashSet::new(),
            constants: Constants::default(),
            version,
            not_loaded,
        };

        // Use core-only functions to populate extension list
        if (context.version >= Version::new(3, 0, None, String::from("")))
            || (context.version >= Version::new_embedded(3, 0, String::from("")))
        {
            let num_extensions = context.get_parameter_i32(NUM_EXTENSIONS);
            for i in 0..num_extensions {
                let extension_name = context.get_parameter_indexed_string(EXTENSIONS, i as u32);
                context.extensions.insert(extension_name);
            }
        } else {
            // Fallback
            context.extensions.extend(
                context
                    .get_parameter_string(EXTENSIONS)
                    .split(' ')
                    .map(|s| s.to_string()),
            );
        };

        // After the extensions are known, we can populate constants (including
        // constants that depend on extensions being enabled)
        context.constants.max_label_length = if context.supports_debug() {
            context.get_parameter_i32(MAX_LABEL_LENGTH)
        } else {
            0
        };

        context
    }

    pub unsafe fn from_loader_function<F>(loader_function: F) -> Self
    where
        F: Fn(&str) -> *const std::os::raw::c_void,
    {
        Self::from_loader_function_cstr(move |name| loader_function(name.to_str().unwrap()))
    }

    /// Creates a texture from an external GL name.
    ///
    /// This can be useful when a texture is created outside of glow (e.g. OpenXR surface) but glow
    /// still needs access to it for rendering.
    #[deprecated = "Use the NativeTexture constructor instead"]
    pub unsafe fn create_texture_from_gl_name(gl_name: gl::GLuint) -> NativeTexture {
        NativeTexture(non_zero_gl_name(gl_name))
    }

    /// Creates a framebuffer from an external GL name.
    ///
    /// This can be useful when a framebuffer is created outside of glow (e.g: via `surfman` or another
    /// crate that supports sharing of buffers between GL contexts), but glow needs to set it as a target.
    #[deprecated = "Use the NativeFramebuffer constructor instead"]
    pub unsafe fn create_framebuffer_from_gl_name(gl_name: gl::GLuint) -> NativeFramebuffer {
        NativeFramebuffer(non_zero_gl_name(gl_name))
    }
}

impl std::fmt::Debug for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Native_GL_Context")
    }
}

fn non_zero_gl_name(value: gl::GLuint) -> NonZeroU32 {
    NonZeroU32::new(value as u32).expect("expected non-zero GL name")
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NativeShader(pub NonZeroU32);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NativeProgram(pub NonZeroU32);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NativeBuffer(pub NonZeroU32);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NativeVertexArray(pub NonZeroU32);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NativeTexture(pub NonZeroU32);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NativeSampler(pub NonZeroU32);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NativeFence(pub gl::GLsync);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NativeFramebuffer(pub NonZeroU32);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NativeRenderbuffer(pub NonZeroU32);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NativeQuery(pub NonZeroU32);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NativeUniformLocation(pub gl::GLuint);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NativeTransformFeedback(pub NonZeroU32);

impl HasContext for Context {
    type Shader = NativeShader;
    type Program = NativeProgram;
    type Buffer = NativeBuffer;
    type VertexArray = NativeVertexArray;
    type Texture = NativeTexture;
    type Sampler = NativeSampler;
    type Fence = NativeFence;
    type Framebuffer = NativeFramebuffer;
    type Renderbuffer = NativeRenderbuffer;
    type Query = NativeQuery;
    type UniformLocation = NativeUniformLocation;
    type TransformFeedback = NativeTransformFeedback;

    fn supported_extensions(&self) -> &HashSet<String> {
        &self.extensions
    }

    fn supports_debug(&self) -> bool {
        self.extensions.contains("GL_KHR_debug")
    }

    fn version(&self) -> &Version {
        &self.version
    }

    unsafe fn create_framebuffer(&self) -> Result<Self::Framebuffer, String> {
        let mut name = 0;
        gl::glGenFramebuffers(1, &mut name);
        Ok(NativeFramebuffer(non_zero_gl_name(name)))
    }

    unsafe fn is_framebuffer(&self, framebuffer: Self::Framebuffer) -> bool {
        gl::glIsFramebuffer(framebuffer.0.get()) != 0
    }

    unsafe fn create_query(&self) -> Result<Self::Query, String> {
        let mut name = 0;
        gl::glGenQueries(1, &mut name);
        Ok(NativeQuery(non_zero_gl_name(name)))
    }

    unsafe fn create_renderbuffer(&self) -> Result<Self::Renderbuffer, String> {
        let mut name = 0;
        gl::glGenRenderbuffers(1, &mut name);
        Ok(NativeRenderbuffer(non_zero_gl_name(name)))
    }

    unsafe fn is_renderbuffer(&self, renderbuffer: Self::Renderbuffer) -> bool {
        gl::glIsRenderbuffer(renderbuffer.0.get()) != 0
    }

    unsafe fn create_sampler(&self) -> Result<Self::Sampler, String> {
        let mut name = 0;
        gl::glGenSamplers(1, &mut name);
        Ok(NativeSampler(non_zero_gl_name(name)))
    }

    unsafe fn create_shader(&self, shader_type: u32) -> Result<Self::Shader, String> {
        Ok(NativeShader(non_zero_gl_name(gl::glCreateShader(
            shader_type as u32,
        ))))
    }

    unsafe fn is_shader(&self, shader: Self::Shader) -> bool {
        gl::glIsShader(shader.0.get()) != 0
    }

    unsafe fn create_texture(&self) -> Result<Self::Texture, String> {
        let mut name = 0;
        gl::glGenTextures(1, &mut name);
        Ok(NativeTexture(non_zero_gl_name(name)))
    }

    unsafe fn create_named_texture(&self, target: u32) -> Result<Self::Texture, String> {
        let mut name = 0;
        gl::glCreateTextures(target, 1, &mut name);
        Ok(NativeTexture(non_zero_gl_name(name)))
    }

    unsafe fn is_texture(&self, texture: Self::Texture) -> bool {
        gl::glIsTexture(texture.0.get()) != 0
    }

    unsafe fn delete_shader(&self, shader: Self::Shader) {
        gl::glDeleteShader(shader.0.get());
    }

    unsafe fn shader_source(&self, shader: Self::Shader, source: &str) {
        gl::glShaderSource(
            shader.0.get(),
            1,
            &(source.as_ptr() as *const gl::GLchar),
            &(source.len() as gl::GLint),
        );
    }

    unsafe fn compile_shader(&self, shader: Self::Shader) {
        gl::glCompileShader(shader.0.get());
    }

    unsafe fn get_shader_completion_status(&self, shader: Self::Shader) -> bool {
        let mut status = 0;
        gl::glGetShaderiv(shader.0.get(), COMPLETION_STATUS, &mut status);
        1 == status
    }

    unsafe fn get_shader_compile_status(&self, shader: Self::Shader) -> bool {
        let mut status = 0;
        gl::glGetShaderiv(shader.0.get(), COMPILE_STATUS, &mut status);
        1 == status
    }

    unsafe fn get_shader_info_log(&self, shader: Self::Shader) -> String {
        let mut max_length = 0;
        gl::glGetShaderiv(shader.0.get(), INFO_LOG_LENGTH, &mut max_length);
        if max_length > 0 {
            let mut log = String::with_capacity(max_length as usize);
            log.extend(std::iter::repeat('\0').take(max_length as usize));
            let mut actual_length = 0;
            gl::glGetShaderInfoLog(
                shader.0.get(),
                max_length as u32,
                &mut actual_length,
                (&log[..]).as_ptr() as *mut gl::GLchar,
            );
            log.truncate(actual_length as usize);
            log
        } else {
            String::from("")
        }
    }

    unsafe fn get_tex_image(
        &self,
        target: u32,
        level: i32,
        format: u32,
        ty: u32,
        pixels: PixelPackData,
    ) {
        gl::glGetTexImage(
            target,
            level,
            format,
            ty,
            match pixels {
                PixelPackData::BufferOffset(offset) => offset as *mut std::ffi::c_void,
                PixelPackData::Slice(data) => data.as_mut_ptr() as *mut std::ffi::c_void,
            },
        );
    }

    unsafe fn create_program(&self) -> Result<Self::Program, String> {
        Ok(NativeProgram(non_zero_gl_name(gl::glCreateProgram())))
    }

    unsafe fn is_program(&self, program: Self::Program) -> bool {
        gl::glIsProgram(program.0.get()) != 0
    }

    unsafe fn delete_program(&self, program: Self::Program) {
        gl::glDeleteProgram(program.0.get());
    }

    unsafe fn attach_shader(&self, program: Self::Program, shader: Self::Shader) {
        gl::glAttachShader(program.0.get(), shader.0.get());
    }

    unsafe fn detach_shader(&self, program: Self::Program, shader: Self::Shader) {
        gl::glDetachShader(program.0.get(), shader.0.get());
    }

    unsafe fn link_program(&self, program: Self::Program) {
        gl::glLinkProgram(program.0.get());
    }

    unsafe fn get_program_completion_status(&self, program: Self::Program) -> bool {
        let mut status = 0;
        gl::glGetProgramiv(program.0.get(), COMPLETION_STATUS, &mut status);
        1 == status
    }

    unsafe fn get_program_link_status(&self, program: Self::Program) -> bool {
        let mut status = 0;
        gl::glGetProgramiv(program.0.get(), LINK_STATUS, &mut status);
        1 == status
    }

    unsafe fn get_program_info_log(&self, program: Self::Program) -> String {
        let mut max_length = 0;
        gl::glGetProgramiv(program.0.get(), INFO_LOG_LENGTH, &mut max_length);
        if max_length > 0 {
            let mut log = String::with_capacity(max_length as usize);
            log.extend(std::iter::repeat('\0').take(max_length as usize));
            let mut actual_length = 032;
            gl::glGetProgramInfoLog(
                program.0.get(),
                max_length as u32,
                &mut actual_length,
                (&log[..]).as_ptr() as *mut gl::GLchar,
            );
            log.truncate(actual_length as usize);
            log
        } else {
            String::from("")
        }
    }

    unsafe fn get_active_uniforms(&self, program: Self::Program) -> u32 {
        let mut count = 0;
        gl::glGetProgramiv(program.0.get(), ACTIVE_UNIFORMS, &mut count);
        count as u32
    }

    unsafe fn get_active_uniform(
        &self,
        program: Self::Program,
        index: u32,
    ) -> Option<ActiveUniform> {
        let mut uniform_max_size = 0;
        gl::glGetProgramiv(
            program.0.get(),
            ACTIVE_UNIFORM_MAX_LENGTH,
            &mut uniform_max_size,
        );

        let mut name = String::with_capacity(uniform_max_size as usize);
        name.extend(std::iter::repeat('\0').take(uniform_max_size as usize));
        let mut length = 0;
        let mut size = 0;
        let mut utype = 0;
        gl::glGetActiveUniform(
            program.0.get(),
            index,
            uniform_max_size as u32,
            &mut length,
            &mut size,
            &mut utype,
            name.as_ptr() as *mut gl::GLchar,
        );
        name.truncate(length as usize);

        Some(ActiveUniform { size, utype, name })
    }

    unsafe fn use_program(&self, program: Option<Self::Program>) {
        gl::glUseProgram(program.map(|p| p.0.get()).unwrap_or(0));
    }

    unsafe fn create_buffer(&self) -> Result<Self::Buffer, String> {
        let mut buffer = 0;
        gl::glGenBuffers(1, &mut buffer);
        Ok(NativeBuffer(non_zero_gl_name(buffer)))
    }

    unsafe fn create_named_buffer(&self) -> Result<Self::Buffer, String> {
        let mut buffer = 0;
        gl::glCreateBuffers(1, &mut buffer);
        Ok(NativeBuffer(non_zero_gl_name(buffer)))
    }

    unsafe fn is_buffer(&self, buffer: Self::Buffer) -> bool {
        gl::glIsBuffer(buffer.0.get()) != 0
    }

    unsafe fn bind_buffer(&self, target: u32, buffer: Option<Self::Buffer>) {
        gl::glBindBuffer(target, buffer.map(|b| b.0.get()).unwrap_or(0));
    }

    unsafe fn bind_buffer_base(&self, target: u32, index: u32, buffer: Option<Self::Buffer>) {
        gl::glBindBufferBase(target, index, buffer.map(|b| b.0.get()).unwrap_or(0));
    }

    unsafe fn bind_buffer_range(
        &self,
        target: u32,
        index: u32,
        buffer: Option<Self::Buffer>,
        offset: i32,
        size: i32,
    ) {
        gl::glBindBufferRange(
            target,
            index,
            buffer.map(|b| b.0.get()).unwrap_or(0),
            offset as isize,
            size as isize,
        );
    }

    unsafe fn bind_vertex_buffer(
        &self,
        binding_index: u32,
        buffer: Option<Buffer>,
        offset: i32,
        stride: i32,
    ) {
        gl::glBindVertexBuffer(
            binding_index,
            buffer.map(|b| b.0.get()).unwrap_or(0),
            offset as isize,
            stride as u32,
        );
    }

    unsafe fn bind_framebuffer(&self, target: u32, framebuffer: Option<Self::Framebuffer>) {
        gl::glBindFramebuffer(target, framebuffer.map(|fb| fb.0.get()).unwrap_or(0));
    }

    unsafe fn bind_renderbuffer(&self, target: u32, renderbuffer: Option<Self::Renderbuffer>) {
        gl::glBindRenderbuffer(target, renderbuffer.map(|rb| rb.0.get()).unwrap_or(0));
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
        gl::glBlitFramebuffer(
            src_x0, src_y0, src_x1, src_y1, dst_x0, dst_y0, dst_x1, dst_y1, mask, filter,
        );
    }

    unsafe fn create_vertex_array(&self) -> Result<Self::VertexArray, String> {
        let mut vertex_array = 0;
        gl::glGenVertexArrays(1, &mut vertex_array);
        Ok(NativeVertexArray(non_zero_gl_name(vertex_array)))
    }

    unsafe fn delete_vertex_array(&self, vertex_array: Self::VertexArray) {
        gl::glDeleteVertexArrays(1, &vertex_array.0.get());
    }

    unsafe fn bind_vertex_array(&self, vertex_array: Option<Self::VertexArray>) {
        gl::glBindVertexArray(vertex_array.map(|va| va.0.get()).unwrap_or(0));
    }

    unsafe fn clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        gl::glClearColor(red, green, blue, alpha);
    }

    unsafe fn supports_f64_precision() -> bool {
        // TODO: Handle OpenGL ES
        true
    }

    unsafe fn clear_depth_f64(&self, depth: f64) {
        gl::glClearDepth(depth);
    }

    unsafe fn clear_depth_f32(&self, depth: f32) {
        gl::glClearDepthf(depth);
    }

    unsafe fn clear_stencil(&self, stencil: i32) {
        gl::glClearStencil(stencil);
    }

    unsafe fn clear(&self, mask: u32) {
        gl::glClear(mask);
    }

    unsafe fn patch_parameter_i32(&self, parameter: u32, value: i32) {
        gl::glPatchParameteri(parameter, value);
    }

    unsafe fn pixel_store_i32(&self, parameter: u32, value: i32) {
        gl::glPixelStorei(parameter, value);
    }

    unsafe fn pixel_store_bool(&self, parameter: u32, value: bool) {
        gl::glPixelStorei(parameter, value as i32);
    }

    unsafe fn bind_frag_data_location(
        &self,
        program: Self::Program,
        color_number: u32,
        name: &str,
    ) {
        gl::glBindFragDataLocation(
            program.0.get(),
            color_number,
            name.as_ptr() as *const gl::GLchar,
        );
    }

    unsafe fn buffer_data_size(&self, target: u32, size: i32, usage: u32) {
        gl::glBufferData(target, size as isize, std::ptr::null(), usage);
    }

    unsafe fn buffer_data_u8_slice(&self, target: u32, data: &[u8], usage: u32) {
        gl::glBufferData(
            target,
            data.len() as isize,
            data.as_ptr() as *const std::ffi::c_void,
            usage,
        );
    }

    unsafe fn named_buffer_data_u8_slice(&self, buffer: Self::Buffer, data: &[u8], usage: u32) {
        gl::glNamedBufferData(
            buffer.0.get(),
            data.len() as isize,
            data.as_ptr() as *const std::ffi::c_void,
            usage,
        );
    }

    unsafe fn buffer_sub_data_u8_slice(&self, target: u32, offset: i32, src_data: &[u8]) {
        gl::glBufferSubData(
            target,
            offset as isize,
            src_data.len() as isize,
            src_data.as_ptr() as *const std::ffi::c_void,
        );
    }

    unsafe fn get_buffer_sub_data(&self, target: u32, offset: i32, dst_data: &mut [u8]) {
        gl::glGetBufferSubData(
            target,
            offset as isize,
            dst_data.len() as isize,
            dst_data.as_mut_ptr() as *mut std::ffi::c_void,
        );
    }

    unsafe fn buffer_storage(&self, target: u32, size: i32, data: Option<&[u8]>, flags: u32) {
        let size = size as isize;
        let data = data.map(|p| p.as_ptr()).unwrap_or(std::ptr::null()) as *const std::ffi::c_void;

        if self.is_loaded("glBufferStorage") {
            gl::glBufferStorage(target, size, data, flags);
        } else {
            gl::glBufferStorageEXT(target, size, data, flags);
        }
    }

    unsafe fn check_framebuffer_status(&self, target: u32) -> u32 {
        gl::glCheckFramebufferStatus(target)
    }

    unsafe fn clear_buffer_i32_slice(&self, target: u32, draw_buffer: u32, values: &[i32]) {
        gl::glClearBufferiv(target, draw_buffer as i32, values.as_ptr());
    }

    unsafe fn clear_buffer_u32_slice(&self, target: u32, draw_buffer: u32, values: &[u32]) {
        gl::glClearBufferuiv(target, draw_buffer as i32, values.as_ptr());
    }

    unsafe fn clear_buffer_f32_slice(&self, target: u32, draw_buffer: u32, values: &[f32]) {
        gl::glClearBufferfv(target, draw_buffer as i32, values.as_ptr());
    }

    unsafe fn clear_buffer_depth_stencil(
        &self,
        target: u32,
        draw_buffer: u32,
        depth: f32,
        stencil: i32,
    ) {
        gl::glClearBufferfi(target, draw_buffer as i32, depth, stencil);
    }

    unsafe fn client_wait_sync(&self, fence: Self::Fence, flags: u32, timeout: i32) -> u32 {
        gl::glClientWaitSync(fence.0, flags, timeout as u64)
    }

    unsafe fn wait_sync(&self, fence: Self::Fence, flags: u32, timeout: u64) {
        gl::glWaitSync(fence.0, flags, timeout)
    }

    unsafe fn copy_buffer_sub_data(
        &self,
        src_target: u32,
        dst_target: u32,
        src_offset: i32,
        dst_offset: i32,
        size: i32,
    ) {
        gl::glCopyBufferSubData(
            src_target,
            dst_target,
            src_offset as isize,
            dst_offset as isize,
            size as isize,
        );
    }

    unsafe fn copy_image_sub_data(
        &self,
        src_name: Self::Texture,
        src_target: u32,
        src_level: i32,
        src_x: i32,
        src_y: i32,
        src_z: i32,
        dst_name: Self::Texture,
        dst_target: u32,
        dst_level: i32,
        dst_x: i32,
        dst_y: i32,
        dst_z: i32,
        src_width: i32,
        src_height: i32,
        src_depth: i32,
    ) {
        gl::glCopyImageSubData(
            src_name.0.get(),
            src_target,
            src_level,
            src_x,
            src_y,
            src_z,
            dst_name.0.get(),
            dst_target,
            dst_level,
            dst_x,
            dst_y,
            dst_z,
            src_width as u32,
            src_height as u32,
            src_depth as u32,
        );
    }

    unsafe fn copy_tex_image_2d(
        &self,
        target: u32,
        level: i32,
        internal_format: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        border: i32,
    ) {
        gl::glCopyTexImage2D(
            target,
            level,
            internal_format,
            x,
            y,
            width as u32,
            height as u32,
            border,
        );
    }

    unsafe fn copy_tex_sub_image_2d(
        &self,
        target: u32,
        level: i32,
        x_offset: i32,
        y_offset: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        gl::glCopyTexSubImage2D(
            target,
            level,
            x_offset,
            y_offset,
            x,
            y,
            width as u32,
            height as u32,
        );
    }

    unsafe fn copy_tex_sub_image_3d(
        &self,
        target: u32,
        level: i32,
        x_offset: i32,
        y_offset: i32,
        z_offset: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        gl::glCopyTexSubImage3D(
            target,
            level,
            x_offset,
            y_offset,
            z_offset,
            x,
            y,
            width as u32,
            height as u32,
        );
    }

    unsafe fn delete_buffer(&self, buffer: Self::Buffer) {
        gl::glDeleteBuffers(1, &buffer.0.get());
    }

    unsafe fn delete_framebuffer(&self, framebuffer: Self::Framebuffer) {
        gl::glDeleteFramebuffers(1, &framebuffer.0.get());
    }

    unsafe fn delete_query(&self, query: Self::Query) {
        gl::glDeleteQueries(1, &query.0.get());
    }

    unsafe fn delete_renderbuffer(&self, renderbuffer: Self::Renderbuffer) {
        gl::glDeleteRenderbuffers(1, &renderbuffer.0.get());
    }

    unsafe fn delete_sampler(&self, sampler: Self::Sampler) {
        gl::glDeleteSamplers(1, &sampler.0.get());
    }

    unsafe fn delete_sync(&self, fence: Self::Fence) {
        gl::glDeleteSync(fence.0);
    }

    unsafe fn delete_texture(&self, texture: Self::Texture) {
        gl::glDeleteTextures(1, &texture.0.get());
    }

    unsafe fn disable(&self, parameter: u32) {
        gl::glDisable(parameter);
    }

    unsafe fn disable_draw_buffer(&self, parameter: u32, draw_buffer: u32) {
        gl::glDisablei(parameter, draw_buffer);
    }

    unsafe fn disable_vertex_attrib_array(&self, index: u32) {
        gl::glDisableVertexAttribArray(index);
    }

    unsafe fn dispatch_compute(&self, groups_x: u32, groups_y: u32, groups_z: u32) {
        gl::glDispatchCompute(groups_x, groups_y, groups_z);
    }

    unsafe fn dispatch_compute_indirect(&self, offset: i32) {
        gl::glDispatchComputeIndirect(offset as isize);
    }

    unsafe fn draw_arrays(&self, mode: u32, first: i32, count: i32) {
        gl::glDrawArrays(mode as u32, first, count as u32);
    }

    unsafe fn draw_arrays_instanced(&self, mode: u32, first: i32, count: i32, instance_count: i32) {
        gl::glDrawArraysInstanced(mode as u32, first, count as u32, instance_count as u32);
    }

    unsafe fn draw_arrays_instanced_base_instance(
        &self,
        mode: u32,
        first: i32,
        count: i32,
        instance_count: i32,
        base_instance: u32,
    ) {
        gl::glDrawArraysInstancedBaseInstance(
            mode as u32,
            first,
            count as u32,
            instance_count as u32,
            base_instance,
        );
    }

    unsafe fn draw_arrays_indirect_offset(&self, mode: u32, offset: i32) {
        gl::glDrawArraysIndirect(mode, offset as *const std::ffi::c_void);
    }

    unsafe fn draw_buffer(&self, draw_buffer: u32) {
        gl::glDrawBuffer(draw_buffer);
    }

    unsafe fn draw_buffers(&self, buffers: &[u32]) {
        gl::glDrawBuffers(buffers.len() as u32, buffers.as_ptr());
    }

    unsafe fn draw_elements(&self, mode: u32, count: i32, element_type: u32, offset: i32) {
        gl::glDrawElements(
            mode as u32,
            count as u32,
            element_type as u32,
            offset as *const std::ffi::c_void,
        );
    }

    unsafe fn draw_elements_base_vertex(
        &self,
        mode: u32,
        count: i32,
        element_type: u32,
        offset: i32,
        base_vertex: i32,
    ) {
        gl::glDrawElementsBaseVertex(
            mode as u32,
            count as u32,
            element_type as u32,
            offset as *const std::ffi::c_void,
            base_vertex,
        );
    }

    unsafe fn draw_elements_instanced(
        &self,
        mode: u32,
        count: i32,
        element_type: u32,
        offset: i32,
        instance_count: i32,
    ) {
        gl::glDrawElementsInstanced(
            mode as u32,
            count as u32,
            element_type as u32,
            offset as *const std::ffi::c_void,
            instance_count as u32,
        );
    }

    unsafe fn draw_elements_instanced_base_vertex(
        &self,
        mode: u32,
        count: i32,
        element_type: u32,
        offset: i32,
        instance_count: i32,
        base_vertex: i32,
    ) {
        gl::glDrawElementsInstancedBaseVertex(
            mode as u32,
            count as u32,
            element_type as u32,
            offset as *const std::ffi::c_void,
            instance_count as u32,
            base_vertex,
        );
    }

    unsafe fn draw_elements_instanced_base_vertex_base_instance(
        &self,
        mode: u32,
        count: i32,
        element_type: u32,
        offset: i32,
        instance_count: i32,
        base_vertex: i32,
        base_instance: u32,
    ) {
        gl::glDrawElementsInstancedBaseVertexBaseInstance(
            mode as u32,
            count as u32,
            element_type as u32,
            offset as *const std::ffi::c_void,
            instance_count as u32,
            base_vertex,
            base_instance,
        );
    }

    unsafe fn draw_elements_indirect_offset(&self, mode: u32, element_type: u32, offset: i32) {
        gl::glDrawElementsIndirect(mode, element_type, offset as *const std::ffi::c_void);
    }

    unsafe fn enable(&self, parameter: u32) {
        gl::glEnable(parameter);
    }

    unsafe fn is_enabled(&self, parameter: u32) -> bool {
        gl::glIsEnabled(parameter) != 0
    }

    unsafe fn enable_draw_buffer(&self, parameter: u32, draw_buffer: u32) {
        gl::glEnablei(parameter, draw_buffer);
    }

    unsafe fn enable_vertex_array_attrib(&self, vao: Self::VertexArray, index: u32) {
        gl::glEnableVertexArrayAttrib(vao.0.get(), index);
    }

    unsafe fn enable_vertex_attrib_array(&self, index: u32) {
        gl::glEnableVertexAttribArray(index);
    }

    unsafe fn flush(&self) {
        gl::glFlush();
    }

    unsafe fn framebuffer_renderbuffer(
        &self,
        target: u32,
        attachment: u32,
        renderbuffer_target: u32,
        renderbuffer: Option<Self::Renderbuffer>,
    ) {
        gl::glFramebufferRenderbuffer(
            target,
            attachment,
            renderbuffer_target,
            renderbuffer.map(|rb| rb.0.get()).unwrap_or(0),
        );
    }

    unsafe fn framebuffer_texture(
        &self,
        target: u32,
        attachment: u32,
        texture: Option<Self::Texture>,
        level: i32,
    ) {
        gl::glFramebufferTexture(
            target,
            attachment,
            texture.map(|t| t.0.get()).unwrap_or(0),
            level,
        );
    }

    unsafe fn framebuffer_texture_2d(
        &self,
        target: u32,
        attachment: u32,
        texture_target: u32,
        texture: Option<Self::Texture>,
        level: i32,
    ) {
        gl::glFramebufferTexture2D(
            target,
            attachment,
            texture_target,
            texture.map(|t| t.0.get()).unwrap_or(0),
            level,
        );
    }

    unsafe fn framebuffer_texture_3d(
        &self,
        target: u32,
        attachment: u32,
        texture_target: u32,
        texture: Option<Self::Texture>,
        level: i32,
        layer: i32,
    ) {
        gl::glFramebufferTexture3D(
            target,
            attachment,
            texture_target,
            texture.map(|t| t.0.get()).unwrap_or(0),
            level,
            layer,
        );
    }

    unsafe fn framebuffer_texture_layer(
        &self,
        target: u32,
        attachment: u32,
        texture: Option<Self::Texture>,
        level: i32,
        layer: i32,
    ) {
        gl::glFramebufferTextureLayer(
            target,
            attachment,
            texture.map(|t| t.0.get()).unwrap_or(0),
            level,
            layer,
        );
    }

    unsafe fn front_face(&self, value: u32) {
        gl::glFrontFace(value as u32);
    }

    unsafe fn get_error(&self) -> u32 {
        gl::glGetError()
    }

    unsafe fn get_tex_parameter_i32(&self, target: u32, parameter: u32) -> i32 {
        let mut value = 0;
        gl::glGetTexParameteriv(target, parameter, &mut value);
        value
    }

    unsafe fn get_buffer_parameter_i32(&self, target: u32, parameter: u32) -> i32 {
        let mut value = 0;
        gl::glGetBufferParameteriv(target, parameter, &mut value);
        value
    }

    unsafe fn get_parameter_i32(&self, parameter: u32) -> i32 {
        let mut value = 0;
        gl::glGetIntegerv(parameter, &mut value);
        value
    }

    unsafe fn get_parameter_i32_slice(&self, parameter: u32, out: &mut [i32]) {
        gl::glGetIntegerv(parameter, &mut out[0]);
    }

    unsafe fn get_parameter_f32(&self, parameter: u32) -> f32 {
        let mut value: f32 = 0.0;
        gl::glGetFloatv(parameter, &mut value);
        value
    }

    unsafe fn get_parameter_f32_slice(&self, parameter: u32, out: &mut [f32]) {
        gl::glGetFloatv(parameter, &mut out[0]);
    }

    unsafe fn get_parameter_indexed_i32(&self, parameter: u32, index: u32) -> i32 {
        let mut value = 0;
        gl::glGetIntegeri_v(parameter, index, &mut value);
        value
    }

    unsafe fn get_parameter_indexed_string(&self, parameter: u32, index: u32) -> String {
        let raw_ptr = gl::glGetStringi(parameter, index);
        std::ffi::CStr::from_ptr(raw_ptr as *const c_char)
            .to_str()
            .unwrap()
            .to_owned()
    }

    unsafe fn get_parameter_string(&self, parameter: u32) -> String {
        let raw_ptr = gl::glGetString(parameter);
        if raw_ptr.is_null() {
            panic!(
                "Get parameter string 0x{:X} failed. Maybe your GL context version is too outdated.",
                parameter
            )
        }
        std::ffi::CStr::from_ptr(raw_ptr as *const c_char)
            .to_str()
            .unwrap()
            .to_owned()
    }

    unsafe fn get_uniform_location(
        &self,
        program: Self::Program,
        name: &str,
    ) -> Option<Self::UniformLocation> {
        let name = CString::new(name).unwrap();
        let uniform_location =
            gl::glGetUniformLocation(program.0.get(), name.as_ptr() as *const gl::GLchar);
        if uniform_location < 0 {
            None
        } else {
            Some(NativeUniformLocation(uniform_location as u32))
        }
    }

    unsafe fn get_attrib_location(&self, program: Self::Program, name: &str) -> Option<u32> {
        let name = CString::new(name).unwrap();
        let attrib_location =
            gl::glGetAttribLocation(program.0.get(), name.as_ptr() as *const gl::GLchar);
        if attrib_location < 0 {
            None
        } else {
            Some(attrib_location as u32)
        }
    }

    unsafe fn bind_attrib_location(&self, program: Self::Program, index: u32, name: &str) {
        let name = CString::new(name).unwrap();
        gl::glBindAttribLocation(program.0.get(), index, name.as_ptr() as *const gl::GLchar);
    }

    unsafe fn get_active_attributes(&self, program: Self::Program) -> u32 {
        let mut count = 0;
        gl::glGetProgramiv(program.0.get(), ACTIVE_ATTRIBUTES, &mut count);
        count as u32
    }

    unsafe fn get_active_attribute(
        &self,
        program: Self::Program,
        index: u32,
    ) -> Option<ActiveAttribute> {
        let mut attribute_max_size = 0;
        gl::glGetProgramiv(
            program.0.get(),
            ACTIVE_ATTRIBUTE_MAX_LENGTH,
            &mut attribute_max_size,
        );
        let mut name = String::with_capacity(attribute_max_size as usize);
        name.extend(std::iter::repeat('\0').take(attribute_max_size as usize));
        let mut length = 0;
        let mut size = 0;
        let mut atype = 0;
        gl::glGetActiveAttrib(
            program.0.get(),
            index,
            attribute_max_size as u32,
            &mut length,
            &mut size,
            &mut atype,
            name.as_ptr() as *mut gl::GLchar,
        );

        name.truncate(length as usize);

        Some(ActiveAttribute { name, size, atype })
    }

    unsafe fn get_sync_status(&self, fence: Self::Fence) -> u32 {
        let mut len = 0;
        let mut values = [UNSIGNALED as i32];
        gl::glGetSynciv(
            fence.0,
            SYNC_STATUS,
            values.len() as u32,
            &mut len,
            values.as_mut_ptr(),
        );
        values[0] as u32
    }

    unsafe fn is_sync(&self, fence: Self::Fence) -> bool {
        1 == gl::glIsSync(fence.0)
    }

    unsafe fn renderbuffer_storage(
        &self,
        target: u32,
        internal_format: u32,
        width: i32,
        height: i32,
    ) {
        gl::glRenderbufferStorage(target, internal_format, width as u32, height as u32);
    }

    unsafe fn renderbuffer_storage_multisample(
        &self,
        target: u32,
        samples: i32,
        internal_format: u32,
        width: i32,
        height: i32,
    ) {
        gl::glRenderbufferStorageMultisample(
            target,
            samples as u32,
            internal_format,
            width as u32,
            height as u32,
        );
    }

    unsafe fn sampler_parameter_f32(&self, sampler: Self::Sampler, name: u32, value: f32) {
        gl::glSamplerParameterf(sampler.0.get(), name, value);
    }

    unsafe fn sampler_parameter_f32_slice(&self, sampler: Self::Sampler, name: u32, value: &[f32]) {
        gl::glSamplerParameterfv(sampler.0.get(), name, value.as_ptr());
    }

    unsafe fn sampler_parameter_i32(&self, sampler: Self::Sampler, name: u32, value: i32) {
        gl::glSamplerParameteri(sampler.0.get(), name, value);
    }

    unsafe fn generate_mipmap(&self, target: u32) {
        gl::glGenerateMipmap(target);
    }

    unsafe fn generate_texture_mipmap(&self, texture: Self::Texture) {
        gl::glGenerateTextureMipmap(texture.0.get());
    }

    unsafe fn tex_image_1d(
        &self,
        target: u32,
        level: i32,
        internal_format: i32,
        width: i32,
        border: i32,
        format: u32,
        ty: u32,
        pixels: Option<&[u8]>,
    ) {
        gl::glTexImage1D(
            target,
            level,
            internal_format,
            width as u32,
            border,
            format,
            ty,
            pixels.map(|p| p.as_ptr()).unwrap_or(std::ptr::null()) as *const std::ffi::c_void,
        );
    }

    unsafe fn compressed_tex_image_1d(
        &self,
        target: u32,
        level: i32,
        internal_format: i32,
        width: i32,
        border: i32,
        image_size: i32,
        pixels: &[u8],
    ) {
        gl::glCompressedTexImage1D(
            target,
            level,
            internal_format as u32,
            width as u32,
            border,
            image_size as u32,
            pixels.as_ptr() as *const std::ffi::c_void,
        );
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
        gl::glTexImage2D(
            target,
            level,
            internal_format,
            width as u32,
            height as u32,
            border,
            format,
            ty,
            pixels.map(|p| p.as_ptr()).unwrap_or(std::ptr::null()) as *const std::ffi::c_void,
        );
    }

    unsafe fn tex_image_2d_multisample(
        &self,
        target: u32,
        samples: i32,
        internal_format: i32,
        width: i32,
        height: i32,
        fixed_sample_locations: bool,
    ) {
        gl::glTexImage2DMultisample(
            target,
            samples as u32,
            internal_format as u32,
            width as u32,
            height as u32,
            if fixed_sample_locations { 1 } else { 0 },
        );
    }

    unsafe fn compressed_tex_image_2d(
        &self,
        target: u32,
        level: i32,
        internal_format: i32,
        width: i32,
        height: i32,
        border: i32,
        image_size: i32,
        pixels: &[u8],
    ) {
        gl::glCompressedTexImage2D(
            target,
            level,
            internal_format as u32,
            width as u32,
            height as u32,
            border,
            image_size as u32,
            pixels.as_ptr() as *const std::ffi::c_void,
        );
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
        gl::glTexImage3D(
            target,
            level,
            internal_format,
            width as u32,
            height as u32,
            depth as u32,
            border,
            format,
            ty,
            pixels.map(|p| p.as_ptr()).unwrap_or(std::ptr::null()) as *const std::ffi::c_void,
        );
    }

    unsafe fn compressed_tex_image_3d(
        &self,
        target: u32,
        level: i32,
        internal_format: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        image_size: i32,
        pixels: &[u8],
    ) {
        gl::glCompressedTexImage3D(
            target,
            level,
            internal_format as u32,
            width as u32,
            height as u32,
            depth as u32,
            border,
            image_size as u32,
            pixels.as_ptr() as *const std::ffi::c_void,
        );
    }

    unsafe fn tex_storage_1d(&self, target: u32, levels: i32, internal_format: u32, width: i32) {
        gl::glTexStorage1D(target, levels as u32, internal_format, width as u32);
    }

    unsafe fn tex_storage_2d(
        &self,
        target: u32,
        levels: i32,
        internal_format: u32,
        width: i32,
        height: i32,
    ) {
        gl::glTexStorage2D(
            target,
            levels as u32,
            internal_format,
            width as u32,
            height as u32,
        );
    }

    unsafe fn tex_storage_2d_multisample(
        &self,
        target: u32,
        samples: i32,
        internal_format: u32,
        width: i32,
        height: i32,
        fixed_sample_locations: bool,
    ) {
        gl::glTexStorage2DMultisample(
            target,
            samples as u32,
            internal_format,
            width as u32,
            height as u32,
            if fixed_sample_locations { 1 } else { 0 },
        );
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
        gl::glTexStorage3D(
            target,
            levels as u32,
            internal_format,
            width as u32,
            height as u32,
            depth as u32,
        );
    }

    unsafe fn texture_storage_3d(
        &self,
        texture: Self::Texture,
        levels: i32,
        internal_format: u32,
        width: i32,
        height: i32,
        depth: i32,
    ) {
        gl::glTextureStorage3D(
            texture.0.get(),
            levels as u32,
            internal_format,
            width as u32,
            height as u32,
            depth as u32,
        );
    }

    unsafe fn get_uniform_i32(
        &self,
        program: Self::Program,
        location: &Self::UniformLocation,
        v: &mut [i32],
    ) {
        gl::glGetUniformiv(
            program.0.get() as u32,
            location.0 as i32,
            v.as_mut_ptr() as *mut i32,
        )
    }

    unsafe fn get_uniform_f32(
        &self,
        program: Self::Program,
        location: &Self::UniformLocation,
        v: &mut [f32],
    ) {
        gl::glGetUniformfv(
            program.0.get() as u32,
            location.0 as i32,
            v.as_mut_ptr() as *mut f32,
        )
    }

    unsafe fn uniform_1_i32(&self, location: Option<&Self::UniformLocation>, x: i32) {
        if let Some(loc) = location {
            gl::glUniform1i(loc.0 as i32, x);
        }
    }

    unsafe fn uniform_2_i32(&self, location: Option<&Self::UniformLocation>, x: i32, y: i32) {
        if let Some(loc) = location {
            gl::glUniform2i(loc.0 as i32, x, y);
        }
    }

    unsafe fn uniform_3_i32(
        &self,
        location: Option<&Self::UniformLocation>,
        x: i32,
        y: i32,
        z: i32,
    ) {
        if let Some(loc) = location {
            gl::glUniform3i(loc.0 as i32, x, y, z);
        }
    }

    unsafe fn uniform_4_i32(
        &self,
        location: Option<&Self::UniformLocation>,
        x: i32,
        y: i32,
        z: i32,
        w: i32,
    ) {
        if let Some(loc) = location {
            gl::glUniform4i(loc.0 as i32, x, y, z, w);
        }
    }

    unsafe fn uniform_1_i32_slice(&self, location: Option<&Self::UniformLocation>, v: &[i32]) {
        if let Some(loc) = location {
            gl::glUniform1iv(loc.0 as i32, v.len() as u32, v.as_ptr());
        }
    }

    unsafe fn uniform_2_i32_slice(&self, location: Option<&Self::UniformLocation>, v: &[i32]) {
        if let Some(loc) = location {
            gl::glUniform2iv(loc.0 as i32, v.len() as u32 / 2, v.as_ptr());
        }
    }

    unsafe fn uniform_3_i32_slice(&self, location: Option<&Self::UniformLocation>, v: &[i32]) {
        if let Some(loc) = location {
            gl::glUniform3iv(loc.0 as i32, v.len() as u32 / 3, v.as_ptr());
        }
    }

    unsafe fn uniform_4_i32_slice(&self, location: Option<&Self::UniformLocation>, v: &[i32]) {
        if let Some(loc) = location {
            gl::glUniform4iv(loc.0 as i32, v.len() as u32 / 4, v.as_ptr());
        }
    }

    unsafe fn uniform_1_u32(&self, location: Option<&Self::UniformLocation>, x: u32) {
        if let Some(loc) = location {
            gl::glUniform1ui(loc.0 as i32, x);
        }
    }

    unsafe fn uniform_2_u32(&self, location: Option<&Self::UniformLocation>, x: u32, y: u32) {
        if let Some(loc) = location {
            gl::glUniform2ui(loc.0 as i32, x, y);
        }
    }

    unsafe fn uniform_3_u32(
        &self,
        location: Option<&Self::UniformLocation>,
        x: u32,
        y: u32,
        z: u32,
    ) {
        if let Some(loc) = location {
            gl::glUniform3ui(loc.0 as i32, x, y, z);
        }
    }

    unsafe fn uniform_4_u32(
        &self,
        location: Option<&Self::UniformLocation>,
        x: u32,
        y: u32,
        z: u32,
        w: u32,
    ) {
        if let Some(loc) = location {
            gl::glUniform4ui(loc.0 as i32, x, y, z, w);
        }
    }

    unsafe fn uniform_1_u32_slice(&self, location: Option<&Self::UniformLocation>, v: &[u32]) {
        if let Some(loc) = location {
            gl::glUniform1uiv(loc.0 as i32, v.len() as u32, v.as_ptr());
        }
    }

    unsafe fn uniform_2_u32_slice(&self, location: Option<&Self::UniformLocation>, v: &[u32]) {
        if let Some(loc) = location {
            gl::glUniform2uiv(loc.0 as i32, v.len() as u32 / 2, v.as_ptr());
        }
    }

    unsafe fn uniform_3_u32_slice(&self, location: Option<&Self::UniformLocation>, v: &[u32]) {
        if let Some(loc) = location {
            gl::glUniform3uiv(loc.0 as i32, v.len() as u32 / 3, v.as_ptr());
        }
    }

    unsafe fn uniform_4_u32_slice(&self, location: Option<&Self::UniformLocation>, v: &[u32]) {
        if let Some(loc) = location {
            gl::glUniform4uiv(loc.0 as i32, v.len() as u32 / 4, v.as_ptr());
        }
    }

    unsafe fn uniform_1_f32(&self, location: Option<&Self::UniformLocation>, x: f32) {
        if let Some(loc) = location {
            gl::glUniform1f(loc.0 as i32, x);
        }
    }

    unsafe fn uniform_2_f32(&self, location: Option<&Self::UniformLocation>, x: f32, y: f32) {
        if let Some(loc) = location {
            gl::glUniform2f(loc.0 as i32, x, y);
        }
    }

    unsafe fn uniform_3_f32(
        &self,
        location: Option<&Self::UniformLocation>,
        x: f32,
        y: f32,
        z: f32,
    ) {
        if let Some(loc) = location {
            gl::glUniform3f(loc.0 as i32, x, y, z);
        }
    }

    unsafe fn uniform_4_f32(
        &self,
        location: Option<&Self::UniformLocation>,
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    ) {
        if let Some(loc) = location {
            gl::glUniform4f(loc.0 as i32, x, y, z, w);
        }
    }

    unsafe fn uniform_1_f32_slice(&self, location: Option<&Self::UniformLocation>, v: &[f32]) {
        if let Some(loc) = location {
            gl::glUniform1fv(loc.0 as i32, v.len() as u32, v.as_ptr());
        }
    }

    unsafe fn uniform_2_f32_slice(&self, location: Option<&Self::UniformLocation>, v: &[f32]) {
        if let Some(loc) = location {
            gl::glUniform2fv(loc.0 as i32, v.len() as u32 / 2, v.as_ptr());
        }
    }

    unsafe fn uniform_3_f32_slice(&self, location: Option<&Self::UniformLocation>, v: &[f32]) {
        if let Some(loc) = location {
            gl::glUniform3fv(loc.0 as i32, v.len() as u32 / 3, v.as_ptr());
        }
    }

    unsafe fn uniform_4_f32_slice(&self, location: Option<&Self::UniformLocation>, v: &[f32]) {
        if let Some(loc) = location {
            gl::glUniform4fv(loc.0 as i32, v.len() as u32 / 4, v.as_ptr());
        }
    }

    unsafe fn uniform_matrix_2_f32_slice(
        &self,
        location: Option<&Self::UniformLocation>,
        transpose: bool,
        v: &[f32],
    ) {
        if let Some(loc) = location {
            gl::glUniformMatrix2fv(
                loc.0 as i32,
                v.len() as u32 / 4,
                transpose as u32,
                v.as_ptr(),
            );
        }
    }

    unsafe fn uniform_matrix_3_f32_slice(
        &self,
        location: Option<&Self::UniformLocation>,
        transpose: bool,
        v: &[f32],
    ) {
        if let Some(loc) = location {
            gl::glUniformMatrix3fv(
                loc.0 as i32,
                v.len() as u32 / 9,
                transpose as u32,
                v.as_ptr(),
            );
        }
    }

    unsafe fn uniform_matrix_4_f32_slice(
        &self,
        location: Option<&Self::UniformLocation>,
        transpose: bool,
        v: &[f32],
    ) {
        if let Some(loc) = location {
            gl::glUniformMatrix4fv(
                loc.0 as i32,
                v.len() as u32 / 16,
                transpose as u32,
                v.as_ptr(),
            );
        }
    }

    unsafe fn unmap_buffer(&self, target: u32) {
        gl::glUnmapBuffer(target);
    }

    unsafe fn cull_face(&self, value: u32) {
        gl::glCullFace(value as u32);
    }

    unsafe fn color_mask(&self, red: bool, green: bool, blue: bool, alpha: bool) {
        gl::glColorMask(red as u32, green as u32, blue as u32, alpha as u32);
    }

    unsafe fn color_mask_draw_buffer(
        &self,
        draw_buffer: u32,
        red: bool,
        green: bool,
        blue: bool,
        alpha: bool,
    ) {
        gl::glColorMaski(
            draw_buffer,
            red as u32,
            green as u32,
            blue as u32,
            alpha as u32,
        );
    }

    unsafe fn depth_mask(&self, value: bool) {
        gl::glDepthMask(value as u32);
    }

    unsafe fn blend_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        gl::glBlendColor(red, green, blue, alpha);
    }

    unsafe fn line_width(&self, width: f32) {
        gl::glLineWidth(width);
    }

    unsafe fn map_buffer_range(
        &self,
        target: u32,
        offset: i32,
        length: i32,
        access: u32,
    ) -> *mut u8 {
        gl::glMapBufferRange(target, offset as isize, length as isize, access) as *mut u8
    }

    unsafe fn flush_mapped_buffer_range(&self, target: u32, offset: i32, length: i32) {
        gl::glFlushMappedBufferRange(target, offset as isize, length as isize)
    }

    unsafe fn invalidate_buffer_sub_data(&self, target: u32, offset: i32, length: i32) {
        gl::glInvalidateBufferSubData(target, offset as isize, length as isize)
    }

    unsafe fn invalidate_framebuffer(&self, target: u32, attachments: &[u32]) {
        gl::glInvalidateFramebuffer(target, attachments.len() as u32, attachments.as_ptr());
    }

    unsafe fn polygon_offset(&self, factor: f32, units: f32) {
        gl::glPolygonOffset(factor, units);
    }

    unsafe fn polygon_mode(&self, face: u32, mode: u32) {
        gl::glPolygonMode(face as u32, mode as u32);
    }

    unsafe fn finish(&self) {
        gl::glFinish();
    }

    unsafe fn bind_texture(&self, target: u32, texture: Option<Self::Texture>) {
        gl::glBindTexture(target, texture.map(|t| t.0.get()).unwrap_or(0));
    }

    unsafe fn bind_sampler(&self, unit: u32, sampler: Option<Self::Sampler>) {
        gl::glBindSampler(unit, sampler.map(|s| s.0.get()).unwrap_or(0));
    }

    unsafe fn active_texture(&self, unit: u32) {
        gl::glActiveTexture(unit);
    }

    unsafe fn fence_sync(&self, condition: u32, flags: u32) -> Result<Self::Fence, String> {
        Ok(NativeFence(gl::glFenceSync(condition as u32, flags)))
    }

    unsafe fn tex_parameter_f32(&self, target: u32, parameter: u32, value: f32) {
        gl::glTexParameterf(target, parameter, value);
    }

    unsafe fn tex_parameter_i32(&self, target: u32, parameter: u32, value: i32) {
        gl::glTexParameteri(target, parameter, value);
    }

    unsafe fn texture_parameter_i32(&self, texture: Self::Texture, parameter: u32, value: i32) {
        gl::glTextureParameteri(texture.0.get(), parameter, value);
    }

    unsafe fn tex_parameter_f32_slice(&self, target: u32, parameter: u32, values: &[f32]) {
        gl::glTexParameterfv(target, parameter, values.as_ptr());
    }

    unsafe fn tex_parameter_i32_slice(&self, target: u32, parameter: u32, values: &[i32]) {
        gl::glTexParameteriv(target, parameter, values.as_ptr());
    }

    unsafe fn tex_sub_image_2d(
        &self,
        target: u32,
        level: i32,
        x_offset: i32,
        y_offset: i32,
        width: i32,
        height: i32,
        format: u32,
        ty: u32,
        pixels: PixelUnpackData,
    ) {
        gl::glTexSubImage2D(
            target,
            level,
            x_offset,
            y_offset,
            width as u32,
            height as u32,
            format,
            ty,
            match pixels {
                PixelUnpackData::BufferOffset(offset) => offset as *const std::ffi::c_void,
                PixelUnpackData::Slice(data) => data.as_ptr() as *const std::ffi::c_void,
            },
        );
    }

    unsafe fn compressed_tex_sub_image_2d(
        &self,
        target: u32,
        level: i32,
        x_offset: i32,
        y_offset: i32,
        width: i32,
        height: i32,
        format: u32,
        pixels: CompressedPixelUnpackData,
    ) {
        let (data, image_size) = match pixels {
            CompressedPixelUnpackData::BufferRange(ref range) => (
                range.start as *const std::ffi::c_void,
                (range.end - range.start) as i32,
            ),
            CompressedPixelUnpackData::Slice(data) => {
                (data.as_ptr() as *const std::ffi::c_void, data.len() as i32)
            }
        };

        gl::glCompressedTexSubImage2D(
            target,
            level,
            x_offset,
            y_offset,
            width as u32,
            height as u32,
            format,
            image_size as u32,
            data,
        );
    }

    unsafe fn tex_sub_image_3d(
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
        pixels: PixelUnpackData,
    ) {
        gl::glTexSubImage3D(
            target,
            level,
            x_offset,
            y_offset,
            z_offset,
            width as u32,
            height as u32,
            depth as u32,
            format,
            ty,
            match pixels {
                PixelUnpackData::BufferOffset(offset) => offset as *const std::ffi::c_void,
                PixelUnpackData::Slice(data) => data.as_ptr() as *const std::ffi::c_void,
            },
        );
    }

    unsafe fn texture_sub_image_3d(
        &self,
        texture: Self::Texture,
        level: i32,
        x_offset: i32,
        y_offset: i32,
        z_offset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        ty: u32,
        pixels: PixelUnpackData,
    ) {
        gl::glTextureSubImage3D(
            texture.0.get(),
            level,
            x_offset,
            y_offset,
            z_offset,
            width as u32,
            height as u32,
            depth as u32,
            format,
            ty,
            match pixels {
                PixelUnpackData::BufferOffset(offset) => offset as *const std::ffi::c_void,
                PixelUnpackData::Slice(data) => data.as_ptr() as *const std::ffi::c_void,
            },
        );
    }

    unsafe fn compressed_tex_sub_image_3d(
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
        pixels: CompressedPixelUnpackData,
    ) {
        let (data, image_size) = match pixels {
            CompressedPixelUnpackData::BufferRange(ref range) => (
                range.start as *const std::ffi::c_void,
                (range.end - range.start) as i32,
            ),
            CompressedPixelUnpackData::Slice(data) => {
                (data.as_ptr() as *const std::ffi::c_void, data.len() as i32)
            }
        };

        gl::glCompressedTexSubImage3D(
            target,
            level,
            x_offset,
            y_offset,
            z_offset,
            width as u32,
            height as u32,
            depth as u32,
            format,
            image_size as u32,
            data,
        );
    }

    unsafe fn depth_func(&self, func: u32) {
        gl::glDepthFunc(func as u32);
    }

    unsafe fn depth_range_f32(&self, near: f32, far: f32) {
        gl::glDepthRangef(near, far);
    }

    unsafe fn depth_range_f64(&self, near: f64, far: f64) {
        gl::glDepthRange(near, far);
    }

    unsafe fn depth_range_f64_slice(&self, first: u32, count: i32, values: &[[f64; 2]]) {
        gl::glDepthRangeArrayv(first, count as u32, values.as_ptr() as *const f64);
    }

    unsafe fn scissor(&self, x: i32, y: i32, width: i32, height: i32) {
        gl::glScissor(x, y, width as u32, height as u32);
    }

    unsafe fn scissor_slice(&self, first: u32, count: i32, scissors: &[[i32; 4]]) {
        gl::glScissorArrayv(first, count as u32, scissors.as_ptr() as *const i32);
    }

    unsafe fn vertex_array_attrib_binding_f32(
        &self,
        vao: Self::VertexArray,
        index: u32,
        binding_index: u32,
    ) {
        gl::glVertexArrayAttribBinding(vao.0.get(), index, binding_index);
    }

    unsafe fn vertex_array_attrib_format_f32(
        &self,
        vao: Self::VertexArray,
        index: u32,
        size: i32,
        data_type: u32,
        normalized: bool,
        relative_offset: u32,
    ) {
        gl::glVertexArrayAttribFormat(
            vao.0.get(),
            index,
            size,
            data_type,
            normalized as u32,
            relative_offset,
        );
    }

    unsafe fn vertex_array_attrib_format_i32(
        &self,
        vao: Self::VertexArray,
        index: u32,
        size: i32,
        data_type: u32,
        relative_offset: u32,
    ) {
        gl::glVertexArrayAttribIFormat(vao.0.get(), index, size, data_type, relative_offset);
    }

    unsafe fn vertex_array_element_buffer(
        &self,
        vao: Self::VertexArray,
        buffer: Option<Self::Buffer>,
    ) {
        gl::glVertexArrayElementBuffer(vao.0.get(), buffer.map(|b| b.0.get()).unwrap_or(0));
    }

    unsafe fn vertex_array_vertex_buffer(
        &self,
        vao: Self::VertexArray,
        binding_index: u32,
        buffer: Option<Self::Buffer>,
        offset: i32,
        stride: i32,
    ) {
        gl::glVertexArrayVertexBuffer(
            vao.0.get(),
            binding_index,
            buffer.map(|b| b.0.get()).unwrap_or(0),
            offset as isize,
            stride as u32,
        );
    }

    unsafe fn vertex_attrib_divisor(&self, index: u32, divisor: u32) {
        gl::glVertexAttribDivisor(index, divisor);
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
        gl::glVertexAttribPointer(
            index,
            size,
            data_type,
            normalized as u32,
            stride as u32,
            offset as *const std::ffi::c_void,
        );
    }

    unsafe fn vertex_attrib_pointer_i32(
        &self,
        index: u32,
        size: i32,
        data_type: u32,
        stride: i32,
        offset: i32,
    ) {
        gl::glVertexAttribIPointer(
            index,
            size,
            data_type,
            stride as u32,
            offset as *const std::ffi::c_void,
        );
    }

    unsafe fn vertex_attrib_pointer_f64(
        &self,
        index: u32,
        size: i32,
        data_type: u32,
        stride: i32,
        offset: i32,
    ) {
        gl::glVertexAttribLPointer(
            index,
            size,
            data_type,
            stride as u32,
            offset as *const std::ffi::c_void,
        );
    }

    unsafe fn vertex_attrib_format_f32(
        &self,
        index: u32,
        size: i32,
        data_type: u32,
        normalized: bool,
        relative_offset: u32,
    ) {
        gl::glVertexAttribFormat(index, size, data_type, normalized as u32, relative_offset);
    }

    unsafe fn vertex_attrib_format_i32(
        &self,
        index: u32,
        size: i32,
        data_type: u32,
        relative_offset: u32,
    ) {
        gl::glVertexAttribIFormat(index, size, data_type, relative_offset);
    }

    unsafe fn vertex_attrib_1_f32(&self, index: u32, x: f32) {
        gl::glVertexAttrib1f(index, x);
    }

    unsafe fn vertex_attrib_2_f32(&self, index: u32, x: f32, y: f32) {
        gl::glVertexAttrib2f(index, x, y);
    }

    unsafe fn vertex_attrib_3_f32(&self, index: u32, x: f32, y: f32, z: f32) {
        gl::glVertexAttrib3f(index, x, y, z);
    }

    unsafe fn vertex_attrib_4_f32(&self, index: u32, x: f32, y: f32, z: f32, w: f32) {
        gl::glVertexAttrib4f(index, x, y, z, w);
    }

    unsafe fn vertex_attrib_1_f32_slice(&self, index: u32, v: &[f32]) {
        gl::glVertexAttrib1fv(index, v.as_ptr());
    }

    unsafe fn vertex_attrib_2_f32_slice(&self, index: u32, v: &[f32]) {
        gl::glVertexAttrib2fv(index, v.as_ptr());
    }

    unsafe fn vertex_attrib_3_f32_slice(&self, index: u32, v: &[f32]) {
        gl::glVertexAttrib3fv(index, v.as_ptr());
    }

    unsafe fn vertex_attrib_4_f32_slice(&self, index: u32, v: &[f32]) {
        gl::glVertexAttrib4fv(index, v.as_ptr());
    }

    unsafe fn vertex_attrib_binding(&self, attrib_index: u32, binding_index: u32) {
        gl::glVertexAttribBinding(attrib_index, binding_index);
    }

    unsafe fn vertex_binding_divisor(&self, binding_index: u32, divisor: u32) {
        gl::glVertexBindingDivisor(binding_index, divisor);
    }

    unsafe fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        gl::glViewport(x, y, width as u32, height as u32);
    }

    unsafe fn viewport_f32_slice(&self, first: u32, count: i32, values: &[[f32; 4]]) {
        gl::glViewportArrayv(first, count as u32, values.as_ptr() as *const f32);
    }

    unsafe fn blend_equation(&self, mode: u32) {
        gl::glBlendEquation(mode as u32);
    }

    unsafe fn blend_equation_draw_buffer(&self, draw_buffer: u32, mode: u32) {
        gl::glBlendEquationi(draw_buffer, mode as u32);
    }

    unsafe fn blend_equation_separate(&self, mode_rgb: u32, mode_alpha: u32) {
        gl::glBlendEquationSeparate(mode_rgb as u32, mode_alpha as u32);
    }

    unsafe fn blend_equation_separate_draw_buffer(
        &self,
        draw_buffer: u32,
        mode_rgb: u32,
        mode_alpha: u32,
    ) {
        gl::glBlendEquationSeparatei(draw_buffer, mode_rgb as u32, mode_alpha as u32);
    }

    unsafe fn blend_func(&self, src: u32, dst: u32) {
        gl::glBlendFunc(src as u32, dst as u32);
    }

    unsafe fn blend_func_draw_buffer(&self, draw_buffer: u32, src: u32, dst: u32) {
        gl::glBlendFunci(draw_buffer, src as u32, dst as u32);
    }

    unsafe fn blend_func_separate(
        &self,
        src_rgb: u32,
        dst_rgb: u32,
        src_alpha: u32,
        dst_alpha: u32,
    ) {
        gl::glBlendFuncSeparate(
            src_rgb as u32,
            dst_rgb as u32,
            src_alpha as u32,
            dst_alpha as u32,
        );
    }

    unsafe fn blend_func_separate_draw_buffer(
        &self,
        draw_buffer: u32,
        src_rgb: u32,
        dst_rgb: u32,
        src_alpha: u32,
        dst_alpha: u32,
    ) {
        gl::glBlendFuncSeparatei(
            draw_buffer,
            src_rgb as u32,
            dst_rgb as u32,
            src_alpha as u32,
            dst_alpha as u32,
        );
    }

    unsafe fn stencil_func(&self, func: u32, reference: i32, mask: u32) {
        gl::glStencilFunc(func as u32, reference, mask);
    }

    unsafe fn stencil_func_separate(&self, face: u32, func: u32, reference: i32, mask: u32) {
        gl::glStencilFuncSeparate(face as u32, func as u32, reference, mask);
    }

    unsafe fn stencil_mask(&self, mask: u32) {
        gl::glStencilMask(mask);
    }

    unsafe fn stencil_mask_separate(&self, face: u32, mask: u32) {
        gl::glStencilMaskSeparate(face as u32, mask);
    }

    unsafe fn stencil_op(&self, stencil_fail: u32, depth_fail: u32, pass: u32) {
        gl::glStencilOp(stencil_fail as u32, depth_fail as u32, pass as u32);
    }

    unsafe fn stencil_op_separate(&self, face: u32, stencil_fail: u32, depth_fail: u32, pass: u32) {
        gl::glStencilOpSeparate(
            face as u32,
            stencil_fail as u32,
            depth_fail as u32,
            pass as u32,
        );
    }

    unsafe fn debug_message_control(
        &self,
        source: u32,
        msg_type: u32,
        severity: u32,
        ids: &[u32],
        enabled: bool,
    ) {
        let ids_ptr = if ids.is_empty() {
            std::ptr::null()
        } else {
            ids.as_ptr()
        };

        gl::glDebugMessageControl(
            source,
            msg_type,
            severity,
            ids.len() as u32,
            ids_ptr,
            enabled as u32,
        );
    }

    unsafe fn debug_message_insert<S>(
        &self,
        source: u32,
        msg_type: u32,
        id: u32,
        severity: u32,
        msg: S,
    ) where
        S: AsRef<str>,
    {
        let message = msg.as_ref().as_bytes();
        let length = message.len() as i32;
        gl::glDebugMessageInsert(
            source,
            msg_type,
            id,
            severity,
            length as u32,
            message.as_ptr() as *const gl::GLchar,
        );
    }

    unsafe fn debug_message_callback<F>(&self, mut callback: F)
    where
        F: FnMut(u32, u32, u32, u32, &str),
    {
        gl::glDebugMessageCallback(
            Some(raw_debug_message_callback::<F>),
            &mut callback as *mut _ as *mut std::ffi::c_void,
        );
    }

    unsafe fn get_debug_message_log(&self, count: u32) -> Vec<DebugMessageLogEntry> {
        let ct = count as usize;
        let mut sources = Vec::with_capacity(ct);
        let mut types = Vec::with_capacity(ct);
        let mut ids = Vec::with_capacity(ct);
        let mut severities = Vec::with_capacity(ct);
        let mut lengths = Vec::with_capacity(ct);
        let buf_size = (count * MAX_DEBUG_MESSAGE_LENGTH) as i32;
        let mut message_log = Vec::with_capacity(buf_size as usize);

        let received = gl::glGetDebugMessageLog(
            count,
            buf_size as u32,
            sources.as_mut_ptr(),
            types.as_mut_ptr(),
            ids.as_mut_ptr(),
            severities.as_mut_ptr(),
            lengths.as_mut_ptr(),
            message_log.as_mut_ptr(),
        ) as usize;

        sources.set_len(received);
        types.set_len(received);
        ids.set_len(received);
        severities.set_len(received);
        lengths.set_len(received);
        message_log.set_len(buf_size as usize);

        let mut entries = Vec::new();
        let mut offset = 0;
        for i in 0..received {
            let message = std::ffi::CStr::from_ptr(message_log[offset..].as_ptr() as *const c_char)
                .to_string_lossy();
            offset += lengths[i] as usize;
            entries.push(DebugMessageLogEntry {
                source: sources[i],
                msg_type: types[i],
                id: ids[i],
                severity: severities[i],
                message: message.to_string(),
            });
        }

        entries
    }

    unsafe fn push_debug_group<S>(&self, source: u32, id: u32, message: S)
    where
        S: AsRef<str>,
    {
        let msg = message.as_ref().as_bytes();
        let length = msg.len() as i32;
        gl::glPushDebugGroup(source, id, length as u32, msg.as_ptr() as *const gl::GLchar);
    }

    unsafe fn pop_debug_group(&self) {
        gl::glPopDebugGroup();
    }

    unsafe fn object_label<S>(&self, identifier: u32, name: u32, label: Option<S>)
    where
        S: AsRef<str>,
    {
        match label {
            Some(l) => {
                let lbl = l.as_ref().as_bytes();
                let length = lbl.len() as i32;
                gl::glObjectLabel(
                    identifier,
                    name,
                    length as u32,
                    lbl.as_ptr() as *const gl::GLchar,
                );
            }
            None => gl::glObjectLabel(identifier, name, 0, std::ptr::null()),
        }
    }

    unsafe fn get_object_label(&self, identifier: u32, name: u32) -> String {
        let mut len = 0;
        let mut label_buf = Vec::with_capacity(self.constants.max_label_length as usize);
        gl::glGetObjectLabel(
            identifier,
            name,
            self.constants.max_label_length as u32,
            &mut len,
            label_buf.as_mut_ptr(),
        );
        label_buf.set_len(len as usize);
        std::ffi::CStr::from_ptr(label_buf.as_ptr() as *const c_char)
            .to_str()
            .unwrap()
            .to_owned()
    }

    unsafe fn object_ptr_label<S>(&self, sync: Self::Fence, label: Option<S>)
    where
        S: AsRef<str>,
    {
        match label {
            Some(l) => {
                let lbl = l.as_ref().as_bytes();
                let length = lbl.len() as i32;
                gl::glObjectPtrLabel(
                    sync.0 as *mut std::ffi::c_void,
                    length as u32,
                    lbl.as_ptr() as *const gl::GLchar,
                );
            }
            None => gl::glObjectPtrLabel(sync.0 as *mut std::ffi::c_void, 0, std::ptr::null()),
        }
    }

    unsafe fn get_object_ptr_label(&self, sync: Self::Fence) -> String {
        let mut len = 0;
        let mut label_buf = Vec::with_capacity(self.constants.max_label_length as usize);
        gl::glGetObjectPtrLabel(
            sync.0 as *mut std::ffi::c_void,
            self.constants.max_label_length as u32,
            &mut len,
            label_buf.as_mut_ptr(),
        );
        label_buf.set_len(len as usize);
        std::ffi::CStr::from_ptr(label_buf.as_ptr() as *const c_char)
            .to_str()
            .unwrap()
            .to_owned()
    }

    unsafe fn get_uniform_block_index(&self, program: Self::Program, name: &str) -> Option<u32> {
        let name = CString::new(name).unwrap();
        let index = gl::glGetUniformBlockIndex(program.0.get(), name.as_ptr() as *const u8);
        if index == INVALID_INDEX {
            None
        } else {
            Some(index)
        }
    }

    unsafe fn uniform_block_binding(&self, program: Self::Program, index: u32, binding: u32) {
        gl::glUniformBlockBinding(program.0.get(), index, binding);
    }

    unsafe fn get_shader_storage_block_index(
        &self,
        program: Self::Program,
        name: &str,
    ) -> Option<u32> {
        let name = CString::new(name).unwrap();
        let index = gl::glGetProgramResourceIndex(
            program.0.get(),
            SHADER_STORAGE_BLOCK,
            name.as_ptr() as *const u8,
        );
        if index == INVALID_INDEX {
            None
        } else {
            Some(index)
        }
    }

    unsafe fn shader_storage_block_binding(
        &self,
        program: Self::Program,
        index: u32,
        binding: u32,
    ) {
        gl::glShaderStorageBlockBinding(program.0.get(), index, binding);
    }

    unsafe fn read_buffer(&self, src: u32) {
        gl::glReadBuffer(src);
    }

    unsafe fn read_pixels(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: u32,
        gltype: u32,
        pixels: PixelPackData,
    ) {
        gl::glReadPixels(
            x,
            y,
            width as u32,
            height as u32,
            format,
            gltype,
            match pixels {
                PixelPackData::BufferOffset(offset) => offset as *mut std::ffi::c_void,
                PixelPackData::Slice(data) => data.as_mut_ptr() as *mut std::ffi::c_void,
            },
        );
    }

    unsafe fn begin_query(&self, target: u32, query: Self::Query) {
        gl::glBeginQuery(target, query.0.get());
    }

    unsafe fn end_query(&self, target: u32) {
        gl::glEndQuery(target);
    }

    unsafe fn get_query_parameter_u32(&self, query: Self::Query, parameter: u32) -> u32 {
        let mut value = 0;
        gl::glGetQueryObjectuiv(query.0.get(), parameter, &mut value);
        value
    }

    unsafe fn create_transform_feedback(&self) -> Result<Self::TransformFeedback, String> {
        let mut name = 0;
        gl::glGenTransformFeedbacks(1, &mut name);
        Ok(NativeTransformFeedback(non_zero_gl_name(name)))
    }

    unsafe fn delete_transform_feedback(&self, transform_feedback: Self::TransformFeedback) {
        gl::glDeleteTransformFeedbacks(1, &transform_feedback.0.get());
    }

    unsafe fn bind_transform_feedback(
        &self,
        target: u32,
        transform_feedback: Option<Self::TransformFeedback>,
    ) {
        gl::glBindTransformFeedback(target, transform_feedback.map(|tf| tf.0.get()).unwrap_or(0));
    }

    unsafe fn begin_transform_feedback(&self, primitive_mode: u32) {
        gl::glBeginTransformFeedback(primitive_mode);
    }

    unsafe fn end_transform_feedback(&self) {
        gl::glEndTransformFeedback();
    }

    unsafe fn pause_transform_feedback(&self) {
        gl::glPauseTransformFeedback();
    }

    unsafe fn resume_transform_feedback(&self) {
        gl::glResumeTransformFeedback();
    }

    unsafe fn transform_feedback_varyings(
        &self,
        program: Self::Program,
        varyings: &[&str],
        buffer_mode: u32,
    ) {
        let strings: Vec<CString> = varyings
            .iter()
            .copied()
            .map(CString::new)
            .collect::<Result<_, _>>()
            .unwrap();
        let varyings: Vec<_> = strings.iter().map(|c_str| c_str.as_ptr()).collect();

        gl::glTransformFeedbackVaryings(
            program.0.get(),
            varyings.len() as u32,
            varyings.as_ptr() as *const *const u8,
            buffer_mode,
        );
    }

    unsafe fn get_transform_feedback_varying(
        &self,
        program: Self::Program,
        index: u32,
    ) -> Option<ActiveTransformFeedback> {
        const buf_size: usize = 256;
        const bytes: [u8; buf_size] = [0; buf_size];

        let mut size: u32 = 0;
        let tftype: u32 = 0;
        let c_name = CString::new(bytes.to_vec()).unwrap();
        let c_name_buf = c_name.into_raw() as *mut u8;

        gl::glGetTransformFeedbackVarying(
            program.0.get(),
            index,
            buf_size as u32,
            std::ptr::null_mut(),
            &mut size,
            tftype as *mut u32,
            c_name_buf,
        );

        let name = CString::from_raw(c_name_buf as *mut c_char)
            .into_string()
            .unwrap();

        Some(ActiveTransformFeedback {
            size: size as i32,
            tftype,
            name,
        })
    }

    unsafe fn memory_barrier(&self, barriers: u32) {
        gl::glMemoryBarrier(barriers);
    }

    unsafe fn memory_barrier_by_region(&self, barriers: u32) {
        gl::glMemoryBarrierByRegion(barriers);
    }

    unsafe fn bind_image_texture(
        &self,
        unit: u32,
        texture: Self::Texture,
        level: i32,
        layered: bool,
        layer: i32,
        access: u32,
        format: u32,
    ) {
        gl::glBindImageTexture(
            unit,
            texture.0.get(),
            level,
            layered as u32,
            layer,
            access,
            format,
        );
    }
    unsafe fn get_active_uniform_block_parameter_i32(
        &self,
        program: Self::Program,
        uniform_block_index: u32,
        parameter: u32,
    ) -> i32 {
        let mut value = 0;
        gl::glGetActiveUniformBlockiv(program.0.get(), uniform_block_index, parameter, &mut value);
        value
    }

    unsafe fn get_active_uniform_block_parameter_i32_slice(
        &self,
        program: Self::Program,
        uniform_block_index: u32,
        parameter: u32,
        out: &mut [i32],
    ) {
        gl::glGetActiveUniformBlockiv(
            program.0.get(),
            uniform_block_index,
            parameter,
            out.as_mut_ptr(),
        );
    }
    unsafe fn get_active_uniform_block_name(
        &self,
        program: Self::Program,
        uniform_block_index: u32,
    ) -> String {
        // Probe for the length of the name of the uniform block, and, failing
        // that, fall back to allocating a buffer that is 256 bytes long. This
        // should be good enough for pretty much all contexts, including faulty
        // or partially faulty ones.
        let len = self.get_active_uniform_block_parameter_i32(
            program,
            uniform_block_index,
            crate::UNIFORM_BLOCK_NAME_LENGTH,
        );
        let len = if gl::glGetError() == crate::NO_ERROR && len > 0 {
            len as usize
        } else {
            256
        };

        let mut buffer = vec![0; len];
        let mut length = 0;
        gl::glGetActiveUniformBlockName(
            program.0.get(),
            uniform_block_index,
            buffer.len() as _,
            &mut length,
            buffer.as_mut_ptr(),
        );

        if length > 0 {
            assert_eq!(
                std::mem::size_of::<u8>(),
                std::mem::size_of::<gl::GLchar>(),
                "This operation is only safe in systems in which the length of \
                a GLchar is the same as that of an u8"
            );
            assert_eq!(
                std::mem::align_of::<u8>(),
                std::mem::align_of::<gl::GLchar>(),
                "This operation is only safe in systems in which the alignment \
                of a GLchar is the same as that of an u8"
            );
            let buffer = std::slice::from_raw_parts(
                buffer.as_ptr() as *const u8,
                (length as usize + 1).min(buffer.len()),
            );

            let name = CStr::from_bytes_with_nul(&buffer[..])
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned();

            name
        } else {
            String::from("")
        }
    }

    unsafe fn max_shader_compiler_threads(&self, count: u32) {
        if self.is_loaded("glMaxShaderCompilerThreadsKHR") {
            gl::glMaxShaderCompilerThreadsKHR(count);
        } else if self.is_loaded("glMaxShaderCompilerThreadsARB") {
            gl::glMaxShaderCompilerThreadsARB(count);
        }
    }
}

unsafe extern "system" fn raw_debug_message_callback<F>(
    source: u32,
    gltype: u32,
    id: u32,
    severity: u32,
    length: u32,
    message: *const gl::GLchar,
    user_param: *const std::ffi::c_void,
) where
    F: FnMut(u32, u32, u32, u32, &str),
{
    std::panic::catch_unwind(move || unsafe {
        let callback: &mut F = &mut *(user_param as *mut _);
        let slice = std::slice::from_raw_parts(message as *const u8, length as usize);
        let msg = std::str::from_utf8(slice).unwrap();
        (callback)(source, gltype, id, severity, msg);
    })
    .ok();
}
