fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use gl_generator::{Api, Fallbacks, Profile, Registry};
        use std::env;
        use std::fs::File;
        use std::path::Path;
        let out_dir = env::var("OUT_DIR").unwrap();
        let dest = Path::new(&out_dir);
        let mut file = File::create(&dest.join("opengl_bindings.rs")).unwrap();
        Registry::new(
            Api::Gl,
            (4, 6),
            Profile::Core,
            Fallbacks::All,
            [
                "GL_EXT_texture_filter_anisotropic",
                "GL_ARB_draw_buffers_blend",
                "GL_ARB_program_interface_query",
            ],
        )
        .write_bindings(gl_generator::StructGenerator, &mut file)
        .unwrap();
    }
}
