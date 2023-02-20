curl https://raw.githubusercontent.com/KhronosGroup/OpenGL-Registry/01ac568838ce3a93385d885362e3ddc7bca54b08/xml/gl.xml > generated/gl.xml

# phosphorus expects one API, but we're trying to generate bindings for multiple at once.
# We'll work around it for now by renaming GL ES 3.2 to match GL 4.6.
replacements=''
replacements+='s/api="gles2"/api="gl"/g;'
replacements+='s/name="GL_ES_VERSION_3_2"/name="GL_VERSION_4_6"/g;'
replacements+='s/number="3.2"/number="4.6"/g;'
sed --in-place $replacements generated/gl.xml

phosphorus \
    ./generated/gl.xml \
    gl \
    4 6 \
    core \
GL_ARB_debug_output,\
GL_KHR_debug,\
GL_ARB_texture_filter_anisotropic,\
GL_EXT_texture_filter_anisotropic,\
GL_ARB_tessellation_shader,\
GL_ARB_compute_shader,\
GL_ARB_instanced_arrays,\
GL_EXT_draw_buffers2,\
GL_ARB_draw_instanced,\
GL_ARB_base_instance,\
GL_ARB_draw_elements_base_vertex,\
GL_ARB_framebuffer_sRGB,\
GL_ARB_uniform_buffer_object,\
GL_ARB_copy_buffer,\
GL_NV_copy_buffer,\
GL_ARB_sampler_objects,\
GL_ARB_buffer_storage,\
GL_EXT_buffer_storage,\
GL_ARB_vertex_array_object,\
GL_ARB_framebuffer_object,\
GL_ARB_texture_storage,\
GL_ARB_program_interface_query,\
GL_ARB_sync,\
GL_KHR_parallel_shader_compile,\
GL_ARB_parallel_shader_compile\
    > generated/gl46.rs

cp generated/gl46.rs src/gl46.rs
