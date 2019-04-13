#[cfg(not(target_arch = "wasm32"))]
pub mod native;

#[cfg(target_arch = "wasm32")]
pub mod web;

pub trait Context {
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
    type VertexArray: Copy
        + Clone
        + std::fmt::Debug
        + Eq
        + std::hash::Hash
        + Ord
        + PartialEq
        + PartialOrd;
    type Texture: Copy
        + Clone
        + std::fmt::Debug
        + Eq
        + std::hash::Hash
        + Ord
        + PartialEq
        + PartialOrd;
    type Sampler: Copy
        + Clone
        + std::fmt::Debug
        + Eq
        + std::hash::Hash
        + Ord
        + PartialEq
        + PartialOrd;
    type Fence: Copy + Clone + std::fmt::Debug + Eq + std::hash::Hash + Ord + PartialEq + PartialOrd;
    type Framebuffer: Copy
        + Clone
        + std::fmt::Debug
        + Eq
        + std::hash::Hash
        + Ord
        + PartialEq
        + PartialOrd;
    type Renderbuffer: Copy
        + Clone
        + std::fmt::Debug
        + Eq
        + std::hash::Hash
        + Ord
        + PartialEq
        + PartialOrd;
    type UniformLocation: Copy
        + Clone
        + std::fmt::Debug
        + Eq
        + std::hash::Hash
        + Ord
        + PartialEq
        + PartialOrd;

    unsafe fn create_framebuffer(&self) -> Result<Self::Framebuffer, String>;

    unsafe fn create_renderbuffer(&self) -> Result<Self::Renderbuffer, String>;

    unsafe fn create_sampler(&self) -> Result<Self::Sampler, String>;

    unsafe fn create_shader(&self, shader_type: u32) -> Result<Self::Shader, String>;

    unsafe fn create_texture(&self) -> Result<Self::Texture, String>;

    unsafe fn delete_shader(&self, shader: Self::Shader);

    unsafe fn shader_source(&self, shader: Self::Shader, source: &str);

    unsafe fn compile_shader(&self, shader: Self::Shader);

    unsafe fn get_shader_compile_status(&self, shader: Self::Shader) -> bool;

    unsafe fn get_shader_info_log(&self, shader: Self::Shader) -> String;

    unsafe fn get_tex_image(
        &self,
        target: u32,
        level: i32,
        format: u32,
        ty: u32,
        pixels: Option<&[u8]>,
    );

    unsafe fn create_program(&self) -> Result<Self::Program, String>;

    unsafe fn delete_program(&self, program: Self::Program);

    unsafe fn attach_shader(&self, program: Self::Program, shader: Self::Shader);

    unsafe fn detach_shader(&self, program: Self::Program, shader: Self::Shader);

    unsafe fn link_program(&self, program: Self::Program);

    unsafe fn get_program_link_status(&self, program: Self::Program) -> bool;

    unsafe fn get_program_info_log(&self, program: Self::Program) -> String;

    unsafe fn use_program(&self, program: Option<Self::Program>);

    unsafe fn create_buffer(&self) -> Result<Self::Buffer, String>;

    unsafe fn bind_buffer(&self, target: u32, buffer: Option<Self::Buffer>);

    unsafe fn bind_buffer_range(
        &self,
        target: u32,
        index: u32,
        buffer: Option<Self::Buffer>,
        offset: i32,
        size: i32,
    );

    unsafe fn bind_framebuffer(&self, target: u32, framebuffer: Option<Self::Framebuffer>);

    unsafe fn bind_renderbuffer(&self, target: u32, renderbuffer: Option<Self::Renderbuffer>);

    unsafe fn create_vertex_array(&self) -> Result<Self::VertexArray, String>;

    unsafe fn delete_vertex_array(&self, vertex_array: Self::VertexArray);

    unsafe fn bind_vertex_array(&self, vertex_array: Option<Self::VertexArray>);

    unsafe fn clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32);

    unsafe fn supports_f64_precision() -> bool;

    unsafe fn clear_depth_f64(&self, depth: f64);

    unsafe fn clear_depth_f32(&self, depth: f32);

    unsafe fn clear_stencil(&self, stencil: i32);

    unsafe fn clear(&self, mask: u32);

    unsafe fn patch_parameter_i32(&self, parameter: u32, value: i32);

    unsafe fn pixel_store_i32(&self, parameter: u32, value: i32);

    unsafe fn pixel_store_bool(&self, parameter: u32, value: bool);

    unsafe fn bind_frag_data_location(&self, program: Self::Program, color_number: u32, name: &str);

    unsafe fn buffer_data_size(&self, target: u32, size: i32, usage: u32);

    unsafe fn buffer_data_u8_slice(&self, target: u32, data: &[u8], usage: u32);

    unsafe fn buffer_storage(&self, target: u32, size: i32, data: Option<&mut [u8]>, flags: u32);

    unsafe fn check_framebuffer_status(&self, target: u32) -> u32;

    unsafe fn clear_buffer_i32_slice(&self, target: u32, draw_buffer: u32, values: &mut [i32]);

    unsafe fn clear_buffer_u32_slice(&self, target: u32, draw_buffer: u32, values: &mut [u32]);

    unsafe fn clear_buffer_f32_slice(&self, target: u32, draw_buffer: u32, values: &mut [f32]);

    unsafe fn clear_buffer_depth_stencil(
        &self,
        target: u32,
        draw_buffer: u32,
        depth: f32,
        stencil: i32,
    );

    unsafe fn client_wait_sync(&self, fence: Self::Fence, flags: u32, timeout: i32) -> u32;

    unsafe fn copy_buffer_sub_data(
        &self,
        src_target: u32,
        dst_target: u32,
        src_offset: i32,
        dst_offset: i32,
        size: i32,
    );

    unsafe fn delete_buffer(&self, buffer: Self::Buffer);

    unsafe fn delete_framebuffer(&self, framebuffer: Self::Framebuffer);

    unsafe fn delete_renderbuffer(&self, renderbuffer: Self::Renderbuffer);

    unsafe fn delete_sampler(&self, texture: Self::Sampler);

    unsafe fn delete_sync(&self, fence: Self::Fence);

    unsafe fn delete_texture(&self, texture: Self::Texture);

    unsafe fn disable(&self, parameter: u32);

    unsafe fn disable_draw_buffer(&self, parameter: u32, draw_buffer: u32);

    unsafe fn disable_vertex_attrib_array(&self, index: u32);

    unsafe fn dispatch_compute(&self, groups_x: u32, groups_y: u32, groups_z: u32);

    unsafe fn dispatch_compute_indirect(&self, offset: i32);

    unsafe fn draw_arrays(&self, mode: u32, first: i32, count: i32);

    unsafe fn draw_arrays_instanced(&self, mode: u32, first: i32, count: i32, instance_count: i32);

    unsafe fn draw_arrays_instanced_base_instance(
        &self,
        mode: u32,
        first: i32,
        count: i32,
        instance_count: i32,
        base_instance: u32,
    );

    unsafe fn draw_buffer(&self, buffer: u32);

    unsafe fn draw_buffers(&self, buffers: &[u32]);

    unsafe fn draw_elements(&self, mode: u32, count: i32, element_type: u32, offset: i32);

    unsafe fn draw_elements_base_vertex(
        &self,
        mode: u32,
        count: i32,
        element_type: u32,
        offset: i32,
        base_vertex: i32,
    );

    unsafe fn draw_elements_instanced(
        &self,
        mode: u32,
        count: i32,
        element_type: u32,
        offset: i32,
        instance_count: i32,
    );

    unsafe fn draw_elements_instanced_base_vertex(
        &self,
        mode: u32,
        count: i32,
        element_type: u32,
        offset: i32,
        instance_count: i32,
        base_vertex: i32,
    );

    unsafe fn draw_elements_instanced_base_vertex_base_instance(
        &self,
        mode: u32,
        count: i32,
        element_type: u32,
        offset: i32,
        instance_count: i32,
        base_vertex: i32,
        base_instance: u32,
    );

    unsafe fn enable(&self, parameter: u32);

    unsafe fn enable_draw_buffer(&self, parameter: u32, draw_buffer: u32);

    unsafe fn enable_vertex_attrib_array(&self, index: u32);

    unsafe fn flush(&self);

    unsafe fn framebuffer_renderbuffer(
        &self,
        target: u32,
        attachment: u32,
        renderbuffer_target: u32,
        renderbuffer: Option<Self::Renderbuffer>,
    );

    unsafe fn framebuffer_texture(
        &self,
        target: u32,
        attachment: u32,
        texture: Option<Self::Texture>,
        level: i32,
    );

    unsafe fn framebuffer_texture_2d(
        &self,
        target: u32,
        attachment: u32,
        texture_target: u32,
        texture: Option<Self::Texture>,
        level: i32,
    );

    unsafe fn framebuffer_texture_3d(
        &self,
        target: u32,
        attachment: u32,
        texture_target: u32,
        texture: Option<Self::Texture>,
        level: i32,
        layer: i32,
    );

    unsafe fn framebuffer_texture_layer(
        &self,
        target: u32,
        attachment: u32,
        texture: Option<Self::Texture>,
        level: i32,
        layer: i32,
    );

    unsafe fn front_face(&self, value: u32);

    unsafe fn get_error(&self) -> u32;

    unsafe fn get_parameter_i32(&self, parameter: u32) -> i32;

    unsafe fn get_parameter_indexed_i32(&self, parameter: u32, index: u32) -> i32;

    unsafe fn get_parameter_indexed_string(&self, parameter: u32, index: u32) -> String;

    unsafe fn get_parameter_string(&self, parameter: u32) -> String;

    unsafe fn get_uniform_location(
        &self,
        program: Self::Program,
        name: &str,
    ) -> Option<Self::UniformLocation>;

    unsafe fn get_attrib_location(
        &self,
        program: Self::Program,
        name: &str,
    ) -> i32;

    unsafe fn is_sync(&self, fence: Self::Fence) -> bool;

    unsafe fn renderbuffer_storage(
        &self,
        target: u32,
        internal_format: u32,
        width: i32,
        height: i32,
    );

    unsafe fn sampler_parameter_f32(&self, sampler: Self::Sampler, name: u32, value: f32);

    unsafe fn sampler_parameter_f32_slice(
        &self,
        sampler: Self::Sampler,
        name: u32,
        value: &mut [f32],
    );

    unsafe fn sampler_parameter_i32(&self, sampler: Self::Sampler, name: u32, value: i32);

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
    );

    unsafe fn tex_storage_2d(
        &self,
        target: u32,
        levels: i32,
        internal_format: u32,
        width: i32,
        height: i32,
    );

    unsafe fn uniform_1_i32(&self, location: Option<Self::UniformLocation>, x: i32);

    unsafe fn uniform_2_i32(&self, location: Option<Self::UniformLocation>, x: i32, y: i32);

    unsafe fn uniform_3_i32(&self, location: Option<Self::UniformLocation>, x: i32, y: i32, z: i32);

    unsafe fn uniform_4_i32(&self, location: Option<Self::UniformLocation>, x: i32, y: i32, z: i32, w: i32);

    unsafe fn uniform_1_i32_slice(&self, location: Option<Self::UniformLocation>, v: &mut [i32; 1]);

    unsafe fn uniform_2_i32_slice(&self, location: Option<Self::UniformLocation>, v: &mut [i32; 2]);

    unsafe fn uniform_3_i32_slice(&self, location: Option<Self::UniformLocation>, v: &mut [i32; 3]);

    unsafe fn uniform_4_i32_slice(&self, location: Option<Self::UniformLocation>, v: &mut [i32; 4]);

    unsafe fn uniform_1_f32(&self, location: Option<Self::UniformLocation>, x: f32);

    unsafe fn uniform_2_f32(&self, location: Option<Self::UniformLocation>, x: f32, y: f32);

    unsafe fn uniform_3_f32(&self, location: Option<Self::UniformLocation>, x: f32, y: f32, z: f32);

    unsafe fn uniform_4_f32(&self, location: Option<Self::UniformLocation>, x: f32, y: f32, z: f32, w: f32);

    unsafe fn uniform_1_f32_slice(&self, location: Option<Self::UniformLocation>, v: &[f32; 1]);

    unsafe fn uniform_2_f32_slice(&self, location: Option<Self::UniformLocation>, v: &[f32; 2]);

    unsafe fn uniform_3_f32_slice(&self, location: Option<Self::UniformLocation>, v: &[f32; 3]);

    unsafe fn uniform_4_f32_slice(&self, location: Option<Self::UniformLocation>, v: &[f32; 4]);

    unsafe fn uniform_matrix_2_f32_slice(&self, location: Option<Self::UniformLocation>, transpose: bool, v: &[f32; 4]);

    unsafe fn uniform_matrix_3_f32_slice(&self, location: Option<Self::UniformLocation>, transpose: bool, v: &[f32; 9]);

    unsafe fn uniform_matrix_4_f32_slice(&self, location: Option<Self::UniformLocation>, transpose: bool, v: &[f32; 16]);

    unsafe fn unmap_buffer(&self, target: u32);

    unsafe fn cull_face(&self, value: u32);

    unsafe fn color_mask(&self, red: bool, green: bool, blue: bool, alpha: bool);

    unsafe fn color_mask_draw_buffer(
        &self,
        buffer: u32,
        red: bool,
        green: bool,
        blue: bool,
        alpha: bool,
    );

    unsafe fn depth_mask(&self, value: bool);

    unsafe fn blend_color(&self, red: f32, green: f32, blue: f32, alpha: f32);

    unsafe fn line_width(&self, width: f32);

    unsafe fn map_buffer_range(
        &self,
        target: u32,
        offset: i32,
        length: i32,
        access: u32,
    ) -> *mut u8;

    unsafe fn polygon_offset(&self, factor: f32, units: f32);

    unsafe fn polygon_mode(&self, face: u32, mode: u32);

    unsafe fn finish(&self);

    unsafe fn bind_texture(&self, target: u32, texture: Option<Self::Texture>);

    unsafe fn bind_sampler(&self, unit: u32, sampler: Option<Self::Sampler>);

    unsafe fn active_texture(&self, unit: u32);

    unsafe fn fence_sync(&self, condition: u32, flags: u32) -> Result<Self::Fence, String>;

    unsafe fn tex_parameter_f32(&self, target: u32, parameter: u32, value: f32);

    unsafe fn tex_parameter_i32(&self, target: u32, parameter: u32, value: i32);

    unsafe fn tex_parameter_f32_slice(&self, target: u32, parameter: u32, values: &[f32]);

    unsafe fn tex_parameter_i32_slice(&self, target: u32, parameter: u32, values: &[i32]);

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
    );

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
    );

    unsafe fn depth_func(&self, func: u32);

    unsafe fn depth_range_f32(&self, near: f32, far: f32);

    unsafe fn depth_range_f64(&self, near: f64, far: f64);

    unsafe fn depth_range_f64_slice(&self, first: u32, count: i32, values: &[[f64; 2]]);

    unsafe fn scissor(&self, x: i32, y: i32, width: i32, height: i32);

    unsafe fn scissor_slice(&self, first: u32, count: i32, scissors: &[[i32; 4]]);

    unsafe fn vertex_attrib_divisor(&self, index: u32, divisor: u32);

    unsafe fn vertex_attrib_pointer_f32(
        &self,
        index: u32,
        size: i32,
        data_type: u32,
        normalized: bool,
        stride: i32,
        offset: i32,
    );

    unsafe fn vertex_attrib_pointer_i32(
        &self,
        index: u32,
        size: i32,
        data_type: u32,
        stride: i32,
        offset: i32,
    );

    unsafe fn vertex_attrib_pointer_f64(
        &self,
        index: u32,
        size: i32,
        data_type: u32,
        stride: i32,
        offset: i32,
    );

    unsafe fn viewport(&self, x: i32, y: i32, width: i32, height: i32);

    unsafe fn viewport_f32_slice(&self, first: u32, count: i32, values: &[[f32; 4]]);

    unsafe fn blend_equation(&self, mode: u32);

    unsafe fn blend_equation_draw_buffer(&self, draw_buffer: u32, mode: u32);

    unsafe fn blend_equation_separate(&self, mode_rgb: u32, mode_alpha: u32);

    unsafe fn blend_equation_separate_draw_buffer(
        &self,
        buffer: u32,
        mode_rgb: u32,
        mode_alpha: u32,
    );

    unsafe fn blend_func(&self, src: u32, dst: u32);

    unsafe fn blend_func_draw_buffer(&self, draw_buffer: u32, src: u32, dst: u32);

    unsafe fn blend_func_separate(
        &self,
        src_rgb: u32,
        dst_rgb: u32,
        src_alpha: u32,
        dst_alpha: u32,
    );

    unsafe fn blend_func_separate_draw_buffer(
        &self,
        draw_buffer: u32,
        src_rgb: u32,
        dst_rgb: u32,
        src_alpha: u32,
        dst_alpha: u32,
    );

    unsafe fn stencil_func(&self, func: u32, reference: i32, mask: u32);

    unsafe fn stencil_func_separate(&self, face: u32, func: u32, reference: i32, mask: u32);

    unsafe fn stencil_mask(&self, mask: u32);

    unsafe fn stencil_mask_separate(&self, face: u32, mask: u32);

    unsafe fn stencil_op(&self, stencil_fail: u32, depth_fail: u32, pass: u32);

    unsafe fn stencil_op_separate(&self, face: u32, stencil_fail: u32, depth_fail: u32, pass: u32);
}

pub trait RenderLoop {
    type Window;

    fn run<F: FnMut(&mut bool) + 'static>(&self, callback: F);
}

#[allow(non_upper_case_globals)]
pub const ACTIVE_ATOMIC_COUNTER_BUFFERS: u32 = 0x92D9;
#[allow(non_upper_case_globals)]
pub const ACTIVE_ATTRIBUTES: u32 = 0x8B89;
#[allow(non_upper_case_globals)]
pub const ACTIVE_ATTRIBUTE_MAX_LENGTH: u32 = 0x8B8A;
#[allow(non_upper_case_globals)]
pub const ACTIVE_PROGRAM: u32 = 0x8259;
#[allow(non_upper_case_globals)]
pub const ACTIVE_RESOURCES: u32 = 0x92F5;
#[allow(non_upper_case_globals)]
pub const ACTIVE_SUBROUTINES: u32 = 0x8DE5;
#[allow(non_upper_case_globals)]
pub const ACTIVE_SUBROUTINE_MAX_LENGTH: u32 = 0x8E48;
#[allow(non_upper_case_globals)]
pub const ACTIVE_SUBROUTINE_UNIFORMS: u32 = 0x8DE6;
#[allow(non_upper_case_globals)]
pub const ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS: u32 = 0x8E47;
#[allow(non_upper_case_globals)]
pub const ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH: u32 = 0x8E49;
#[allow(non_upper_case_globals)]
pub const ACTIVE_TEXTURE: u32 = 0x84E0;
#[allow(non_upper_case_globals)]
pub const ACTIVE_UNIFORMS: u32 = 0x8B86;
#[allow(non_upper_case_globals)]
pub const ACTIVE_UNIFORM_BLOCKS: u32 = 0x8A36;
#[allow(non_upper_case_globals)]
pub const ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: u32 = 0x8A35;
#[allow(non_upper_case_globals)]
pub const ACTIVE_UNIFORM_MAX_LENGTH: u32 = 0x8B87;
#[allow(non_upper_case_globals)]
pub const ACTIVE_VARIABLES: u32 = 0x9305;
#[allow(non_upper_case_globals)]
pub const ALIASED_LINE_WIDTH_RANGE: u32 = 0x846E;
#[allow(non_upper_case_globals)]
pub const ALL_BARRIER_BITS: u32 = 0xFFFFFFFF;
#[allow(non_upper_case_globals)]
pub const ALL_SHADER_BITS: u32 = 0xFFFFFFFF;
#[allow(non_upper_case_globals)]
pub const ALPHA: u32 = 0x1906;
#[allow(non_upper_case_globals)]
pub const ALREADY_SIGNALED: u32 = 0x911A;
#[allow(non_upper_case_globals)]
pub const ALWAYS: u32 = 0x0207;
#[allow(non_upper_case_globals)]
pub const AND: u32 = 0x1501;
#[allow(non_upper_case_globals)]
pub const AND_INVERTED: u32 = 0x1504;
#[allow(non_upper_case_globals)]
pub const AND_REVERSE: u32 = 0x1502;
#[allow(non_upper_case_globals)]
pub const ANY_SAMPLES_PASSED: u32 = 0x8C2F;
#[allow(non_upper_case_globals)]
pub const ANY_SAMPLES_PASSED_CONSERVATIVE: u32 = 0x8D6A;
#[allow(non_upper_case_globals)]
pub const ARRAY_BUFFER: u32 = 0x8892;
#[allow(non_upper_case_globals)]
pub const ARRAY_BUFFER_BINDING: u32 = 0x8894;
#[allow(non_upper_case_globals)]
pub const ARRAY_SIZE: u32 = 0x92FB;
#[allow(non_upper_case_globals)]
pub const ARRAY_STRIDE: u32 = 0x92FE;
#[allow(non_upper_case_globals)]
pub const ATOMIC_COUNTER_BARRIER_BIT: u32 = 0x00001000;
#[allow(non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER: u32 = 0x92C0;
#[allow(non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS: u32 = 0x92C5;
#[allow(non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES: u32 = 0x92C6;
#[allow(non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_BINDING: u32 = 0x92C1;
#[allow(non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_DATA_SIZE: u32 = 0x92C4;
#[allow(non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_INDEX: u32 = 0x9301;
#[allow(non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER: u32 = 0x90ED;
#[allow(non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER: u32 = 0x92CB;
#[allow(non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER: u32 = 0x92CA;
#[allow(non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER: u32 = 0x92C8;
#[allow(non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER: u32 = 0x92C9;
#[allow(non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER: u32 = 0x92C7;
#[allow(non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_SIZE: u32 = 0x92C3;
#[allow(non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_START: u32 = 0x92C2;
#[allow(non_upper_case_globals)]
pub const ATTACHED_SHADERS: u32 = 0x8B85;
#[allow(non_upper_case_globals)]
pub const AUTO_GENERATE_MIPMAP: u32 = 0x8295;
#[allow(non_upper_case_globals)]
pub const BACK: u32 = 0x0405;
#[allow(non_upper_case_globals)]
pub const BACK_LEFT: u32 = 0x0402;
#[allow(non_upper_case_globals)]
pub const BACK_RIGHT: u32 = 0x0403;
#[allow(non_upper_case_globals)]
pub const BGR: u32 = 0x80E0;
#[allow(non_upper_case_globals)]
pub const BGRA: u32 = 0x80E1;
#[allow(non_upper_case_globals)]
pub const BGRA_INTEGER: u32 = 0x8D9B;
#[allow(non_upper_case_globals)]
pub const BGR_INTEGER: u32 = 0x8D9A;
#[allow(non_upper_case_globals)]
pub const BLEND: u32 = 0x0BE2;
#[allow(non_upper_case_globals)]
pub const BLEND_COLOR: u32 = 0x8005;
#[allow(non_upper_case_globals)]
pub const BLEND_DST: u32 = 0x0BE0;
#[allow(non_upper_case_globals)]
pub const BLEND_DST_ALPHA: u32 = 0x80CA;
#[allow(non_upper_case_globals)]
pub const BLEND_DST_RGB: u32 = 0x80C8;
#[allow(non_upper_case_globals)]
pub const BLEND_EQUATION: u32 = 0x8009;
#[allow(non_upper_case_globals)]
pub const BLEND_EQUATION_ALPHA: u32 = 0x883D;
#[allow(non_upper_case_globals)]
pub const BLEND_EQUATION_RGB: u32 = 0x8009;
#[allow(non_upper_case_globals)]
pub const BLEND_SRC: u32 = 0x0BE1;
#[allow(non_upper_case_globals)]
pub const BLEND_SRC_ALPHA: u32 = 0x80CB;
#[allow(non_upper_case_globals)]
pub const BLEND_SRC_RGB: u32 = 0x80C9;
#[allow(non_upper_case_globals)]
pub const BLOCK_INDEX: u32 = 0x92FD;
#[allow(non_upper_case_globals)]
pub const BLUE: u32 = 0x1905;
#[allow(non_upper_case_globals)]
pub const BLUE_INTEGER: u32 = 0x8D96;
#[allow(non_upper_case_globals)]
pub const BOOL: u32 = 0x8B56;
#[allow(non_upper_case_globals)]
pub const BOOL_VEC2: u32 = 0x8B57;
#[allow(non_upper_case_globals)]
pub const BOOL_VEC3: u32 = 0x8B58;
#[allow(non_upper_case_globals)]
pub const BOOL_VEC4: u32 = 0x8B59;
#[allow(non_upper_case_globals)]
pub const BUFFER: u32 = 0x82E0;
#[allow(non_upper_case_globals)]
pub const BUFFER_ACCESS: u32 = 0x88BB;
#[allow(non_upper_case_globals)]
pub const BUFFER_ACCESS_FLAGS: u32 = 0x911F;
#[allow(non_upper_case_globals)]
pub const BUFFER_BINDING: u32 = 0x9302;
#[allow(non_upper_case_globals)]
pub const BUFFER_DATA_SIZE: u32 = 0x9303;
#[allow(non_upper_case_globals)]
pub const BUFFER_IMMUTABLE_STORAGE: u32 = 0x821F;
#[allow(non_upper_case_globals)]
pub const BUFFER_MAPPED: u32 = 0x88BC;
#[allow(non_upper_case_globals)]
pub const BUFFER_MAP_LENGTH: u32 = 0x9120;
#[allow(non_upper_case_globals)]
pub const BUFFER_MAP_OFFSET: u32 = 0x9121;
#[allow(non_upper_case_globals)]
pub const BUFFER_MAP_POINTER: u32 = 0x88BD;
#[allow(non_upper_case_globals)]
pub const BUFFER_SIZE: u32 = 0x8764;
#[allow(non_upper_case_globals)]
pub const BUFFER_STORAGE_FLAGS: u32 = 0x8220;
#[allow(non_upper_case_globals)]
pub const BUFFER_UPDATE_BARRIER_BIT: u32 = 0x00000200;
#[allow(non_upper_case_globals)]
pub const BUFFER_USAGE: u32 = 0x8765;
#[allow(non_upper_case_globals)]
pub const BUFFER_VARIABLE: u32 = 0x92E5;
#[allow(non_upper_case_globals)]
pub const BYTE: u32 = 0x1400;
#[allow(non_upper_case_globals)]
pub const CAVEAT_SUPPORT: u32 = 0x82B8;
#[allow(non_upper_case_globals)]
pub const CCW: u32 = 0x0901;
#[allow(non_upper_case_globals)]
pub const CLAMP_READ_COLOR: u32 = 0x891C;
#[allow(non_upper_case_globals)]
pub const CLAMP_TO_BORDER: u32 = 0x812D;
#[allow(non_upper_case_globals)]
pub const CLAMP_TO_EDGE: u32 = 0x812F;
#[allow(non_upper_case_globals)]
pub const CLEAR: u32 = 0x1500;
#[allow(non_upper_case_globals)]
pub const CLEAR_BUFFER: u32 = 0x82B4;
#[allow(non_upper_case_globals)]
pub const CLEAR_TEXTURE: u32 = 0x9365;
#[allow(non_upper_case_globals)]
pub const CLIENT_MAPPED_BUFFER_BARRIER_BIT: u32 = 0x00004000;
#[allow(non_upper_case_globals)]
pub const CLIENT_STORAGE_BIT: u32 = 0x0200;
#[allow(non_upper_case_globals)]
pub const CLIPPING_INPUT_PRIMITIVES: u32 = 0x82F6;
#[allow(non_upper_case_globals)]
pub const CLIPPING_OUTPUT_PRIMITIVES: u32 = 0x82F7;
#[allow(non_upper_case_globals)]
pub const CLIP_DEPTH_MODE: u32 = 0x935D;
#[allow(non_upper_case_globals)]
pub const CLIP_DISTANCE0: u32 = 0x3000;
#[allow(non_upper_case_globals)]
pub const CLIP_DISTANCE1: u32 = 0x3001;
#[allow(non_upper_case_globals)]
pub const CLIP_DISTANCE2: u32 = 0x3002;
#[allow(non_upper_case_globals)]
pub const CLIP_DISTANCE3: u32 = 0x3003;
#[allow(non_upper_case_globals)]
pub const CLIP_DISTANCE4: u32 = 0x3004;
#[allow(non_upper_case_globals)]
pub const CLIP_DISTANCE5: u32 = 0x3005;
#[allow(non_upper_case_globals)]
pub const CLIP_DISTANCE6: u32 = 0x3006;
#[allow(non_upper_case_globals)]
pub const CLIP_DISTANCE7: u32 = 0x3007;
#[allow(non_upper_case_globals)]
pub const CLIP_ORIGIN: u32 = 0x935C;
#[allow(non_upper_case_globals)]
pub const COLOR: u32 = 0x1800;
#[allow(non_upper_case_globals)]
pub const COLOR_ATTACHMENT0: u32 = 0x8CE0;
#[allow(non_upper_case_globals)]
pub const COLOR_ATTACHMENT1: u32 = 0x8CE1;
#[allow(non_upper_case_globals)]
pub const COLOR_ATTACHMENT10: u32 = 0x8CEA;
#[allow(non_upper_case_globals)]
pub const COLOR_ATTACHMENT11: u32 = 0x8CEB;
#[allow(non_upper_case_globals)]
pub const COLOR_ATTACHMENT12: u32 = 0x8CEC;
#[allow(non_upper_case_globals)]
pub const COLOR_ATTACHMENT13: u32 = 0x8CED;
#[allow(non_upper_case_globals)]
pub const COLOR_ATTACHMENT14: u32 = 0x8CEE;
#[allow(non_upper_case_globals)]
pub const COLOR_ATTACHMENT15: u32 = 0x8CEF;
#[allow(non_upper_case_globals)]
pub const COLOR_ATTACHMENT16: u32 = 0x8CF0;
#[allow(non_upper_case_globals)]
pub const COLOR_ATTACHMENT17: u32 = 0x8CF1;
#[allow(non_upper_case_globals)]
pub const COLOR_ATTACHMENT18: u32 = 0x8CF2;
#[allow(non_upper_case_globals)]
pub const COLOR_ATTACHMENT19: u32 = 0x8CF3;
#[allow(non_upper_case_globals)]
pub const COLOR_ATTACHMENT2: u32 = 0x8CE2;
#[allow(non_upper_case_globals)]
pub const COLOR_ATTACHMENT20: u32 = 0x8CF4;
#[allow(non_upper_case_globals)]
pub const COLOR_ATTACHMENT21: u32 = 0x8CF5;
#[allow(non_upper_case_globals)]
pub const COLOR_ATTACHMENT22: u32 = 0x8CF6;
#[allow(non_upper_case_globals)]
pub const COLOR_ATTACHMENT23: u32 = 0x8CF7;
#[allow(non_upper_case_globals)]
pub const COLOR_ATTACHMENT24: u32 = 0x8CF8;
#[allow(non_upper_case_globals)]
pub const COLOR_ATTACHMENT25: u32 = 0x8CF9;
#[allow(non_upper_case_globals)]
pub const COLOR_ATTACHMENT26: u32 = 0x8CFA;
#[allow(non_upper_case_globals)]
pub const COLOR_ATTACHMENT27: u32 = 0x8CFB;
#[allow(non_upper_case_globals)]
pub const COLOR_ATTACHMENT28: u32 = 0x8CFC;
#[allow(non_upper_case_globals)]
pub const COLOR_ATTACHMENT29: u32 = 0x8CFD;
#[allow(non_upper_case_globals)]
pub const COLOR_ATTACHMENT3: u32 = 0x8CE3;
#[allow(non_upper_case_globals)]
pub const COLOR_ATTACHMENT30: u32 = 0x8CFE;
#[allow(non_upper_case_globals)]
pub const COLOR_ATTACHMENT31: u32 = 0x8CFF;
#[allow(non_upper_case_globals)]
pub const COLOR_ATTACHMENT4: u32 = 0x8CE4;
#[allow(non_upper_case_globals)]
pub const COLOR_ATTACHMENT5: u32 = 0x8CE5;
#[allow(non_upper_case_globals)]
pub const COLOR_ATTACHMENT6: u32 = 0x8CE6;
#[allow(non_upper_case_globals)]
pub const COLOR_ATTACHMENT7: u32 = 0x8CE7;
#[allow(non_upper_case_globals)]
pub const COLOR_ATTACHMENT8: u32 = 0x8CE8;
#[allow(non_upper_case_globals)]
pub const COLOR_ATTACHMENT9: u32 = 0x8CE9;
#[allow(non_upper_case_globals)]
pub const COLOR_BUFFER_BIT: u32 = 0x00004000;
#[allow(non_upper_case_globals)]
pub const COLOR_CLEAR_VALUE: u32 = 0x0C22;
#[allow(non_upper_case_globals)]
pub const COLOR_COMPONENTS: u32 = 0x8283;
#[allow(non_upper_case_globals)]
pub const COLOR_ENCODING: u32 = 0x8296;
#[allow(non_upper_case_globals)]
pub const COLOR_LOGIC_OP: u32 = 0x0BF2;
#[allow(non_upper_case_globals)]
pub const COLOR_RENDERABLE: u32 = 0x8286;
#[allow(non_upper_case_globals)]
pub const COLOR_WRITEMASK: u32 = 0x0C23;
#[allow(non_upper_case_globals)]
pub const COMMAND_BARRIER_BIT: u32 = 0x00000040;
#[allow(non_upper_case_globals)]
pub const COMPARE_REF_TO_TEXTURE: u32 = 0x884E;
#[allow(non_upper_case_globals)]
pub const COMPATIBLE_SUBROUTINES: u32 = 0x8E4B;
#[allow(non_upper_case_globals)]
pub const COMPILE_STATUS: u32 = 0x8B81;
#[allow(non_upper_case_globals)]
pub const COMPRESSED_R11_EAC: u32 = 0x9270;
#[allow(non_upper_case_globals)]
pub const COMPRESSED_RED: u32 = 0x8225;
#[allow(non_upper_case_globals)]
pub const COMPRESSED_RED_RGTC1: u32 = 0x8DBB;
#[allow(non_upper_case_globals)]
pub const COMPRESSED_RG: u32 = 0x8226;
#[allow(non_upper_case_globals)]
pub const COMPRESSED_RG11_EAC: u32 = 0x9272;
#[allow(non_upper_case_globals)]
pub const COMPRESSED_RGB: u32 = 0x84ED;
#[allow(non_upper_case_globals)]
pub const COMPRESSED_RGB8_ETC2: u32 = 0x9274;
#[allow(non_upper_case_globals)]
pub const COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: u32 = 0x9276;
#[allow(non_upper_case_globals)]
pub const COMPRESSED_RGBA: u32 = 0x84EE;
#[allow(non_upper_case_globals)]
pub const COMPRESSED_RGBA8_ETC2_EAC: u32 = 0x9278;
#[allow(non_upper_case_globals)]
pub const COMPRESSED_RGBA_BPTC_UNORM: u32 = 0x8E8C;
#[allow(non_upper_case_globals)]
pub const COMPRESSED_RGB_BPTC_SIGNED_FLOAT: u32 = 0x8E8E;
#[allow(non_upper_case_globals)]
pub const COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT: u32 = 0x8E8F;
#[allow(non_upper_case_globals)]
pub const COMPRESSED_RG_RGTC2: u32 = 0x8DBD;
#[allow(non_upper_case_globals)]
pub const COMPRESSED_SIGNED_R11_EAC: u32 = 0x9271;
#[allow(non_upper_case_globals)]
pub const COMPRESSED_SIGNED_RED_RGTC1: u32 = 0x8DBC;
#[allow(non_upper_case_globals)]
pub const COMPRESSED_SIGNED_RG11_EAC: u32 = 0x9273;
#[allow(non_upper_case_globals)]
pub const COMPRESSED_SIGNED_RG_RGTC2: u32 = 0x8DBE;
#[allow(non_upper_case_globals)]
pub const COMPRESSED_SRGB: u32 = 0x8C48;
#[allow(non_upper_case_globals)]
pub const COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: u32 = 0x9279;
#[allow(non_upper_case_globals)]
pub const COMPRESSED_SRGB8_ETC2: u32 = 0x9275;
#[allow(non_upper_case_globals)]
pub const COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: u32 = 0x9277;
#[allow(non_upper_case_globals)]
pub const COMPRESSED_SRGB_ALPHA: u32 = 0x8C49;
#[allow(non_upper_case_globals)]
pub const COMPRESSED_SRGB_ALPHA_BPTC_UNORM: u32 = 0x8E8D;
#[allow(non_upper_case_globals)]
pub const COMPRESSED_TEXTURE_FORMATS: u32 = 0x86A3;
#[allow(non_upper_case_globals)]
pub const COMPUTE_SHADER: u32 = 0x91B9;
#[allow(non_upper_case_globals)]
pub const COMPUTE_SHADER_BIT: u32 = 0x00000020;
#[allow(non_upper_case_globals)]
pub const COMPUTE_SHADER_INVOCATIONS: u32 = 0x82F5;
#[allow(non_upper_case_globals)]
pub const COMPUTE_SUBROUTINE: u32 = 0x92ED;
#[allow(non_upper_case_globals)]
pub const COMPUTE_SUBROUTINE_UNIFORM: u32 = 0x92F3;
#[allow(non_upper_case_globals)]
pub const COMPUTE_TEXTURE: u32 = 0x82A0;
#[allow(non_upper_case_globals)]
pub const COMPUTE_WORK_GROUP_SIZE: u32 = 0x8267;
#[allow(non_upper_case_globals)]
pub const CONDITION_SATISFIED: u32 = 0x911C;
#[allow(non_upper_case_globals)]
pub const CONSTANT_ALPHA: u32 = 0x8003;
#[allow(non_upper_case_globals)]
pub const CONSTANT_COLOR: u32 = 0x8001;
#[allow(non_upper_case_globals)]
pub const CONTEXT_COMPATIBILITY_PROFILE_BIT: u32 = 0x00000002;
#[allow(non_upper_case_globals)]
pub const CONTEXT_CORE_PROFILE_BIT: u32 = 0x00000001;
#[allow(non_upper_case_globals)]
pub const CONTEXT_FLAGS: u32 = 0x821E;
#[allow(non_upper_case_globals)]
pub const CONTEXT_FLAG_DEBUG_BIT: u32 = 0x00000002;
#[allow(non_upper_case_globals)]
pub const CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: u32 = 0x00000001;
#[allow(non_upper_case_globals)]
pub const CONTEXT_FLAG_NO_ERROR_BIT: u32 = 0x00000008;
#[allow(non_upper_case_globals)]
pub const CONTEXT_FLAG_ROBUST_ACCESS_BIT: u32 = 0x00000004;
#[allow(non_upper_case_globals)]
pub const CONTEXT_LOST: u32 = 0x0507;
#[allow(non_upper_case_globals)]
pub const CONTEXT_PROFILE_MASK: u32 = 0x9126;
#[allow(non_upper_case_globals)]
pub const CONTEXT_RELEASE_BEHAVIOR: u32 = 0x82FB;
#[allow(non_upper_case_globals)]
pub const CONTEXT_RELEASE_BEHAVIOR_FLUSH: u32 = 0x82FC;
#[allow(non_upper_case_globals)]
pub const COPY: u32 = 0x1503;
#[allow(non_upper_case_globals)]
pub const COPY_INVERTED: u32 = 0x150C;
#[allow(non_upper_case_globals)]
pub const COPY_READ_BUFFER: u32 = 0x8F36;
#[allow(non_upper_case_globals)]
pub const COPY_READ_BUFFER_BINDING: u32 = 0x8F36;
#[allow(non_upper_case_globals)]
pub const COPY_WRITE_BUFFER: u32 = 0x8F37;
#[allow(non_upper_case_globals)]
pub const COPY_WRITE_BUFFER_BINDING: u32 = 0x8F37;
#[allow(non_upper_case_globals)]
pub const CULL_FACE: u32 = 0x0B44;
#[allow(non_upper_case_globals)]
pub const CULL_FACE_MODE: u32 = 0x0B45;
#[allow(non_upper_case_globals)]
pub const CURRENT_PROGRAM: u32 = 0x8B8D;
#[allow(non_upper_case_globals)]
pub const CURRENT_QUERY: u32 = 0x8865;
#[allow(non_upper_case_globals)]
pub const CURRENT_VERTEX_ATTRIB: u32 = 0x8626;
#[allow(non_upper_case_globals)]
pub const CW: u32 = 0x0900;
#[allow(non_upper_case_globals)]
pub const DEBUG_CALLBACK_FUNCTION: u32 = 0x8244;
#[allow(non_upper_case_globals)]
pub const DEBUG_CALLBACK_USER_PARAM: u32 = 0x8245;
#[allow(non_upper_case_globals)]
pub const DEBUG_GROUP_STACK_DEPTH: u32 = 0x826D;
#[allow(non_upper_case_globals)]
pub const DEBUG_LOGGED_MESSAGES: u32 = 0x9145;
#[allow(non_upper_case_globals)]
pub const DEBUG_NEXT_LOGGED_MESSAGE_LENGTH: u32 = 0x8243;
#[allow(non_upper_case_globals)]
pub const DEBUG_OUTPUT: u32 = 0x92E0;
#[allow(non_upper_case_globals)]
pub const DEBUG_OUTPUT_SYNCHRONOUS: u32 = 0x8242;
#[allow(non_upper_case_globals)]
pub const DEBUG_SEVERITY_HIGH: u32 = 0x9146;
#[allow(non_upper_case_globals)]
pub const DEBUG_SEVERITY_LOW: u32 = 0x9148;
#[allow(non_upper_case_globals)]
pub const DEBUG_SEVERITY_MEDIUM: u32 = 0x9147;
#[allow(non_upper_case_globals)]
pub const DEBUG_SEVERITY_NOTIFICATION: u32 = 0x826B;
#[allow(non_upper_case_globals)]
pub const DEBUG_SOURCE_API: u32 = 0x8246;
#[allow(non_upper_case_globals)]
pub const DEBUG_SOURCE_APPLICATION: u32 = 0x824A;
#[allow(non_upper_case_globals)]
pub const DEBUG_SOURCE_OTHER: u32 = 0x824B;
#[allow(non_upper_case_globals)]
pub const DEBUG_SOURCE_SHADER_COMPILER: u32 = 0x8248;
#[allow(non_upper_case_globals)]
pub const DEBUG_SOURCE_THIRD_PARTY: u32 = 0x8249;
#[allow(non_upper_case_globals)]
pub const DEBUG_SOURCE_WINDOW_SYSTEM: u32 = 0x8247;
#[allow(non_upper_case_globals)]
pub const DEBUG_TYPE_DEPRECATED_BEHAVIOR: u32 = 0x824D;
#[allow(non_upper_case_globals)]
pub const DEBUG_TYPE_ERROR: u32 = 0x824C;
#[allow(non_upper_case_globals)]
pub const DEBUG_TYPE_MARKER: u32 = 0x8268;
#[allow(non_upper_case_globals)]
pub const DEBUG_TYPE_OTHER: u32 = 0x8251;
#[allow(non_upper_case_globals)]
pub const DEBUG_TYPE_PERFORMANCE: u32 = 0x8250;
#[allow(non_upper_case_globals)]
pub const DEBUG_TYPE_POP_GROUP: u32 = 0x826A;
#[allow(non_upper_case_globals)]
pub const DEBUG_TYPE_PORTABILITY: u32 = 0x824F;
#[allow(non_upper_case_globals)]
pub const DEBUG_TYPE_PUSH_GROUP: u32 = 0x8269;
#[allow(non_upper_case_globals)]
pub const DEBUG_TYPE_UNDEFINED_BEHAVIOR: u32 = 0x824E;
#[allow(non_upper_case_globals)]
pub const DECR: u32 = 0x1E03;
#[allow(non_upper_case_globals)]
pub const DECR_WRAP: u32 = 0x8508;
#[allow(non_upper_case_globals)]
pub const DELETE_STATUS: u32 = 0x8B80;
#[allow(non_upper_case_globals)]
pub const DEPTH: u32 = 0x1801;
#[allow(non_upper_case_globals)]
pub const DEPTH24_STENCIL8: u32 = 0x88F0;
#[allow(non_upper_case_globals)]
pub const DEPTH32F_STENCIL8: u32 = 0x8CAD;
#[allow(non_upper_case_globals)]
pub const DEPTH_ATTACHMENT: u32 = 0x8D00;
#[allow(non_upper_case_globals)]
pub const DEPTH_BUFFER_BIT: u32 = 0x00000100;
#[allow(non_upper_case_globals)]
pub const DEPTH_CLAMP: u32 = 0x864F;
#[allow(non_upper_case_globals)]
pub const DEPTH_CLEAR_VALUE: u32 = 0x0B73;
#[allow(non_upper_case_globals)]
pub const DEPTH_COMPONENT: u32 = 0x1902;
#[allow(non_upper_case_globals)]
pub const DEPTH_COMPONENT16: u32 = 0x81A5;
#[allow(non_upper_case_globals)]
pub const DEPTH_COMPONENT24: u32 = 0x81A6;
#[allow(non_upper_case_globals)]
pub const DEPTH_COMPONENT32: u32 = 0x81A7;
#[allow(non_upper_case_globals)]
pub const DEPTH_COMPONENT32F: u32 = 0x8CAC;
#[allow(non_upper_case_globals)]
pub const DEPTH_COMPONENTS: u32 = 0x8284;
#[allow(non_upper_case_globals)]
pub const DEPTH_FUNC: u32 = 0x0B74;
#[allow(non_upper_case_globals)]
pub const DEPTH_RANGE: u32 = 0x0B70;
#[allow(non_upper_case_globals)]
pub const DEPTH_RENDERABLE: u32 = 0x8287;
#[allow(non_upper_case_globals)]
pub const DEPTH_STENCIL: u32 = 0x84F9;
#[allow(non_upper_case_globals)]
pub const DEPTH_STENCIL_ATTACHMENT: u32 = 0x821A;
#[allow(non_upper_case_globals)]
pub const DEPTH_STENCIL_TEXTURE_MODE: u32 = 0x90EA;
#[allow(non_upper_case_globals)]
pub const DEPTH_TEST: u32 = 0x0B71;
#[allow(non_upper_case_globals)]
pub const DEPTH_WRITEMASK: u32 = 0x0B72;
#[allow(non_upper_case_globals)]
pub const DISPATCH_INDIRECT_BUFFER: u32 = 0x90EE;
#[allow(non_upper_case_globals)]
pub const DISPATCH_INDIRECT_BUFFER_BINDING: u32 = 0x90EF;
#[allow(non_upper_case_globals)]
pub const DISPLAY_LIST: u32 = 0x82E7;
#[allow(non_upper_case_globals)]
pub const DITHER: u32 = 0x0BD0;
#[allow(non_upper_case_globals)]
pub const DONT_CARE: u32 = 0x1100;
#[allow(non_upper_case_globals)]
pub const DOUBLE: u32 = 0x140A;
#[allow(non_upper_case_globals)]
pub const DOUBLEBUFFER: u32 = 0x0C32;
#[allow(non_upper_case_globals)]
pub const DOUBLE_MAT2: u32 = 0x8F46;
#[allow(non_upper_case_globals)]
pub const DOUBLE_MAT2x3: u32 = 0x8F49;
#[allow(non_upper_case_globals)]
pub const DOUBLE_MAT2x4: u32 = 0x8F4A;
#[allow(non_upper_case_globals)]
pub const DOUBLE_MAT3: u32 = 0x8F47;
#[allow(non_upper_case_globals)]
pub const DOUBLE_MAT3x2: u32 = 0x8F4B;
#[allow(non_upper_case_globals)]
pub const DOUBLE_MAT3x4: u32 = 0x8F4C;
#[allow(non_upper_case_globals)]
pub const DOUBLE_MAT4: u32 = 0x8F48;
#[allow(non_upper_case_globals)]
pub const DOUBLE_MAT4x2: u32 = 0x8F4D;
#[allow(non_upper_case_globals)]
pub const DOUBLE_MAT4x3: u32 = 0x8F4E;
#[allow(non_upper_case_globals)]
pub const DOUBLE_VEC2: u32 = 0x8FFC;
#[allow(non_upper_case_globals)]
pub const DOUBLE_VEC3: u32 = 0x8FFD;
#[allow(non_upper_case_globals)]
pub const DOUBLE_VEC4: u32 = 0x8FFE;
#[allow(non_upper_case_globals)]
pub const DRAW_BUFFER: u32 = 0x0C01;
#[allow(non_upper_case_globals)]
pub const DRAW_BUFFER0: u32 = 0x8825;
#[allow(non_upper_case_globals)]
pub const DRAW_BUFFER1: u32 = 0x8826;
#[allow(non_upper_case_globals)]
pub const DRAW_BUFFER10: u32 = 0x882F;
#[allow(non_upper_case_globals)]
pub const DRAW_BUFFER11: u32 = 0x8830;
#[allow(non_upper_case_globals)]
pub const DRAW_BUFFER12: u32 = 0x8831;
#[allow(non_upper_case_globals)]
pub const DRAW_BUFFER13: u32 = 0x8832;
#[allow(non_upper_case_globals)]
pub const DRAW_BUFFER14: u32 = 0x8833;
#[allow(non_upper_case_globals)]
pub const DRAW_BUFFER15: u32 = 0x8834;
#[allow(non_upper_case_globals)]
pub const DRAW_BUFFER2: u32 = 0x8827;
#[allow(non_upper_case_globals)]
pub const DRAW_BUFFER3: u32 = 0x8828;
#[allow(non_upper_case_globals)]
pub const DRAW_BUFFER4: u32 = 0x8829;
#[allow(non_upper_case_globals)]
pub const DRAW_BUFFER5: u32 = 0x882A;
#[allow(non_upper_case_globals)]
pub const DRAW_BUFFER6: u32 = 0x882B;
#[allow(non_upper_case_globals)]
pub const DRAW_BUFFER7: u32 = 0x882C;
#[allow(non_upper_case_globals)]
pub const DRAW_BUFFER8: u32 = 0x882D;
#[allow(non_upper_case_globals)]
pub const DRAW_BUFFER9: u32 = 0x882E;
#[allow(non_upper_case_globals)]
pub const DRAW_FRAMEBUFFER: u32 = 0x8CA9;
#[allow(non_upper_case_globals)]
pub const DRAW_FRAMEBUFFER_BINDING: u32 = 0x8CA6;
#[allow(non_upper_case_globals)]
pub const DRAW_INDIRECT_BUFFER: u32 = 0x8F3F;
#[allow(non_upper_case_globals)]
pub const DRAW_INDIRECT_BUFFER_BINDING: u32 = 0x8F43;
#[allow(non_upper_case_globals)]
pub const DST_ALPHA: u32 = 0x0304;
#[allow(non_upper_case_globals)]
pub const DST_COLOR: u32 = 0x0306;
#[allow(non_upper_case_globals)]
pub const DYNAMIC_COPY: u32 = 0x88EA;
#[allow(non_upper_case_globals)]
pub const DYNAMIC_DRAW: u32 = 0x88E8;
#[allow(non_upper_case_globals)]
pub const DYNAMIC_READ: u32 = 0x88E9;
#[allow(non_upper_case_globals)]
pub const DYNAMIC_STORAGE_BIT: u32 = 0x0100;
#[allow(non_upper_case_globals)]
pub const ELEMENT_ARRAY_BARRIER_BIT: u32 = 0x00000002;
#[allow(non_upper_case_globals)]
pub const ELEMENT_ARRAY_BUFFER: u32 = 0x8893;
#[allow(non_upper_case_globals)]
pub const ELEMENT_ARRAY_BUFFER_BINDING: u32 = 0x8895;
#[allow(non_upper_case_globals)]
pub const EQUAL: u32 = 0x0202;
#[allow(non_upper_case_globals)]
pub const EQUIV: u32 = 0x1509;
#[allow(non_upper_case_globals)]
pub const EXTENSIONS: u32 = 0x1F03;
#[allow(non_upper_case_globals)]
pub const FALSE: u8 = 0;
#[allow(non_upper_case_globals)]
pub const FASTEST: u32 = 0x1101;
#[allow(non_upper_case_globals)]
pub const FILL: u32 = 0x1B02;
#[allow(non_upper_case_globals)]
pub const FILTER: u32 = 0x829A;
#[allow(non_upper_case_globals)]
pub const FIRST_VERTEX_CONVENTION: u32 = 0x8E4D;
#[allow(non_upper_case_globals)]
pub const FIXED: u32 = 0x140C;
#[allow(non_upper_case_globals)]
pub const FIXED_ONLY: u32 = 0x891D;
#[allow(non_upper_case_globals)]
pub const FLOAT: u32 = 0x1406;
#[allow(non_upper_case_globals)]
pub const FLOAT_32_UNSIGNED_INT_24_8_REV: u32 = 0x8DAD;
#[allow(non_upper_case_globals)]
pub const FLOAT_MAT2: u32 = 0x8B5A;
#[allow(non_upper_case_globals)]
pub const FLOAT_MAT2x3: u32 = 0x8B65;
#[allow(non_upper_case_globals)]
pub const FLOAT_MAT2x4: u32 = 0x8B66;
#[allow(non_upper_case_globals)]
pub const FLOAT_MAT3: u32 = 0x8B5B;
#[allow(non_upper_case_globals)]
pub const FLOAT_MAT3x2: u32 = 0x8B67;
#[allow(non_upper_case_globals)]
pub const FLOAT_MAT3x4: u32 = 0x8B68;
#[allow(non_upper_case_globals)]
pub const FLOAT_MAT4: u32 = 0x8B5C;
#[allow(non_upper_case_globals)]
pub const FLOAT_MAT4x2: u32 = 0x8B69;
#[allow(non_upper_case_globals)]
pub const FLOAT_MAT4x3: u32 = 0x8B6A;
#[allow(non_upper_case_globals)]
pub const FLOAT_VEC2: u32 = 0x8B50;
#[allow(non_upper_case_globals)]
pub const FLOAT_VEC3: u32 = 0x8B51;
#[allow(non_upper_case_globals)]
pub const FLOAT_VEC4: u32 = 0x8B52;
#[allow(non_upper_case_globals)]
pub const FRACTIONAL_EVEN: u32 = 0x8E7C;
#[allow(non_upper_case_globals)]
pub const FRACTIONAL_ODD: u32 = 0x8E7B;
#[allow(non_upper_case_globals)]
pub const FRAGMENT_INTERPOLATION_OFFSET_BITS: u32 = 0x8E5D;
#[allow(non_upper_case_globals)]
pub const FRAGMENT_SHADER: u32 = 0x8B30;
#[allow(non_upper_case_globals)]
pub const FRAGMENT_SHADER_BIT: u32 = 0x00000002;
#[allow(non_upper_case_globals)]
pub const FRAGMENT_SHADER_DERIVATIVE_HINT: u32 = 0x8B8B;
#[allow(non_upper_case_globals)]
pub const FRAGMENT_SHADER_INVOCATIONS: u32 = 0x82F4;
#[allow(non_upper_case_globals)]
pub const FRAGMENT_SUBROUTINE: u32 = 0x92EC;
#[allow(non_upper_case_globals)]
pub const FRAGMENT_SUBROUTINE_UNIFORM: u32 = 0x92F2;
#[allow(non_upper_case_globals)]
pub const FRAGMENT_TEXTURE: u32 = 0x829F;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER: u32 = 0x8D40;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: u32 = 0x8215;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: u32 = 0x8214;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: u32 = 0x8210;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: u32 = 0x8211;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: u32 = 0x8216;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: u32 = 0x8213;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_LAYERED: u32 = 0x8DA7;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: u32 = 0x8CD1;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: u32 = 0x8CD0;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_RED_SIZE: u32 = 0x8212;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: u32 = 0x8217;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: u32 = 0x8CD3;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: u32 = 0x8CD4;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: u32 = 0x8CD2;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_BARRIER_BIT: u32 = 0x00000400;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_BINDING: u32 = 0x8CA6;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_BLEND: u32 = 0x828B;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_COMPLETE: u32 = 0x8CD5;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_DEFAULT: u32 = 0x8218;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS: u32 = 0x9314;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_DEFAULT_HEIGHT: u32 = 0x9311;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_DEFAULT_LAYERS: u32 = 0x9312;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_DEFAULT_SAMPLES: u32 = 0x9313;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_DEFAULT_WIDTH: u32 = 0x9310;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: u32 = 0x8CD6;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: u32 = 0x8CDB;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: u32 = 0x8DA8;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: u32 = 0x8CD7;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: u32 = 0x8D56;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_INCOMPLETE_READ_BUFFER: u32 = 0x8CDC;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_RENDERABLE: u32 = 0x8289;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_RENDERABLE_LAYERED: u32 = 0x828A;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_SRGB: u32 = 0x8DB9;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_UNDEFINED: u32 = 0x8219;
#[allow(non_upper_case_globals)]
pub const FRAMEBUFFER_UNSUPPORTED: u32 = 0x8CDD;
#[allow(non_upper_case_globals)]
pub const FRONT: u32 = 0x0404;
#[allow(non_upper_case_globals)]
pub const FRONT_AND_BACK: u32 = 0x0408;
#[allow(non_upper_case_globals)]
pub const FRONT_FACE: u32 = 0x0B46;
#[allow(non_upper_case_globals)]
pub const FRONT_LEFT: u32 = 0x0400;
#[allow(non_upper_case_globals)]
pub const FRONT_RIGHT: u32 = 0x0401;
#[allow(non_upper_case_globals)]
pub const FULL_SUPPORT: u32 = 0x82B7;
#[allow(non_upper_case_globals)]
pub const FUNC_ADD: u32 = 0x8006;
#[allow(non_upper_case_globals)]
pub const FUNC_REVERSE_SUBTRACT: u32 = 0x800B;
#[allow(non_upper_case_globals)]
pub const FUNC_SUBTRACT: u32 = 0x800A;
#[allow(non_upper_case_globals)]
pub const GEOMETRY_INPUT_TYPE: u32 = 0x8917;
#[allow(non_upper_case_globals)]
pub const GEOMETRY_OUTPUT_TYPE: u32 = 0x8918;
#[allow(non_upper_case_globals)]
pub const GEOMETRY_SHADER: u32 = 0x8DD9;
#[allow(non_upper_case_globals)]
pub const GEOMETRY_SHADER_BIT: u32 = 0x00000004;
#[allow(non_upper_case_globals)]
pub const GEOMETRY_SHADER_INVOCATIONS: u32 = 0x887F;
#[allow(non_upper_case_globals)]
pub const GEOMETRY_SHADER_PRIMITIVES_EMITTED: u32 = 0x82F3;
#[allow(non_upper_case_globals)]
pub const GEOMETRY_SUBROUTINE: u32 = 0x92EB;
#[allow(non_upper_case_globals)]
pub const GEOMETRY_SUBROUTINE_UNIFORM: u32 = 0x92F1;
#[allow(non_upper_case_globals)]
pub const GEOMETRY_TEXTURE: u32 = 0x829E;
#[allow(non_upper_case_globals)]
pub const GEOMETRY_VERTICES_OUT: u32 = 0x8916;
#[allow(non_upper_case_globals)]
pub const GEQUAL: u32 = 0x0206;
#[allow(non_upper_case_globals)]
pub const GET_TEXTURE_IMAGE_FORMAT: u32 = 0x8291;
#[allow(non_upper_case_globals)]
pub const GET_TEXTURE_IMAGE_TYPE: u32 = 0x8292;
#[allow(non_upper_case_globals)]
pub const GREATER: u32 = 0x0204;
#[allow(non_upper_case_globals)]
pub const GREEN: u32 = 0x1904;
#[allow(non_upper_case_globals)]
pub const GREEN_INTEGER: u32 = 0x8D95;
#[allow(non_upper_case_globals)]
pub const GUILTY_CONTEXT_RESET: u32 = 0x8253;
#[allow(non_upper_case_globals)]
pub const HALF_FLOAT: u32 = 0x140B;
#[allow(non_upper_case_globals)]
pub const HIGH_FLOAT: u32 = 0x8DF2;
#[allow(non_upper_case_globals)]
pub const HIGH_INT: u32 = 0x8DF5;
#[allow(non_upper_case_globals)]
pub const IMAGE_1D: u32 = 0x904C;
#[allow(non_upper_case_globals)]
pub const IMAGE_1D_ARRAY: u32 = 0x9052;
#[allow(non_upper_case_globals)]
pub const IMAGE_2D: u32 = 0x904D;
#[allow(non_upper_case_globals)]
pub const IMAGE_2D_ARRAY: u32 = 0x9053;
#[allow(non_upper_case_globals)]
pub const IMAGE_2D_MULTISAMPLE: u32 = 0x9055;
#[allow(non_upper_case_globals)]
pub const IMAGE_2D_MULTISAMPLE_ARRAY: u32 = 0x9056;
#[allow(non_upper_case_globals)]
pub const IMAGE_2D_RECT: u32 = 0x904F;
#[allow(non_upper_case_globals)]
pub const IMAGE_3D: u32 = 0x904E;
#[allow(non_upper_case_globals)]
pub const IMAGE_BINDING_ACCESS: u32 = 0x8F3E;
#[allow(non_upper_case_globals)]
pub const IMAGE_BINDING_FORMAT: u32 = 0x906E;
#[allow(non_upper_case_globals)]
pub const IMAGE_BINDING_LAYER: u32 = 0x8F3D;
#[allow(non_upper_case_globals)]
pub const IMAGE_BINDING_LAYERED: u32 = 0x8F3C;
#[allow(non_upper_case_globals)]
pub const IMAGE_BINDING_LEVEL: u32 = 0x8F3B;
#[allow(non_upper_case_globals)]
pub const IMAGE_BINDING_NAME: u32 = 0x8F3A;
#[allow(non_upper_case_globals)]
pub const IMAGE_BUFFER: u32 = 0x9051;
#[allow(non_upper_case_globals)]
pub const IMAGE_CLASS_10_10_10_2: u32 = 0x82C3;
#[allow(non_upper_case_globals)]
pub const IMAGE_CLASS_11_11_10: u32 = 0x82C2;
#[allow(non_upper_case_globals)]
pub const IMAGE_CLASS_1_X_16: u32 = 0x82BE;
#[allow(non_upper_case_globals)]
pub const IMAGE_CLASS_1_X_32: u32 = 0x82BB;
#[allow(non_upper_case_globals)]
pub const IMAGE_CLASS_1_X_8: u32 = 0x82C1;
#[allow(non_upper_case_globals)]
pub const IMAGE_CLASS_2_X_16: u32 = 0x82BD;
#[allow(non_upper_case_globals)]
pub const IMAGE_CLASS_2_X_32: u32 = 0x82BA;
#[allow(non_upper_case_globals)]
pub const IMAGE_CLASS_2_X_8: u32 = 0x82C0;
#[allow(non_upper_case_globals)]
pub const IMAGE_CLASS_4_X_16: u32 = 0x82BC;
#[allow(non_upper_case_globals)]
pub const IMAGE_CLASS_4_X_32: u32 = 0x82B9;
#[allow(non_upper_case_globals)]
pub const IMAGE_CLASS_4_X_8: u32 = 0x82BF;
#[allow(non_upper_case_globals)]
pub const IMAGE_COMPATIBILITY_CLASS: u32 = 0x82A8;
#[allow(non_upper_case_globals)]
pub const IMAGE_CUBE: u32 = 0x9050;
#[allow(non_upper_case_globals)]
pub const IMAGE_CUBE_MAP_ARRAY: u32 = 0x9054;
#[allow(non_upper_case_globals)]
pub const IMAGE_FORMAT_COMPATIBILITY_BY_CLASS: u32 = 0x90C9;
#[allow(non_upper_case_globals)]
pub const IMAGE_FORMAT_COMPATIBILITY_BY_SIZE: u32 = 0x90C8;
#[allow(non_upper_case_globals)]
pub const IMAGE_FORMAT_COMPATIBILITY_TYPE: u32 = 0x90C7;
#[allow(non_upper_case_globals)]
pub const IMAGE_PIXEL_FORMAT: u32 = 0x82A9;
#[allow(non_upper_case_globals)]
pub const IMAGE_PIXEL_TYPE: u32 = 0x82AA;
#[allow(non_upper_case_globals)]
pub const IMAGE_TEXEL_SIZE: u32 = 0x82A7;
#[allow(non_upper_case_globals)]
pub const IMPLEMENTATION_COLOR_READ_FORMAT: u32 = 0x8B9B;
#[allow(non_upper_case_globals)]
pub const IMPLEMENTATION_COLOR_READ_TYPE: u32 = 0x8B9A;
#[allow(non_upper_case_globals)]
pub const INCR: u32 = 0x1E02;
#[allow(non_upper_case_globals)]
pub const INCR_WRAP: u32 = 0x8507;
#[allow(non_upper_case_globals)]
pub const INDEX: u32 = 0x8222;
#[allow(non_upper_case_globals)]
pub const INFO_LOG_LENGTH: u32 = 0x8B84;
#[allow(non_upper_case_globals)]
pub const INNOCENT_CONTEXT_RESET: u32 = 0x8254;
#[allow(non_upper_case_globals)]
pub const INT: u32 = 0x1404;
#[allow(non_upper_case_globals)]
pub const INTERLEAVED_ATTRIBS: u32 = 0x8C8C;
#[allow(non_upper_case_globals)]
pub const INTERNALFORMAT_ALPHA_SIZE: u32 = 0x8274;
#[allow(non_upper_case_globals)]
pub const INTERNALFORMAT_ALPHA_TYPE: u32 = 0x827B;
#[allow(non_upper_case_globals)]
pub const INTERNALFORMAT_BLUE_SIZE: u32 = 0x8273;
#[allow(non_upper_case_globals)]
pub const INTERNALFORMAT_BLUE_TYPE: u32 = 0x827A;
#[allow(non_upper_case_globals)]
pub const INTERNALFORMAT_DEPTH_SIZE: u32 = 0x8275;
#[allow(non_upper_case_globals)]
pub const INTERNALFORMAT_DEPTH_TYPE: u32 = 0x827C;
#[allow(non_upper_case_globals)]
pub const INTERNALFORMAT_GREEN_SIZE: u32 = 0x8272;
#[allow(non_upper_case_globals)]
pub const INTERNALFORMAT_GREEN_TYPE: u32 = 0x8279;
#[allow(non_upper_case_globals)]
pub const INTERNALFORMAT_PREFERRED: u32 = 0x8270;
#[allow(non_upper_case_globals)]
pub const INTERNALFORMAT_RED_SIZE: u32 = 0x8271;
#[allow(non_upper_case_globals)]
pub const INTERNALFORMAT_RED_TYPE: u32 = 0x8278;
#[allow(non_upper_case_globals)]
pub const INTERNALFORMAT_SHARED_SIZE: u32 = 0x8277;
#[allow(non_upper_case_globals)]
pub const INTERNALFORMAT_STENCIL_SIZE: u32 = 0x8276;
#[allow(non_upper_case_globals)]
pub const INTERNALFORMAT_STENCIL_TYPE: u32 = 0x827D;
#[allow(non_upper_case_globals)]
pub const INTERNALFORMAT_SUPPORTED: u32 = 0x826F;
#[allow(non_upper_case_globals)]
pub const INT_2_10_10_10_REV: u32 = 0x8D9F;
#[allow(non_upper_case_globals)]
pub const INT_IMAGE_1D: u32 = 0x9057;
#[allow(non_upper_case_globals)]
pub const INT_IMAGE_1D_ARRAY: u32 = 0x905D;
#[allow(non_upper_case_globals)]
pub const INT_IMAGE_2D: u32 = 0x9058;
#[allow(non_upper_case_globals)]
pub const INT_IMAGE_2D_ARRAY: u32 = 0x905E;
#[allow(non_upper_case_globals)]
pub const INT_IMAGE_2D_MULTISAMPLE: u32 = 0x9060;
#[allow(non_upper_case_globals)]
pub const INT_IMAGE_2D_MULTISAMPLE_ARRAY: u32 = 0x9061;
#[allow(non_upper_case_globals)]
pub const INT_IMAGE_2D_RECT: u32 = 0x905A;
#[allow(non_upper_case_globals)]
pub const INT_IMAGE_3D: u32 = 0x9059;
#[allow(non_upper_case_globals)]
pub const INT_IMAGE_BUFFER: u32 = 0x905C;
#[allow(non_upper_case_globals)]
pub const INT_IMAGE_CUBE: u32 = 0x905B;
#[allow(non_upper_case_globals)]
pub const INT_IMAGE_CUBE_MAP_ARRAY: u32 = 0x905F;
#[allow(non_upper_case_globals)]
pub const INT_SAMPLER_1D: u32 = 0x8DC9;
#[allow(non_upper_case_globals)]
pub const INT_SAMPLER_1D_ARRAY: u32 = 0x8DCE;
#[allow(non_upper_case_globals)]
pub const INT_SAMPLER_2D: u32 = 0x8DCA;
#[allow(non_upper_case_globals)]
pub const INT_SAMPLER_2D_ARRAY: u32 = 0x8DCF;
#[allow(non_upper_case_globals)]
pub const INT_SAMPLER_2D_MULTISAMPLE: u32 = 0x9109;
#[allow(non_upper_case_globals)]
pub const INT_SAMPLER_2D_MULTISAMPLE_ARRAY: u32 = 0x910C;
#[allow(non_upper_case_globals)]
pub const INT_SAMPLER_2D_RECT: u32 = 0x8DCD;
#[allow(non_upper_case_globals)]
pub const INT_SAMPLER_3D: u32 = 0x8DCB;
#[allow(non_upper_case_globals)]
pub const INT_SAMPLER_BUFFER: u32 = 0x8DD0;
#[allow(non_upper_case_globals)]
pub const INT_SAMPLER_CUBE: u32 = 0x8DCC;
#[allow(non_upper_case_globals)]
pub const INT_SAMPLER_CUBE_MAP_ARRAY: u32 = 0x900E;
#[allow(non_upper_case_globals)]
pub const INT_VEC2: u32 = 0x8B53;
#[allow(non_upper_case_globals)]
pub const INT_VEC3: u32 = 0x8B54;
#[allow(non_upper_case_globals)]
pub const INT_VEC4: u32 = 0x8B55;
#[allow(non_upper_case_globals)]
pub const INVALID_ENUM: u32 = 0x0500;
#[allow(non_upper_case_globals)]
pub const INVALID_FRAMEBUFFER_OPERATION: u32 = 0x0506;
#[allow(non_upper_case_globals)]
pub const INVALID_INDEX: u32 = 0xFFFFFFFF;
#[allow(non_upper_case_globals)]
pub const INVALID_OPERATION: u32 = 0x0502;
#[allow(non_upper_case_globals)]
pub const INVALID_VALUE: u32 = 0x0501;
#[allow(non_upper_case_globals)]
pub const INVERT: u32 = 0x150A;
#[allow(non_upper_case_globals)]
pub const ISOLINES: u32 = 0x8E7A;
#[allow(non_upper_case_globals)]
pub const IS_PER_PATCH: u32 = 0x92E7;
#[allow(non_upper_case_globals)]
pub const IS_ROW_MAJOR: u32 = 0x9300;
#[allow(non_upper_case_globals)]
pub const KEEP: u32 = 0x1E00;
#[allow(non_upper_case_globals)]
pub const LAST_VERTEX_CONVENTION: u32 = 0x8E4E;
#[allow(non_upper_case_globals)]
pub const LAYER_PROVOKING_VERTEX: u32 = 0x825E;
#[allow(non_upper_case_globals)]
pub const LEFT: u32 = 0x0406;
#[allow(non_upper_case_globals)]
pub const LEQUAL: u32 = 0x0203;
#[allow(non_upper_case_globals)]
pub const LESS: u32 = 0x0201;
#[allow(non_upper_case_globals)]
pub const LINE: u32 = 0x1B01;
#[allow(non_upper_case_globals)]
pub const LINEAR: u32 = 0x2601;
#[allow(non_upper_case_globals)]
pub const LINEAR_MIPMAP_LINEAR: u32 = 0x2703;
#[allow(non_upper_case_globals)]
pub const LINEAR_MIPMAP_NEAREST: u32 = 0x2701;
#[allow(non_upper_case_globals)]
pub const LINES: u32 = 0x0001;
#[allow(non_upper_case_globals)]
pub const LINES_ADJACENCY: u32 = 0x000A;
#[allow(non_upper_case_globals)]
pub const LINE_LOOP: u32 = 0x0002;
#[allow(non_upper_case_globals)]
pub const LINE_SMOOTH: u32 = 0x0B20;
#[allow(non_upper_case_globals)]
pub const LINE_SMOOTH_HINT: u32 = 0x0C52;
#[allow(non_upper_case_globals)]
pub const LINE_STRIP: u32 = 0x0003;
#[allow(non_upper_case_globals)]
pub const LINE_STRIP_ADJACENCY: u32 = 0x000B;
#[allow(non_upper_case_globals)]
pub const LINE_WIDTH: u32 = 0x0B21;
#[allow(non_upper_case_globals)]
pub const LINE_WIDTH_GRANULARITY: u32 = 0x0B23;
#[allow(non_upper_case_globals)]
pub const LINE_WIDTH_RANGE: u32 = 0x0B22;
#[allow(non_upper_case_globals)]
pub const LINK_STATUS: u32 = 0x8B82;
#[allow(non_upper_case_globals)]
pub const LOCATION: u32 = 0x930E;
#[allow(non_upper_case_globals)]
pub const LOCATION_COMPONENT: u32 = 0x934A;
#[allow(non_upper_case_globals)]
pub const LOCATION_INDEX: u32 = 0x930F;
#[allow(non_upper_case_globals)]
pub const LOGIC_OP_MODE: u32 = 0x0BF0;
#[allow(non_upper_case_globals)]
pub const LOSE_CONTEXT_ON_RESET: u32 = 0x8252;
#[allow(non_upper_case_globals)]
pub const LOWER_LEFT: u32 = 0x8CA1;
#[allow(non_upper_case_globals)]
pub const LOW_FLOAT: u32 = 0x8DF0;
#[allow(non_upper_case_globals)]
pub const LOW_INT: u32 = 0x8DF3;
#[allow(non_upper_case_globals)]
pub const MAJOR_VERSION: u32 = 0x821B;
#[allow(non_upper_case_globals)]
pub const MANUAL_GENERATE_MIPMAP: u32 = 0x8294;
#[allow(non_upper_case_globals)]
pub const MAP_COHERENT_BIT: u32 = 0x0080;
#[allow(non_upper_case_globals)]
pub const MAP_FLUSH_EXPLICIT_BIT: u32 = 0x0010;
#[allow(non_upper_case_globals)]
pub const MAP_INVALIDATE_BUFFER_BIT: u32 = 0x0008;
#[allow(non_upper_case_globals)]
pub const MAP_INVALIDATE_RANGE_BIT: u32 = 0x0004;
#[allow(non_upper_case_globals)]
pub const MAP_PERSISTENT_BIT: u32 = 0x0040;
#[allow(non_upper_case_globals)]
pub const MAP_READ_BIT: u32 = 0x0001;
#[allow(non_upper_case_globals)]
pub const MAP_UNSYNCHRONIZED_BIT: u32 = 0x0020;
#[allow(non_upper_case_globals)]
pub const MAP_WRITE_BIT: u32 = 0x0002;
#[allow(non_upper_case_globals)]
pub const MATRIX_STRIDE: u32 = 0x92FF;
#[allow(non_upper_case_globals)]
pub const MAX: u32 = 0x8008;
#[allow(non_upper_case_globals)]
pub const MAX_3D_TEXTURE_SIZE: u32 = 0x8073;
#[allow(non_upper_case_globals)]
pub const MAX_ARRAY_TEXTURE_LAYERS: u32 = 0x88FF;
#[allow(non_upper_case_globals)]
pub const MAX_ATOMIC_COUNTER_BUFFER_BINDINGS: u32 = 0x92DC;
#[allow(non_upper_case_globals)]
pub const MAX_ATOMIC_COUNTER_BUFFER_SIZE: u32 = 0x92D8;
#[allow(non_upper_case_globals)]
pub const MAX_CLIP_DISTANCES: u32 = 0x0D32;
#[allow(non_upper_case_globals)]
pub const MAX_COLOR_ATTACHMENTS: u32 = 0x8CDF;
#[allow(non_upper_case_globals)]
pub const MAX_COLOR_TEXTURE_SAMPLES: u32 = 0x910E;
#[allow(non_upper_case_globals)]
pub const MAX_COMBINED_ATOMIC_COUNTERS: u32 = 0x92D7;
#[allow(non_upper_case_globals)]
pub const MAX_COMBINED_ATOMIC_COUNTER_BUFFERS: u32 = 0x92D1;
#[allow(non_upper_case_globals)]
pub const MAX_COMBINED_CLIP_AND_CULL_DISTANCES: u32 = 0x82FA;
#[allow(non_upper_case_globals)]
pub const MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS: u32 = 0x8266;
#[allow(non_upper_case_globals)]
pub const MAX_COMBINED_DIMENSIONS: u32 = 0x8282;
#[allow(non_upper_case_globals)]
pub const MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: u32 = 0x8A33;
#[allow(non_upper_case_globals)]
pub const MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: u32 = 0x8A32;
#[allow(non_upper_case_globals)]
pub const MAX_COMBINED_IMAGE_UNIFORMS: u32 = 0x90CF;
#[allow(non_upper_case_globals)]
pub const MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS: u32 = 0x8F39;
#[allow(non_upper_case_globals)]
pub const MAX_COMBINED_SHADER_OUTPUT_RESOURCES: u32 = 0x8F39;
#[allow(non_upper_case_globals)]
pub const MAX_COMBINED_SHADER_STORAGE_BLOCKS: u32 = 0x90DC;
#[allow(non_upper_case_globals)]
pub const MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS: u32 = 0x8E1E;
#[allow(non_upper_case_globals)]
pub const MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS: u32 = 0x8E1F;
#[allow(non_upper_case_globals)]
pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: u32 = 0x8B4D;
#[allow(non_upper_case_globals)]
pub const MAX_COMBINED_UNIFORM_BLOCKS: u32 = 0x8A2E;
#[allow(non_upper_case_globals)]
pub const MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: u32 = 0x8A31;
#[allow(non_upper_case_globals)]
pub const MAX_COMPUTE_ATOMIC_COUNTERS: u32 = 0x8265;
#[allow(non_upper_case_globals)]
pub const MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS: u32 = 0x8264;
#[allow(non_upper_case_globals)]
pub const MAX_COMPUTE_IMAGE_UNIFORMS: u32 = 0x91BD;
#[allow(non_upper_case_globals)]
pub const MAX_COMPUTE_SHADER_STORAGE_BLOCKS: u32 = 0x90DB;
#[allow(non_upper_case_globals)]
pub const MAX_COMPUTE_SHARED_MEMORY_SIZE: u32 = 0x8262;
#[allow(non_upper_case_globals)]
pub const MAX_COMPUTE_TEXTURE_IMAGE_UNITS: u32 = 0x91BC;
#[allow(non_upper_case_globals)]
pub const MAX_COMPUTE_UNIFORM_BLOCKS: u32 = 0x91BB;
#[allow(non_upper_case_globals)]
pub const MAX_COMPUTE_UNIFORM_COMPONENTS: u32 = 0x8263;
#[allow(non_upper_case_globals)]
pub const MAX_COMPUTE_WORK_GROUP_COUNT: u32 = 0x91BE;
#[allow(non_upper_case_globals)]
pub const MAX_COMPUTE_WORK_GROUP_INVOCATIONS: u32 = 0x90EB;
#[allow(non_upper_case_globals)]
pub const MAX_COMPUTE_WORK_GROUP_SIZE: u32 = 0x91BF;
#[allow(non_upper_case_globals)]
pub const MAX_CUBE_MAP_TEXTURE_SIZE: u32 = 0x851C;
#[allow(non_upper_case_globals)]
pub const MAX_CULL_DISTANCES: u32 = 0x82F9;
#[allow(non_upper_case_globals)]
pub const MAX_DEBUG_GROUP_STACK_DEPTH: u32 = 0x826C;
#[allow(non_upper_case_globals)]
pub const MAX_DEBUG_LOGGED_MESSAGES: u32 = 0x9144;
#[allow(non_upper_case_globals)]
pub const MAX_DEBUG_MESSAGE_LENGTH: u32 = 0x9143;
#[allow(non_upper_case_globals)]
pub const MAX_DEPTH: u32 = 0x8280;
#[allow(non_upper_case_globals)]
pub const MAX_DEPTH_TEXTURE_SAMPLES: u32 = 0x910F;
#[allow(non_upper_case_globals)]
pub const MAX_DRAW_BUFFERS: u32 = 0x8824;
#[allow(non_upper_case_globals)]
pub const MAX_DUAL_SOURCE_DRAW_BUFFERS: u32 = 0x88FC;
#[allow(non_upper_case_globals)]
pub const MAX_ELEMENTS_INDICES: u32 = 0x80E9;
#[allow(non_upper_case_globals)]
pub const MAX_ELEMENTS_VERTICES: u32 = 0x80E8;
#[allow(non_upper_case_globals)]
pub const MAX_ELEMENT_INDEX: u32 = 0x8D6B;
#[allow(non_upper_case_globals)]
pub const MAX_FRAGMENT_ATOMIC_COUNTERS: u32 = 0x92D6;
#[allow(non_upper_case_globals)]
pub const MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS: u32 = 0x92D0;
#[allow(non_upper_case_globals)]
pub const MAX_FRAGMENT_IMAGE_UNIFORMS: u32 = 0x90CE;
#[allow(non_upper_case_globals)]
pub const MAX_FRAGMENT_INPUT_COMPONENTS: u32 = 0x9125;
#[allow(non_upper_case_globals)]
pub const MAX_FRAGMENT_INTERPOLATION_OFFSET: u32 = 0x8E5C;
#[allow(non_upper_case_globals)]
pub const MAX_FRAGMENT_SHADER_STORAGE_BLOCKS: u32 = 0x90DA;
#[allow(non_upper_case_globals)]
pub const MAX_FRAGMENT_UNIFORM_BLOCKS: u32 = 0x8A2D;
#[allow(non_upper_case_globals)]
pub const MAX_FRAGMENT_UNIFORM_COMPONENTS: u32 = 0x8B49;
#[allow(non_upper_case_globals)]
pub const MAX_FRAGMENT_UNIFORM_VECTORS: u32 = 0x8DFD;
#[allow(non_upper_case_globals)]
pub const MAX_FRAMEBUFFER_HEIGHT: u32 = 0x9316;
#[allow(non_upper_case_globals)]
pub const MAX_FRAMEBUFFER_LAYERS: u32 = 0x9317;
#[allow(non_upper_case_globals)]
pub const MAX_FRAMEBUFFER_SAMPLES: u32 = 0x9318;
#[allow(non_upper_case_globals)]
pub const MAX_FRAMEBUFFER_WIDTH: u32 = 0x9315;
#[allow(non_upper_case_globals)]
pub const MAX_GEOMETRY_ATOMIC_COUNTERS: u32 = 0x92D5;
#[allow(non_upper_case_globals)]
pub const MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS: u32 = 0x92CF;
#[allow(non_upper_case_globals)]
pub const MAX_GEOMETRY_IMAGE_UNIFORMS: u32 = 0x90CD;
#[allow(non_upper_case_globals)]
pub const MAX_GEOMETRY_INPUT_COMPONENTS: u32 = 0x9123;
#[allow(non_upper_case_globals)]
pub const MAX_GEOMETRY_OUTPUT_COMPONENTS: u32 = 0x9124;
#[allow(non_upper_case_globals)]
pub const MAX_GEOMETRY_OUTPUT_VERTICES: u32 = 0x8DE0;
#[allow(non_upper_case_globals)]
pub const MAX_GEOMETRY_SHADER_INVOCATIONS: u32 = 0x8E5A;
#[allow(non_upper_case_globals)]
pub const MAX_GEOMETRY_SHADER_STORAGE_BLOCKS: u32 = 0x90D7;
#[allow(non_upper_case_globals)]
pub const MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: u32 = 0x8C29;
#[allow(non_upper_case_globals)]
pub const MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: u32 = 0x8DE1;
#[allow(non_upper_case_globals)]
pub const MAX_GEOMETRY_UNIFORM_BLOCKS: u32 = 0x8A2C;
#[allow(non_upper_case_globals)]
pub const MAX_GEOMETRY_UNIFORM_COMPONENTS: u32 = 0x8DDF;
#[allow(non_upper_case_globals)]
pub const MAX_HEIGHT: u32 = 0x827F;
#[allow(non_upper_case_globals)]
pub const MAX_IMAGE_SAMPLES: u32 = 0x906D;
#[allow(non_upper_case_globals)]
pub const MAX_IMAGE_UNITS: u32 = 0x8F38;
#[allow(non_upper_case_globals)]
pub const MAX_INTEGER_SAMPLES: u32 = 0x9110;
#[allow(non_upper_case_globals)]
pub const MAX_LABEL_LENGTH: u32 = 0x82E8;
#[allow(non_upper_case_globals)]
pub const MAX_LAYERS: u32 = 0x8281;
#[allow(non_upper_case_globals)]
pub const MAX_NAME_LENGTH: u32 = 0x92F6;
#[allow(non_upper_case_globals)]
pub const MAX_NUM_ACTIVE_VARIABLES: u32 = 0x92F7;
#[allow(non_upper_case_globals)]
pub const MAX_NUM_COMPATIBLE_SUBROUTINES: u32 = 0x92F8;
#[allow(non_upper_case_globals)]
pub const MAX_PATCH_VERTICES: u32 = 0x8E7D;
#[allow(non_upper_case_globals)]
pub const MAX_PROGRAM_TEXEL_OFFSET: u32 = 0x8905;
#[allow(non_upper_case_globals)]
pub const MAX_PROGRAM_TEXTURE_GATHER_OFFSET: u32 = 0x8E5F;
#[allow(non_upper_case_globals)]
pub const MAX_RECTANGLE_TEXTURE_SIZE: u32 = 0x84F8;
#[allow(non_upper_case_globals)]
pub const MAX_RENDERBUFFER_SIZE: u32 = 0x84E8;
#[allow(non_upper_case_globals)]
pub const MAX_SAMPLES: u32 = 0x8D57;
#[allow(non_upper_case_globals)]
pub const MAX_SAMPLE_MASK_WORDS: u32 = 0x8E59;
#[allow(non_upper_case_globals)]
pub const MAX_SERVER_WAIT_TIMEOUT: u32 = 0x9111;
#[allow(non_upper_case_globals)]
pub const MAX_SHADER_STORAGE_BLOCK_SIZE: u32 = 0x90DE;
#[allow(non_upper_case_globals)]
pub const MAX_SHADER_STORAGE_BUFFER_BINDINGS: u32 = 0x90DD;
#[allow(non_upper_case_globals)]
pub const MAX_SUBROUTINES: u32 = 0x8DE7;
#[allow(non_upper_case_globals)]
pub const MAX_SUBROUTINE_UNIFORM_LOCATIONS: u32 = 0x8DE8;
#[allow(non_upper_case_globals)]
pub const MAX_TESS_CONTROL_ATOMIC_COUNTERS: u32 = 0x92D3;
#[allow(non_upper_case_globals)]
pub const MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS: u32 = 0x92CD;
#[allow(non_upper_case_globals)]
pub const MAX_TESS_CONTROL_IMAGE_UNIFORMS: u32 = 0x90CB;
#[allow(non_upper_case_globals)]
pub const MAX_TESS_CONTROL_INPUT_COMPONENTS: u32 = 0x886C;
#[allow(non_upper_case_globals)]
pub const MAX_TESS_CONTROL_OUTPUT_COMPONENTS: u32 = 0x8E83;
#[allow(non_upper_case_globals)]
pub const MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS: u32 = 0x90D8;
#[allow(non_upper_case_globals)]
pub const MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS: u32 = 0x8E81;
#[allow(non_upper_case_globals)]
pub const MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS: u32 = 0x8E85;
#[allow(non_upper_case_globals)]
pub const MAX_TESS_CONTROL_UNIFORM_BLOCKS: u32 = 0x8E89;
#[allow(non_upper_case_globals)]
pub const MAX_TESS_CONTROL_UNIFORM_COMPONENTS: u32 = 0x8E7F;
#[allow(non_upper_case_globals)]
pub const MAX_TESS_EVALUATION_ATOMIC_COUNTERS: u32 = 0x92D4;
#[allow(non_upper_case_globals)]
pub const MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS: u32 = 0x92CE;
#[allow(non_upper_case_globals)]
pub const MAX_TESS_EVALUATION_IMAGE_UNIFORMS: u32 = 0x90CC;
#[allow(non_upper_case_globals)]
pub const MAX_TESS_EVALUATION_INPUT_COMPONENTS: u32 = 0x886D;
#[allow(non_upper_case_globals)]
pub const MAX_TESS_EVALUATION_OUTPUT_COMPONENTS: u32 = 0x8E86;
#[allow(non_upper_case_globals)]
pub const MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS: u32 = 0x90D9;
#[allow(non_upper_case_globals)]
pub const MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS: u32 = 0x8E82;
#[allow(non_upper_case_globals)]
pub const MAX_TESS_EVALUATION_UNIFORM_BLOCKS: u32 = 0x8E8A;
#[allow(non_upper_case_globals)]
pub const MAX_TESS_EVALUATION_UNIFORM_COMPONENTS: u32 = 0x8E80;
#[allow(non_upper_case_globals)]
pub const MAX_TESS_GEN_LEVEL: u32 = 0x8E7E;
#[allow(non_upper_case_globals)]
pub const MAX_TESS_PATCH_COMPONENTS: u32 = 0x8E84;
#[allow(non_upper_case_globals)]
pub const MAX_TEXTURE_BUFFER_SIZE: u32 = 0x8C2B;
#[allow(non_upper_case_globals)]
pub const MAX_TEXTURE_IMAGE_UNITS: u32 = 0x8872;
#[allow(non_upper_case_globals)]
pub const MAX_TEXTURE_LOD_BIAS: u32 = 0x84FD;
#[allow(non_upper_case_globals)]
pub const MAX_TEXTURE_MAX_ANISOTROPY: u32 = 0x84FF;
#[allow(non_upper_case_globals)]
pub const MAX_TEXTURE_MAX_ANISOTROPY_EXT: u32 = 0x84FF;
#[allow(non_upper_case_globals)]
pub const MAX_TEXTURE_SIZE: u32 = 0x0D33;
#[allow(non_upper_case_globals)]
pub const MAX_TRANSFORM_FEEDBACK_BUFFERS: u32 = 0x8E70;
#[allow(non_upper_case_globals)]
pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: u32 = 0x8C8A;
#[allow(non_upper_case_globals)]
pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: u32 = 0x8C8B;
#[allow(non_upper_case_globals)]
pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: u32 = 0x8C80;
#[allow(non_upper_case_globals)]
pub const MAX_UNIFORM_BLOCK_SIZE: u32 = 0x8A30;
#[allow(non_upper_case_globals)]
pub const MAX_UNIFORM_BUFFER_BINDINGS: u32 = 0x8A2F;
#[allow(non_upper_case_globals)]
pub const MAX_UNIFORM_LOCATIONS: u32 = 0x826E;
#[allow(non_upper_case_globals)]
pub const MAX_VARYING_COMPONENTS: u32 = 0x8B4B;
#[allow(non_upper_case_globals)]
pub const MAX_VARYING_FLOATS: u32 = 0x8B4B;
#[allow(non_upper_case_globals)]
pub const MAX_VARYING_VECTORS: u32 = 0x8DFC;
#[allow(non_upper_case_globals)]
pub const MAX_VERTEX_ATOMIC_COUNTERS: u32 = 0x92D2;
#[allow(non_upper_case_globals)]
pub const MAX_VERTEX_ATOMIC_COUNTER_BUFFERS: u32 = 0x92CC;
#[allow(non_upper_case_globals)]
pub const MAX_VERTEX_ATTRIBS: u32 = 0x8869;
#[allow(non_upper_case_globals)]
pub const MAX_VERTEX_ATTRIB_BINDINGS: u32 = 0x82DA;
#[allow(non_upper_case_globals)]
pub const MAX_VERTEX_ATTRIB_RELATIVE_OFFSET: u32 = 0x82D9;
#[allow(non_upper_case_globals)]
pub const MAX_VERTEX_ATTRIB_STRIDE: u32 = 0x82E5;
#[allow(non_upper_case_globals)]
pub const MAX_VERTEX_IMAGE_UNIFORMS: u32 = 0x90CA;
#[allow(non_upper_case_globals)]
pub const MAX_VERTEX_OUTPUT_COMPONENTS: u32 = 0x9122;
#[allow(non_upper_case_globals)]
pub const MAX_VERTEX_SHADER_STORAGE_BLOCKS: u32 = 0x90D6;
#[allow(non_upper_case_globals)]
pub const MAX_VERTEX_STREAMS: u32 = 0x8E71;
#[allow(non_upper_case_globals)]
pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: u32 = 0x8B4C;
#[allow(non_upper_case_globals)]
pub const MAX_VERTEX_UNIFORM_BLOCKS: u32 = 0x8A2B;
#[allow(non_upper_case_globals)]
pub const MAX_VERTEX_UNIFORM_COMPONENTS: u32 = 0x8B4A;
#[allow(non_upper_case_globals)]
pub const MAX_VERTEX_UNIFORM_VECTORS: u32 = 0x8DFB;
#[allow(non_upper_case_globals)]
pub const MAX_VIEWPORTS: u32 = 0x825B;
#[allow(non_upper_case_globals)]
pub const MAX_VIEWPORT_DIMS: u32 = 0x0D3A;
#[allow(non_upper_case_globals)]
pub const MAX_WIDTH: u32 = 0x827E;
#[allow(non_upper_case_globals)]
pub const MEDIUM_FLOAT: u32 = 0x8DF1;
#[allow(non_upper_case_globals)]
pub const MEDIUM_INT: u32 = 0x8DF4;
#[allow(non_upper_case_globals)]
pub const MIN: u32 = 0x8007;
#[allow(non_upper_case_globals)]
pub const MINOR_VERSION: u32 = 0x821C;
#[allow(non_upper_case_globals)]
pub const MIN_FRAGMENT_INTERPOLATION_OFFSET: u32 = 0x8E5B;
#[allow(non_upper_case_globals)]
pub const MIN_MAP_BUFFER_ALIGNMENT: u32 = 0x90BC;
#[allow(non_upper_case_globals)]
pub const MIN_PROGRAM_TEXEL_OFFSET: u32 = 0x8904;
#[allow(non_upper_case_globals)]
pub const MIN_PROGRAM_TEXTURE_GATHER_OFFSET: u32 = 0x8E5E;
#[allow(non_upper_case_globals)]
pub const MIN_SAMPLE_SHADING_VALUE: u32 = 0x8C37;
#[allow(non_upper_case_globals)]
pub const MIPMAP: u32 = 0x8293;
#[allow(non_upper_case_globals)]
pub const MIRRORED_REPEAT: u32 = 0x8370;
#[allow(non_upper_case_globals)]
pub const MIRROR_CLAMP_TO_EDGE: u32 = 0x8743;
#[allow(non_upper_case_globals)]
pub const MULTISAMPLE: u32 = 0x809D;
#[allow(non_upper_case_globals)]
pub const NAME_LENGTH: u32 = 0x92F9;
#[allow(non_upper_case_globals)]
pub const NAND: u32 = 0x150E;
#[allow(non_upper_case_globals)]
pub const NEAREST: u32 = 0x2600;
#[allow(non_upper_case_globals)]
pub const NEAREST_MIPMAP_LINEAR: u32 = 0x2702;
#[allow(non_upper_case_globals)]
pub const NEAREST_MIPMAP_NEAREST: u32 = 0x2700;
#[allow(non_upper_case_globals)]
pub const NEGATIVE_ONE_TO_ONE: u32 = 0x935E;
#[allow(non_upper_case_globals)]
pub const NEVER: u32 = 0x0200;
#[allow(non_upper_case_globals)]
pub const NICEST: u32 = 0x1102;
#[allow(non_upper_case_globals)]
pub const NONE: u32 = 0;
#[allow(non_upper_case_globals)]
pub const NOOP: u32 = 0x1505;
#[allow(non_upper_case_globals)]
pub const NOR: u32 = 0x1508;
#[allow(non_upper_case_globals)]
pub const NOTEQUAL: u32 = 0x0205;
#[allow(non_upper_case_globals)]
pub const NO_ERROR: u32 = 0;
#[allow(non_upper_case_globals)]
pub const NO_RESET_NOTIFICATION: u32 = 0x8261;
#[allow(non_upper_case_globals)]
pub const NUM_ACTIVE_VARIABLES: u32 = 0x9304;
#[allow(non_upper_case_globals)]
pub const NUM_COMPATIBLE_SUBROUTINES: u32 = 0x8E4A;
#[allow(non_upper_case_globals)]
pub const NUM_COMPRESSED_TEXTURE_FORMATS: u32 = 0x86A2;
#[allow(non_upper_case_globals)]
pub const NUM_EXTENSIONS: u32 = 0x821D;
#[allow(non_upper_case_globals)]
pub const NUM_PROGRAM_BINARY_FORMATS: u32 = 0x87FE;
#[allow(non_upper_case_globals)]
pub const NUM_SAMPLE_COUNTS: u32 = 0x9380;
#[allow(non_upper_case_globals)]
pub const NUM_SHADER_BINARY_FORMATS: u32 = 0x8DF9;
#[allow(non_upper_case_globals)]
pub const NUM_SHADING_LANGUAGE_VERSIONS: u32 = 0x82E9;
#[allow(non_upper_case_globals)]
pub const NUM_SPIR_V_EXTENSIONS: u32 = 0x9554;
#[allow(non_upper_case_globals)]
pub const OBJECT_TYPE: u32 = 0x9112;
#[allow(non_upper_case_globals)]
pub const OFFSET: u32 = 0x92FC;
#[allow(non_upper_case_globals)]
pub const ONE: u32 = 1;
#[allow(non_upper_case_globals)]
pub const ONE_MINUS_CONSTANT_ALPHA: u32 = 0x8004;
#[allow(non_upper_case_globals)]
pub const ONE_MINUS_CONSTANT_COLOR: u32 = 0x8002;
#[allow(non_upper_case_globals)]
pub const ONE_MINUS_DST_ALPHA: u32 = 0x0305;
#[allow(non_upper_case_globals)]
pub const ONE_MINUS_DST_COLOR: u32 = 0x0307;
#[allow(non_upper_case_globals)]
pub const ONE_MINUS_SRC1_ALPHA: u32 = 0x88FB;
#[allow(non_upper_case_globals)]
pub const ONE_MINUS_SRC1_COLOR: u32 = 0x88FA;
#[allow(non_upper_case_globals)]
pub const ONE_MINUS_SRC_ALPHA: u32 = 0x0303;
#[allow(non_upper_case_globals)]
pub const ONE_MINUS_SRC_COLOR: u32 = 0x0301;
#[allow(non_upper_case_globals)]
pub const OR: u32 = 0x1507;
#[allow(non_upper_case_globals)]
pub const OR_INVERTED: u32 = 0x150D;
#[allow(non_upper_case_globals)]
pub const OR_REVERSE: u32 = 0x150B;
#[allow(non_upper_case_globals)]
pub const OUT_OF_MEMORY: u32 = 0x0505;
#[allow(non_upper_case_globals)]
pub const PACK_ALIGNMENT: u32 = 0x0D05;
#[allow(non_upper_case_globals)]
pub const PACK_COMPRESSED_BLOCK_DEPTH: u32 = 0x912D;
#[allow(non_upper_case_globals)]
pub const PACK_COMPRESSED_BLOCK_HEIGHT: u32 = 0x912C;
#[allow(non_upper_case_globals)]
pub const PACK_COMPRESSED_BLOCK_SIZE: u32 = 0x912E;
#[allow(non_upper_case_globals)]
pub const PACK_COMPRESSED_BLOCK_WIDTH: u32 = 0x912B;
#[allow(non_upper_case_globals)]
pub const PACK_IMAGE_HEIGHT: u32 = 0x806C;
#[allow(non_upper_case_globals)]
pub const PACK_LSB_FIRST: u32 = 0x0D01;
#[allow(non_upper_case_globals)]
pub const PACK_ROW_LENGTH: u32 = 0x0D02;
#[allow(non_upper_case_globals)]
pub const PACK_SKIP_IMAGES: u32 = 0x806B;
#[allow(non_upper_case_globals)]
pub const PACK_SKIP_PIXELS: u32 = 0x0D04;
#[allow(non_upper_case_globals)]
pub const PACK_SKIP_ROWS: u32 = 0x0D03;
#[allow(non_upper_case_globals)]
pub const PACK_SWAP_BYTES: u32 = 0x0D00;
#[allow(non_upper_case_globals)]
pub const PARAMETER_BUFFER: u32 = 0x80EE;
#[allow(non_upper_case_globals)]
pub const PARAMETER_BUFFER_BINDING: u32 = 0x80EF;
#[allow(non_upper_case_globals)]
pub const PATCHES: u32 = 0x000E;
#[allow(non_upper_case_globals)]
pub const PATCH_DEFAULT_INNER_LEVEL: u32 = 0x8E73;
#[allow(non_upper_case_globals)]
pub const PATCH_DEFAULT_OUTER_LEVEL: u32 = 0x8E74;
#[allow(non_upper_case_globals)]
pub const PATCH_VERTICES: u32 = 0x8E72;
#[allow(non_upper_case_globals)]
pub const PIXEL_BUFFER_BARRIER_BIT: u32 = 0x00000080;
#[allow(non_upper_case_globals)]
pub const PIXEL_PACK_BUFFER: u32 = 0x88EB;
#[allow(non_upper_case_globals)]
pub const PIXEL_PACK_BUFFER_BINDING: u32 = 0x88ED;
#[allow(non_upper_case_globals)]
pub const PIXEL_UNPACK_BUFFER: u32 = 0x88EC;
#[allow(non_upper_case_globals)]
pub const PIXEL_UNPACK_BUFFER_BINDING: u32 = 0x88EF;
#[allow(non_upper_case_globals)]
pub const POINT: u32 = 0x1B00;
#[allow(non_upper_case_globals)]
pub const POINTS: u32 = 0x0000;
#[allow(non_upper_case_globals)]
pub const POINT_FADE_THRESHOLD_SIZE: u32 = 0x8128;
#[allow(non_upper_case_globals)]
pub const POINT_SIZE: u32 = 0x0B11;
#[allow(non_upper_case_globals)]
pub const POINT_SIZE_GRANULARITY: u32 = 0x0B13;
#[allow(non_upper_case_globals)]
pub const POINT_SIZE_RANGE: u32 = 0x0B12;
#[allow(non_upper_case_globals)]
pub const POINT_SPRITE_COORD_ORIGIN: u32 = 0x8CA0;
#[allow(non_upper_case_globals)]
pub const POLYGON_MODE: u32 = 0x0B40;
#[allow(non_upper_case_globals)]
pub const POLYGON_OFFSET_CLAMP: u32 = 0x8E1B;
#[allow(non_upper_case_globals)]
pub const POLYGON_OFFSET_FACTOR: u32 = 0x8038;
#[allow(non_upper_case_globals)]
pub const POLYGON_OFFSET_FILL: u32 = 0x8037;
#[allow(non_upper_case_globals)]
pub const POLYGON_OFFSET_LINE: u32 = 0x2A02;
#[allow(non_upper_case_globals)]
pub const POLYGON_OFFSET_POINT: u32 = 0x2A01;
#[allow(non_upper_case_globals)]
pub const POLYGON_OFFSET_UNITS: u32 = 0x2A00;
#[allow(non_upper_case_globals)]
pub const POLYGON_SMOOTH: u32 = 0x0B41;
#[allow(non_upper_case_globals)]
pub const POLYGON_SMOOTH_HINT: u32 = 0x0C53;
#[allow(non_upper_case_globals)]
pub const PRIMITIVES_GENERATED: u32 = 0x8C87;
#[allow(non_upper_case_globals)]
pub const PRIMITIVES_SUBMITTED: u32 = 0x82EF;
#[allow(non_upper_case_globals)]
pub const PRIMITIVE_RESTART: u32 = 0x8F9D;
#[allow(non_upper_case_globals)]
pub const PRIMITIVE_RESTART_FIXED_INDEX: u32 = 0x8D69;
#[allow(non_upper_case_globals)]
pub const PRIMITIVE_RESTART_FOR_PATCHES_SUPPORTED: u32 = 0x8221;
#[allow(non_upper_case_globals)]
pub const PRIMITIVE_RESTART_INDEX: u32 = 0x8F9E;
#[allow(non_upper_case_globals)]
pub const PROGRAM: u32 = 0x82E2;
#[allow(non_upper_case_globals)]
pub const PROGRAM_BINARY_FORMATS: u32 = 0x87FF;
#[allow(non_upper_case_globals)]
pub const PROGRAM_BINARY_LENGTH: u32 = 0x8741;
#[allow(non_upper_case_globals)]
pub const PROGRAM_BINARY_RETRIEVABLE_HINT: u32 = 0x8257;
#[allow(non_upper_case_globals)]
pub const PROGRAM_INPUT: u32 = 0x92E3;
#[allow(non_upper_case_globals)]
pub const PROGRAM_OUTPUT: u32 = 0x92E4;
#[allow(non_upper_case_globals)]
pub const PROGRAM_PIPELINE: u32 = 0x82E4;
#[allow(non_upper_case_globals)]
pub const PROGRAM_PIPELINE_BINDING: u32 = 0x825A;
#[allow(non_upper_case_globals)]
pub const PROGRAM_POINT_SIZE: u32 = 0x8642;
#[allow(non_upper_case_globals)]
pub const PROGRAM_SEPARABLE: u32 = 0x8258;
#[allow(non_upper_case_globals)]
pub const PROVOKING_VERTEX: u32 = 0x8E4F;
#[allow(non_upper_case_globals)]
pub const PROXY_TEXTURE_1D: u32 = 0x8063;
#[allow(non_upper_case_globals)]
pub const PROXY_TEXTURE_1D_ARRAY: u32 = 0x8C19;
#[allow(non_upper_case_globals)]
pub const PROXY_TEXTURE_2D: u32 = 0x8064;
#[allow(non_upper_case_globals)]
pub const PROXY_TEXTURE_2D_ARRAY: u32 = 0x8C1B;
#[allow(non_upper_case_globals)]
pub const PROXY_TEXTURE_2D_MULTISAMPLE: u32 = 0x9101;
#[allow(non_upper_case_globals)]
pub const PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: u32 = 0x9103;
#[allow(non_upper_case_globals)]
pub const PROXY_TEXTURE_3D: u32 = 0x8070;
#[allow(non_upper_case_globals)]
pub const PROXY_TEXTURE_CUBE_MAP: u32 = 0x851B;
#[allow(non_upper_case_globals)]
pub const PROXY_TEXTURE_CUBE_MAP_ARRAY: u32 = 0x900B;
#[allow(non_upper_case_globals)]
pub const PROXY_TEXTURE_RECTANGLE: u32 = 0x84F7;
#[allow(non_upper_case_globals)]
pub const QUADS: u32 = 0x0007;
#[allow(non_upper_case_globals)]
pub const QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: u32 = 0x8E4C;
#[allow(non_upper_case_globals)]
pub const QUERY: u32 = 0x82E3;
#[allow(non_upper_case_globals)]
pub const QUERY_BUFFER: u32 = 0x9192;
#[allow(non_upper_case_globals)]
pub const QUERY_BUFFER_BARRIER_BIT: u32 = 0x00008000;
#[allow(non_upper_case_globals)]
pub const QUERY_BUFFER_BINDING: u32 = 0x9193;
#[allow(non_upper_case_globals)]
pub const QUERY_BY_REGION_NO_WAIT: u32 = 0x8E16;
#[allow(non_upper_case_globals)]
pub const QUERY_BY_REGION_NO_WAIT_INVERTED: u32 = 0x8E1A;
#[allow(non_upper_case_globals)]
pub const QUERY_BY_REGION_WAIT: u32 = 0x8E15;
#[allow(non_upper_case_globals)]
pub const QUERY_BY_REGION_WAIT_INVERTED: u32 = 0x8E19;
#[allow(non_upper_case_globals)]
pub const QUERY_COUNTER_BITS: u32 = 0x8864;
#[allow(non_upper_case_globals)]
pub const QUERY_NO_WAIT: u32 = 0x8E14;
#[allow(non_upper_case_globals)]
pub const QUERY_NO_WAIT_INVERTED: u32 = 0x8E18;
#[allow(non_upper_case_globals)]
pub const QUERY_RESULT: u32 = 0x8866;
#[allow(non_upper_case_globals)]
pub const QUERY_RESULT_AVAILABLE: u32 = 0x8867;
#[allow(non_upper_case_globals)]
pub const QUERY_RESULT_NO_WAIT: u32 = 0x9194;
#[allow(non_upper_case_globals)]
pub const QUERY_TARGET: u32 = 0x82EA;
#[allow(non_upper_case_globals)]
pub const QUERY_WAIT: u32 = 0x8E13;
#[allow(non_upper_case_globals)]
pub const QUERY_WAIT_INVERTED: u32 = 0x8E17;
#[allow(non_upper_case_globals)]
pub const R11F_G11F_B10F: u32 = 0x8C3A;
#[allow(non_upper_case_globals)]
pub const R16: u32 = 0x822A;
#[allow(non_upper_case_globals)]
pub const R16F: u32 = 0x822D;
#[allow(non_upper_case_globals)]
pub const R16I: u32 = 0x8233;
#[allow(non_upper_case_globals)]
pub const R16UI: u32 = 0x8234;
#[allow(non_upper_case_globals)]
pub const R16_SNORM: u32 = 0x8F98;
#[allow(non_upper_case_globals)]
pub const R32F: u32 = 0x822E;
#[allow(non_upper_case_globals)]
pub const R32I: u32 = 0x8235;
#[allow(non_upper_case_globals)]
pub const R32UI: u32 = 0x8236;
#[allow(non_upper_case_globals)]
pub const R3_G3_B2: u32 = 0x2A10;
#[allow(non_upper_case_globals)]
pub const R8: u32 = 0x8229;
#[allow(non_upper_case_globals)]
pub const R8I: u32 = 0x8231;
#[allow(non_upper_case_globals)]
pub const R8UI: u32 = 0x8232;
#[allow(non_upper_case_globals)]
pub const R8_SNORM: u32 = 0x8F94;
#[allow(non_upper_case_globals)]
pub const RASTERIZER_DISCARD: u32 = 0x8C89;
#[allow(non_upper_case_globals)]
pub const READ_BUFFER: u32 = 0x0C02;
#[allow(non_upper_case_globals)]
pub const READ_FRAMEBUFFER: u32 = 0x8CA8;
#[allow(non_upper_case_globals)]
pub const READ_FRAMEBUFFER_BINDING: u32 = 0x8CAA;
#[allow(non_upper_case_globals)]
pub const READ_ONLY: u32 = 0x88B8;
#[allow(non_upper_case_globals)]
pub const READ_PIXELS: u32 = 0x828C;
#[allow(non_upper_case_globals)]
pub const READ_PIXELS_FORMAT: u32 = 0x828D;
#[allow(non_upper_case_globals)]
pub const READ_PIXELS_TYPE: u32 = 0x828E;
#[allow(non_upper_case_globals)]
pub const READ_WRITE: u32 = 0x88BA;
#[allow(non_upper_case_globals)]
pub const RED: u32 = 0x1903;
#[allow(non_upper_case_globals)]
pub const RED_INTEGER: u32 = 0x8D94;
#[allow(non_upper_case_globals)]
pub const REFERENCED_BY_COMPUTE_SHADER: u32 = 0x930B;
#[allow(non_upper_case_globals)]
pub const REFERENCED_BY_FRAGMENT_SHADER: u32 = 0x930A;
#[allow(non_upper_case_globals)]
pub const REFERENCED_BY_GEOMETRY_SHADER: u32 = 0x9309;
#[allow(non_upper_case_globals)]
pub const REFERENCED_BY_TESS_CONTROL_SHADER: u32 = 0x9307;
#[allow(non_upper_case_globals)]
pub const REFERENCED_BY_TESS_EVALUATION_SHADER: u32 = 0x9308;
#[allow(non_upper_case_globals)]
pub const REFERENCED_BY_VERTEX_SHADER: u32 = 0x9306;
#[allow(non_upper_case_globals)]
pub const RENDERBUFFER: u32 = 0x8D41;
#[allow(non_upper_case_globals)]
pub const RENDERBUFFER_ALPHA_SIZE: u32 = 0x8D53;
#[allow(non_upper_case_globals)]
pub const RENDERBUFFER_BINDING: u32 = 0x8CA7;
#[allow(non_upper_case_globals)]
pub const RENDERBUFFER_BLUE_SIZE: u32 = 0x8D52;
#[allow(non_upper_case_globals)]
pub const RENDERBUFFER_DEPTH_SIZE: u32 = 0x8D54;
#[allow(non_upper_case_globals)]
pub const RENDERBUFFER_GREEN_SIZE: u32 = 0x8D51;
#[allow(non_upper_case_globals)]
pub const RENDERBUFFER_HEIGHT: u32 = 0x8D43;
#[allow(non_upper_case_globals)]
pub const RENDERBUFFER_INTERNAL_FORMAT: u32 = 0x8D44;
#[allow(non_upper_case_globals)]
pub const RENDERBUFFER_RED_SIZE: u32 = 0x8D50;
#[allow(non_upper_case_globals)]
pub const RENDERBUFFER_SAMPLES: u32 = 0x8CAB;
#[allow(non_upper_case_globals)]
pub const RENDERBUFFER_STENCIL_SIZE: u32 = 0x8D55;
#[allow(non_upper_case_globals)]
pub const RENDERBUFFER_WIDTH: u32 = 0x8D42;
#[allow(non_upper_case_globals)]
pub const RENDERER: u32 = 0x1F01;
#[allow(non_upper_case_globals)]
pub const REPEAT: u32 = 0x2901;
#[allow(non_upper_case_globals)]
pub const REPLACE: u32 = 0x1E01;
#[allow(non_upper_case_globals)]
pub const RESET_NOTIFICATION_STRATEGY: u32 = 0x8256;
#[allow(non_upper_case_globals)]
pub const RG: u32 = 0x8227;
#[allow(non_upper_case_globals)]
pub const RG16: u32 = 0x822C;
#[allow(non_upper_case_globals)]
pub const RG16F: u32 = 0x822F;
#[allow(non_upper_case_globals)]
pub const RG16I: u32 = 0x8239;
#[allow(non_upper_case_globals)]
pub const RG16UI: u32 = 0x823A;
#[allow(non_upper_case_globals)]
pub const RG16_SNORM: u32 = 0x8F99;
#[allow(non_upper_case_globals)]
pub const RG32F: u32 = 0x8230;
#[allow(non_upper_case_globals)]
pub const RG32I: u32 = 0x823B;
#[allow(non_upper_case_globals)]
pub const RG32UI: u32 = 0x823C;
#[allow(non_upper_case_globals)]
pub const RG8: u32 = 0x822B;
#[allow(non_upper_case_globals)]
pub const RG8I: u32 = 0x8237;
#[allow(non_upper_case_globals)]
pub const RG8UI: u32 = 0x8238;
#[allow(non_upper_case_globals)]
pub const RG8_SNORM: u32 = 0x8F95;
#[allow(non_upper_case_globals)]
pub const RGB: u32 = 0x1907;
#[allow(non_upper_case_globals)]
pub const RGB10: u32 = 0x8052;
#[allow(non_upper_case_globals)]
pub const RGB10_A2: u32 = 0x8059;
#[allow(non_upper_case_globals)]
pub const RGB10_A2UI: u32 = 0x906F;
#[allow(non_upper_case_globals)]
pub const RGB12: u32 = 0x8053;
#[allow(non_upper_case_globals)]
pub const RGB16: u32 = 0x8054;
#[allow(non_upper_case_globals)]
pub const RGB16F: u32 = 0x881B;
#[allow(non_upper_case_globals)]
pub const RGB16I: u32 = 0x8D89;
#[allow(non_upper_case_globals)]
pub const RGB16UI: u32 = 0x8D77;
#[allow(non_upper_case_globals)]
pub const RGB16_SNORM: u32 = 0x8F9A;
#[allow(non_upper_case_globals)]
pub const RGB32F: u32 = 0x8815;
#[allow(non_upper_case_globals)]
pub const RGB32I: u32 = 0x8D83;
#[allow(non_upper_case_globals)]
pub const RGB32UI: u32 = 0x8D71;
#[allow(non_upper_case_globals)]
pub const RGB4: u32 = 0x804F;
#[allow(non_upper_case_globals)]
pub const RGB5: u32 = 0x8050;
#[allow(non_upper_case_globals)]
pub const RGB565: u32 = 0x8D62;
#[allow(non_upper_case_globals)]
pub const RGB5_A1: u32 = 0x8057;
#[allow(non_upper_case_globals)]
pub const RGB8: u32 = 0x8051;
#[allow(non_upper_case_globals)]
pub const RGB8I: u32 = 0x8D8F;
#[allow(non_upper_case_globals)]
pub const RGB8UI: u32 = 0x8D7D;
#[allow(non_upper_case_globals)]
pub const RGB8_SNORM: u32 = 0x8F96;
#[allow(non_upper_case_globals)]
pub const RGB9_E5: u32 = 0x8C3D;
#[allow(non_upper_case_globals)]
pub const RGBA: u32 = 0x1908;
#[allow(non_upper_case_globals)]
pub const RGBA12: u32 = 0x805A;
#[allow(non_upper_case_globals)]
pub const RGBA16: u32 = 0x805B;
#[allow(non_upper_case_globals)]
pub const RGBA16F: u32 = 0x881A;
#[allow(non_upper_case_globals)]
pub const RGBA16I: u32 = 0x8D88;
#[allow(non_upper_case_globals)]
pub const RGBA16UI: u32 = 0x8D76;
#[allow(non_upper_case_globals)]
pub const RGBA16_SNORM: u32 = 0x8F9B;
#[allow(non_upper_case_globals)]
pub const RGBA2: u32 = 0x8055;
#[allow(non_upper_case_globals)]
pub const RGBA32F: u32 = 0x8814;
#[allow(non_upper_case_globals)]
pub const RGBA32I: u32 = 0x8D82;
#[allow(non_upper_case_globals)]
pub const RGBA32UI: u32 = 0x8D70;
#[allow(non_upper_case_globals)]
pub const RGBA4: u32 = 0x8056;
#[allow(non_upper_case_globals)]
pub const RGBA8: u32 = 0x8058;
#[allow(non_upper_case_globals)]
pub const RGBA8I: u32 = 0x8D8E;
#[allow(non_upper_case_globals)]
pub const RGBA8UI: u32 = 0x8D7C;
#[allow(non_upper_case_globals)]
pub const RGBA8_SNORM: u32 = 0x8F97;
#[allow(non_upper_case_globals)]
pub const RGBA_INTEGER: u32 = 0x8D99;
#[allow(non_upper_case_globals)]
pub const RGB_INTEGER: u32 = 0x8D98;
#[allow(non_upper_case_globals)]
pub const RG_INTEGER: u32 = 0x8228;
#[allow(non_upper_case_globals)]
pub const RIGHT: u32 = 0x0407;
#[allow(non_upper_case_globals)]
pub const SAMPLER: u32 = 0x82E6;
#[allow(non_upper_case_globals)]
pub const SAMPLER_1D: u32 = 0x8B5D;
#[allow(non_upper_case_globals)]
pub const SAMPLER_1D_ARRAY: u32 = 0x8DC0;
#[allow(non_upper_case_globals)]
pub const SAMPLER_1D_ARRAY_SHADOW: u32 = 0x8DC3;
#[allow(non_upper_case_globals)]
pub const SAMPLER_1D_SHADOW: u32 = 0x8B61;
#[allow(non_upper_case_globals)]
pub const SAMPLER_2D: u32 = 0x8B5E;
#[allow(non_upper_case_globals)]
pub const SAMPLER_2D_ARRAY: u32 = 0x8DC1;
#[allow(non_upper_case_globals)]
pub const SAMPLER_2D_ARRAY_SHADOW: u32 = 0x8DC4;
#[allow(non_upper_case_globals)]
pub const SAMPLER_2D_MULTISAMPLE: u32 = 0x9108;
#[allow(non_upper_case_globals)]
pub const SAMPLER_2D_MULTISAMPLE_ARRAY: u32 = 0x910B;
#[allow(non_upper_case_globals)]
pub const SAMPLER_2D_RECT: u32 = 0x8B63;
#[allow(non_upper_case_globals)]
pub const SAMPLER_2D_RECT_SHADOW: u32 = 0x8B64;
#[allow(non_upper_case_globals)]
pub const SAMPLER_2D_SHADOW: u32 = 0x8B62;
#[allow(non_upper_case_globals)]
pub const SAMPLER_3D: u32 = 0x8B5F;
#[allow(non_upper_case_globals)]
pub const SAMPLER_BINDING: u32 = 0x8919;
#[allow(non_upper_case_globals)]
pub const SAMPLER_BUFFER: u32 = 0x8DC2;
#[allow(non_upper_case_globals)]
pub const SAMPLER_CUBE: u32 = 0x8B60;
#[allow(non_upper_case_globals)]
pub const SAMPLER_CUBE_MAP_ARRAY: u32 = 0x900C;
#[allow(non_upper_case_globals)]
pub const SAMPLER_CUBE_MAP_ARRAY_SHADOW: u32 = 0x900D;
#[allow(non_upper_case_globals)]
pub const SAMPLER_CUBE_SHADOW: u32 = 0x8DC5;
#[allow(non_upper_case_globals)]
pub const SAMPLES: u32 = 0x80A9;
#[allow(non_upper_case_globals)]
pub const SAMPLES_PASSED: u32 = 0x8914;
#[allow(non_upper_case_globals)]
pub const SAMPLE_ALPHA_TO_COVERAGE: u32 = 0x809E;
#[allow(non_upper_case_globals)]
pub const SAMPLE_ALPHA_TO_ONE: u32 = 0x809F;
#[allow(non_upper_case_globals)]
pub const SAMPLE_BUFFERS: u32 = 0x80A8;
#[allow(non_upper_case_globals)]
pub const SAMPLE_COVERAGE: u32 = 0x80A0;
#[allow(non_upper_case_globals)]
pub const SAMPLE_COVERAGE_INVERT: u32 = 0x80AB;
#[allow(non_upper_case_globals)]
pub const SAMPLE_COVERAGE_VALUE: u32 = 0x80AA;
#[allow(non_upper_case_globals)]
pub const SAMPLE_MASK: u32 = 0x8E51;
#[allow(non_upper_case_globals)]
pub const SAMPLE_MASK_VALUE: u32 = 0x8E52;
#[allow(non_upper_case_globals)]
pub const SAMPLE_POSITION: u32 = 0x8E50;
#[allow(non_upper_case_globals)]
pub const SAMPLE_SHADING: u32 = 0x8C36;
#[allow(non_upper_case_globals)]
pub const SCISSOR_BOX: u32 = 0x0C10;
#[allow(non_upper_case_globals)]
pub const SCISSOR_TEST: u32 = 0x0C11;
#[allow(non_upper_case_globals)]
pub const SEPARATE_ATTRIBS: u32 = 0x8C8D;
#[allow(non_upper_case_globals)]
pub const SET: u32 = 0x150F;
#[allow(non_upper_case_globals)]
pub const SHADER: u32 = 0x82E1;
#[allow(non_upper_case_globals)]
pub const SHADER_BINARY_FORMATS: u32 = 0x8DF8;
#[allow(non_upper_case_globals)]
pub const SHADER_BINARY_FORMAT_SPIR_V: u32 = 0x9551;
#[allow(non_upper_case_globals)]
pub const SHADER_COMPILER: u32 = 0x8DFA;
#[allow(non_upper_case_globals)]
pub const SHADER_IMAGE_ACCESS_BARRIER_BIT: u32 = 0x00000020;
#[allow(non_upper_case_globals)]
pub const SHADER_IMAGE_ATOMIC: u32 = 0x82A6;
#[allow(non_upper_case_globals)]
pub const SHADER_IMAGE_LOAD: u32 = 0x82A4;
#[allow(non_upper_case_globals)]
pub const SHADER_IMAGE_STORE: u32 = 0x82A5;
#[allow(non_upper_case_globals)]
pub const SHADER_SOURCE_LENGTH: u32 = 0x8B88;
#[allow(non_upper_case_globals)]
pub const SHADER_STORAGE_BARRIER_BIT: u32 = 0x00002000;
#[allow(non_upper_case_globals)]
pub const SHADER_STORAGE_BLOCK: u32 = 0x92E6;
#[allow(non_upper_case_globals)]
pub const SHADER_STORAGE_BUFFER: u32 = 0x90D2;
#[allow(non_upper_case_globals)]
pub const SHADER_STORAGE_BUFFER_BINDING: u32 = 0x90D3;
#[allow(non_upper_case_globals)]
pub const SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT: u32 = 0x90DF;
#[allow(non_upper_case_globals)]
pub const SHADER_STORAGE_BUFFER_SIZE: u32 = 0x90D5;
#[allow(non_upper_case_globals)]
pub const SHADER_STORAGE_BUFFER_START: u32 = 0x90D4;
#[allow(non_upper_case_globals)]
pub const SHADER_TYPE: u32 = 0x8B4F;
#[allow(non_upper_case_globals)]
pub const SHADING_LANGUAGE_VERSION: u32 = 0x8B8C;
#[allow(non_upper_case_globals)]
pub const SHORT: u32 = 0x1402;
#[allow(non_upper_case_globals)]
pub const SIGNALED: u32 = 0x9119;
#[allow(non_upper_case_globals)]
pub const SIGNED_NORMALIZED: u32 = 0x8F9C;
#[allow(non_upper_case_globals)]
pub const SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST: u32 = 0x82AC;
#[allow(non_upper_case_globals)]
pub const SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE: u32 = 0x82AE;
#[allow(non_upper_case_globals)]
pub const SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST: u32 = 0x82AD;
#[allow(non_upper_case_globals)]
pub const SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE: u32 = 0x82AF;
#[allow(non_upper_case_globals)]
pub const SMOOTH_LINE_WIDTH_GRANULARITY: u32 = 0x0B23;
#[allow(non_upper_case_globals)]
pub const SMOOTH_LINE_WIDTH_RANGE: u32 = 0x0B22;
#[allow(non_upper_case_globals)]
pub const SMOOTH_POINT_SIZE_GRANULARITY: u32 = 0x0B13;
#[allow(non_upper_case_globals)]
pub const SMOOTH_POINT_SIZE_RANGE: u32 = 0x0B12;
#[allow(non_upper_case_globals)]
pub const SPIR_V_BINARY: u32 = 0x9552;
#[allow(non_upper_case_globals)]
pub const SPIR_V_EXTENSIONS: u32 = 0x9553;
#[allow(non_upper_case_globals)]
pub const SRC1_ALPHA: u32 = 0x8589;
#[allow(non_upper_case_globals)]
pub const SRC1_COLOR: u32 = 0x88F9;
#[allow(non_upper_case_globals)]
pub const SRC_ALPHA: u32 = 0x0302;
#[allow(non_upper_case_globals)]
pub const SRC_ALPHA_SATURATE: u32 = 0x0308;
#[allow(non_upper_case_globals)]
pub const SRC_COLOR: u32 = 0x0300;
#[allow(non_upper_case_globals)]
pub const SRGB: u32 = 0x8C40;
#[allow(non_upper_case_globals)]
pub const SRGB8: u32 = 0x8C41;
#[allow(non_upper_case_globals)]
pub const SRGB8_ALPHA8: u32 = 0x8C43;
#[allow(non_upper_case_globals)]
pub const SRGB_ALPHA: u32 = 0x8C42;
#[allow(non_upper_case_globals)]
pub const SRGB_READ: u32 = 0x8297;
#[allow(non_upper_case_globals)]
pub const SRGB_WRITE: u32 = 0x8298;
#[allow(non_upper_case_globals)]
pub const STACK_OVERFLOW: u32 = 0x0503;
#[allow(non_upper_case_globals)]
pub const STACK_UNDERFLOW: u32 = 0x0504;
#[allow(non_upper_case_globals)]
pub const STATIC_COPY: u32 = 0x88E6;
#[allow(non_upper_case_globals)]
pub const STATIC_DRAW: u32 = 0x88E4;
#[allow(non_upper_case_globals)]
pub const STATIC_READ: u32 = 0x88E5;
#[allow(non_upper_case_globals)]
pub const STENCIL: u32 = 0x1802;
#[allow(non_upper_case_globals)]
pub const STENCIL_ATTACHMENT: u32 = 0x8D20;
#[allow(non_upper_case_globals)]
pub const STENCIL_BACK_FAIL: u32 = 0x8801;
#[allow(non_upper_case_globals)]
pub const STENCIL_BACK_FUNC: u32 = 0x8800;
#[allow(non_upper_case_globals)]
pub const STENCIL_BACK_PASS_DEPTH_FAIL: u32 = 0x8802;
#[allow(non_upper_case_globals)]
pub const STENCIL_BACK_PASS_DEPTH_PASS: u32 = 0x8803;
#[allow(non_upper_case_globals)]
pub const STENCIL_BACK_REF: u32 = 0x8CA3;
#[allow(non_upper_case_globals)]
pub const STENCIL_BACK_VALUE_MASK: u32 = 0x8CA4;
#[allow(non_upper_case_globals)]
pub const STENCIL_BACK_WRITEMASK: u32 = 0x8CA5;
#[allow(non_upper_case_globals)]
pub const STENCIL_BUFFER_BIT: u32 = 0x00000400;
#[allow(non_upper_case_globals)]
pub const STENCIL_CLEAR_VALUE: u32 = 0x0B91;
#[allow(non_upper_case_globals)]
pub const STENCIL_COMPONENTS: u32 = 0x8285;
#[allow(non_upper_case_globals)]
pub const STENCIL_FAIL: u32 = 0x0B94;
#[allow(non_upper_case_globals)]
pub const STENCIL_FUNC: u32 = 0x0B92;
#[allow(non_upper_case_globals)]
pub const STENCIL_INDEX: u32 = 0x1901;
#[allow(non_upper_case_globals)]
pub const STENCIL_INDEX1: u32 = 0x8D46;
#[allow(non_upper_case_globals)]
pub const STENCIL_INDEX16: u32 = 0x8D49;
#[allow(non_upper_case_globals)]
pub const STENCIL_INDEX4: u32 = 0x8D47;
#[allow(non_upper_case_globals)]
pub const STENCIL_INDEX8: u32 = 0x8D48;
#[allow(non_upper_case_globals)]
pub const STENCIL_PASS_DEPTH_FAIL: u32 = 0x0B95;
#[allow(non_upper_case_globals)]
pub const STENCIL_PASS_DEPTH_PASS: u32 = 0x0B96;
#[allow(non_upper_case_globals)]
pub const STENCIL_REF: u32 = 0x0B97;
#[allow(non_upper_case_globals)]
pub const STENCIL_RENDERABLE: u32 = 0x8288;
#[allow(non_upper_case_globals)]
pub const STENCIL_TEST: u32 = 0x0B90;
#[allow(non_upper_case_globals)]
pub const STENCIL_VALUE_MASK: u32 = 0x0B93;
#[allow(non_upper_case_globals)]
pub const STENCIL_WRITEMASK: u32 = 0x0B98;
#[allow(non_upper_case_globals)]
pub const STEREO: u32 = 0x0C33;
#[allow(non_upper_case_globals)]
pub const STREAM_COPY: u32 = 0x88E2;
#[allow(non_upper_case_globals)]
pub const STREAM_DRAW: u32 = 0x88E0;
#[allow(non_upper_case_globals)]
pub const STREAM_READ: u32 = 0x88E1;
#[allow(non_upper_case_globals)]
pub const SUBPIXEL_BITS: u32 = 0x0D50;
#[allow(non_upper_case_globals)]
pub const SYNC_CONDITION: u32 = 0x9113;
#[allow(non_upper_case_globals)]
pub const SYNC_FENCE: u32 = 0x9116;
#[allow(non_upper_case_globals)]
pub const SYNC_FLAGS: u32 = 0x9115;
#[allow(non_upper_case_globals)]
pub const SYNC_FLUSH_COMMANDS_BIT: u32 = 0x00000001;
#[allow(non_upper_case_globals)]
pub const SYNC_GPU_COMMANDS_COMPLETE: u32 = 0x9117;
#[allow(non_upper_case_globals)]
pub const SYNC_STATUS: u32 = 0x9114;
#[allow(non_upper_case_globals)]
pub const TESS_CONTROL_OUTPUT_VERTICES: u32 = 0x8E75;
#[allow(non_upper_case_globals)]
pub const TESS_CONTROL_SHADER: u32 = 0x8E88;
#[allow(non_upper_case_globals)]
pub const TESS_CONTROL_SHADER_BIT: u32 = 0x00000008;
#[allow(non_upper_case_globals)]
pub const TESS_CONTROL_SHADER_PATCHES: u32 = 0x82F1;
#[allow(non_upper_case_globals)]
pub const TESS_CONTROL_SUBROUTINE: u32 = 0x92E9;
#[allow(non_upper_case_globals)]
pub const TESS_CONTROL_SUBROUTINE_UNIFORM: u32 = 0x92EF;
#[allow(non_upper_case_globals)]
pub const TESS_CONTROL_TEXTURE: u32 = 0x829C;
#[allow(non_upper_case_globals)]
pub const TESS_EVALUATION_SHADER: u32 = 0x8E87;
#[allow(non_upper_case_globals)]
pub const TESS_EVALUATION_SHADER_BIT: u32 = 0x00000010;
#[allow(non_upper_case_globals)]
pub const TESS_EVALUATION_SHADER_INVOCATIONS: u32 = 0x82F2;
#[allow(non_upper_case_globals)]
pub const TESS_EVALUATION_SUBROUTINE: u32 = 0x92EA;
#[allow(non_upper_case_globals)]
pub const TESS_EVALUATION_SUBROUTINE_UNIFORM: u32 = 0x92F0;
#[allow(non_upper_case_globals)]
pub const TESS_EVALUATION_TEXTURE: u32 = 0x829D;
#[allow(non_upper_case_globals)]
pub const TESS_GEN_MODE: u32 = 0x8E76;
#[allow(non_upper_case_globals)]
pub const TESS_GEN_POINT_MODE: u32 = 0x8E79;
#[allow(non_upper_case_globals)]
pub const TESS_GEN_SPACING: u32 = 0x8E77;
#[allow(non_upper_case_globals)]
pub const TESS_GEN_VERTEX_ORDER: u32 = 0x8E78;
#[allow(non_upper_case_globals)]
pub const TEXTURE: u32 = 0x1702;
#[allow(non_upper_case_globals)]
pub const TEXTURE0: u32 = 0x84C0;
#[allow(non_upper_case_globals)]
pub const TEXTURE1: u32 = 0x84C1;
#[allow(non_upper_case_globals)]
pub const TEXTURE10: u32 = 0x84CA;
#[allow(non_upper_case_globals)]
pub const TEXTURE11: u32 = 0x84CB;
#[allow(non_upper_case_globals)]
pub const TEXTURE12: u32 = 0x84CC;
#[allow(non_upper_case_globals)]
pub const TEXTURE13: u32 = 0x84CD;
#[allow(non_upper_case_globals)]
pub const TEXTURE14: u32 = 0x84CE;
#[allow(non_upper_case_globals)]
pub const TEXTURE15: u32 = 0x84CF;
#[allow(non_upper_case_globals)]
pub const TEXTURE16: u32 = 0x84D0;
#[allow(non_upper_case_globals)]
pub const TEXTURE17: u32 = 0x84D1;
#[allow(non_upper_case_globals)]
pub const TEXTURE18: u32 = 0x84D2;
#[allow(non_upper_case_globals)]
pub const TEXTURE19: u32 = 0x84D3;
#[allow(non_upper_case_globals)]
pub const TEXTURE2: u32 = 0x84C2;
#[allow(non_upper_case_globals)]
pub const TEXTURE20: u32 = 0x84D4;
#[allow(non_upper_case_globals)]
pub const TEXTURE21: u32 = 0x84D5;
#[allow(non_upper_case_globals)]
pub const TEXTURE22: u32 = 0x84D6;
#[allow(non_upper_case_globals)]
pub const TEXTURE23: u32 = 0x84D7;
#[allow(non_upper_case_globals)]
pub const TEXTURE24: u32 = 0x84D8;
#[allow(non_upper_case_globals)]
pub const TEXTURE25: u32 = 0x84D9;
#[allow(non_upper_case_globals)]
pub const TEXTURE26: u32 = 0x84DA;
#[allow(non_upper_case_globals)]
pub const TEXTURE27: u32 = 0x84DB;
#[allow(non_upper_case_globals)]
pub const TEXTURE28: u32 = 0x84DC;
#[allow(non_upper_case_globals)]
pub const TEXTURE29: u32 = 0x84DD;
#[allow(non_upper_case_globals)]
pub const TEXTURE3: u32 = 0x84C3;
#[allow(non_upper_case_globals)]
pub const TEXTURE30: u32 = 0x84DE;
#[allow(non_upper_case_globals)]
pub const TEXTURE31: u32 = 0x84DF;
#[allow(non_upper_case_globals)]
pub const TEXTURE4: u32 = 0x84C4;
#[allow(non_upper_case_globals)]
pub const TEXTURE5: u32 = 0x84C5;
#[allow(non_upper_case_globals)]
pub const TEXTURE6: u32 = 0x84C6;
#[allow(non_upper_case_globals)]
pub const TEXTURE7: u32 = 0x84C7;
#[allow(non_upper_case_globals)]
pub const TEXTURE8: u32 = 0x84C8;
#[allow(non_upper_case_globals)]
pub const TEXTURE9: u32 = 0x84C9;
#[allow(non_upper_case_globals)]
pub const TEXTURE_1D: u32 = 0x0DE0;
#[allow(non_upper_case_globals)]
pub const TEXTURE_1D_ARRAY: u32 = 0x8C18;
#[allow(non_upper_case_globals)]
pub const TEXTURE_2D: u32 = 0x0DE1;
#[allow(non_upper_case_globals)]
pub const TEXTURE_2D_ARRAY: u32 = 0x8C1A;
#[allow(non_upper_case_globals)]
pub const TEXTURE_2D_MULTISAMPLE: u32 = 0x9100;
#[allow(non_upper_case_globals)]
pub const TEXTURE_2D_MULTISAMPLE_ARRAY: u32 = 0x9102;
#[allow(non_upper_case_globals)]
pub const TEXTURE_3D: u32 = 0x806F;
#[allow(non_upper_case_globals)]
pub const TEXTURE_ALPHA_SIZE: u32 = 0x805F;
#[allow(non_upper_case_globals)]
pub const TEXTURE_ALPHA_TYPE: u32 = 0x8C13;
#[allow(non_upper_case_globals)]
pub const TEXTURE_BASE_LEVEL: u32 = 0x813C;
#[allow(non_upper_case_globals)]
pub const TEXTURE_BINDING_1D: u32 = 0x8068;
#[allow(non_upper_case_globals)]
pub const TEXTURE_BINDING_1D_ARRAY: u32 = 0x8C1C;
#[allow(non_upper_case_globals)]
pub const TEXTURE_BINDING_2D: u32 = 0x8069;
#[allow(non_upper_case_globals)]
pub const TEXTURE_BINDING_2D_ARRAY: u32 = 0x8C1D;
#[allow(non_upper_case_globals)]
pub const TEXTURE_BINDING_2D_MULTISAMPLE: u32 = 0x9104;
#[allow(non_upper_case_globals)]
pub const TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: u32 = 0x9105;
#[allow(non_upper_case_globals)]
pub const TEXTURE_BINDING_3D: u32 = 0x806A;
#[allow(non_upper_case_globals)]
pub const TEXTURE_BINDING_BUFFER: u32 = 0x8C2C;
#[allow(non_upper_case_globals)]
pub const TEXTURE_BINDING_CUBE_MAP: u32 = 0x8514;
#[allow(non_upper_case_globals)]
pub const TEXTURE_BINDING_CUBE_MAP_ARRAY: u32 = 0x900A;
#[allow(non_upper_case_globals)]
pub const TEXTURE_BINDING_RECTANGLE: u32 = 0x84F6;
#[allow(non_upper_case_globals)]
pub const TEXTURE_BLUE_SIZE: u32 = 0x805E;
#[allow(non_upper_case_globals)]
pub const TEXTURE_BLUE_TYPE: u32 = 0x8C12;
#[allow(non_upper_case_globals)]
pub const TEXTURE_BORDER_COLOR: u32 = 0x1004;
#[allow(non_upper_case_globals)]
pub const TEXTURE_BUFFER: u32 = 0x8C2A;
#[allow(non_upper_case_globals)]
pub const TEXTURE_BUFFER_BINDING: u32 = 0x8C2A;
#[allow(non_upper_case_globals)]
pub const TEXTURE_BUFFER_DATA_STORE_BINDING: u32 = 0x8C2D;
#[allow(non_upper_case_globals)]
pub const TEXTURE_BUFFER_OFFSET: u32 = 0x919D;
#[allow(non_upper_case_globals)]
pub const TEXTURE_BUFFER_OFFSET_ALIGNMENT: u32 = 0x919F;
#[allow(non_upper_case_globals)]
pub const TEXTURE_BUFFER_SIZE: u32 = 0x919E;
#[allow(non_upper_case_globals)]
pub const TEXTURE_COMPARE_FUNC: u32 = 0x884D;
#[allow(non_upper_case_globals)]
pub const TEXTURE_COMPARE_MODE: u32 = 0x884C;
#[allow(non_upper_case_globals)]
pub const TEXTURE_COMPRESSED: u32 = 0x86A1;
#[allow(non_upper_case_globals)]
pub const TEXTURE_COMPRESSED_BLOCK_HEIGHT: u32 = 0x82B2;
#[allow(non_upper_case_globals)]
pub const TEXTURE_COMPRESSED_BLOCK_SIZE: u32 = 0x82B3;
#[allow(non_upper_case_globals)]
pub const TEXTURE_COMPRESSED_BLOCK_WIDTH: u32 = 0x82B1;
#[allow(non_upper_case_globals)]
pub const TEXTURE_COMPRESSED_IMAGE_SIZE: u32 = 0x86A0;
#[allow(non_upper_case_globals)]
pub const TEXTURE_COMPRESSION_HINT: u32 = 0x84EF;
#[allow(non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP: u32 = 0x8513;
#[allow(non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP_ARRAY: u32 = 0x9009;
#[allow(non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP_NEGATIVE_X: u32 = 0x8516;
#[allow(non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: u32 = 0x8518;
#[allow(non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: u32 = 0x851A;
#[allow(non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP_POSITIVE_X: u32 = 0x8515;
#[allow(non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP_POSITIVE_Y: u32 = 0x8517;
#[allow(non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP_POSITIVE_Z: u32 = 0x8519;
#[allow(non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP_SEAMLESS: u32 = 0x884F;
#[allow(non_upper_case_globals)]
pub const TEXTURE_DEPTH: u32 = 0x8071;
#[allow(non_upper_case_globals)]
pub const TEXTURE_DEPTH_SIZE: u32 = 0x884A;
#[allow(non_upper_case_globals)]
pub const TEXTURE_DEPTH_TYPE: u32 = 0x8C16;
#[allow(non_upper_case_globals)]
pub const TEXTURE_FETCH_BARRIER_BIT: u32 = 0x00000008;
#[allow(non_upper_case_globals)]
pub const TEXTURE_FIXED_SAMPLE_LOCATIONS: u32 = 0x9107;
#[allow(non_upper_case_globals)]
pub const TEXTURE_GATHER: u32 = 0x82A2;
#[allow(non_upper_case_globals)]
pub const TEXTURE_GATHER_SHADOW: u32 = 0x82A3;
#[allow(non_upper_case_globals)]
pub const TEXTURE_GREEN_SIZE: u32 = 0x805D;
#[allow(non_upper_case_globals)]
pub const TEXTURE_GREEN_TYPE: u32 = 0x8C11;
#[allow(non_upper_case_globals)]
pub const TEXTURE_HEIGHT: u32 = 0x1001;
#[allow(non_upper_case_globals)]
pub const TEXTURE_IMAGE_FORMAT: u32 = 0x828F;
#[allow(non_upper_case_globals)]
pub const TEXTURE_IMAGE_TYPE: u32 = 0x8290;
#[allow(non_upper_case_globals)]
pub const TEXTURE_IMMUTABLE_FORMAT: u32 = 0x912F;
#[allow(non_upper_case_globals)]
pub const TEXTURE_IMMUTABLE_LEVELS: u32 = 0x82DF;
#[allow(non_upper_case_globals)]
pub const TEXTURE_INTERNAL_FORMAT: u32 = 0x1003;
#[allow(non_upper_case_globals)]
pub const TEXTURE_LOD_BIAS: u32 = 0x8501;
#[allow(non_upper_case_globals)]
pub const TEXTURE_MAG_FILTER: u32 = 0x2800;
#[allow(non_upper_case_globals)]
pub const TEXTURE_MAX_ANISOTROPY: u32 = 0x84FE;
#[allow(non_upper_case_globals)]
pub const TEXTURE_MAX_ANISOTROPY_EXT: u32 = 0x84FE;
#[allow(non_upper_case_globals)]
pub const TEXTURE_MAX_LEVEL: u32 = 0x813D;
#[allow(non_upper_case_globals)]
pub const TEXTURE_MAX_LOD: u32 = 0x813B;
#[allow(non_upper_case_globals)]
pub const TEXTURE_MIN_FILTER: u32 = 0x2801;
#[allow(non_upper_case_globals)]
pub const TEXTURE_MIN_LOD: u32 = 0x813A;
#[allow(non_upper_case_globals)]
pub const TEXTURE_RECTANGLE: u32 = 0x84F5;
#[allow(non_upper_case_globals)]
pub const TEXTURE_RED_SIZE: u32 = 0x805C;
#[allow(non_upper_case_globals)]
pub const TEXTURE_RED_TYPE: u32 = 0x8C10;
#[allow(non_upper_case_globals)]
pub const TEXTURE_SAMPLES: u32 = 0x9106;
#[allow(non_upper_case_globals)]
pub const TEXTURE_SHADOW: u32 = 0x82A1;
#[allow(non_upper_case_globals)]
pub const TEXTURE_SHARED_SIZE: u32 = 0x8C3F;
#[allow(non_upper_case_globals)]
pub const TEXTURE_STENCIL_SIZE: u32 = 0x88F1;
#[allow(non_upper_case_globals)]
pub const TEXTURE_SWIZZLE_A: u32 = 0x8E45;
#[allow(non_upper_case_globals)]
pub const TEXTURE_SWIZZLE_B: u32 = 0x8E44;
#[allow(non_upper_case_globals)]
pub const TEXTURE_SWIZZLE_G: u32 = 0x8E43;
#[allow(non_upper_case_globals)]
pub const TEXTURE_SWIZZLE_R: u32 = 0x8E42;
#[allow(non_upper_case_globals)]
pub const TEXTURE_SWIZZLE_RGBA: u32 = 0x8E46;
#[allow(non_upper_case_globals)]
pub const TEXTURE_TARGET: u32 = 0x1006;
#[allow(non_upper_case_globals)]
pub const TEXTURE_UPDATE_BARRIER_BIT: u32 = 0x00000100;
#[allow(non_upper_case_globals)]
pub const TEXTURE_VIEW: u32 = 0x82B5;
#[allow(non_upper_case_globals)]
pub const TEXTURE_VIEW_MIN_LAYER: u32 = 0x82DD;
#[allow(non_upper_case_globals)]
pub const TEXTURE_VIEW_MIN_LEVEL: u32 = 0x82DB;
#[allow(non_upper_case_globals)]
pub const TEXTURE_VIEW_NUM_LAYERS: u32 = 0x82DE;
#[allow(non_upper_case_globals)]
pub const TEXTURE_VIEW_NUM_LEVELS: u32 = 0x82DC;
#[allow(non_upper_case_globals)]
pub const TEXTURE_WIDTH: u32 = 0x1000;
#[allow(non_upper_case_globals)]
pub const TEXTURE_WRAP_R: u32 = 0x8072;
#[allow(non_upper_case_globals)]
pub const TEXTURE_WRAP_S: u32 = 0x2802;
#[allow(non_upper_case_globals)]
pub const TEXTURE_WRAP_T: u32 = 0x2803;
#[allow(non_upper_case_globals)]
pub const TIMEOUT_EXPIRED: u32 = 0x911B;
#[allow(non_upper_case_globals)]
pub const TIMEOUT_IGNORED: u64 = 0xFFFFFFFFFFFFFFFF;
#[allow(non_upper_case_globals)]
pub const TIMESTAMP: u32 = 0x8E28;
#[allow(non_upper_case_globals)]
pub const TIME_ELAPSED: u32 = 0x88BF;
#[allow(non_upper_case_globals)]
pub const TOP_LEVEL_ARRAY_SIZE: u32 = 0x930C;
#[allow(non_upper_case_globals)]
pub const TOP_LEVEL_ARRAY_STRIDE: u32 = 0x930D;
#[allow(non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK: u32 = 0x8E22;
#[allow(non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_ACTIVE: u32 = 0x8E24;
#[allow(non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BARRIER_BIT: u32 = 0x00000800;
#[allow(non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BINDING: u32 = 0x8E25;
#[allow(non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BUFFER: u32 = 0x8C8E;
#[allow(non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BUFFER_ACTIVE: u32 = 0x8E24;
#[allow(non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BUFFER_BINDING: u32 = 0x8C8F;
#[allow(non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BUFFER_INDEX: u32 = 0x934B;
#[allow(non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BUFFER_MODE: u32 = 0x8C7F;
#[allow(non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BUFFER_PAUSED: u32 = 0x8E23;
#[allow(non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BUFFER_SIZE: u32 = 0x8C85;
#[allow(non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BUFFER_START: u32 = 0x8C84;
#[allow(non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BUFFER_STRIDE: u32 = 0x934C;
#[allow(non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_OVERFLOW: u32 = 0x82EC;
#[allow(non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_PAUSED: u32 = 0x8E23;
#[allow(non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: u32 = 0x8C88;
#[allow(non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_STREAM_OVERFLOW: u32 = 0x82ED;
#[allow(non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_VARYING: u32 = 0x92F4;
#[allow(non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_VARYINGS: u32 = 0x8C83;
#[allow(non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: u32 = 0x8C76;
#[allow(non_upper_case_globals)]
pub const TRIANGLES: u32 = 0x0004;
#[allow(non_upper_case_globals)]
pub const TRIANGLES_ADJACENCY: u32 = 0x000C;
#[allow(non_upper_case_globals)]
pub const TRIANGLE_FAN: u32 = 0x0006;
#[allow(non_upper_case_globals)]
pub const TRIANGLE_STRIP: u32 = 0x0005;
#[allow(non_upper_case_globals)]
pub const TRIANGLE_STRIP_ADJACENCY: u32 = 0x000D;
#[allow(non_upper_case_globals)]
pub const TRUE: u8 = 1;
#[allow(non_upper_case_globals)]
pub const TYPE: u32 = 0x92FA;
#[allow(non_upper_case_globals)]
pub const UNDEFINED_VERTEX: u32 = 0x8260;
#[allow(non_upper_case_globals)]
pub const UNIFORM: u32 = 0x92E1;
#[allow(non_upper_case_globals)]
pub const UNIFORM_ARRAY_STRIDE: u32 = 0x8A3C;
#[allow(non_upper_case_globals)]
pub const UNIFORM_ATOMIC_COUNTER_BUFFER_INDEX: u32 = 0x92DA;
#[allow(non_upper_case_globals)]
pub const UNIFORM_BARRIER_BIT: u32 = 0x00000004;
#[allow(non_upper_case_globals)]
pub const UNIFORM_BLOCK: u32 = 0x92E2;
#[allow(non_upper_case_globals)]
pub const UNIFORM_BLOCK_ACTIVE_UNIFORMS: u32 = 0x8A42;
#[allow(non_upper_case_globals)]
pub const UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: u32 = 0x8A43;
#[allow(non_upper_case_globals)]
pub const UNIFORM_BLOCK_BINDING: u32 = 0x8A3F;
#[allow(non_upper_case_globals)]
pub const UNIFORM_BLOCK_DATA_SIZE: u32 = 0x8A40;
#[allow(non_upper_case_globals)]
pub const UNIFORM_BLOCK_INDEX: u32 = 0x8A3A;
#[allow(non_upper_case_globals)]
pub const UNIFORM_BLOCK_NAME_LENGTH: u32 = 0x8A41;
#[allow(non_upper_case_globals)]
pub const UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER: u32 = 0x90EC;
#[allow(non_upper_case_globals)]
pub const UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: u32 = 0x8A46;
#[allow(non_upper_case_globals)]
pub const UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: u32 = 0x8A45;
#[allow(non_upper_case_globals)]
pub const UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER: u32 = 0x84F0;
#[allow(non_upper_case_globals)]
pub const UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER: u32 = 0x84F1;
#[allow(non_upper_case_globals)]
pub const UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: u32 = 0x8A44;
#[allow(non_upper_case_globals)]
pub const UNIFORM_BUFFER: u32 = 0x8A11;
#[allow(non_upper_case_globals)]
pub const UNIFORM_BUFFER_BINDING: u32 = 0x8A28;
#[allow(non_upper_case_globals)]
pub const UNIFORM_BUFFER_OFFSET_ALIGNMENT: u32 = 0x8A34;
#[allow(non_upper_case_globals)]
pub const UNIFORM_BUFFER_SIZE: u32 = 0x8A2A;
#[allow(non_upper_case_globals)]
pub const UNIFORM_BUFFER_START: u32 = 0x8A29;
#[allow(non_upper_case_globals)]
pub const UNIFORM_IS_ROW_MAJOR: u32 = 0x8A3E;
#[allow(non_upper_case_globals)]
pub const UNIFORM_MATRIX_STRIDE: u32 = 0x8A3D;
#[allow(non_upper_case_globals)]
pub const UNIFORM_NAME_LENGTH: u32 = 0x8A39;
#[allow(non_upper_case_globals)]
pub const UNIFORM_OFFSET: u32 = 0x8A3B;
#[allow(non_upper_case_globals)]
pub const UNIFORM_SIZE: u32 = 0x8A38;
#[allow(non_upper_case_globals)]
pub const UNIFORM_TYPE: u32 = 0x8A37;
#[allow(non_upper_case_globals)]
pub const UNKNOWN_CONTEXT_RESET: u32 = 0x8255;
#[allow(non_upper_case_globals)]
pub const UNPACK_ALIGNMENT: u32 = 0x0CF5;
#[allow(non_upper_case_globals)]
pub const UNPACK_COMPRESSED_BLOCK_DEPTH: u32 = 0x9129;
#[allow(non_upper_case_globals)]
pub const UNPACK_COMPRESSED_BLOCK_HEIGHT: u32 = 0x9128;
#[allow(non_upper_case_globals)]
pub const UNPACK_COMPRESSED_BLOCK_SIZE: u32 = 0x912A;
#[allow(non_upper_case_globals)]
pub const UNPACK_COMPRESSED_BLOCK_WIDTH: u32 = 0x9127;
#[allow(non_upper_case_globals)]
pub const UNPACK_IMAGE_HEIGHT: u32 = 0x806E;
#[allow(non_upper_case_globals)]
pub const UNPACK_LSB_FIRST: u32 = 0x0CF1;
#[allow(non_upper_case_globals)]
pub const UNPACK_ROW_LENGTH: u32 = 0x0CF2;
#[allow(non_upper_case_globals)]
pub const UNPACK_SKIP_IMAGES: u32 = 0x806D;
#[allow(non_upper_case_globals)]
pub const UNPACK_SKIP_PIXELS: u32 = 0x0CF4;
#[allow(non_upper_case_globals)]
pub const UNPACK_SKIP_ROWS: u32 = 0x0CF3;
#[allow(non_upper_case_globals)]
pub const UNPACK_SWAP_BYTES: u32 = 0x0CF0;
#[allow(non_upper_case_globals)]
pub const UNSIGNALED: u32 = 0x9118;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_BYTE: u32 = 0x1401;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_BYTE_2_3_3_REV: u32 = 0x8362;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_BYTE_3_3_2: u32 = 0x8032;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_INT: u32 = 0x1405;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_INT_10F_11F_11F_REV: u32 = 0x8C3B;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_INT_10_10_10_2: u32 = 0x8036;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_INT_24_8: u32 = 0x84FA;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_INT_2_10_10_10_REV: u32 = 0x8368;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_INT_5_9_9_9_REV: u32 = 0x8C3E;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_INT_8_8_8_8: u32 = 0x8035;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_INT_8_8_8_8_REV: u32 = 0x8367;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_INT_ATOMIC_COUNTER: u32 = 0x92DB;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_INT_IMAGE_1D: u32 = 0x9062;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_INT_IMAGE_1D_ARRAY: u32 = 0x9068;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_INT_IMAGE_2D: u32 = 0x9063;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_INT_IMAGE_2D_ARRAY: u32 = 0x9069;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_INT_IMAGE_2D_MULTISAMPLE: u32 = 0x906B;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY: u32 = 0x906C;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_INT_IMAGE_2D_RECT: u32 = 0x9065;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_INT_IMAGE_3D: u32 = 0x9064;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_INT_IMAGE_BUFFER: u32 = 0x9067;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_INT_IMAGE_CUBE: u32 = 0x9066;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY: u32 = 0x906A;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_1D: u32 = 0x8DD1;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_1D_ARRAY: u32 = 0x8DD6;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_2D: u32 = 0x8DD2;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_2D_ARRAY: u32 = 0x8DD7;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: u32 = 0x910A;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: u32 = 0x910D;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_2D_RECT: u32 = 0x8DD5;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_3D: u32 = 0x8DD3;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_BUFFER: u32 = 0x8DD8;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_CUBE: u32 = 0x8DD4;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY: u32 = 0x900F;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_INT_VEC2: u32 = 0x8DC6;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_INT_VEC3: u32 = 0x8DC7;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_INT_VEC4: u32 = 0x8DC8;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_NORMALIZED: u32 = 0x8C17;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_SHORT: u32 = 0x1403;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_SHORT_1_5_5_5_REV: u32 = 0x8366;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_SHORT_4_4_4_4: u32 = 0x8033;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_SHORT_4_4_4_4_REV: u32 = 0x8365;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_SHORT_5_5_5_1: u32 = 0x8034;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_SHORT_5_6_5: u32 = 0x8363;
#[allow(non_upper_case_globals)]
pub const UNSIGNED_SHORT_5_6_5_REV: u32 = 0x8364;
#[allow(non_upper_case_globals)]
pub const UPPER_LEFT: u32 = 0x8CA2;
#[allow(non_upper_case_globals)]
pub const VALIDATE_STATUS: u32 = 0x8B83;
#[allow(non_upper_case_globals)]
pub const VENDOR: u32 = 0x1F00;
#[allow(non_upper_case_globals)]
pub const VERSION: u32 = 0x1F02;
#[allow(non_upper_case_globals)]
pub const VERTEX_ARRAY: u32 = 0x8074;
#[allow(non_upper_case_globals)]
pub const VERTEX_ARRAY_BINDING: u32 = 0x85B5;
#[allow(non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_BARRIER_BIT: u32 = 0x00000001;
#[allow(non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: u32 = 0x889F;
#[allow(non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_DIVISOR: u32 = 0x88FE;
#[allow(non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_ENABLED: u32 = 0x8622;
#[allow(non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_INTEGER: u32 = 0x88FD;
#[allow(non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_LONG: u32 = 0x874E;
#[allow(non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: u32 = 0x886A;
#[allow(non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_POINTER: u32 = 0x8645;
#[allow(non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_SIZE: u32 = 0x8623;
#[allow(non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_STRIDE: u32 = 0x8624;
#[allow(non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_TYPE: u32 = 0x8625;
#[allow(non_upper_case_globals)]
pub const VERTEX_ATTRIB_BINDING: u32 = 0x82D4;
#[allow(non_upper_case_globals)]
pub const VERTEX_ATTRIB_RELATIVE_OFFSET: u32 = 0x82D5;
#[allow(non_upper_case_globals)]
pub const VERTEX_BINDING_BUFFER: u32 = 0x8F4F;
#[allow(non_upper_case_globals)]
pub const VERTEX_BINDING_DIVISOR: u32 = 0x82D6;
#[allow(non_upper_case_globals)]
pub const VERTEX_BINDING_OFFSET: u32 = 0x82D7;
#[allow(non_upper_case_globals)]
pub const VERTEX_BINDING_STRIDE: u32 = 0x82D8;
#[allow(non_upper_case_globals)]
pub const VERTEX_PROGRAM_POINT_SIZE: u32 = 0x8642;
#[allow(non_upper_case_globals)]
pub const VERTEX_SHADER: u32 = 0x8B31;
#[allow(non_upper_case_globals)]
pub const VERTEX_SHADER_BIT: u32 = 0x00000001;
#[allow(non_upper_case_globals)]
pub const VERTEX_SHADER_INVOCATIONS: u32 = 0x82F0;
#[allow(non_upper_case_globals)]
pub const VERTEX_SUBROUTINE: u32 = 0x92E8;
#[allow(non_upper_case_globals)]
pub const VERTEX_SUBROUTINE_UNIFORM: u32 = 0x92EE;
#[allow(non_upper_case_globals)]
pub const VERTEX_TEXTURE: u32 = 0x829B;
#[allow(non_upper_case_globals)]
pub const VERTICES_SUBMITTED: u32 = 0x82EE;
#[allow(non_upper_case_globals)]
pub const VIEWPORT: u32 = 0x0BA2;
#[allow(non_upper_case_globals)]
pub const VIEWPORT_BOUNDS_RANGE: u32 = 0x825D;
#[allow(non_upper_case_globals)]
pub const VIEWPORT_INDEX_PROVOKING_VERTEX: u32 = 0x825F;
#[allow(non_upper_case_globals)]
pub const VIEWPORT_SUBPIXEL_BITS: u32 = 0x825C;
#[allow(non_upper_case_globals)]
pub const VIEW_CLASS_128_BITS: u32 = 0x82C4;
#[allow(non_upper_case_globals)]
pub const VIEW_CLASS_16_BITS: u32 = 0x82CA;
#[allow(non_upper_case_globals)]
pub const VIEW_CLASS_24_BITS: u32 = 0x82C9;
#[allow(non_upper_case_globals)]
pub const VIEW_CLASS_32_BITS: u32 = 0x82C8;
#[allow(non_upper_case_globals)]
pub const VIEW_CLASS_48_BITS: u32 = 0x82C7;
#[allow(non_upper_case_globals)]
pub const VIEW_CLASS_64_BITS: u32 = 0x82C6;
#[allow(non_upper_case_globals)]
pub const VIEW_CLASS_8_BITS: u32 = 0x82CB;
#[allow(non_upper_case_globals)]
pub const VIEW_CLASS_96_BITS: u32 = 0x82C5;
#[allow(non_upper_case_globals)]
pub const VIEW_CLASS_BPTC_FLOAT: u32 = 0x82D3;
#[allow(non_upper_case_globals)]
pub const VIEW_CLASS_BPTC_UNORM: u32 = 0x82D2;
#[allow(non_upper_case_globals)]
pub const VIEW_CLASS_RGTC1_RED: u32 = 0x82D0;
#[allow(non_upper_case_globals)]
pub const VIEW_CLASS_RGTC2_RG: u32 = 0x82D1;
#[allow(non_upper_case_globals)]
pub const VIEW_CLASS_S3TC_DXT1_RGB: u32 = 0x82CC;
#[allow(non_upper_case_globals)]
pub const VIEW_CLASS_S3TC_DXT1_RGBA: u32 = 0x82CD;
#[allow(non_upper_case_globals)]
pub const VIEW_CLASS_S3TC_DXT3_RGBA: u32 = 0x82CE;
#[allow(non_upper_case_globals)]
pub const VIEW_CLASS_S3TC_DXT5_RGBA: u32 = 0x82CF;
#[allow(non_upper_case_globals)]
pub const VIEW_COMPATIBILITY_CLASS: u32 = 0x82B6;
#[allow(non_upper_case_globals)]
pub const WAIT_FAILED: u32 = 0x911D;
#[allow(non_upper_case_globals)]
pub const WRITE_ONLY: u32 = 0x88B9;
#[allow(non_upper_case_globals)]
pub const XOR: u32 = 0x1506;
#[allow(non_upper_case_globals)]
pub const ZERO: u32 = 0;
#[allow(non_upper_case_globals)]
pub const ZERO_TO_ONE: u32 = 0x935F;
