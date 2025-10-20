# Step 05: Event Loop

**Phase:** 1 - Foundation
**Difficulty:** Intermediate
**Estimated Time:** 45 minutes

## Goal

Improve your event loop with better structure and frame timing.

## What You'll Learn

- Frame timing and delta time
- Organizing code into functions
- The difference between polling and waiting for events
- Mutable references (`&mut`)
- Basic Rust ownership concepts

## Background

A game engine's event loop (also called the "game loop") is its heartbeat. The basic structure is:

```
loop:
    1. Process input
    2. Update game state (physics, AI, etc.)
    3. Render the frame
    4. Repeat
```

Right now, your loop just processes events and swaps buffers. Let's make it more structured and add timing so you can track:
- **FPS (Frames Per Second)** - How fast your engine runs
- **Delta time** - Time elapsed since last frame (for smooth movement)

## Task

### 1. Add Timing

Modify your `src/main.rs` to include frame timing:

```rust
extern crate glfw;

use glfw::{Action, Context, Key};
use std::time::Instant;

fn main() {
    // Initialize GLFW
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)
        .expect("Failed to initialize GLFW");

    // Create window
    let (mut window, events) = glfw
        .create_window(800, 600, "RustGL - Event Loop", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);

    // Frame timing
    let mut last_frame = Instant::now();
    let mut frame_count = 0;
    let mut fps_timer = Instant::now();

    // Main loop
    while !window.should_close() {
        // Calculate delta time
        let current_frame = Instant::now();
        let delta_time = current_frame.duration_since(last_frame).as_secs_f32();
        last_frame = current_frame;

        // Update FPS counter every second
        frame_count += 1;
        if fps_timer.elapsed().as_secs() >= 1 {
            println!("FPS: {} | Frame time: {:.2}ms", frame_count, delta_time * 1000.0);
            frame_count = 0;
            fps_timer = Instant::now();
        }

        // Process events
        process_events(&mut window, &events);

        // Update (empty for now)
        update(delta_time);

        // Render (empty for now)
        render(&mut window);
    }
}

fn process_events(window: &mut glfw::Window, events: &glfw::GlfwReceiver<(f64, glfw::WindowEvent)>) {
    window.glfw.poll_events();  // Access glfw through the window
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

fn update(_delta_time: f32) {
    // Game logic will go here
    // delta_time tells us how much time has passed since the last frame
    // This allows for smooth, framerate-independent movement
}

fn render(window: &mut glfw::Window) {
    // Rendering will go here
    window.swap_buffers();
}
```

### 2. Understanding the Code

**Key point:** The glfw-rs crate allows you to access the `Glfw` instance through `window.glfw`, which is why this works!

```rust
extern crate glfw;

use glfw::{Action, Context, Key};
use std::time::Instant;

fn main() {
    let mut glfw = glfw::init_no_callbacks()
        .expect("Failed to initialize GLFW");

    let (mut window, events) = glfw
        .create_window(800, 600, "RustGL - Event Loop", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);

    let mut last_frame = Instant::now();
    let mut frame_count = 0;
    let mut fps_timer = Instant::now();

    while !window.should_close() {
        let current_frame = Instant::now();
        let delta_time = current_frame.duration_since(last_frame).as_secs_f32();
        last_frame = current_frame;

        frame_count += 1;
        if fps_timer.elapsed().as_secs() >= 1 {
            println!("FPS: {} | Frame time: {:.2}ms", frame_count, delta_time * 1000.0);
            frame_count = 0;
            fps_timer = Instant::now();
        }

        // Process events
        process_events(&mut window, &events);

        // Update game state
        update(delta_time);

        // Render
        render(&mut window);
    }
}

fn process_events(window: &mut glfw::Window, events: &glfw::GlfwReceiver<(f64, glfw::WindowEvent)>) {
    window.glfw.poll_events();  // Access glfw through the window
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

fn update(_delta_time: f32) {
    // Game logic will go here
}

fn render(window: &mut glfw::Window) {
    // Rendering will go here
    window.swap_buffers();
}
```

### 3. Build and Run

```bash
cargo run
```

You should see FPS information printed to the console every second!

## Understanding the Code

**Delta Time:**
```rust
let delta_time = current_frame.duration_since(last_frame).as_secs_f32();
```
- Measures time elapsed since the last frame
- Used to make movement framerate-independent
- Example: `position += velocity * delta_time`

**FPS Counter:**
```rust
if fps_timer.elapsed().as_secs() >= 1 {
    println!("FPS: {}", frame_count);
    frame_count = 0;
    fps_timer = Instant::now();
}
```
- Counts frames
- Every second, prints FPS and resets counter

**Function Signatures:**
```rust
fn update(_delta_time: f32) {
```
- The `_` prefix means "intentionally unused" (suppresses warnings)
- `f32` is a 32-bit floating-point number

**Mutable References:**
```rust
fn render(window: &mut glfw::Window) {
```
- `&mut` means "mutable borrow"
- The function can modify the window
- Only one mutable borrow can exist at a time (Rust's ownership rules)

## Challenges

1. **Track maximum FPS:** Keep track of the highest FPS seen and print it

2. **Add a simple animation:** Create a counter that increments by delta_time:
   ```rust
   // At top of main:
   let mut time_elapsed = 0.0f32;

   // In update:
   fn update(delta_time: f32, time_elapsed: &mut f32) {
       *time_elapsed += delta_time;
       println!("Time elapsed: {:.2}s", time_elapsed);
   }
   ```

3. **Limit framerate:** Implement a simple frame limiter:
   ```rust
   use std::thread;
   use std::time::Duration;

   const TARGET_FPS: u32 = 60;
   const FRAME_TIME: f32 = 1.0 / TARGET_FPS as f32;

   // At end of main loop:
   let frame_duration = current_frame.elapsed().as_secs_f32();
   if frame_duration < FRAME_TIME {
       let sleep_time = FRAME_TIME - frame_duration;
       thread::sleep(Duration::from_secs_f32(sleep_time));
   }
   ```

4. **More input:** Add handling for arrow keys, WASD, etc.

## Success Criteria

- [ ] Your program compiles and runs
- [ ] FPS is printed to the console every second
- [ ] Delta time is calculated each frame
- [ ] The code is organized into separate functions
- [ ] Pressing Space prints a message
- [ ] Pressing Escape closes the window
- [ ] (Optional) You've tried the challenges

## Common Issues

**"borrow of moved value: `glfw`"**
- This is Rust's ownership system at work
- Make sure `glfw` and `window` are both in `main()`, not passed around unnecessarily
- The version in step 2 should work

**FPS is very high (thousands)**
- This is normal! We're not doing any rendering yet
- The window is just swapping empty buffers
- FPS will drop once we start rendering

**Warning: "unused variable: delta_time"**
- Prefix with underscore: `_delta_time`
- Or actually use it in the challenges!

## Next Step

Perfect! You now have a solid foundation with a properly structured event loop. You're ready to start rendering! Continue to [Phase 2, Step 06: OpenGL Context](../phase-02-core-rendering/step-06-opengl-context.md).

## Notes

- `Instant::now()` uses your system's high-resolution timer
- Delta time is crucial for smooth, framerate-independent gameplay
- The `update`/`render` separation is standard in game engines
- Later, we'll add input state management (tracking which keys are currently held)
- We're using `poll_events()` which doesn't block - some engines use `wait_events()` to save CPU
