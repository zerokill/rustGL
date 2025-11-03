extern crate gl;
extern crate glfw;

mod camera;
mod light;
mod material;
mod mesh;
mod shader;
mod texture;
mod transform;
mod scene;

use camera::{Camera, CameraMovement};
use glfw::{Action, Context, Key};
use light::Light;
use material::Material;
use mesh::Mesh;
use nalgebra_glm as glm;
use shader::Shader;
use std::time::Instant;
use texture::Texture;
use transform::Transform;
use scene::Scene;

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

    let shader = Shader::new("shader/basic.vert", "shader/basic.frag");
    // Load a test texture
    let texture = Texture::new("resources/textures/livia.png").expect("Failed to load texture");
    let mut scene = Scene::new();

    scene.add_object(
        Mesh::plane(10.0, 10.0, [0.3, 0.3, 0.3]),
        Material::matte(glm::vec3(0.2, 1.0, 0.3)),
        Transform::from_position(glm::vec3(0.0, -2.0, 0.0)),
    );

    // Add rotating sphere (left)
    scene.add_object(
        Mesh::sphere(1.0, 32, 16, [0.3, 0.7, 1.0]),
        Material::plastic(glm::vec3(0.3, 0.7, 1.0)),
        Transform::from_position(glm::vec3(-4.0, 0.0, 0.0)),
    );

    // Add rotating cube (center-left)
    scene.add_object(
        Mesh::cube([1.0, 0.5, 0.2]),
        Material::metal(glm::vec3(1.0, 0.5, 0.2)),
        Transform::from_position(glm::vec3(-2.0, 0.0, 0.0)),
    );

    // Add rotating cylinder (center)
    scene.add_object(
        Mesh::cylinder(0.5, 2.0, 32, [0.2, 1.0, 0.3]),
        Material::matte(glm::vec3(0.2, 1.0, 0.3)),
        Transform::from_position(glm::vec3(0.0, 0.0, 0.0)),
    );

    // Add rotating torus (center-right)
    scene.add_object(
        Mesh::torus(1.0, 0.3, 32, 16, [1.0, 0.3, 0.7]),
        Material::rubber(glm::vec3(1.0, 0.3, 0.7)),
        Transform::from_position(glm::vec3(2.0, 0.0, 0.0)),
    );

    // Add small chrome sphere (right)
    scene.add_object(
        Mesh::sphere(1.0, 32, 16, [0.8, 0.8, 0.8]),
        Material::chrome(),
        Transform::from_position_scale(glm::vec3(4.0, 0.0, 0.0), glm::vec3(0.8, 0.8, 0.8)),
    );

    // Add lights
    scene.add_light(Light::long_range(
        glm::vec3(5.0, 5.0, 5.0),
        glm::vec3(5.0, 5.0, 5.0),
    ));
    scene.add_light(Light::medium_range(
        glm::vec3(-5.0, 2.0, 0.0),
        glm::vec3(4.0, 0.6, 0.6),
    ));
    scene.add_light(Light::medium_range(
        glm::vec3(5.0, 2.0, -3.0),
        glm::vec3(0.6, 1.2, 4.0),
    ));
    scene.add_light(Light::short_range(
        glm::vec3(0.0, 1.0, 5.0),
        glm::vec3(1.0, 3.0, 1.0),
    ));

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

        process_events(
            &mut window,
            &events,
            &mut camera,
            &mut wireframe_mode,
            &mut use_texture,
            delta_time,
        );
        update(delta_time, &mut time, &mut scene);
        render(
            &mut window,
            &scene,
            &shader,
            &texture,
            &camera,
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
            println!(
                "Wireframe mode: {}",
                if *wireframe_mode { "ON" } else { "OFF" }
            );
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

fn update(delta_time: f32, time: &mut f32, scene: &mut Scene) {
    // Game logic
    *time += delta_time;

    // Animate objects by updating their transforms
    // Object indices: 0=plane, 1=sphere, 2=cube, 3=cylinder, 4=torus, 5=chrome sphere

    if let Some(sphere) = scene.get_object_mut(1) {
        sphere.transform.rotate(0.0, 0.5 * delta_time, 0.0);
        sphere.transform.rotate_x(0.3 * delta_time);
    }

    if let Some(cube) = scene.get_object_mut(2) {
        cube.transform.rotate(0.7 * delta_time, 0.7 * delta_time, 0.0);
    }

    if let Some(cylinder) = scene.get_object_mut(3) {
        cylinder.transform.rotate(0.3 * delta_time, 0.4 * delta_time, 0.0);
    }

    if let Some(torus) = scene.get_object_mut(4) {
        torus.transform.rotate(0.0, 0.6 * delta_time, 0.0);
        torus.transform.rotate_x(0.6 * delta_time * 0.5);
    }

    if let Some(chrome_sphere) = scene.get_object_mut(5) {
        chrome_sphere.transform.rotate(0.8 * delta_time * 0.5, 0.8 * delta_time, 0.8 * delta_time * 0.5);
    }
}

fn render(
    window: &mut glfw::Window,
    scene: &Scene,
    shader: &Shader,
    texture: &Texture,
    camera: &Camera,
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

        // Set camera-related uniforms
        shader.set_vec3("viewPos", &camera.position);

        texture.bind(0); // Bind to texture unit 0
        shader.set_int("textureSampler", 0); // Tell shader to use texture unit 0
        shader.set_bool("useTexture", use_texture); // Toggle texture based on key press

        let view = camera.get_view_matrix();
        let projection = glm::perspective(1024.0 / 768.0, camera.zoom.to_radians(), 0.1, 100.0);

        scene.render(&shader, &view, &projection);
    }
    window.swap_buffers();
}
