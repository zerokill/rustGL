# Step 10: Transformations

**Phase:** 2 - Core Rendering
**Difficulty:** Intermediate-Advanced
**Estimated Time:** 2-3 hours

## Goal

Learn how to move, rotate, and scale objects using transformation matrices - the foundation of 3D graphics.

## What You'll Learn

- What transformation matrices are and why they're fundamental to 3D graphics
- Translation (moving objects)
- Rotation (spinning objects)
- Scaling (resizing objects)
- Matrix multiplication and combining transformations
- Uniform variables in shaders
- The `nalgebra-glm` crate (Rust's OpenGL math library)
- How to pass matrices from Rust to GLSL shaders

## Background

Right now, your meshes are stuck at their vertex positions. In Step 09, you created new mesh data with different positions - extremely inefficient!

**The problem:** Creating a mesh with 1000 vertices at position (0, 0, 0), then wanting to move it to (5, 3, 2) means:
- L **Bad approach:** Create 1000 new vertices with adjusted positions
-  **Good approach:** Keep original vertices, apply a transformation matrix

**Transformation matrices** are the core of 3D graphics. They allow you to:
- Use the **same mesh data** for multiple objects
- Move, rotate, and scale objects without changing vertex data
- Animate objects smoothly
- Implement cameras and projections

### How It Works

Instead of modifying vertex positions in Rust:
```rust
// L Inefficient - creating new data
let mesh1 = Mesh::quad_at([1.0, 0.0, 0.0], -0.5, 0.0);
let mesh2 = Mesh::quad_at([1.0, 0.0, 0.0], 0.5, 0.0);  // Duplicate vertex data!
```

You keep ONE mesh and transform it with matrices:
```rust
//  Efficient - reuse mesh, different transforms
let mesh = Mesh::quad([1.0, 0.0, 0.0]);
let transform1 = Mat4::translate(-0.5, 0.0, 0.0);
let transform2 = Mat4::translate(0.5, 0.0, 0.0);
```

The GPU applies the transformation in the vertex shader:
```glsl
gl_Position = transform * vec4(aPos, 1.0);
```

## Mathematics Quick Reference

### Transformation Matrix (4x4)

A 4x4 matrix can represent translation, rotation, and scale:

```
[ Sx  0   0   Tx ]    Sx, Sy, Sz = Scale
[ 0   Sy  0   Ty ]    Tx, Ty, Tz = Translation
[ 0   0   Sz  Tz ]    (Other elements = Rotation)
[ 0   0   0   1  ]
```

### Matrix Multiplication Order

**IMPORTANT:** Matrix multiplication is **not commutative**! Order matters:
- `Translate * Rotate ` Rotate * Translate`
- OpenGL uses **column-major** matrices
- Transformations are applied **right to left**:
  - `M * v` means: apply `M` to vector `v`
  - `A * B * v` means: apply `B` first, then `A`

### Common Transformations

**Translation (moving):**
```
translate(x, y, z) = move object by (x, y, z)
```

**Rotation (spinning):**
```
rotate(angle, axis) = rotate by 'angle' radians around 'axis'
```

**Scaling (resizing):**
```
scale(x, y, z) = multiply size by (x, y, z)
```

## Task

### Part 1: Add nalgebra-glm Dependency

**Edit `rustgl/Cargo.toml`:**

```toml
[package]
name = "rustgl"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8"
colored = "2.0"
glfw = "0.54"
gl = "0.14"
nalgebra-glm = "0.18"  # NEW! OpenGL math library
```

**Run to download:**
```bash
cargo build
```

**About nalgebra-glm:**
- Rust port of GLM (OpenGL Mathematics)
- Provides `Vec3`, `Vec4`, `Mat4` types
- Functions: `translate()`, `rotate()`, `scale()`
- Compatible with OpenGL's column-major matrices

### Part 2: Update Shaders for Transformations

**Update `rustgl/shader/basic.vert`:**

```glsl
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;

out vec3 ourColor;

uniform mat4 model;  // NEW! Transformation matrix

void main() {
    gl_Position = model * vec4(aPos, 1.0);  // Apply transformation
    ourColor = aColor;
}
```

**What changed:**
- Added `uniform mat4 model` - receives transformation matrix from Rust
- Changed `gl_Position = vec4(aPos, 1.0)` to `model * vec4(aPos, 1.0)`
- The matrix transforms the vertex position before outputting

**Fragment shader stays the same** (no changes needed).

### Part 3: Add Uniform Methods to Shader

**Update `rustgl/src/shader.rs`:**

Add these methods to the `impl Shader` block:

```rust
use std::ffi::CString;
use std::fs;
use std::ptr;

// Add this import at the top
use nalgebra_glm as glm;

// ... existing code ...

impl Shader {
    // ... existing methods (new, use_program, etc.) ...

    /// Sets a mat4 uniform in the shader
    pub fn set_mat4(&self, name: &str, matrix: &glm::Mat4) {
        unsafe {
            let c_name = CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.id, c_name.as_ptr());
            gl::UniformMatrix4fv(
                location,
                1,
                gl::FALSE,
                matrix.as_ptr(),
            );
        }
    }

    /// Sets a vec3 uniform in the shader
    pub fn set_vec3(&self, name: &str, value: &glm::Vec3) {
        unsafe {
            let c_name = CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.id, c_name.as_ptr());
            gl::Uniform3f(location, value.x, value.y, value.z);
        }
    }

    /// Sets a float uniform in the shader
    pub fn set_float(&self, name: &str, value: f32) {
        unsafe {
            let c_name = CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.id, c_name.as_ptr());
            gl::Uniform1f(location, value);
        }
    }
}
```

**What these do:**
- `set_mat4()` - Sends a 4x4 matrix to the shader
- `set_vec3()` - Sends a 3D vector (we'll use this later for colors/lighting)
- `set_float()` - Sends a single float (useful for time, alpha, etc.)

### Part 4: Update Main.rs to Use Transformations

**Update `rustgl/src/main.rs`:**

```rust
extern crate gl;
extern crate glfw;

mod shader;
mod mesh;

use glfw::{Action, Context, Key};
use std::time::Instant;
use shader::Shader;
use mesh::Mesh;
use nalgebra_glm as glm;  // NEW!

fn main() {
    // ... GLFW initialization (no changes) ...

    // Load OpenGL
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    unsafe {
        let version = std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8);
        println!("OpenGL Version: {}", version.to_str().unwrap());
    }

    // Create ONE quad mesh (we'll reuse it with different transforms!)
    let quad = Mesh::quad([1.0, 1.0, 1.0]);  // White quad

    let shader = Shader::new("shader/basic.vert", "shader/basic.frag");

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
                "RustGL by mau | FPS: {} | Frame time: {:.2}ms",
                frame_count,
                delta_time * 1000.0
            );
            window.set_title(&title);
            frame_count = 0;
            fps_timer = Instant::now();
        }

        process_events(&mut window, &events);
        update(delta_time, &mut time);
        render(&mut window, &quad, &shader, time);
    }
}

// ... process_events, handle_window_event (no changes) ...

fn update(delta_time: f32, time: &mut f32) {
    *time += delta_time;
}

fn render(window: &mut glfw::Window, mesh: &Mesh, shader: &Shader, time: f32) {
    unsafe {
        gl::ClearColor(0.1, 0.1, 0.2, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        shader.use_program();

        // Example 1: Static translation
        let model1 = glm::translate(&glm::Mat4::identity(), &glm::vec3(-0.6, 0.5, 0.0));
        shader.set_mat4("model", &model1);
        mesh.draw();

        // Example 2: Rotation (animated)
        let mut model2 = glm::Mat4::identity();
        model2 = glm::translate(&model2, &glm::vec3(0.0, 0.5, 0.0));
        model2 = glm::rotate(&model2, time, &glm::vec3(0.0, 0.0, 1.0));  // Rotate around Z-axis
        model2 = glm::scale(&model2, &glm::vec3(0.5, 0.5, 0.5));  // Scale to 50%
        shader.set_mat4("model", &model2);
        mesh.draw();

        // Example 3: Scaling (pulsing)
        let scale = 1.0 + 0.5 * (time * 2.0).sin();  // Pulse between 0.5 and 1.5
        let mut model3 = glm::Mat4::identity();
        model3 = glm::translate(&model3, &glm::vec3(0.6, 0.5, 0.0));
        model3 = glm::scale(&model3, &glm::vec3(scale, scale, 1.0));
        shader.set_mat4("model", &model3);
        mesh.draw();

        // Example 4: Combined transformation (orbit)
        let orbit_radius = 0.3;
        let orbit_x = orbit_radius * (time * 1.5).cos();
        let orbit_y = orbit_radius * (time * 1.5).sin();
        let mut model4 = glm::Mat4::identity();
        model4 = glm::translate(&model4, &glm::vec3(orbit_x, -0.4, 0.0));
        model4 = glm::rotate(&model4, time * 2.0, &glm::vec3(0.0, 0.0, 1.0));
        model4 = glm::scale(&model4, &glm::vec3(0.3, 0.3, 1.0));
        shader.set_mat4("model", &model4);
        mesh.draw();
    }
    window.swap_buffers();
}
```

### Part 5: Build and Run

```bash
cd rustgl
cargo run
```

You should see **FOUR quads**, all from the SAME mesh data:
1. **Top-left**: Static position
2. **Top-center**: Rotating continuously
3. **Top-right**: Pulsing (scaling up and down)
4. **Bottom**: Orbiting in a circle while rotating

## Understanding the Code

### Matrix Identity

```rust
let model = glm::Mat4::identity();
```

The identity matrix does nothing - it's like multiplying by 1:
```
[ 1  0  0  0 ]
[ 0  1  0  0 ]
[ 0  0  1  0 ]
[ 0  0  0  1 ]
```

Start with identity, then apply transformations.

### Translation

```rust
let model = glm::translate(&glm::Mat4::identity(), &glm::vec3(x, y, z));
```

Moves the object by (x, y, z).

### Rotation

```rust
let model = glm::rotate(&model, angle_radians, &glm::vec3(0.0, 0.0, 1.0));
```

- `angle_radians` - rotation amount (in radians, not degrees!)
- `vec3(0, 0, 1)` - rotation axis (Z-axis = rotate in XY plane)

**Radians vs Degrees:**
- À radians = 180 degrees
- 2À radians = 360 degrees (full circle)
- Use `angle.to_radians()` to convert degrees to radians

### Scaling

```rust
let model = glm::scale(&model, &glm::vec3(sx, sy, sz));
```

Multiplies object size by (sx, sy, sz).
- `(2.0, 2.0, 2.0)` = double size
- `(0.5, 0.5, 0.5)` = half size

### Combining Transformations

**Order matters!**

```rust
let mut model = glm::Mat4::identity();
model = glm::translate(&model, &glm::vec3(5.0, 0.0, 0.0));  // 1. Move right
model = glm::rotate(&model, angle, &glm::vec3(0.0, 0.0, 1.0));  // 2. Rotate
model = glm::scale(&model, &glm::vec3(0.5, 0.5, 1.0));  // 3. Scale
```

This applies transformations in order: Scale ’ Rotate ’ Translate (right to left).

**Different order = different result:**
```rust
// Rotate around origin, THEN move
model = glm::translate(&model, &glm::vec3(5.0, 0.0, 0.0));
model = glm::rotate(&model, angle, &glm::vec3(0.0, 0.0, 1.0));

// Move first, THEN rotate around NEW position (orbit!)
model = glm::rotate(&model, angle, &glm::vec3(0.0, 0.0, 1.0));
model = glm::translate(&model, &glm::vec3(5.0, 0.0, 0.0));
```

## Challenges

### Challenge 1: Create a Solar System

Create a sun (center), planet (orbits sun), and moon (orbits planet):

```rust
// Sun (center, pulsing)
let sun_scale = 1.0 + 0.2 * (time * 2.0).sin();
let sun_model = glm::scale(&glm::Mat4::identity(), &glm::vec3(sun_scale, sun_scale, 1.0));

// Planet (orbits sun)
let planet_angle = time * 0.5;
let mut planet_model = glm::Mat4::identity();
planet_model = glm::rotate(&planet_model, planet_angle, &glm::vec3(0.0, 0.0, 1.0));
planet_model = glm::translate(&planet_model, &glm::vec3(0.6, 0.0, 0.0));
planet_model = glm::scale(&planet_model, &glm::vec3(0.2, 0.2, 1.0));

// Moon (orbits planet) - hint: combine planet's transform with moon's orbit!
```

### Challenge 2: Spinning Triangle

Make a triangle that spins continuously:
```rust
let triangle = Mesh::triangle([1.0, 0.0, 0.0]);
let angle = time * 2.0;  // 2 radians per second
let model = glm::rotate(&glm::Mat4::identity(), angle, &glm::vec3(0.0, 0.0, 1.0));
```

### Challenge 3: Figure-8 Motion

Make an object move in a figure-8 pattern (Lissajous curve):
```rust
let x = 0.5 * (time).sin();
let y = 0.5 * (time * 2.0).sin();
let model = glm::translate(&glm::Mat4::identity(), &glm::vec3(x, y, 0.0));
```

### Challenge 4: Hierarchical Transformations

Create a "robot arm" with multiple segments, where each segment is relative to the previous:

```rust
// Shoulder (rotates)
let mut shoulder = glm::Mat4::identity();
shoulder = glm::rotate(&shoulder, time, &glm::vec3(0.0, 0.0, 1.0));
shoulder = glm::translate(&shoulder, &glm::vec3(0.3, 0.0, 0.0));

// Elbow (relative to shoulder)
let mut elbow = shoulder.clone();  // Start with shoulder's transform
elbow = glm::translate(&elbow, &glm::vec3(0.3, 0.0, 0.0));
elbow = glm::rotate(&elbow, time * 2.0, &glm::vec3(0.0, 0.0, 1.0));
```

## Success Criteria

- [ ] You've added `nalgebra-glm` to `Cargo.toml`
- [ ] Updated vertex shader to accept `uniform mat4 model`
- [ ] Added `set_mat4()` method to `Shader`
- [ ] Used `glm::translate()`, `glm::rotate()`, and `glm::scale()`
- [ ] Rendered multiple objects from ONE mesh using different transforms
- [ ] You understand matrix multiplication order
- [ ] You can create animated transformations
- [ ] (Optional) Tried the challenges

## Common Issues

**"cannot find type `Mat4` in module `glm`"**
- Make sure you added `use nalgebra_glm as glm;` at the top of `main.rs`
- Run `cargo build` to download the crate

**Objects don't appear or are at the wrong position**
- Check that you're multiplying `model * vec4(aPos, 1.0)` in the shader (not `vec4(aPos, 1.0) * model`)
- Verify you're calling `shader.set_mat4("model", &model)` before `mesh.draw()`

**Rotation is wrong direction**
- OpenGL uses right-hand rule: thumb = rotation axis, fingers = rotation direction
- Use negative angle to rotate the opposite direction

**Objects are distorted or stretched**
- Check your scale values - non-uniform scaling (different x, y, z) will stretch
- Make sure you're not accidentally scaling by huge numbers

**"the trait bound `&Mat4: AsPtr` is not satisfied"**
- In `set_mat4()`, use `matrix.as_ptr()` not `matrix.as_slice().as_ptr()`
- `nalgebra` matrices implement `AsPtr` directly

## Next Step

Excellent! You've learned the foundation of 3D graphics - transformation matrices!

Next: [Step 11: Camera System](./step-11-camera-system.md), where you'll implement a camera to view your 3D world from different angles!

## Notes

### Why 4x4 Matrices for 3D?

We use 4x4 matrices (homogeneous coordinates) instead of 3x3 because:
- **Translation** can't be represented with 3x3 matrices
- The 4th component allows us to distinguish points from vectors
- Enables perspective projection (3D ’ 2D)

### Matrix-Vector Multiplication

When you do `model * vec4(aPos, 1.0)`:
```
[ m00 m01 m02 m03 ]   [ x ]   [ m00*x + m01*y + m02*z + m03*1 ]
[ m10 m11 m12 m13 ] * [ y ] = [ m10*x + m11*y + m12*z + m13*1 ]
[ m20 m21 m22 m23 ]   [ z ]   [ m20*x + m21*y + m22*z + m23*1 ]
[ m30 m31 m32 m33 ]   [ 1 ]   [ m30*x + m31*y + m32*z + m33*1 ]
```

The last column (m03, m13, m23) contains translation.

### Performance Notes

- Transformations happen on the GPU - extremely fast!
- One mesh = many objects (instancing, which we'll learn later)
- Matrix multiplication order matters for correctness, not performance
- Modern GPUs can transform millions of vertices per frame

### Coordinate Systems

Right now we're working in **Normalized Device Coordinates (NDC)**:
- X: -1 (left) to +1 (right)
- Y: -1 (bottom) to +1 (top)
- Z: -1 (far) to +1 (near)

Later (Step 11), we'll add:
- **View matrix** (camera/eye space)
- **Projection matrix** (perspective/orthographic)
- Together: **MVP (Model-View-Projection) matrix**

### GLM vs cgmath

Rust has two main math libraries for graphics:
- **nalgebra-glm** - Port of C++ GLM, familiar to OpenGL developers
- **cgmath** - Pure Rust alternative

We chose `nalgebra-glm` because:
- Drop-in replacement for C++ GLM
- Excellent OpenGL compatibility
- Well-documented
- Actively maintained

### Debugging Transforms

Print matrices to debug:
```rust
println!("Model matrix: {:?}", model);
```

Or individual positions:
```rust
let pos = glm::vec3(5.0, 3.0, 0.0);
let transformed = model * glm::vec4(pos.x, pos.y, pos.z, 1.0);
println!("Position {} transformed to {}", pos, transformed);
```
