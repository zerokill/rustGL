extern crate gl;
extern crate glfw;

use glfw::{Action, Context, Key};
use std::time::Instant;

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

    // At the top of main(), after loading OpenGL:
    let vertices: [f32; 9] = [
        // x     y      z
        -0.5, -0.5, 0.0, // Bottom left
        0.5, -0.5, 0.0, // Bottom right
        0.0, 0.5, 0.0, // Top center
    ];

    const VERTEX_SHADER_SOURCE: &str = r#"
        #version 330 core
        layout (location = 0) in vec3 aPos;

        void main() {
            gl_Position = vec4(aPos, 1.0);
        }
    "#;

    const FRAGMENT_SHADER_SOURCE: &str = r#"
        #version 330 core
        out vec4 FragColor;

        void main() {
            FragColor = vec4(1.0, 0.5, 0.2, 1.0);  // Orange color
        }
    "#;

    let (vao, vbo) = unsafe {
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as isize,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * std::mem::size_of::<f32>()) as i32,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        (vao, vbo)
    };

    let shader_program = create_shader_program(VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE);

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
        render(&mut window, vao, shader_program);
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

fn render(window: &mut glfw::Window, vao: u32, shader_program: u32) {
    unsafe {
        gl::ClearColor(0.1, 0.1, 0.2, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        check_gl_error("clear");

        gl::UseProgram(shader_program);

        gl::BindVertexArray(vao);
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
        gl::BindVertexArray(0);
    }
    window.swap_buffers();
}

fn compile_shader(source: &str, shader_type: gl::types::GLenum) -> u32 {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        let c_str = std::ffi::CString::new(source.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        // Check for compilation errors
        let mut success = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer = vec![0u8; len as usize];
            gl::GetShaderInfoLog(
                shader,
                len,
                std::ptr::null_mut(),
                buffer.as_mut_ptr() as *mut i8,
            );
            panic!(
                "Shader compilation failed: {}",
                String::from_utf8_lossy(&buffer)
            );
        }

        shader
    }
}

fn create_shader_program(vertex_src: &str, fragment_src: &str) -> u32 {
    unsafe {
        let vertex_shader = compile_shader(vertex_src, gl::VERTEX_SHADER);
        let fragment_shader = compile_shader(fragment_src, gl::FRAGMENT_SHADER);

        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);

        // Check for linking errors
        let mut success = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            let mut len = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer = vec![0u8; len as usize];
            gl::GetProgramInfoLog(
                program,
                len,
                std::ptr::null_mut(),
                buffer.as_mut_ptr() as *mut i8,
            );
            panic!(
                "Program linking failed: {}",
                String::from_utf8_lossy(&buffer)
            );
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        program
    }
}
