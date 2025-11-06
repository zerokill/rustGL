extern crate gl;
extern crate glfw;

mod bloom_renderer;
mod camera;
mod framebuffer;
mod light;
mod material;
mod mesh;
mod scene;
mod shader;
mod texture;
mod transform;
mod godray_renderer;

use bloom_renderer::BloomRenderer;
use camera::{Camera, CameraMovement};
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
use godray_renderer::GodRayRenderer;
use egui_glfw::egui;

struct AppState {
    wireframe_mode: bool,
    use_texture: bool,
    skybox_enabled: bool,

    bloom_threshold: f32,
    bloom_strength: f32,
    bloom_enabled: bool,

    godray_strength: f32,
    godray_exposure: f32,
    godray_decay: f32,
    godray_debug_mode: u8,  // 0 = off, 1 = occlusion, 2 = radial blur, 3 = rays only
}

impl AppState {
    fn new() -> Self {
        AppState {
            wireframe_mode: false,
            use_texture: true,
            skybox_enabled: true,

            bloom_threshold: 0.8,
            bloom_strength: 1.0,
            bloom_enabled: true,

            godray_strength: 1.0,
            godray_exposure: 0.5,
            godray_decay: 0.97,
            godray_debug_mode: 0,
        }
    }
}

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
    window.set_cursor_pos_polling(true);
    window.set_mouse_button_polling(true);
    window.set_scroll_polling(true);
    window.set_char_polling(true);

    // Enable V-Sync to cap FPS at monitor refresh rate (usually 60 FPS)
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    // Load OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // Print OpenGL version info
    unsafe {
        let version = std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8);
        println!("OpenGL Version: {}", version.to_str().unwrap());
    }

    // Get actual framebuffer size (important for HiDPI/Retina displays)
    let (fb_width, fb_height) = window.get_framebuffer_size();

    // Initialize egui
    let mut egui_painter = egui_glfw::Painter::new(&mut window);
    // CRITICAL: Set painter size to framebuffer dimensions (physical pixels) for HiDPI
    egui_painter.set_size(fb_width as u32, fb_height as u32);
    let egui_ctx = egui::Context::default();

    // Set initial pixels_per_point for HiDPI displays
    let native_pixels_per_point = window.get_content_scale().0;
    egui_ctx.set_pixels_per_point(native_pixels_per_point);

    // Use window size (logical pixels) for screen_rect
    let (window_width, window_height) = window.get_size();
    let mut egui_input = egui_glfw::EguiInputState::new(egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::new(0f32, 0f32),
            egui::vec2(window_width as f32, window_height as f32),
        )),
        ..Default::default()
    });
    egui_input.input.time = Some(0.01);

    let shader = Shader::new("shader/basic.vert", "shader/basic.frag");
    // Load a test texture
    let texture = Texture::new("resources/textures/livia.png").expect("Failed to load texture");

    // Create bloom renderer (handles all framebuffers and post-processing)
    let mut bloom_renderer = BloomRenderer::new(fb_width as u32, fb_height as u32);
    let mut godray_renderer = GodRayRenderer::new(fb_width as u32, fb_height as u32);

    let mut state = AppState::new();

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
            let bloom_status = if state.bloom_enabled { "ON" } else { "OFF" };
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
            &mut state,
            &mut bloom_renderer,
            &mut godray_renderer,
            &mut egui_painter,
            &mut egui_input,
            &egui_ctx,
            delta_time,
        );
        update(delta_time, &mut time, &mut scene);

        // Render scene with bloom post-processing
        let (fb_width, fb_height) = window.get_framebuffer_size();
        bloom_renderer.render(
            || {
                render_scene(
                    &scene,
                    &shader,
                    &texture,
                    &camera,
                    &state,
                );
            },
            state.bloom_threshold,
            state.bloom_strength,
            state.bloom_enabled,
            fb_width,
            fb_height,
        );

        // In render loop - after bloom
        let light_pos = scene.lights()[3].position;
        let view = camera.get_view_matrix();
        let projection = glm::perspective(fb_width as f32 / fb_height as f32, camera.zoom.to_radians(), 0.1, 100.0);

        // Update godray parameters from UI state
        godray_renderer.exposure = state.godray_exposure;
        godray_renderer.decay = state.godray_decay;

        godray_renderer.apply(
            bloom_renderer.composite_texture(),
            &scene,
            6,  // orb_index
            light_pos,
            &view,
            &projection,
            state.godray_strength,
            state.godray_debug_mode,
            fb_width,
            fb_height,
        );

        // Render UI
        egui_input.input.time = Some(glfw.get_time());

        // IMPORTANT: Update screen_rect every frame because take() consumes it
        // Use window size (logical pixels) - egui scales with pixels_per_point
        let (width, height) = window.get_size();
        egui_input.input.screen_rect = Some(egui::Rect::from_min_size(
            egui::Pos2::new(0f32, 0f32),
            egui::vec2(width as f32, height as f32),
        ));

        egui_ctx.begin_frame(egui_input.input.take());
        render_ui(&egui_ctx, &mut state, delta_time, frame_count, &camera);

        let egui::FullOutput {
            platform_output,
            textures_delta,
            shapes,
            pixels_per_point,
            ..
        } = egui_ctx.end_frame();

        // Apply pixels_per_point from egui back to context for next frame
        egui_ctx.set_pixels_per_point(pixels_per_point);

        // Handle clipboard
        if !platform_output.copied_text.is_empty() {
            egui_glfw::copy_to_clipboard(&mut egui_input, platform_output.copied_text);
        }

        let clipped_shapes = egui_ctx.tessellate(shapes, pixels_per_point);

        // Set up OpenGL state for egui rendering
        let (fb_width, fb_height) = window.get_framebuffer_size();
        unsafe {
            gl::Viewport(0, 0, fb_width, fb_height);
            gl::Disable(gl::DEPTH_TEST);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        egui_painter.paint_and_update_textures(pixels_per_point, &clipped_shapes, &textures_delta);

        window.swap_buffers();
    }
}

fn process_events(
    window: &mut glfw::Window,
    events: &glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    camera: &mut Camera,
    state: &mut AppState,
    bloom_renderer: &mut BloomRenderer,
    godray_renderer: &mut GodRayRenderer,
    egui_painter: &mut egui_glfw::Painter,
    egui_input: &mut egui_glfw::EguiInputState,
    egui_ctx: &egui::Context,
    delta_time: f32,
) {
    window.glfw.poll_events();

    // Handle events
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::Close => {
                window.set_should_close(true);
            }
            glfw::WindowEvent::FramebufferSize(width, height) => {
                bloom_renderer.resize(width as u32, height as u32);
                godray_renderer.resize(width as u32, height as u32);

                let (win_width, win_height) = window.get_size();

                unsafe {
                    gl::Viewport(0, 0, width, height);
                }

                // Update egui painter canvas size (physical pixels)
                egui_painter.set_size(width as u32, height as u32);

                // IMPORTANT: Let egui_glfw handle resize to update screen_rect
                // Pass window size (logical pixels), not framebuffer size
                egui_glfw::handle_event(
                    glfw::WindowEvent::FramebufferSize(win_width, win_height),
                    egui_input
                );
            }
            glfw::WindowEvent::Key(key, _, action, _) => {
                handle_key_event(key, action, state, window);
            }
            glfw::WindowEvent::CursorPos(x, y) => {
                // Let egui_glfw handle cursor events normally (expects window coordinates)
                egui_glfw::handle_event(glfw::WindowEvent::CursorPos(x, y), egui_input);
            }
            _ => {
                egui_glfw::handle_event(event, egui_input);
            }
        }
    }

    // Process camera input EVERY FRAME (not event-based)
    // This ensures smooth, consistent movement
    // Only block camera if UI has pointer focus (dragging sliders, clicking buttons)
    // We don't have text input fields, so keyboard is always available for camera
    if !egui_ctx.wants_pointer_input() {
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
}

fn handle_key_event(
    key: Key,
    action: Action,
    _state: &mut AppState,
    window: &mut glfw::Window,
) {
    match (key, action) {
        (Key::Escape, Action::Press) => {
            window.set_should_close(true);
        }
        _ => {}
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
    let orbit_speed = 0.5; // radians per second
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
    scene: &Scene,
    shader: &Shader,
    texture: &Texture,
    camera: &Camera,
    state: &AppState,
) {
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::ClearColor(0.1, 0.1, 0.2, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

        // Set polygon mode based on wireframe toggle
        if state.wireframe_mode {
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
        shader.set_bool("useTexture", state.use_texture);

        // Scene renders skybox internally, then objects
        scene.render(&shader, &view, &projection, state.skybox_enabled);
    }
}

fn render_ui(
    egui_ctx: &egui::Context,
    state: &mut AppState,
    delta_time: f32,
    frame_count: u32,
    camera: &Camera,
) {
    // Main debug panel
    egui::Window::new("🎮 RustGL Debug Panel")
        .default_width(300.0)
        .show(egui_ctx, |ui| {
            ui.heading("Performance");
            ui.separator();

            // FPS display
            let fps = 1.0 / delta_time;
            ui.label(format!("FPS: {:.0}", fps));

            ui.add_space(10.0);

            // Camera position
            ui.heading("Camera");
            ui.separator();
            ui.label(format!(
                "Position: ({:.1}, {:.1}, {:.1})",
                camera.position.x, camera.position.y, camera.position.z
            ));

            ui.add_space(10.0);

            // Rendering toggles
            ui.heading("Rendering");
            ui.separator();
            ui.checkbox(&mut state.wireframe_mode, "Wireframe Mode");
            ui.checkbox(&mut state.use_texture, "Use Textures");
            ui.checkbox(&mut state.skybox_enabled, "Skybox");

            ui.add_space(10.0);

            // Bloom controls
            ui.heading("Bloom Post-Processing");
            ui.separator();
            ui.checkbox(&mut state.bloom_enabled, "Enable Bloom");

            if state.bloom_enabled {
                ui.add(
                    egui::Slider::new(&mut state.bloom_threshold, 0.0..=2.0)
                        .text("Threshold")
                );
                ui.add(
                    egui::Slider::new(&mut state.bloom_strength, 0.0..=3.0)
                        .text("Strength")
                );
            }

            ui.add_space(10.0);

            // God ray controls
            ui.heading("God Rays");
            ui.separator();

            ui.add(
                egui::Slider::new(&mut state.godray_strength, 0.0..=2.0)
                    .text("Strength")
            );
            ui.add(
                egui::Slider::new(&mut state.godray_exposure, 0.0..=2.0)
                    .text("Exposure")
            );
            ui.add(
                egui::Slider::new(&mut state.godray_decay, 0.8..=1.0)
                    .text("Decay")
            );

            ui.add_space(5.0);
            ui.label("Debug Mode:");
            ui.radio_value(&mut state.godray_debug_mode, 0, "Off (Normal)");
            ui.radio_value(&mut state.godray_debug_mode, 1, "Occlusion Buffer");
            ui.radio_value(&mut state.godray_debug_mode, 2, "Radial Blur");

            ui.add_space(10.0);

            // Keyboard shortcuts help
            ui.heading("Controls");
            ui.separator();
            ui.label("WASD - Move camera");
            ui.label("QE - Move up/down");
            ui.label("Arrows - Look around");
            ui.label("ESC - Quit");
        });
}
