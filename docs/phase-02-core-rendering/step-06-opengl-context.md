# Step 06: OpenGL Context

**Phase:** 2 - Core Rendering
**Difficulty:** Intermediate
**Estimated Time:** 1 hour

## Goal

Initialize OpenGL and clear the window with a color.

## What You'll Learn

- OpenGL function loading
- The `gl` crate
- OpenGL state machine concepts
- Clearing the screen with a color
- The OpenGL coordinate system

## Background

OpenGL is a graphics API that lets you communicate with your GPU. To use OpenGL from Rust, you need:

1. **OpenGL context** - Created by GLFW (we already have this!)
2. **Function pointers** - Rust bindings to OpenGL functions
3. **Function loading** - Loading the correct OpenGL functions for your system

The `gl` crate provides Rust bindings for OpenGL. GLFW helps us load the function pointers.

## Task

### 1. Add the gl Dependency

Add `gl` to your `Cargo.toml`:

```toml
[dependencies]
glfw = "0.54"
gl = "0.14"
```

### 2. Load OpenGL Functions

Modify your `main.rs`:

```rust
extern crate glfw;
extern crate gl;

use glfw::{Action, Context, Key};
use std::time::Instant;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)
        .expect("Failed to initialize GLFW");

    // Request OpenGL 4.5 Core Profile for Linux
    // Note: For initial learning steps, we'll start with 3.3 for compatibility
    // Later steps will upgrade to 4.5+ for advanced features
    #[cfg(target_os = "linux")]
    {
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 5));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    }

    // macOS limited to OpenGL 4.1 maximum
    #[cfg(target_os = "macos")]
    {
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    }

    // Windows - request OpenGL 4.5
    #[cfg(target_os = "windows")]
    {
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 5));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    }

    let (mut window, events) = glfw
        .create_window(800, 600, "RustGL - OpenGL Context", glfw::WindowMode::Windowed)
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

    let mut last_frame = Instant::now();
    let mut frame_count = 0;
    let mut fps_timer = Instant::now();

    // Main loop
    while !window.should_close() {
        let current_frame = Instant::now();
        let delta_time = current_frame.duration_since(last_frame).as_secs_f32();
        last_frame = current_frame;

        frame_count += 1;
        if fps_timer.elapsed().as_secs() >= 1 {
            println!("FPS: {}", frame_count);
            frame_count = 0;
            fps_timer = Instant::now();
        }

        process_events(&mut glfw, &mut window, &events);
        update(delta_time);
        render(&mut window);
    }
}

fn process_events(
    glfw: &mut glfw::Glfw,
    window: &mut glfw::Window,
    events: &glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
) {
    glfw.poll_events();
    for (_, event) in glfw::flush_messages(events) {
        handle_window_event(window, event);
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        glfw::WindowEvent::FramebufferSize(width, height) => {
            unsafe {
                gl::Viewport(0, 0, width, height);
            }
        }
        _ => {}
    }
}

fn update(_delta_time: f32) {}

fn render(_window: &mut glfw::Window) {
    unsafe {
        // Set clear color (R, G, B, A) - dark blue
        gl::ClearColor(0.1, 0.1, 0.2, 1.0);

        // Clear the color buffer
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }

    _window.swap_buffers();
}
```

### 3. Build and Run

```bash
cargo run
```

You should see a dark blue window! The OpenGL version should be printed to the console.

## Understanding the Code

**OpenGL Version Hints:**
```rust
#[cfg(target_os = "linux")]
glfw.window_hint(glfw::WindowHint::ContextVersion(4, 5));
```
- On Linux, we request OpenGL 4.5 Core Profile for maximum feature access
- Core profile = modern OpenGL (no deprecated features)
- macOS is limited to OpenGL 4.1 (legacy limitation by Apple)
- We use conditional compilation (`#[cfg(target_os = "...")]`) for portability
- Later steps will use OpenGL 4.x features like compute shaders and tessellation

**Loading Function Pointers:**
```rust
gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
```
- GLFW finds the OpenGL functions on your system
- The `gl` crate stores these function pointers
- This must happen AFTER creating the OpenGL context

**Unsafe Blocks:**
```rust
unsafe {
    gl::ClearColor(0.1, 0.1, 0.2, 1.0);
}
```
- All OpenGL calls are marked `unsafe` in Rust
- This is because OpenGL is a C API with no safety guarantees
- Incorrect OpenGL usage can cause crashes or undefined behavior
- `unsafe` means "I promise I'm using this correctly"

**Setting Clear Color:**
```rust
gl::ClearColor(0.1, 0.1, 0.2, 1.0);
```
- RGBA values from 0.0 to 1.0
- (0.1, 0.1, 0.2, 1.0) = dark blue, fully opaque
- This sets the clear color in OpenGL's state machine

**Clearing the Screen:**
```rust
gl::Clear(gl::COLOR_BUFFER_BIT);
```
- Clears the color buffer
- Uses the color set by `glClearColor`
- Later we'll also clear the depth buffer

**Viewport Resize:**
```rust
glfw::WindowEvent::FramebufferSize(width, height) => {
    unsafe { gl::Viewport(0, 0, width, height); }
}
```
- When the window is resized, update OpenGL's viewport
- Viewport = the region where OpenGL renders
- `(0, 0, width, height)` = full window

## Challenges

1. **Different colors:** Try different clear colors:
   - Red: `(1.0, 0.0, 0.0, 1.0)`
   - Green: `(0.0, 1.0, 0.0, 1.0)`
   - Purple: `(0.5, 0.0, 0.5, 1.0)`

2. **Animated color:** Make the clear color change over time:
   ```rust
   // In main, before loop:
   let mut time = 0.0f32;

   // In update:
   fn update(delta_time: f32, time: &mut f32) {
       *time += delta_time;
   }

   // In render:
   fn render(window: &mut glfw::Window, time: f32) {
       unsafe {
           let r = (time.sin() + 1.0) / 2.0;  // Oscillate between 0 and 1
           gl::ClearColor(r, 0.2, 0.4, 1.0);
           gl::Clear(gl::COLOR_BUFFER_BIT);
       }
       window.swap_buffers();
   }
   ```

3. **Check for errors:** Add OpenGL error checking:
   ```rust
   fn check_gl_error(location: &str) {
       unsafe {
           let err = gl::GetError();
           if err != gl::NO_ERROR {
               println!("OpenGL Error at {}: {}", location, err);
           }
       }
   }

   // Use it after OpenGL calls:
   gl::Clear(gl::COLOR_BUFFER_BIT);
   check_gl_error("clear");
   ```

## Success Criteria

- [ ] Your program compiles and runs
- [ ] The window is colored (not black)
- [ ] OpenGL version is printed to the console
- [ ] The window can be resized without issues
- [ ] (Optional) You've tried different colors

## Common Issues

**Black screen instead of colored**
- Make sure you're calling `gl::ClearColor` BEFORE `gl::Clear`
- Check that `gl::load_with` was called after `make_current`

**"failed to load symbol"**
- Your system might not support the requested OpenGL version
- On Linux, check your driver: `glxinfo | grep "OpenGL version"`
- On Linux, ensure you have proper GPU drivers installed (nvidia, mesa, amd)
- Try lowering the version (e.g., 4.3 instead of 4.5) if needed

**Crash on startup**
- Make sure you load OpenGL functions AFTER creating the window
- Check that you're calling `window.make_current()` before `gl::load_with`

**Linux-specific issues**
- Ensure you have the correct GPU drivers installed
- For NVIDIA: install proprietary drivers via your package manager
- For AMD/Intel: mesa drivers should work (`sudo apt install mesa-utils`)
- Verify with: `glxinfo | grep "OpenGL version"` (should show 4.5 or higher)

**macOS-specific issues**
- Make sure you have the forward compatibility hint: `OpenGlForwardCompat(true)`
- macOS only supports up to OpenGL 4.1 (Apple limitation)
- Advanced features in later steps won't work on macOS

## Next Step

Fantastic! You now have OpenGL initialized and can clear the screen. Next: [Step 07: First Triangle](./step-07-first-triangle.md), where you'll finally render some geometry!

## Notes

- OpenGL is a state machine - functions change global state
- We're targeting OpenGL 4.5 on Linux for advanced features
- All rendering happens between `Clear` and `swap_buffers`
- The framebuffer has two buffers: front (displayed) and back (drawing)
- `swap_buffers` swaps them (double buffering prevents flicker)
- **Platform portability**: We use `#[cfg(target_os = "...")]` to request the highest OpenGL version each platform supports
- Linux will give us access to compute shaders, tessellation, and other OpenGL 4.x features
