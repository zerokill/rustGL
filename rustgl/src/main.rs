extern crate gl;
extern crate glfw;

mod shader;
mod mesh;
mod camera;
mod texture;

use glfw::{Action, Context, Key};
use std::time::Instant;
use shader::Shader;
use mesh::Mesh;
use camera::{Camera, CameraMovement};
use nalgebra_glm as glm;
use texture::Texture;

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

    // Enable V-Sync to cap FPS at monitor refresh rate (usually 60 FPS)
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    // Load OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // Print OpenGL version info
    unsafe {
        let version = std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8);
        println!("OpenGL Version: {}", version.to_str().unwrap());
    }

    // Create all primitive shapes
    let sphere = Mesh::sphere(1.0, 32, 16, [0.3, 0.7, 1.0]);  // Blue sphere
    let cube = Mesh::cube([1.0, 0.5, 0.2]);  // Orange cube
    let cylinder = Mesh::cylinder(0.5, 2.0, 32, [0.2, 1.0, 0.3]);  // Green cylinder
    let torus = Mesh::torus(1.0, 0.3, 32, 16, [1.0, 0.3, 0.7]);  // Pink torus
    let plane = Mesh::plane(10.0, 10.0, [0.3, 0.3, 0.3]);  // Gray plane

    let shader = Shader::new("shader/basic.vert", "shader/basic.frag");
    // Load a test texture
    let texture = Texture::new("resources/textures/livia.png")
        .expect("Failed to load texture");

    let mut camera = Camera::default();

    const TARGET_FPS: f32 = 60.0;
    const TARGET_FRAME_TIME: f32 = 1.0 / TARGET_FPS;

    let mut last_frame_time = glfw.get_time() as f32;
    let mut frame_count = 0;
    let mut fps_timer = Instant::now();
    let mut time = 0.0f32;

    // Rendering state toggles
    let mut wireframe_mode = false;
    let mut use_texture = true;

    // Window loop - keep the window open
    while !window.should_close() {
        // Frame timing - wait until target frame time has elapsed
        let mut delta_time = glfw.get_time() as f32 - last_frame_time;
        while delta_time < TARGET_FRAME_TIME {
            delta_time = glfw.get_time() as f32 - last_frame_time;
        }
        last_frame_time = glfw.get_time() as f32;

        frame_count += 1;
        if fps_timer.elapsed().as_secs() >= 1 {
            // Update window title with FPS
            let title = format!(
                "RustGL by mau | FPS: {} | Frame time: {:.2}ms | Pos: ({:.1}, {:.1}, {:.1})",
                frame_count,
                delta_time * 1000.0,
                camera.position.x,
                camera.position.y,
                camera.position.z,
            );
            window.set_title(&title);
            frame_count = 0;
            fps_timer = Instant::now();
        }

        process_events(&mut window, &events, &mut camera, &mut wireframe_mode, &mut use_texture, delta_time);
        update(delta_time, &mut time);
        render(
            &mut window,
            &sphere,
            &cube,
            &cylinder,
            &torus,
            &plane,
            &shader,
            &texture,
            &camera,
            time,
            wireframe_mode,
            use_texture,
        );
    }
}

fn process_events(
    window: &mut glfw::Window,
    events: &glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    camera: &mut Camera,
    wireframe_mode: &mut bool,
    use_texture: &mut bool,
    delta_time: f32,
) {
    window.glfw.poll_events();

    // Handle window events (resize, key presses, etc.)
    for (_, event) in glfw::flush_messages(events) {
        handle_window_event(window, event, wireframe_mode, use_texture);
    }

    // Process camera input EVERY FRAME (not event-based)
    // This ensures smooth, consistent movement

    // WASD for movement (relative to camera orientation)
    if window.get_key(Key::W) == Action::Press {
        camera.process_keyboard(CameraMovement::Forward, delta_time);
    }
    if window.get_key(Key::S) == Action::Press {
        camera.process_keyboard(CameraMovement::Backward, delta_time);
    }
    if window.get_key(Key::A) == Action::Press {
        camera.process_keyboard(CameraMovement::Left, delta_time);
    }
    if window.get_key(Key::D) == Action::Press {
        camera.process_keyboard(CameraMovement::Right, delta_time);
    }
    if window.get_key(Key::Q) == Action::Press {
        camera.process_keyboard(CameraMovement::Down, delta_time);
    }
    if window.get_key(Key::E) == Action::Press {
        camera.process_keyboard(CameraMovement::Up, delta_time);
    }

    // Arrow keys for looking around
    let look_speed = 250.0; // degrees per second
    if window.get_key(Key::Left) == Action::Press {
        camera.process_mouse_movement(-look_speed * delta_time, 0.0, true);
    }
    if window.get_key(Key::Right) == Action::Press {
        camera.process_mouse_movement(look_speed * delta_time, 0.0, true);
    }
    if window.get_key(Key::Up) == Action::Press {
        camera.process_mouse_movement(0.0, look_speed * delta_time, true);
    }
    if window.get_key(Key::Down) == Action::Press {
        camera.process_mouse_movement(0.0, -look_speed * delta_time, true);
    }
}

fn handle_window_event(
    window: &mut glfw::Window,
    event: glfw::WindowEvent,
    wireframe_mode: &mut bool,
    use_texture: &mut bool,
) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        glfw::WindowEvent::Key(Key::Num1, _, Action::Press, _) => {
            *wireframe_mode = !*wireframe_mode;
            println!("Wireframe mode: {}", if *wireframe_mode { "ON" } else { "OFF" });
        }
        glfw::WindowEvent::Key(Key::Num2, _, Action::Press, _) => {
            *use_texture = !*use_texture;
            println!("Texture: {}", if *use_texture { "ON" } else { "OFF" });
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

fn render(
    window: &mut glfw::Window,
    sphere: &Mesh,
    cube: &Mesh,
    cylinder: &Mesh,
    torus: &Mesh,
    plane: &Mesh,
    shader: &Shader,
    texture: &Texture,
    camera: &Camera,
    time: f32,
    wireframe_mode: bool,
    use_texture: bool,
) {
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::ClearColor(0.1, 0.1, 0.2, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        check_gl_error("clear");

        // Set polygon mode based on wireframe toggle
        if wireframe_mode {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        } else {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
        }

        shader.use_program();

        // NEW: Set lighting uniforms
        let light_pos = glm::vec3(5.0, 5.0, 5.0);           // Light position in world space
        let light_color = glm::vec3(1.0, 1.0, 1.0);         // White light
        shader.set_vec3("lightPos", &light_pos);
        shader.set_vec3("viewPos", &camera.position);        // Camera position
        shader.set_vec3("lightColor", &light_color);

        texture.bind(0);                        // Bind to texture unit 0
        shader.set_int("textureSampler", 0);    // Tell shader to use texture unit 0
        shader.set_bool("useTexture", use_texture);  // Toggle texture based on key press

        let view = camera.get_view_matrix();
        shader.set_mat4("view", &view);

        let projection = glm::perspective(
            1024.0 / 768.0,
            camera.zoom.to_radians(),
            0.1,
            100.0,
        );
        shader.set_mat4("projection", &projection);

        // Draw plane (ground)
        let mut model = glm::Mat4::identity();
        model = glm::translate(&model, &glm::vec3(0.0, -2.0, 0.0));
        shader.set_mat4("model", &model);
        plane.draw();

        // Draw sphere (left, rotating)
        let mut model = glm::Mat4::identity();
        model = glm::translate(&model, &glm::vec3(-4.0, 0.0, 0.0));
        model = glm::rotate(&model, time * 0.5, &glm::vec3(0.0, 1.0, 0.0));
        model = glm::rotate(&model, time * 0.3, &glm::vec3(1.0, 0.0, 0.0));
        shader.set_mat4("model", &model);
        sphere.draw();

        // Draw cube (center-left, rotating)
        let mut model = glm::Mat4::identity();
        model = glm::translate(&model, &glm::vec3(-2.0, 0.0, 0.0));
        model = glm::rotate(&model, time * 0.7, &glm::vec3(1.0, 1.0, 0.0));
        shader.set_mat4("model", &model);
        cube.draw();

        // Draw cylinder (center, rotating)
        let mut model = glm::Mat4::identity();
        model = glm::translate(&model, &glm::vec3(0.0, 0.0, 0.0));
        model = glm::rotate(&model, time * 0.4, &glm::vec3(0.0, 1.0, 0.0));
        model = glm::rotate(&model, time * 0.3, &glm::vec3(1.0, 0.0, 0.0));
        shader.set_mat4("model", &model);
        cylinder.draw();

        // Draw torus (center-right, rotating)
        let mut model = glm::Mat4::identity();
        model = glm::translate(&model, &glm::vec3(2.0, 0.0, 0.0));
        model = glm::rotate(&model, time * 0.6, &glm::vec3(1.0, 0.5, 0.0));
        shader.set_mat4("model", &model);
        torus.draw();

        // Draw another sphere (right, different rotation)
        let mut model = glm::Mat4::identity();
        model = glm::translate(&model, &glm::vec3(4.0, 0.0, 0.0));
        model = glm::rotate(&model, time * 0.8, &glm::vec3(0.5, 1.0, 0.5));
        model = glm::scale(&model, &glm::vec3(0.8, 0.8, 0.8));
        shader.set_mat4("model", &model);
        sphere.draw();
    }
    window.swap_buffers();
}
