use glow::{self, Context, RenderLoop};

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
        let (_window, gl, _events_loop, render_loop, shader_version) = {
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
            (
                (),
                glow::web::Context::from_webgl2_context(webgl2_context),
                (),
                glow::web::RenderLoop::from_request_animation_frame(),
                "#version 300 es",
            )
        };

        // Create a context from a glutin window on non-wasm32 targets
        #[cfg(feature = "window-glutin")]
        let (gl, mut events_loop, render_loop, shader_version) = {
            use glutin::GlContext;
            let events_loop = glutin::EventsLoop::new();
            let window_builder = glutin::WindowBuilder::new()
                .with_title("Hello triangle!")
                .with_dimensions(glutin::dpi::LogicalSize::new(1024.0, 768.0));
            let context_builder = glutin::ContextBuilder::new().with_vsync(true);
            let window =
                glutin::GlWindow::new(window_builder, context_builder, &events_loop).unwrap();
            window.make_current().unwrap();
            let context = glow::native::Context::from_loader_function(|s| {
                window.get_proc_address(s) as *const _
            });
            let render_loop =
                glow::native::RenderLoop::<glutin::GlWindow>::from_glutin_window(window);
            (context, events_loop, render_loop, "#version 410")
        };

        // Create a context from a sdl2 window
        #[cfg(feature = "window-sdl2")]
        let (gl, mut events_loop, render_loop, shader_version, _gl_context) = {
            let sdl = sdl2::init().unwrap();
            let video = sdl.video().unwrap();
            let gl_attr = video.gl_attr();
            gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
            gl_attr.set_context_version(3, 0);

            let window = video
                .window("Hello triangle!", 1024, 769)
                .opengl()
                .resizable()
                .build()
                .unwrap();
            let gl_context = window.gl_create_context().unwrap();
            let context = glow::native::Context::from_loader_function(|s| {
                video.gl_get_proc_address(s) as *const _
            });
            let render_loop =
                glow::native::RenderLoop::<sdl2::video::Window>::from_sdl_window(window);
            let event_loop = sdl.event_pump().unwrap();
            (context, event_loop, render_loop, "#version 410", gl_context)
        };

        let vertex_array = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        gl.bind_vertex_array(Some(vertex_array));

        let program = gl.create_program().expect("Cannot create program");

        let (vertex_shader_source, fragment_shader_source) = (
            r#"const vec2 verts[3] = vec2[3](
                vec2(0.5f, 1.0f),
                vec2(0.0f, 0.0f),
                vec2(1.0f, 0.0f)
            );
            out vec2 vert;
            void main() {
                vert = verts[gl_VertexID];
                gl_Position = vec4(vert - 0.5, 0.0, 1.0);
            }"#,
            r#"precision mediump float;
            in vec2 vert;
            out vec4 color;
            void main() {
                color = vec4(vert, 0.5, 1.0);
            }"#,
        );

        let shader_sources = [
            (glow::VERTEX_SHADER, vertex_shader_source),
            (glow::FRAGMENT_SHADER, fragment_shader_source),
        ];

        let mut shaders = Vec::with_capacity(shader_sources.len());

        for (shader_type, shader_source) in shader_sources.iter() {
            let shader = gl
                .create_shader(*shader_type)
                .expect("Cannot create shader");
            gl.shader_source(shader, &format!("{}\n{}", shader_version, shader_source));
            gl.compile_shader(shader);
            if !gl.get_shader_compile_status(shader) {
                panic!(gl.get_shader_info_log(shader));
            }
            gl.attach_shader(program, shader);
            shaders.push(shader);
        }

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!(gl.get_program_info_log(program));
        }

        for shader in shaders {
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);
        }

        gl.use_program(Some(program));
        gl.clear_color(0.1, 0.2, 0.3, 1.0);

        render_loop.run(move |running: &mut bool| {
            // Handle events differently between targets
            #[cfg(feature = "window-glutin")]
            {
                events_loop.poll_events(|event| match event {
                    glutin::Event::WindowEvent { event, .. } => match event {
                        glutin::WindowEvent::CloseRequested => *running = false,
                        _ => (),
                    },
                    _ => (),
                });
            }

            #[cfg(feature = "window-sdl2")]
            {
                for event in events_loop.poll_iter() {
                    match event {
                        sdl2::event::Event::Quit { .. } => *running = false,
                        _ => {}
                    }
                }
            }

            gl.clear(glow::COLOR_BUFFER_BIT);
            gl.draw_arrays(glow::TRIANGLES, 0, 3);

            if !*running {
                gl.delete_program(program);
                gl.delete_vertex_array(vertex_array);
            }
        });
    }
}
