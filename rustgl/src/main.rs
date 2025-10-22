extern crate gl;
extern crate glfw;

mod shader;
mod mesh;

use glfw::{Action, Context, Key};
use std::time::Instant;
use shader::Shader;
use mesh::{Mesh, Vertex};

fn main() {
    // Initialize GLFW
    let mut glfw = glfw::init_no_callbacks().expect("Failed to initialize GLFW");

    // Request OpenGL 4.5 Core Profile for Linux
    // Note: For initial learning steps, we'll start with 3.3 for compatibility
    // Later steps will upgrade to 4.5+ for advanced features
    #[cfg(target_os = "linux")]
    {
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 5));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
    }

    // macOS limited to OpenGL 4.1 maximum
    #[cfg(target_os = "macos")]
    {
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    }

    // Create a window
    let (mut window, events) = glfw
        .create_window(
            1024,            // Width
            768,             // Height
            "RustGL by mau", // Title
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // Load OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // Print OpenGL version info
    unsafe {
        let version = std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8);
        println!("OpenGL Version: {}", version.to_str().unwrap());
    }

    let triangle_vertices = vec![
        Vertex::new([-0.5, -0.5, 0.0,], [   1.0, 0.0, 0.0, ]), // Bottom left (red)
        Vertex::new([0.5, -0.5, 0.0, ], [  0.0, 1.0, 0.0,  ]),// Bottom right (green)
        Vertex::new([0.0,  0.5, 0.0, ], [  0.0, 0.0, 1.0,  ]),// Top center (blue)
    ];
    let triangle = Mesh::new(&triangle_vertices);

    let triangle2_vertices = vec![
        Vertex::new([0.1, -0.5, 0.0], [1.0, 1.0, 0.0]),   // Bottom left (yellow)
        Vertex::new([1.1, -0.5, 0.0], [0.0, 1.0, 1.0]),   // Bottom right (cyan)
        Vertex::new([0.6, 0.5, 0.0], [1.0, 0.0, 1.0]),    // Top center (magenta)
    ];
    let triangle2 = Mesh::new(&triangle2_vertices);

    let shader = Shader::new("shader/basic.vert", "shader/basic.frag");

    let mut last_frame = Instant::now();
    let mut frame_count = 0;
    let mut fps_timer = Instant::now();
    let mut time = 0.0f32;

    // Window loop - keep the window open
    while !window.should_close() {
        let current_frame = Instant::now();
        let delta_time = current_frame.duration_since(last_frame).as_secs_f32();
        last_frame = current_frame;

        frame_count += 1;
        if fps_timer.elapsed().as_secs() >= 1 {
            // Update window title with FPS
            let title = format!(
                "RustGL by mau | FPS: {} | Frame time: {:.2}ms",
                frame_count,
                delta_time * 1000.0
            );
            window.set_title(&title);
            frame_count = 0;
            fps_timer = Instant::now();
        }

        process_events(&mut window, &events);
        update(delta_time, &mut time);
        render(&mut window, &triangle, &triangle2, &shader);
    }
}

fn process_events(
    window: &mut glfw::Window,
    events: &glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
) {
    window.glfw.poll_events();
    for (_, event) in glfw::flush_messages(events) {
        handle_window_event(window, event);
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        glfw::WindowEvent::Key(Key::Space, _, Action::Press, _) => {
            println!("Space pressed!");
        }
        glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
            gl::Viewport(0, 0, width, height);
        },
        _ => {}
    }
}

fn check_gl_error(location: &str) {
    unsafe {
        let err = gl::GetError();
        if err != gl::NO_ERROR {
            println!("OpenGL Error at {}: {}", location, err);
        }
    }
}

fn update(delta_time: f32, time: &mut f32) {
    // Game logic
    *time += delta_time;
}

fn render(window: &mut glfw::Window, triangle: &Mesh, triangle2: &Mesh, shader: &Shader) {
    unsafe {
        gl::ClearColor(0.1, 0.1, 0.2, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        check_gl_error("clear");

        shader.use_program();
        triangle.draw();
        triangle2.draw();
    }
    window.swap_buffers();
}
