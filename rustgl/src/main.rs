extern crate glfw;

use glfw::{Action, Context, Key};

fn main() {
    // Initialize GLFW
    let mut glfw = glfw::init_no_callbacks()
        .expect("Failed to initialize GLFW");

    // Create a window
    let (mut window, events) = glfw
        .create_window(
            1024,           // Width
            768,           // Height
            "RustGL by mau",      // Title
            glfw::WindowMode::Windowed
        )
        .expect("Failed to create GLFW window");

    // Make the window's context current
    window.make_current();

    // Enable key event polling
    window.set_key_polling(true);

    // Window loop - keep the window open
    while !window.should_close() {
        // Poll for events
        glfw.poll_events();

        // Check for events
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        // Swap front and back buffers
        window.swap_buffers();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        _ => {}
    }
}
