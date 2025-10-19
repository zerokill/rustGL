# Step 04: Window Creation

**Phase:** 1 - Foundation
**Difficulty:** Intermediate
**Estimated Time:** 1 hour

## Goal

Create a graphical window that you can see on your screen.

## What You'll Learn

- Choosing between `glfw` and `winit`
- Initializing a window library
- Creating a window with specific dimensions
- Basic error handling with `Result`
- The `?` operator for error propagation

## Background

Before you can render graphics, you need a window! There are two popular options in Rust:

**Option A: GLFW** (Recommended for this project)
- ✅ Similar to what SwiftGL uses
- ✅ Simple API
- ✅ Includes OpenGL context creation
- ✅ Good documentation
- ❌ Requires C libraries

**Option B: winit**
- ✅ Pure Rust
- ✅ Modern, actively maintained
- ✅ Used by many Rust game engines
- ❌ Doesn't handle OpenGL context (need `glutin`)
- ❌ More complex API

**For learning purposes, we'll use GLFW** because it's simpler and matches your SwiftGL experience.

## Task

### 1. Add Dependencies

Add `glfw` to your `Cargo.toml`:

```toml
[dependencies]
glfw = "0.54"
```

### 2. Install GLFW System Library

**macOS:**
```bash
brew install glfw
```

**Ubuntu/Debian:**
```bash
sudo apt-get install libglfw3-dev
```

**Windows:**
- Download from [glfw.org](https://www.glfw.org/download.html)
- Or use `vcpkg`: `vcpkg install glfw3`

### 3. Create Your Window

Replace the contents of `src/main.rs`:

```rust
extern crate glfw;

use glfw::{Action, Context, Key};

fn main() {
    // Initialize GLFW
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)
        .expect("Failed to initialize GLFW");

    // Create a window
    let (mut window, events) = glfw
        .create_window(
            800,           // Width
            600,           // Height
            "RustGL",      // Title
            glfw::WindowMode::Windowed,
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
```

### 4. Build and Run

```bash
cargo run
```

You should see a black window appear! Press Escape to close it.

## Understanding the Code

Let's break down what's happening:

**Initialization:**
```rust
let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)
    .expect("Failed to initialize GLFW");
```
- Initializes the GLFW library
- `FAIL_ON_ERRORS` - Errors trigger a callback
- `expect()` - Unwraps the `Result`, panics with a message if it fails

**Window Creation:**
```rust
let (mut window, events) = glfw.create_window(...)
    .expect("Failed to create GLFW window");
```
- Returns a tuple: `(Window, Receiver<WindowEvent>)`
- `window` - The window handle
- `events` - A channel for receiving window events

**Event Loop:**
```rust
while !window.should_close() {
    glfw.poll_events();  // Check for new events
    // ... handle events ...
    window.swap_buffers();  // Display what was drawn
}
```
- Continues until the window is closed
- `poll_events()` - Processes input, window resize, etc.
- `swap_buffers()` - Swaps the front and back buffers (double buffering)

**Event Handling:**
```rust
match event {
    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
        window.set_should_close(true);
    }
    _ => {}
}
```
- Pattern matching on events
- When Escape is pressed, mark window for closing
- `_` ignores other events for now

## Challenges

1. **Change window size:** Make the window 1024x768
2. **Change the title:** Use a different window title
3. **Fullscreen mode:** Try creating a fullscreen window:
   ```rust
   glfw::WindowMode::FullScreen(glfw.with_primary_monitor(|_, m| m.unwrap()))
   ```
4. **More keys:** Handle more keys (e.g., print a message when Space is pressed):
   ```rust
   glfw::WindowEvent::Key(Key::Space, _, Action::Press, _) => {
       println!("Space pressed!");
   }
   ```
5. **Window callbacks:** Enable window size polling and print when the window is resized:
   ```rust
   window.set_size_polling(true);

   // In event loop:
   glfw::WindowEvent::Size(width, height) => {
       println!("Window resized to {}x{}", width, height);
   }
   ```

## Success Criteria

- [ ] GLFW library is installed on your system
- [ ] Your program compiles without errors
- [ ] A black window appears when you run the program
- [ ] The window has the title "RustGL"
- [ ] Pressing Escape closes the window
- [ ] (Optional) You've tried the challenges

## Common Issues

**"could not find native static library `glfw3`"**
- GLFW system library isn't installed
- Run `brew install glfw` (macOS) or equivalent for your OS
- On Windows, ensure GLFW is in your PATH

**Window appears then immediately closes**
- Make sure you have the event loop (`while !window.should_close()`)
- Check that you're calling `glfw.poll_events()`

**"multiple applicable items in scope"**
- Add `use glfw::Context;` at the top
- This imports the `Context` trait needed for some methods

**Compilation errors about traits**
- Make sure you have `use glfw::{Action, Context, Key};`
- These are needed for the window event handling

## Next Step

Excellent! You now have a window. Next: [Step 05: Event Loop](./step-05-event-loop.md), where you'll improve your event handling and prepare for rendering!

## Notes

- The black window is normal - we haven't rendered anything yet!
- `swap_buffers()` is necessary even if we're not drawing (it updates the window)
- GLFW handles OpenGL context creation for us automatically
- The event loop is the heart of any game engine
- We're using double buffering (draw to back buffer, swap to front)
