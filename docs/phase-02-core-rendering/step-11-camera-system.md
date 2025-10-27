# Step 11: Camera System

**Phase:** 2 - Core Rendering
**Difficulty:** Advanced
**Estimated Time:** 2-3 hours

## Goal

Implement a camera system with view and projection matrices to view your 3D world from different positions and angles.

## What You'll Learn

- The MVP (Model-View-Projection) transformation pipeline
- View matrices (camera position and orientation)
- Projection matrices (perspective vs orthographic)
- Creating a reusable `Camera` struct
- LookAt transformation
- Keyboard controls for camera movement
- Understanding 3D coordinate spaces

## Background

In Step 10, you learned the **Model** matrix - transforming objects in world space. But you're still viewing everything from a fixed position at the origin, looking down the -Z axis.

**The problem:**
- You can't move around the scene
- You can't look at objects from different angles
- Everything is in "screen space" (-1 to +1 range)

**The solution: The MVP Matrix Pipeline**

Every 3D engine uses three matrices:
1. **Model** - Object space ’ World space (you learned this!)
2. **View** - World space ’ Camera/Eye space (NEW!)
3. **Projection** - Camera space ’ Clip space (NEW!)

```
Vertex (object space)
  “ [Model Matrix]
World space
  “ [View Matrix]
Camera/Eye space
  “ [Projection Matrix]
Clip space
  “ [GPU perspective divide]
Screen space
```

### View Matrix (Camera)

The view matrix transforms the entire world as if the camera is at the origin looking down -Z.

**Intuition:** Instead of moving the camera, we move the world in the opposite direction!

Example:
- Camera moves forward (+Z): World moves backward (-Z)
- Camera rotates left: World rotates right
- Camera moves up (+Y): World moves down (-Y)

### Projection Matrix

Converts 3D camera space to 2D screen space with depth.

**Two types:**

**Perspective** (realistic 3D):
- Objects farther away appear smaller
- Like a camera lens
- Uses field of view (FOV)

**Orthographic** (2D games, CAD):
- Objects same size regardless of distance
- No perspective distortion
- Parallel projection

## Task

### Part 1: Create Camera Module

**Create `rustgl/src/camera.rs`:**

```rust
use nalgebra_glm as glm;

/// A camera for viewing the 3D world
pub struct Camera {
    pub position: glm::Vec3,
    pub front: glm::Vec3,
    pub up: glm::Vec3,
    pub right: glm::Vec3,
    pub world_up: glm::Vec3,

    // Euler angles
    pub yaw: f32,
    pub pitch: f32,

    // Camera options
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
    pub zoom: f32,
}

impl Camera {
    /// Creates a new camera at the given position
    pub fn new(position: glm::Vec3, up: glm::Vec3, yaw: f32, pitch: f32) -> Self {
        let mut camera = Camera {
            position,
            front: glm::vec3(0.0, 0.0, -1.0),
            up: glm::vec3(0.0, 0.0, 0.0),
            right: glm::vec3(0.0, 0.0, 0.0),
            world_up: up,
            yaw,
            pitch,
            movement_speed: 2.5,
            mouse_sensitivity: 0.1,
            zoom: 45.0,
        };
        camera.update_camera_vectors();
        camera
    }

    /// Creates a default camera at (0, 0, 3) looking at origin
    pub fn default() -> Self {
        Camera::new(
            glm::vec3(0.0, 0.0, 3.0),  // Position
            glm::vec3(0.0, 1.0, 0.0),  // World up
            -90.0,                      // Yaw (looking down -Z)
            0.0,                        // Pitch (level)
        )
    }

    /// Returns the view matrix calculated using LookAt
    pub fn get_view_matrix(&self) -> glm::Mat4 {
        glm::look_at(&self.position, &(self.position + self.front), &self.up)
    }

    /// Processes keyboard input
    pub fn process_keyboard(&mut self, direction: CameraMovement, delta_time: f32) {
        let velocity = self.movement_speed * delta_time;
        match direction {
            CameraMovement::Forward => {
                self.position += self.front * velocity;
            }
            CameraMovement::Backward => {
                self.position -= self.front * velocity;
            }
            CameraMovement::Left => {
                self.position -= self.right * velocity;
            }
            CameraMovement::Right => {
                self.position += self.right * velocity;
            }
            CameraMovement::Up => {
                self.position += self.up * velocity;
            }
            CameraMovement::Down => {
                self.position -= self.up * velocity;
            }
        }
    }

    /// Processes mouse movement (for mouse look, which we'll add later)
    pub fn process_mouse_movement(&mut self, x_offset: f32, y_offset: f32, constrain_pitch: bool) {
        let x_offset = x_offset * self.mouse_sensitivity;
        let y_offset = y_offset * self.mouse_sensitivity;

        self.yaw += x_offset;
        self.pitch += y_offset;

        // Constrain pitch to prevent screen flip
        if constrain_pitch {
            if self.pitch > 89.0 {
                self.pitch = 89.0;
            }
            if self.pitch < -89.0 {
                self.pitch = -89.0;
            }
        }

        self.update_camera_vectors();
    }

    /// Processes mouse scroll (for zoom, which we'll add later)
    pub fn process_mouse_scroll(&mut self, y_offset: f32) {
        self.zoom -= y_offset;
        if self.zoom < 1.0 {
            self.zoom = 1.0;
        }
        if self.zoom > 45.0 {
            self.zoom = 45.0;
        }
    }

    /// Calculates front, right, and up vectors from euler angles
    fn update_camera_vectors(&mut self) {
        // Calculate new front vector
        let front = glm::vec3(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        );
        self.front = glm::normalize(&front);

        // Recalculate right and up vectors
        self.right = glm::normalize(&glm::cross(&self.front, &self.world_up));
        self.up = glm::normalize(&glm::cross(&self.right, &self.front));
    }
}

/// Camera movement directions
#[derive(Debug, Clone, Copy)]
pub enum CameraMovement {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
}
```

**Key concepts:**

1. **Position** - Where the camera is in world space
2. **Front** - Direction camera is looking
3. **Up** - Camera's "up" direction (usually Y+)
4. **Right** - Camera's "right" direction (cross product of front and up)
5. **Yaw** - Rotation around Y-axis (left/right)
6. **Pitch** - Rotation around X-axis (up/down)

### Part 2: Update Vertex Shader for MVP

**Update `rustgl/shader/basic.vert`:**

```glsl
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;

out vec3 ourColor;

uniform mat4 model;
uniform mat4 view;        // NEW!
uniform mat4 projection;  // NEW!

void main() {
    gl_Position = projection * view * model * vec4(aPos, 1.0);
    ourColor = aColor;
}
```

**What changed:**
- Added `uniform mat4 view` and `uniform mat4 projection`
- Changed to full MVP pipeline: `projection * view * model * vec4(aPos, 1.0)`
- **Order matters!** Read right-to-left: Model ’ View ’ Projection

### Part 3: Update Main.rs

**Update `rustgl/src/main.rs`:**

```rust
extern crate gl;
extern crate glfw;

mod shader;
mod mesh;
mod camera;  // NEW!

use glfw::{Action, Context, Key};
use std::time::Instant;
use shader::Shader;
use mesh::Mesh;
use nalgebra_glm as glm;
use camera::{Camera, CameraMovement};  // NEW!

fn main() {
    // ... GLFW initialization (no changes) ...

    // Load OpenGL
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    unsafe {
        let version = std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8);
        println!("OpenGL Version: {}", version.to_str().unwrap());
    }

    // Create a cube mesh (we'll make this in a moment)
    let cube = Mesh::cube([1.0, 0.5, 0.2]);

    let shader = Shader::new("shader/basic.vert", "shader/basic.frag");

    // Create camera
    let mut camera = Camera::default();

    let mut last_frame = Instant::now();
    let mut frame_count = 0;
    let mut fps_timer = Instant::now();
    let mut time = 0.0f32;

    // Window loop
    while !window.should_close() {
        let current_frame = Instant::now();
        let delta_time = current_frame.duration_since(last_frame).as_secs_f32();
        last_frame = current_frame;

        frame_count += 1;
        if fps_timer.elapsed().as_secs() >= 1 {
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

        process_events(&mut window, &events, &mut camera, delta_time);
        update(delta_time, &mut time);
        render(&mut window, &cube, &shader, &camera, time);
    }
}

fn process_events(
    window: &mut glfw::Window,
    events: &glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    camera: &mut Camera,
    delta_time: f32,
) {
    window.glfw.poll_events();
    for (_, event) in glfw::flush_messages(events) {
        handle_window_event(window, event, camera, delta_time);
    }
}

fn handle_window_event(
    window: &mut glfw::Window,
    event: glfw::WindowEvent,
    camera: &mut Camera,
    delta_time: f32,
) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
            gl::Viewport(0, 0, width, height);
        },
        _ => {}
    }

    // Camera controls (WASD + QE)
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
}

fn update(delta_time: f32, time: &mut f32) {
    *time += delta_time;
}

fn render(window: &mut glfw::Window, mesh: &Mesh, shader: &Shader, camera: &Camera, time: f32) {
    unsafe {
        gl::ClearColor(0.1, 0.1, 0.2, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        shader.use_program();

        // View matrix (camera)
        let view = camera.get_view_matrix();
        shader.set_mat4("view", &view);

        // Projection matrix (perspective)
        let projection = glm::perspective(
            1024.0 / 768.0,                // Aspect ratio
            camera.zoom.to_radians(),      // FOV
            0.1,                            // Near plane
            100.0,                          // Far plane
        );
        shader.set_mat4("projection", &projection);

        // Draw multiple cubes at different positions
        let cube_positions = [
            glm::vec3(0.0, 0.0, 0.0),
            glm::vec3(2.0, 5.0, -15.0),
            glm::vec3(-1.5, -2.2, -2.5),
            glm::vec3(-3.8, -2.0, -12.3),
            glm::vec3(2.4, -0.4, -3.5),
            glm::vec3(-1.7, 3.0, -7.5),
            glm::vec3(1.3, -2.0, -2.5),
            glm::vec3(1.5, 2.0, -2.5),
            glm::vec3(1.5, 0.2, -1.5),
            glm::vec3(-1.3, 1.0, -1.5),
        ];

        for (i, position) in cube_positions.iter().enumerate() {
            let mut model = glm::Mat4::identity();
            model = glm::translate(&model, position);
            let angle = 20.0 * i as f32 + time * 10.0;
            model = glm::rotate(&model, angle.to_radians(), &glm::vec3(1.0, 0.3, 0.5));
            shader.set_mat4("model", &model);
            mesh.draw();
        }
    }
    window.swap_buffers();
}
```

### Part 4: Add Cube Mesh Helper

**Add to `rustgl/src/mesh.rs`:**

```rust
impl Mesh {
    // ... existing methods ...

    /// Creates a 3D cube mesh using indexed rendering
    pub fn cube(color: [f32; 3]) -> Self {
        let normal = [0.0, 0.0, 1.0];  // We'll fix normals in lighting lesson

        // 8 unique vertices for a cube
        let vertices = vec![
            // Front face
            Vertex::new([-0.5, -0.5, 0.5], color, normal),  // 0
            Vertex::new([0.5, -0.5, 0.5], color, normal),   // 1
            Vertex::new([0.5, 0.5, 0.5], color, normal),    // 2
            Vertex::new([-0.5, 0.5, 0.5], color, normal),   // 3
            // Back face
            Vertex::new([-0.5, -0.5, -0.5], color, normal), // 4
            Vertex::new([0.5, -0.5, -0.5], color, normal),  // 5
            Vertex::new([0.5, 0.5, -0.5], color, normal),   // 6
            Vertex::new([-0.5, 0.5, -0.5], color, normal),  // 7
        ];

        // 36 indices for 12 triangles (6 faces × 2 triangles)
        let indices = vec![
            // Front
            0, 1, 2, 2, 3, 0,
            // Right
            1, 5, 6, 6, 2, 1,
            // Back
            5, 4, 7, 7, 6, 5,
            // Left
            4, 0, 3, 3, 7, 4,
            // Top
            3, 2, 6, 6, 7, 3,
            // Bottom
            4, 5, 1, 1, 0, 4,
        ];

        Mesh::new_indexed(&vertices, &indices)
    }
}
```

### Part 5: Add Camera Module Declaration

**Update `rustgl/src/main.rs` (top):**

Make sure you have:
```rust
mod camera;  // Add this line

use camera::{Camera, CameraMovement};  // Add this line
```

### Part 6: Build and Run

```bash
cd rustgl
cargo run
```

**Controls:**
- **W** - Move forward
- **S** - Move backward
- **A** - Move left (strafe)
- **D** - Move right (strafe)
- **Q** - Move down
- **E** - Move up
- **ESC** - Quit

You should see **10 rotating cubes** in 3D space that you can fly through!

## Understanding the Code

### LookAt Matrix

```rust
glm::look_at(&position, &target, &up)
```

Creates a view matrix that:
- Places camera at `position`
- Points camera at `target`
- Orients camera with `up` direction

We use: `look_at(&self.position, &(self.position + self.front), &self.up)`
- Target = position + front direction vector

### Perspective Projection

```rust
glm::perspective(aspect_ratio, fov, near, far)
```

- `aspect_ratio` - Width / Height (e.g., 16/9, 4/3)
- `fov` - Field of view in radians (45° = 0.785 rad)
- `near` - Near clipping plane (0.1 = objects closer than 0.1 invisible)
- `far` - Far clipping plane (100.0 = objects farther than 100 invisible)

### Euler Angles (Yaw and Pitch)

**Yaw** - Rotation around Y-axis (left/right):
```
  +Y
   |
   |___+X
  /
 /
+Z

Yaw = 0°   ’ Looking down +X
Yaw = 90°  ’ Looking down +Z
Yaw = 180° ’ Looking down -X
Yaw = 270° ’ Looking down -Z
```

**Pitch** - Rotation around X-axis (up/down):
```
Pitch = 0°    ’ Looking straight ahead
Pitch = +90°  ’ Looking straight up
Pitch = -90°  ’ Looking straight down
```

We constrain pitch to ±89° to prevent gimbal lock.

### Camera Movement

Movement is relative to camera orientation:
- **Forward/Backward**: Move along `front` vector
- **Left/Right**: Move along `right` vector (perpendicular to front)
- **Up/Down**: Move along `up` vector

## Challenges

### Challenge 1: Orthographic Projection

Create a toggle to switch between perspective and orthographic:

```rust
// In render():
let projection = if use_ortho {
    glm::ortho(-10.0, 10.0, -10.0, 10.0, 0.1, 100.0)
} else {
    glm::perspective(aspect, fov, 0.1, 100.0)
};
```

Press a key (e.g., `P`) to toggle.

### Challenge 2: Camera Speed Control

Add keys to increase/decrease camera speed:
```rust
if window.get_key(Key::LeftShift) == Action::Press {
    camera.movement_speed = 10.0;  // Sprint
} else {
    camera.movement_speed = 2.5;   // Normal
}
```

### Challenge 3: Look Around with Arrow Keys

Add arrow key controls to rotate camera:
```rust
if window.get_key(Key::Left) == Action::Press {
    camera.process_mouse_movement(-50.0 * delta_time, 0.0, true);
}
// Add Right, Up, Down...
```

### Challenge 4: Circle Formation

Arrange cubes in a circle:
```rust
let cube_positions: Vec<glm::Vec3> = (0..20)
    .map(|i| {
        let angle = (i as f32 / 20.0) * 2.0 * std::f32::consts::PI;
        glm::vec3(angle.cos() * 5.0, 0.0, angle.sin() * 5.0)
    })
    .collect();
```

## Success Criteria

- [ ] You've created `camera.rs` module
- [ ] Updated vertex shader with `view` and `projection` uniforms
- [ ] Implemented MVP transformation pipeline
- [ ] Camera moves with WASD/QE controls
- [ ] You can see 10 cubes in 3D space
- [ ] Camera position shows in window title
- [ ] You understand perspective projection
- [ ] (Optional) Tried the challenges

## Common Issues

**Cubes don't appear**
- Check that camera is at position (0, 0, 3) looking at origin
- Verify cubes are at different Z positions (some negative)
- Make sure projection near/far planes contain the cubes

**Movement is too fast/slow**
- Adjust `camera.movement_speed` (default 2.5)
- Check that you're multiplying by `delta_time`

**Camera moves in wrong direction**
- Verify `update_camera_vectors()` is called after changing yaw/pitch
- Check that `front`, `right`, `up` vectors are normalized

**Cubes look flat or distorted**
- Check aspect ratio in `glm::perspective()` matches window size
- Verify FOV is in radians (use `.to_radians()`)

**"cannot find type `Camera` in this scope"**
- Make sure you added `mod camera;` at top of `main.rs`
- Verify `use camera::{Camera, CameraMovement};`

## Next Step

Fantastic! You now have a full 3D camera system!

Next: [Step 12: Primitives](./step-12-primitives.md), where you'll create more mesh types (spheres, cylinders, planes) and organize your rendering code!

## Notes

### Coordinate System

OpenGL uses a **right-handed coordinate system**:
```
    +Y (up)
    |
    |___+X (right)
   /
  /
+Z (towards you)
```

In NDC after projection:
- X: -1 (left) to +1 (right)
- Y: -1 (bottom) to +1 (top)
- Z: -1 (near) to +1 (far)

### Clipping Planes

Objects outside [near, far] range are clipped (not rendered):
- **Near too small** (< 0.001): Z-fighting artifacts
- **Far too large** (> 10000): Loss of depth precision
- Good defaults: near=0.1, far=100.0

### Field of View

Typical FOV values:
- **45°** - Standard (what we use)
- **60°** - Slightly wider (FPS games)
- **90°** - Very wide (quake-style)
- **30°** - Narrow (telephoto lens)

### Performance

- View and projection matrices are calculated once per frame
- Model matrix is calculated per object
- Matrix multiplication happens on GPU (very fast!)

### Camera vs World

Two equivalent approaches:
1. **Move camera** in world space
2. **Move world** in opposite direction (what we actually do)

The view matrix does #2 - it's the inverse of the camera's transform.
