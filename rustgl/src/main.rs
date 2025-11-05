extern crate gl;
extern crate glfw;

mod camera;
mod framebuffer;
mod light;
mod material;
mod mesh;
mod scene;
mod shader;
mod texture;
mod transform;

use camera::{Camera, CameraMovement};
use framebuffer::Framebuffer;
use gl::types::*;
use glfw::{Action, Context, Key};
use light::Light;
use material::Material;
use mesh::Mesh;
use nalgebra_glm as glm;
use scene::Scene;
use shader::Shader;
use std::time::Instant;
use texture::Texture;
use transform::Transform;

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

    // Get actual framebuffer size (important for HiDPI/Retina displays)
    let (fb_width, fb_height) = window.get_framebuffer_size();
    let mut framebuffer = Framebuffer::new(fb_width as u32, fb_height as u32);

    let mut bright_pass_fbo = Framebuffer::new(fb_width as u32, fb_height as u32);
    let mut blur_fbo1 = Framebuffer::new(fb_width as u32, fb_height as u32);
    let mut blur_fbo2 = Framebuffer::new(fb_width as u32, fb_height as u32);

    let bright_pass_shader = Shader::new("shader/screen.vert", "shader/bright_pass.frag");
    let blur_shader = Shader::new("shader/screen.vert", "shader/blur.frag");
    let bloom_composite_shader = Shader::new("shader/screen.vert", "shader/bloom_composite.frag");

    let mut bloom_threshold = 0.8;
    let mut bloom_strength = 1.0;
    let blur_iterations = 5;
    let mut bloom_enabled = true;

    let screen_quad = Mesh::screen_quad();
    let screen_shader = Shader::new("shader/screen.vert", "shader/screen.frag");

    let mut scene = Scene::new();

    // Set up skybox
    let skybox_texture = Texture::new_cubemap([
        "resources/textures/skybox/right.jpg",
        "resources/textures/skybox/left.jpg",
        "resources/textures/skybox/top.jpg",
        "resources/textures/skybox/bottom.jpg",
        "resources/textures/skybox/front.jpg",
        "resources/textures/skybox/back.jpg",
    ])
    .expect("Failed to load skybox");
    let skybox_mesh = Mesh::skybox_cube();
    let skybox_shader = Shader::new("shader/skybox.vert", "shader/skybox.frag");
    scene.set_skybox(skybox_mesh, skybox_shader, skybox_texture);

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

    // Add orbiting light sphere (bright white, small)
    scene.add_object(
        Mesh::sphere(1.0, 16, 8, [1.0, 1.0, 1.0]),
        Material::new(
            glm::vec3(1.0, 1.0, 1.0), // High ambient (self-illuminated look)
            glm::vec3(1.0, 1.0, 1.0), // White diffuse
            glm::vec3(1.0, 1.0, 1.0), // White specular
            32.0,                     // Shininess
        ),
        Transform::from_position_scale(glm::vec3(6.0, 2.0, 0.0), glm::vec3(0.3, 0.3, 0.3)),
    );

    // Add static lights
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

    // Add orbiting light (attached to sphere)
    scene.add_light(Light::medium_range(
        glm::vec3(6.0, 2.0, 0.0),
        glm::vec3(10.0, 10.0, 10.0), // Very bright white light
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
    let mut skybox_enabled = true;

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
            let bloom_status = if bloom_enabled { "ON" } else { "OFF" };
            let title = format!(
                "RustGL by mau | FPS: {} | Frame time: {:.2}ms | Pos: ({:.1}, {:.1}, {:.1}) | Bloom: {}",
                frame_count,
                delta_time * 1000.0,
                camera.position.x,
                camera.position.y,
                camera.position.z,
                bloom_status,
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
            &mut skybox_enabled,
            &mut bloom_threshold,
            &mut bloom_strength,
            &mut bloom_enabled,
            delta_time,
            &mut framebuffer,
            &mut bright_pass_fbo,
            &mut blur_fbo1,
            &mut blur_fbo2,
        );
        update(delta_time, &mut time, &mut scene);

        framebuffer.bind();
        render_scene(
            &mut window,
            &scene,
            &shader,
            &texture,
            &camera,
            wireframe_mode,
            use_texture,
            skybox_enabled,
        );

        bright_pass_fbo.bind();
        unsafe {
            gl::Disable(gl::DEPTH_TEST);  // No depth test for post-processing
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);  // Clear to black
            gl::Clear(gl::COLOR_BUFFER_BIT);
            bright_pass_shader.use_program();
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, framebuffer.texture());
            bright_pass_shader.set_int("screenTexture", 0);
            bright_pass_shader.set_float("threshold", bloom_threshold);
            screen_quad.draw();
        }

        let mut horizontal = true;
        let mut first_iteration = true;

        for _ in 0..blur_iterations * 2 {
            if horizontal {
                blur_fbo1.bind();
            } else {
                blur_fbo2.bind();
            }
            unsafe {
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);  // Clear to black
                gl::Clear(gl::COLOR_BUFFER_BIT);
                blur_shader.use_program();
                gl::ActiveTexture(gl::TEXTURE0);

                let source_texture = if first_iteration {
                    bright_pass_fbo.texture()
                } else if horizontal {
                    blur_fbo2.texture()
                } else {
                    blur_fbo1.texture()
                };

                gl::BindTexture(gl::TEXTURE_2D, source_texture);
                blur_shader.set_int("image", 0);
                blur_shader.set_bool("horizontal", horizontal);
                screen_quad.draw();
            }
            horizontal = !horizontal;
            if first_iteration {
                first_iteration = false;
            }
        }

        Framebuffer::unbind();

        unsafe {
            // Restore viewport to window size
            let (fb_width, fb_height) = window.get_framebuffer_size();
            gl::Viewport(0, 0, fb_width, fb_height);

            gl::Disable(gl::DEPTH_TEST);  // No depth test for screen quad
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            if bloom_enabled {
                // Bloom composite
                bloom_composite_shader.use_program();
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, framebuffer.texture());
                bloom_composite_shader.set_int("scene", 0);
                gl::ActiveTexture(gl::TEXTURE1);
                gl::BindTexture(gl::TEXTURE_2D, blur_fbo2.texture());
                bloom_composite_shader.set_int("bloomBlur", 1);
                bloom_composite_shader.set_float("bloomStrength", bloom_strength);
                screen_quad.draw();
            } else {
                // Show raw scene without bloom
                screen_shader.use_program();
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, framebuffer.texture());
                screen_shader.set_int("screenTexture", 0);
                screen_quad.draw();
            }
        }

        window.swap_buffers();
    }
}

fn process_events(
    window: &mut glfw::Window,
    events: &glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    camera: &mut Camera,
    wireframe_mode: &mut bool,
    use_texture: &mut bool,
    skybox_enabled: &mut bool,
    bloom_threshold: &mut f32,
    bloom_strength: &mut f32,
    bloom_enabled: &mut bool,
    delta_time: f32,
    framebuffer: &mut Framebuffer,
    bright_pass_fbo: &mut Framebuffer,
    blur_fbo1: &mut Framebuffer,
    blur_fbo2: &mut Framebuffer,
) {
    window.glfw.poll_events();

    // Handle window events (resize, key presses, etc.)
    for (_, event) in glfw::flush_messages(events) {
        handle_window_event(
            window,
            event,
            wireframe_mode,
            use_texture,
            skybox_enabled,
            bloom_threshold,
            bloom_strength,
            bloom_enabled,
            framebuffer,
            bright_pass_fbo,
            blur_fbo1,
            blur_fbo2,
        );
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
    skybox_enabled: &mut bool,
    bloom_threshold: &mut f32,
    bloom_strength: &mut f32,
    bloom_enabled: &mut bool,
    framebuffer: &mut Framebuffer,
    bright_pass_fbo: &mut Framebuffer,
    blur_fbo1: &mut Framebuffer,
    blur_fbo2: &mut Framebuffer,
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
        glfw::WindowEvent::Key(Key::Num3, _, Action::Press, _) => {
            *bloom_threshold += 0.1;
            println!("Bloom threshold: {:.1}", *bloom_threshold);
        }
        glfw::WindowEvent::Key(Key::Num4, _, Action::Press, _) => {
            *bloom_threshold = (*bloom_threshold - 0.1).max(0.0);
            println!("Bloom threshold: {:.1}", *bloom_threshold);
        }
        glfw::WindowEvent::Key(Key::Num5, _, Action::Press, _) => {
            *bloom_strength += 0.1;
            println!("Bloom strength: {:.1}", *bloom_strength);
        }
        glfw::WindowEvent::Key(Key::Num6, _, Action::Press, _) => {
            *bloom_strength = (*bloom_strength - 0.1).max(0.0);
            println!("Bloom strength: {:.1}", *bloom_strength);
        }
        glfw::WindowEvent::Key(Key::Num7, _, Action::Press, _) => {
            *bloom_enabled = !*bloom_enabled;
            println!("Bloom: {}", if *bloom_enabled { "ON" } else { "OFF" });
        }
        glfw::WindowEvent::Key(Key::Num8, _, Action::Press, _) => {
            *skybox_enabled = !*skybox_enabled;
            println!("Skybox: {}", if *skybox_enabled { "ON" } else { "OFF" });
        }
        glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
            gl::Viewport(0, 0, width, height);
            framebuffer.resize(width as u32, height as u32);
            bright_pass_fbo.resize(width as u32, height as u32);
            blur_fbo1.resize(width as u32, height as u32);
            blur_fbo2.resize(width as u32, height as u32);
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
    // Object indices: 0=plane, 1=sphere, 2=cube, 3=cylinder, 4=torus, 5=chrome sphere, 6=orbiting light sphere

    if let Some(sphere) = scene.get_object_mut(1) {
        sphere.transform.rotate(0.0, 0.5 * delta_time, 0.0);
        sphere.transform.rotate_x(0.3 * delta_time);
    }

    if let Some(cube) = scene.get_object_mut(2) {
        cube.transform
            .rotate(0.7 * delta_time, 0.7 * delta_time, 0.0);
    }

    if let Some(cylinder) = scene.get_object_mut(3) {
        cylinder
            .transform
            .rotate(0.3 * delta_time, 0.4 * delta_time, 0.0);
    }

    if let Some(torus) = scene.get_object_mut(4) {
        torus.transform.rotate(0.0, 0.6 * delta_time, 0.0);
        torus.transform.rotate_x(0.6 * delta_time * 0.5);
    }

    if let Some(chrome_sphere) = scene.get_object_mut(5) {
        chrome_sphere.transform.rotate(
            0.8 * delta_time * 0.5,
            0.8 * delta_time,
            0.8 * delta_time * 0.5,
        );
    }

    // Update orbiting light sphere position
    let orbit_radius = 6.0;
    let orbit_speed = 0.7; // radians per second
    let orbit_height = 2.0;
    let angle = *time * orbit_speed;

    let light_pos = glm::vec3(
        angle.cos() * orbit_radius,
        orbit_height,
        angle.sin() * orbit_radius,
    );

    if let Some(light_sphere) = scene.get_object_mut(6) {
        light_sphere.transform.position = light_pos;
    }

    // Update the orbiting light position to match the sphere
    scene.update_light_position(3, light_pos);
}

fn render_scene(
    window: &mut glfw::Window,
    scene: &Scene,
    shader: &Shader,
    texture: &Texture,
    camera: &Camera,
    wireframe_mode: bool,
    use_texture: bool,
    skybox_enabled: bool,
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

        let view = camera.get_view_matrix();
        let projection = glm::perspective(1024.0 / 768.0, camera.zoom.to_radians(), 0.1, 100.0);

        // Set up scene shader uniforms before rendering
        shader.use_program();
        shader.set_vec3("viewPos", &camera.position);
        texture.bind(0);
        shader.set_int("textureSampler", 0);
        shader.set_bool("useTexture", use_texture);

        // Scene renders skybox internally, then objects
        scene.render(&shader, &view, &projection, skybox_enabled);
    }
}

fn render_to_screen(
    window: &mut glfw::Window,
    screen_quad: &Mesh,
    screen_shader: &Shader,
    texture_id: GLuint,
) {
    unsafe {
        gl::Disable(gl::DEPTH_TEST); // No depth test for screen quad
        gl::ClearColor(1.0, 1.0, 1.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        screen_shader.use_program();
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);
        screen_shader.set_int("screenTexture", 0);

        screen_quad.draw();
    }
    window.swap_buffers();
}
