use glow::{self, RenderingContext};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn wasm_main() {
    main();
}

fn main() {
    unsafe {
        // Create a context from a WebGL2 context on wasm32 targets
        #[cfg(target_arch = "wasm32")]
        let context = {
            use wasm_bindgen::JsCast;
            let canvas = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("canvas")
                .unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .unwrap();
            let webgl2_context = canvas
                .get_context("webgl2")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::WebGl2RenderingContext>()
                .unwrap();
            glow::WebRenderingContext::from_webgl2_context(webgl2_context)
        };

        // Create a context from a glutin window on non-wasm32 targets
        #[cfg(not(target_arch = "wasm32"))]
        let context = {
            use glutin::GlContext;
            let events_loop = glutin::EventsLoop::new();
            let window_builder = glutin::WindowBuilder::new()
                .with_title("Hello triangle!")
                .with_dimensions(glutin::dpi::LogicalSize::new(1024.0, 768.0));
            let context_builder = glutin::ContextBuilder::new().with_vsync(true);
            let window =
                glutin::GlWindow::new(window_builder, context_builder, &events_loop).unwrap();
            window.make_current().unwrap();
            glow::NativeRenderingContext::from_glutin_window(&window)
        };

        let program = context.create_program().expect("Cannot create program");

        // Provide different shaders for wasm32 and non-wasm32 targets
        #[cfg(target_arch = "wasm32")]
        let (vertex_shader_source, fragment_shader_source) = (
            r#"#version 300 es
        in vec4 position;
        void main() {
            gl_Position = position;
        }
        "#,
            r#"#version 300 es
        precision mediump float;
        out vec4 color;
        void main() {
            color = vec4(1.0, 1.0, 1.0, 1.0);
        }
        "#,
        );
        #[cfg(not(target_arch = "wasm32"))]
        let (vertex_shader_source, fragment_shader_source) = (
            r#"#version 410
        in vec4 position;
        void main() {
            gl_Position = position;
        }
        "#,
            r#"#version 410
        precision mediump float;
        out vec4 color;
        void main() {
            color = vec4(1.0, 1.0, 1.0, 1.0);
        }
        "#,
        );

        let shaders = [
            (glow::ShaderType::Vertex, vertex_shader_source),
            (glow::ShaderType::Fragment, fragment_shader_source),
        ];

        for (shader_type, shader_source) in shaders.iter() {
            let shader = context
                .create_shader(*shader_type)
                .expect("Cannot create shader");
            context.shader_source(shader, shader_source);
            context.compile_shader(shader);
            if !context.get_shader_compile_status(shader) {
                panic!(context.get_shader_info_log(shader));
            }
            context.attach_shader(program, shader);
        }

        context.link_program(program);
        if !context.get_program_link_status(program) {
            panic!(context.get_program_info_log(program));
        }
    }
}
