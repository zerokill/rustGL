extern crate gl;
extern crate glfw;

mod shader;
mod mesh;

use glfw::{Action, Context, Key};
use std::time::Instant;
use shader::Shader;
use mesh::Mesh;
use nalgebra_glm as glm;

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

    let quad = Mesh::quad([0.0, 1.0, 1.0]);

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
        render(
            &mut window,
            &quad,
            &shader,
            time,
        );
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

fn render(window: &mut glfw::Window, mesh: &Mesh, shader: &Shader, time: f32) {
    unsafe {
        gl::ClearColor(0.1, 0.1, 0.2, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        check_gl_error("clear");

        shader.use_program();

        // Example 1: Static translation
        let model1 = glm::translate(&glm::Mat4::identity(), &glm::vec3(-0.6, 0.5, 0.0));
        shader.set_mat4("model", &model1);
        mesh.draw();

        // Example 2: Rotation (animated)
        let mut model2 = glm::Mat4::identity();
        model2 = glm::translate(&model2, &glm::vec3(0.0, 0.5, 0.0));
        model2 = glm::rotate(&model2, time, &glm::vec3(0.0, 0.0, 1.0));  // Rotate around Z-axis
        model2 = glm::scale(&model2, &glm::vec3(0.5, 0.5, 0.5));  // Scale to 50%
        shader.set_mat4("model", &model2);
        mesh.draw();

        // Example 3: Scaling (pulsing)
        let scale = 1.0 + 0.5 * (time * 2.0).sin();  // Pulse between 0.5 and 1.5
        let mut model3 = glm::Mat4::identity();
        model3 = glm::translate(&model3, &glm::vec3(0.6, 0.5, 0.0));
        model3 = glm::scale(&model3, &glm::vec3(scale, scale, 1.0));
        shader.set_mat4("model", &model3);
        mesh.draw();

        // Example 4: Combined transformation (orbit)
        let orbit_radius = 0.3;
        let orbit_x = orbit_radius * (time * 1.5).cos();
        let orbit_y = orbit_radius * (time * 1.5).sin();
        let mut model4 = glm::Mat4::identity();
        model4 = glm::translate(&model4, &glm::vec3(orbit_x, -0.4, 0.0));
        model4 = glm::rotate(&model4, time * 2.0, &glm::vec3(0.0, 0.0, 1.0));
        model4 = glm::scale(&model4, &glm::vec3(0.3, 0.3, 1.0));
        shader.set_mat4("model", &model4);
        mesh.draw();

    }
    window.swap_buffers();
}
