extern crate glfw;

use glfw::{Action, Context, Key};
use std::time::Instant;

fn main() {
    // Initialize GLFW
    let mut glfw = glfw::init_no_callbacks().expect("Failed to initialize GLFW");

    // Create a window
    let (mut window, events) = glfw
        .create_window(
            1024,            // Width
            768,             // Height
            "RustGL by mau", // Title
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window");

    // Make the window's context current
    window.make_current();

    // Enable key event polling
    window.set_key_polling(true);

    let mut last_frame = Instant::now();
    let mut frame_count = 0;
    let mut fps_timer = Instant::now();

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

        update(delta_time);

        render(&mut window);
    }
}

fn process_events(
    window: &mut glfw::Window,
    events: &glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
) {
    window.glfw.poll_events();
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true);
            }
            glfw::WindowEvent::Key(Key::Space, _, Action::Press, _) => {
                println!("Space pressed!");
            }
            _ => {}
        }
    }
}

fn update(delta_time: f32) {
    // Game logic
}

fn render(window: &mut glfw::Window) {
    window.swap_buffers();
}
