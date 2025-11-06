# Step 41: UI Integration with egui

**Goal:** Add an in-engine GUI to control debug modes, view performance stats, and adjust rendering parameters in real-time using sliders, buttons, and checkboxes.

**Estimated Time:** 2-3 hours

---

## What You'll Learn

- Integrating egui (a pure-Rust immediate mode GUI library) with GLFW
- Creating UI panels, windows, and widgets
- Handling UI input without interfering with game input
- Real-time parameter adjustment with sliders
- Displaying performance statistics

---

## Current State Check

Looking at your current implementation (main.rs:31-65), you have many debug parameters controlled by keyboard shortcuts:

 **Already implemented:**
- AppState struct with debug parameters
- Wireframe mode toggle (Key 1)
- Texture toggle (Key 2)
- Bloom settings (Keys 3-7): threshold, strength, enabled
- Skybox toggle (Key 8)
- God ray settings (Keys 9, O, P, 0): enabled, exposure, debug mode
- FPS counter in window title

L **Still needed:**
- egui integration with GLFW and OpenGL
- GUI panels for controlling parameters
- Sliders for continuous values (bloom threshold, strength, exposure, etc.)
- Checkboxes for boolean toggles
- Performance stats display
- UI input handling

---

## Why egui?

**egui** is an excellent choice for in-engine UI because:
- **Pure Rust** - No C++ dependencies
- **Immediate mode** - Simple to use, no state management
- **Lightweight** - Minimal performance overhead
- **Beautiful** - Modern, polished default styling
- **Flexible** - Easy to customize and extend
- **GLFW compatible** - Works well with our existing setup

---

## Part 1: Add Dependencies

### Task 1.1: Update Cargo.toml

Add the egui dependencies to your `Cargo.toml`:

```toml
[dependencies]
# ... your existing dependencies ...

# UI
egui = "0.29"
egui-glfw = { version = "0.29", features = ["clipboard", "persist"] }
```

**What these do:**
- `egui` - The core immediate mode GUI library
- `egui-glfw` - Integration layer between egui and GLFW
  - `clipboard` feature - Enables copy/paste
  - `persist` feature - Saves window positions/sizes between sessions

### Task 1.2: Install and verify

Run `cargo build` to download and compile the new dependencies. This might take a few minutes the first time.

```bash
cd rustgl
cargo build
```

**Expected output:** Should compile without errors (might see some warnings)

---

## Part 2: Initialize egui

### Task 2.1: Create egui context in main()

You need to initialize egui after creating your GLFW window and OpenGL context.

**Location:** In `main()` after `gl::load_with(...)` (around line 110)

Add:

```rust
// Initialize egui
let mut egui_glfw = egui_glfw::EguiGlfw::new(&mut window, &mut glfw);
```

**What this does:**
- Creates an egui context
- Sets up input handling
- Initializes the OpenGL renderer for egui

### Task 2.2: Handle egui events

egui needs to process window events to handle mouse clicks, keyboard input, etc.

**Location:** In `process_events()` function, right after `window.glfw.poll_events()` (around line 321)

**Before the event loop**, add:

```rust
// Let egui handle events first
egui_glfw.handle_event(&window, &events);
```

This should go BEFORE your existing `for (_, event) in glfw::flush_messages(events)` loop.

**Why first?** egui needs to consume UI-related input (like clicking a button) so it doesn't also trigger game input.

---

## Part 3: Create the UI Layout

### Task 3.1: Create the UI rendering function

Create a new function that defines your UI layout. Add this after the `render_scene()` function (around line 553):

```rust
fn render_ui(
    egui_ctx: &egui::Context,
    state: &mut AppState,
    delta_time: f32,
    frame_count: u32,
    camera: &Camera,
) {
    // Main debug panel
    egui::Window::new("<® RustGL Debug Panel")
        .default_width(300.0)
        .show(egui_ctx, |ui| {
            ui.heading("Performance");
            ui.separator();

            // FPS display
            let fps = 1.0 / delta_time;
            ui.label(format!("FPS: {:.0}", fps));
            ui.label(format!("Frame time: {:.2} ms", delta_time * 1000.0));
            ui.label(format!("Frames: {}", frame_count));

            ui.add_space(10.0);

            // Camera position
            ui.heading("Camera");
            ui.separator();
            ui.label(format!(
                "Position: ({:.1}, {:.1}, {:.1})",
                camera.position.x,
                camera.position.y,
                camera.position.z
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
                        .step_by(0.1)
                );
                ui.add(
                    egui::Slider::new(&mut state.bloom_strength, 0.0..=3.0)
                        .text("Strength")
                        .step_by(0.1)
                );
            }

            ui.add_space(10.0);

            // God ray controls
            ui.heading("God Rays");
            ui.separator();
            ui.checkbox(&mut state.godray_enabled, "Enable God Rays");

            if state.godray_enabled {
                ui.add(
                    egui::Slider::new(&mut state.godray_strength, 0.0..=2.0)
                        .text("Strength")
                        .step_by(0.1)
                );
                ui.add(
                    egui::Slider::new(&mut state.godray_exposure, 0.0..=2.0)
                        .text("Exposure")
                        .step_by(0.05)
                );
                ui.add(
                    egui::Slider::new(&mut state.godray_decay, 0.8..=1.0)
                        .text("Decay")
                        .step_by(0.01)
                );

                ui.add_space(5.0);
                ui.label("Debug Mode:");
                ui.radio_value(&mut state.godray_debug_mode, 0, "Off (Normal)");
                ui.radio_value(&mut state.godray_debug_mode, 1, "Occlusion Buffer");
                ui.radio_value(&mut state.godray_debug_mode, 2, "Radial Blur");
                ui.radio_value(&mut state.godray_debug_mode, 3, "Rays Only");
            }

            ui.add_space(10.0);

            // Keyboard shortcuts help
            ui.heading("Keyboard Shortcuts");
            ui.separator();
            ui.label("WASD - Move camera");
            ui.label("QE - Up/Down");
            ui.label("Arrow keys - Look around");
            ui.label("ESC - Quit");
        });
}
```

**What this creates:**
- A draggable, collapsible debug panel
- Performance stats (FPS, frame time)
- Camera position display
- Checkboxes for toggles (wireframe, textures, skybox, bloom, god rays)
- Sliders for continuous values (bloom threshold/strength, god ray parameters)
- Radio buttons for god ray debug modes
- Keyboard shortcut reference

### Task 3.2: Add frame counter to main loop

You need to pass the frame count to the UI. In `main()`, change the frame counter to be outside the FPS timer scope.

**Find this code** (around line 226):

```rust
let mut frame_count = 0;
let mut fps_timer = Instant::now();
```

Keep it, but **change the FPS display code** (around line 240):

```rust
if fps_timer.elapsed().as_secs() >= 1 {
    // Just update window title with basic info
    let title = format!("RustGL by mau | FPS: {}", frame_count);
    window.set_title(&title);
    frame_count = 0;  // Keep this to reset counter
    fps_timer = Instant::now();
}
```

**Why?** The UI will now show detailed stats, so the window title can be simpler.

---

## Part 4: Render the UI

### Task 4.1: Begin and end UI frame

In your main loop, you need to:
1. Begin the egui frame
2. Build the UI (call your render_ui function)
3. End the frame and render egui

**Location:** In the main `while !window.should_close()` loop, **after** your scene rendering and **before** `window.swap_buffers()` (around line 307)

Add:

```rust
// Render UI
egui_glfw.begin_frame(&window);
render_ui(
    egui_glfw.egui_ctx(),
    &mut state,
    delta_time,
    frame_count,
    &camera,
);
let (egui_output, egui_shapes) = egui_glfw.end_frame();
egui_glfw.paint(&window, egui_output, egui_shapes);
```

**What this does:**
1. `begin_frame()` - Starts a new UI frame, processes input
2. `render_ui()` - Your code that defines the UI layout
3. `end_frame()` - Finalizes the UI, gets draw data
4. `paint()` - Renders the UI on top of the scene

### Task 4.2: Pass egui_glfw to process_events

Your `process_events` function signature needs to accept the egui context.

**Find:** (around line 312)
```rust
fn process_events(
    window: &mut glfw::Window,
    events: &glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    camera: &mut Camera,
    state: &mut AppState,
    bloom_renderer: &mut BloomRenderer,
    godray_renderer: &mut GodRayRenderer,
    delta_time: f32,
)
```

**Change to:**
```rust
fn process_events(
    window: &mut glfw::Window,
    events: &glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    camera: &mut Camera,
    state: &mut AppState,
    bloom_renderer: &mut BloomRenderer,
    godray_renderer: &mut GodRayRenderer,
    egui_glfw: &mut egui_glfw::EguiGlfw,
    delta_time: f32,
)
```

**And update the call** (around line 257):
```rust
process_events(
    &mut window,
    &events,
    &mut camera,
    &mut state,
    &mut bloom_renderer,
    &mut godray_renderer,
    &mut egui_glfw,  // Add this
    delta_time,
);
```

### Task 4.3: Handle egui input capture

When the user is interacting with the UI (clicking buttons, typing in text boxes), you don't want camera movement to trigger.

**In `process_events()`**, wrap the camera input handling with an egui check.

**Find:** (around line 335)
```rust
// WASD for movement (relative to camera orientation)
if window.get_key(Key::W) == Action::Press {
    camera.process_keyboard(CameraMovement::Forward, delta_time);
}
// ... rest of camera controls ...
```

**Wrap with:**
```rust
// Only process camera input if UI isn't capturing input
if !egui_glfw.egui_ctx().wants_pointer_input()
    && !egui_glfw.egui_ctx().wants_keyboard_input()
{
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
    let look_speed = 250.0;
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
```

**What this does:**
- Checks if egui wants input (e.g., user is dragging a slider)
- If yes, skips camera movement
- If no, processes camera input normally

---

## Part 5: Test and Verify

### Task 5.1: Build and run

```bash
cd rustgl
cargo build --release
cargo run --release
```

**Expected behavior:**
- Application starts normally
- You see a debug panel on the left side (draggable)
- FPS counter updates in real-time
- Checkboxes toggle rendering features
- Sliders adjust bloom and god ray parameters
- When you interact with UI, camera doesn't move

### Task 5.2: Test all controls

Go through each UI element and verify:
-  FPS display updates every frame
-  Camera position displays correctly
-  Wireframe checkbox toggles wireframe mode
-  Texture checkbox toggles textures
-  Skybox checkbox toggles skybox
-  Bloom checkbox enables/disables bloom
-  Bloom sliders adjust threshold and strength
-  God ray checkbox enables/disables god rays
-  God ray sliders adjust parameters
-  God ray debug radio buttons cycle modes
-  Camera doesn't move when dragging sliders

### Task 5.3: Optional - Keep keyboard shortcuts

You can keep your existing keyboard shortcuts (Keys 1-9, etc.) as alternatives to the GUI. They work alongside the UI!

The keyboard handlers in `handle_window_event()` will still function and update the same `state` values that the UI modifies.

---

## Part 6: Customization (Optional Challenges)

### Challenge 1: Add more panels

Create separate windows for different categories:
- Performance stats in one window
- Rendering settings in another
- Post-processing in a third

### Challenge 2: Add color pickers

Use `ui.color_edit_button_rgb()` to add color pickers for:
- Light colors
- Clear color (background)
- Material colors

### Challenge 3: Add graphs

Use `egui::plot::Plot` to show frame time history as a graph.

### Challenge 4: Save/load settings

Use the `persist` feature to save UI settings between sessions:
```rust
egui_glfw.save_settings("settings.json");
```

### Challenge 5: Add spawn buttons

Add buttons to spawn new objects:
```rust
if ui.button("Spawn Sphere").clicked() {
    scene.add_object(...);
}
```

---

## Common Issues and Solutions

### Issue 1: UI is tiny on HiDPI displays

**Solution:** Set the egui scale factor:
```rust
egui_glfw.egui_ctx().set_pixels_per_point(2.0);
```

### Issue 2: UI flickers or renders under scene

**Solution:** Make sure UI rendering happens AFTER scene rendering and BEFORE swap_buffers.

### Issue 3: Can't click UI elements

**Solution:** Verify `egui_glfw.handle_event()` is called BEFORE flushing GLFW events.

### Issue 4: Compilation errors about traits

**Solution:** Make sure you have the correct egui-glfw version (0.29) matching egui version.

---

## What You've Learned

 How to integrate egui with GLFW and OpenGL
 Creating immediate mode UI layouts
 Using widgets: windows, labels, checkboxes, sliders, radio buttons
 Handling UI input without interfering with game input
 Real-time parameter visualization and adjustment
 Displaying performance statistics

---

## Next Steps

You now have a fully functional debug UI! This will be invaluable for:
- Tuning visual effects in real-time
- Performance profiling
- Debugging rendering issues
- Experimenting with parameters

**Optional:** If you want to continue, Step 42 focuses on performance profiling and optimization - and your new UI will be perfect for displaying those metrics!

---

## Recap: Files Modified

- `rustgl/Cargo.toml` - Added egui dependencies
- `rustgl/src/main.rs` - Added UI initialization, rendering, and layout

**Estimated total additions:** ~150 lines of code

Great job adding professional UI to your engine! <‰
