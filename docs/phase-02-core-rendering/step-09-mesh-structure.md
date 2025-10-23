# Step 09: Mesh Structure

**Phase:** 2 - Core Rendering
**Difficulty:** Intermediate
**Estimated Time:** 1-2 hours

## Goal

Create a reusable `Mesh` struct to encapsulate vertex data and VAO/VBO management, making it easy to render multiple objects.

## What You'll Learn

- Creating a `Mesh` struct to manage geometry
- Working with `Vec<f32>` (dynamic arrays)
- Understanding ownership and lifetimes with OpenGL objects
- The builder pattern (optional)
- How to render multiple objects efficiently
- Vertex data organization strategies

## Background

Right now, your VAO and VBO are created directly in `main()`. This works for one triangle, but what if you want to render:
- Multiple triangles
- A quad (rectangle)
- A cube
- Different colored objects

You'd need to manage multiple VAOs and VBOs, track vertex counts, and remember the attribute layout for each one. That gets messy fast!

**Solution:** Create a `Mesh` struct that encapsulates:
- Vertex data (positions, colors, etc.)
- VAO and VBO handles
- Vertex count
- Drawing logic

This is a fundamental pattern in game engines - every renderable object is a mesh!

## Task

### Part 1: Create the Mesh Module

**Create `rustgl/src/mesh.rs`:**

```rust
use std::mem;
use std::ptr;

/// Represents a single vertex with position and color
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],  // x, y, z
    pub color: [f32; 3],     // r, g, b
}

impl Vertex {
    /// Creates a new vertex with position and color
    pub fn new(position: [f32; 3], color: [f32; 3]) -> Self {
        Vertex { position, color }
    }
}

/// A mesh holds vertex data and OpenGL buffer objects
pub struct Mesh {
    vao: u32,
    vbo: u32,
    vertex_count: i32,
}

impl Mesh {
    /// Creates a new mesh from a list of vertices
    ///
    /// # Arguments
    /// * `vertices` - Slice of Vertex structs to upload to GPU
    ///
    /// # Example
    /// ```
    /// let vertices = vec![
    ///     Vertex::new([-0.5, -0.5, 0.0], [1.0, 0.0, 0.0]),
    ///     Vertex::new([0.5, -0.5, 0.0], [0.0, 1.0, 0.0]),
    ///     Vertex::new([0.0, 0.5, 0.0], [0.0, 0.0, 1.0]),
    /// ];
    /// let mesh = Mesh::new(&vertices);
    /// ```
    pub fn new(vertices: &[Vertex]) -> Self {
        let mut vao = 0;
        let mut vbo = 0;

        unsafe {
            // Generate VAO and VBO
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            // Bind VAO first
            gl::BindVertexArray(vao);

            // Upload vertex data to VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<Vertex>()) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // Position attribute (location = 0)
            gl::VertexAttribPointer(
                0,                                    // location
                3,                                    // size (x, y, z)
                gl::FLOAT,                            // type
                gl::FALSE,                            // normalized
                mem::size_of::<Vertex>() as i32,      // stride (size of entire Vertex)
                ptr::null(),                          // offset (0 for position)
            );
            gl::EnableVertexAttribArray(0);

            // Color attribute (location = 1)
            gl::VertexAttribPointer(
                1,                                                 // location
                3,                                                 // size (r, g, b)
                gl::FLOAT,                                         // type
                gl::FALSE,                                         // normalized
                mem::size_of::<Vertex>() as i32,                   // stride
                (3 * mem::size_of::<f32>()) as *const std::ffi::c_void,  // offset (3 floats)
            );
            gl::EnableVertexAttribArray(1);

            // Unbind
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Mesh {
            vao,
            vbo,
            vertex_count: vertices.len() as i32,
        }
    }

    /// Renders the mesh
    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count);
            gl::BindVertexArray(0);
        }
    }

    /// Returns the VAO handle (useful for debugging)
    pub fn vao(&self) -> u32 {
        self.vao
    }

    /// Returns the vertex count
    pub fn vertex_count(&self) -> i32 {
        self.vertex_count
    }
}

// Cleanup when Mesh is dropped
impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}
```

**Key Rust concepts:**

1. **`#[derive(Copy, Clone, Debug)]`** - Auto-implement these traits for `Vertex`
   - `Copy` - Can be copied with simple bitwise copy (cheap)
   - `Clone` - Explicit cloning with `.clone()`
   - `Debug` - Can be printed with `{:?}`

2. **`&[Vertex]`** - Slice (borrowed view of an array/vector)
   - Works with both `&[Vertex; 3]` (arrays) and `&Vec<Vertex>` (vectors)

3. **`mem::size_of::<Vertex>()`** - Gets size of type at compile time
   - Much cleaner than manual calculation!

4. **`as *const _`** - Cast to raw pointer for OpenGL
   - The `_` lets Rust infer the type

### Part 2: Update Main.rs

**Add the module declaration at the top of `rustgl/src/main.rs`:**

```rust
extern crate gl;
extern crate glfw;

mod shader;
mod mesh;  // NEW!

use glfw::{Action, Context, Key};
use std::time::Instant;
use shader::Shader;
use mesh::{Mesh, Vertex};  // NEW!
```

**Replace the vertex array and VAO/VBO code with the Mesh:**

Find this section in `main()`:
```rust
let vertices: [f32; 18] = [
    // positions        // colors
    -0.5, -0.5, 0.0,   1.0, 0.0, 0.0,  // Bottom left (red)
     0.5, -0.5, 0.0,   0.0, 1.0, 0.0,  // Bottom right (green)
     0.0,  0.5, 0.0,   0.0, 0.0, 1.0,  // Top center (blue)
];

let (vao, vbo) = unsafe {
    // ... all the VAO/VBO setup code ...
};
```

**Replace it with:**

```rust
// Create a triangle mesh using the Vertex struct
let triangle_vertices = vec![
    Vertex::new([-0.5, -0.5, 0.0], [1.0, 0.0, 0.0]),  // Bottom left (red)
    Vertex::new([0.5, -0.5, 0.0], [0.0, 1.0, 0.0]),   // Bottom right (green)
    Vertex::new([0.0, 0.5, 0.0], [0.0, 0.0, 1.0]),    // Top center (blue)
];
let triangle = Mesh::new(&triangle_vertices);
```

**Update the render function signature and call:**

Change:
```rust
render(&mut window, vao, &shader);
```

To:
```rust
render(&mut window, &triangle, &shader);
```

**Update the render function:**

```rust
fn render(window: &mut glfw::Window, mesh: &Mesh, shader: &Shader) {
    unsafe {
        gl::ClearColor(0.1, 0.1, 0.2, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        shader.use_program();
        mesh.draw();  // Much cleaner!
    }
    window.swap_buffers();
}
```

**Remove the `check_gl_error()` function if you want** (we'll add better error handling later).

### Part 3: Build and Run

```bash
cd rustgl
cargo run
```

You should see the **same gradient triangle** as before! But now the code is much cleaner and more reusable.

### Part 4: Add More Meshes!

Now let's see the power of the `Mesh` struct. Add a second triangle:

**After creating the first triangle, add:**

```rust
// Create a second triangle (offset to the right, different colors)
let triangle2_vertices = vec![
    Vertex::new([0.1, -0.5, 0.0], [1.0, 1.0, 0.0]),   // Bottom left (yellow)
    Vertex::new([1.1, -0.5, 0.0], [0.0, 1.0, 1.0]),   // Bottom right (cyan)
    Vertex::new([0.6, 0.5, 0.0], [1.0, 0.0, 1.0]),    // Top center (magenta)
];
let triangle2 = Mesh::new(&triangle2_vertices);
```

**Update the render function to draw both:**

```rust
fn render(window: &mut glfw::Window, triangle1: &Mesh, triangle2: &Mesh, shader: &Shader) {
    unsafe {
        gl::ClearColor(0.1, 0.1, 0.2, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        shader.use_program();
        triangle1.draw();
        triangle2.draw();
    }
    window.swap_buffers();
}
```

**And update the call in the main loop:**

```rust
render(&mut window, &triangle, &triangle2, &shader);
```

Now you should see **two triangles** side by side!

## Understanding the Vertex Struct

Using a struct for vertices is much better than a raw float array:

**Before (error-prone):**
```rust
let vertices: [f32; 18] = [
    -0.5, -0.5, 0.0, 1.0, 0.0, 0.0,  // Is this position or color?
    0.5, -0.5, 0.0, 0.0, 1.0, 0.0,   // Easy to mix up!
    // ...
];
```

**After (clear and type-safe):**
```rust
let vertices = vec![
    Vertex::new([-0.5, -0.5, 0.0], [1.0, 0.0, 0.0]),  // Clear: position, then color
    Vertex::new([0.5, -0.5, 0.0], [0.0, 1.0, 0.0]),
    // ...
];
```

**Benefits:**
- **Type safety** - Can't accidentally swap position and color
- **Self-documenting** - Clear what each value represents
- **Automatic stride calculation** - `mem::size_of::<Vertex>()` is always correct
- **Easy to extend** - Add normals, UVs, etc. later

## Challenges

### Challenge 1: Create a Quad (Rectangle)

Create a mesh with 6 vertices (two triangles) to form a quad:

```rust
let quad_vertices = vec![
    // First triangle
    Vertex::new([-0.5, -0.5, 0.0], [1.0, 0.0, 0.0]),  // Bottom left
    Vertex::new([0.5, -0.5, 0.0], [0.0, 1.0, 0.0]),   // Bottom right
    Vertex::new([0.5, 0.5, 0.0], [0.0, 0.0, 1.0]),    // Top right

    // Second triangle
    Vertex::new([0.5, 0.5, 0.0], [0.0, 0.0, 1.0]),    // Top right
    Vertex::new([-0.5, 0.5, 0.0], [1.0, 1.0, 0.0]),   // Top left
    Vertex::new([-0.5, -0.5, 0.0], [1.0, 0.0, 0.0]),  // Bottom left
];
let quad = Mesh::new(&quad_vertices);
```

### Challenge 2: Helper Functions

Add helper functions to create common shapes:

```rust
impl Mesh {
    /// Creates a triangle mesh centered at origin
    pub fn triangle(color: [f32; 3]) -> Self {
        let vertices = vec![
            Vertex::new([-0.5, -0.5, 0.0], color),
            Vertex::new([0.5, -0.5, 0.0], color),
            Vertex::new([0.0, 0.5, 0.0], color),
        ];
        Mesh::new(&vertices)
    }

    /// Creates a quad mesh
    pub fn quad(color: [f32; 3]) -> Self {
        // TODO: implement this!
    }
}
```

Use it:
```rust
let red_triangle = Mesh::triangle([1.0, 0.0, 0.0]);
```

### Challenge 3: Vertex with Normals (Preview)

Add a normal vector to `Vertex` (we'll use this for lighting later):

```rust
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub normal: [f32; 3],  // NEW!
}
```

Update the vertex attribute pointers accordingly. For now, normals can be `[0.0, 0.0, 1.0]` (pointing towards camera).

### Challenge 4: Indexed Meshes (Advanced)

Right now we use `gl::DrawArrays`. For a quad, we duplicate vertices (6 vertices for 4 unique positions).

**Better:** Use an **Element Buffer Object (EBO)** with indices:

```rust
// Only 4 unique vertices
let vertices = vec![
    Vertex::new([-0.5, -0.5, 0.0], [1.0, 0.0, 0.0]),  // 0: Bottom left
    Vertex::new([0.5, -0.5, 0.0], [0.0, 1.0, 0.0]),   // 1: Bottom right
    Vertex::new([0.5, 0.5, 0.0], [0.0, 0.0, 1.0]),    // 2: Top right
    Vertex::new([-0.5, 0.5, 0.0], [1.0, 1.0, 0.0]),   // 3: Top left
];

// Indices define triangles (reuses vertices)
let indices: Vec<u32> = vec![
    0, 1, 2,  // First triangle
    2, 3, 0,  // Second triangle
];
```

This is more efficient for complex meshes!

## Success Criteria

- [ ] You've created `rustgl/src/mesh.rs` module
- [ ] Created the `Vertex` struct with position and color
- [ ] Created the `Mesh` struct with `new()` and `draw()` methods
- [ ] Added `mod mesh;` to `main.rs`
- [ ] Refactored triangle rendering to use `Mesh`
- [ ] Your code compiles and displays the gradient triangle
- [ ] You understand the benefits of encapsulation
- [ ] (Optional) You've tried rendering multiple meshes

## Common Issues

**"cannot find type `Vertex` in this scope"**
- Make sure you added `use mesh::{Mesh, Vertex};` at the top of `main.rs`
- Check that `Vertex` is marked `pub` in `mesh.rs`

**"mismatched types: expected `&[Vertex]`, found `Vec<Vertex>`"**
- Use `&triangle_vertices` (with `&`) when calling `Mesh::new()`
- The function expects a slice reference, not ownership

**Triangle doesn't appear or is corrupted**
- Check that `Vertex` struct layout matches the shader attributes
- Verify position is first (location = 0), color is second (location = 1)
- Make sure `#[derive(Copy, Clone)]` is on the `Vertex` struct

**"memory alignment" or weird rendering**
- Make sure `Vertex` fields are in the correct order
- The struct memory layout must match what you tell OpenGL with `VertexAttribPointer`

**Compile error about `mem::size_of`**
- Add `use std::mem;` at the top of `mesh.rs`

## Next Step

Excellent! You've learned how to organize rendering code with the `Mesh` abstraction. This pattern will scale to hundreds of objects.

Next: [Step 10: Transformations](./step-10-transformations.md), where you'll learn to move, rotate, and scale your meshes using matrices!

## Notes

- **The Mesh struct is a fundamental building block** in game engines
- Every 3D model (character, tree, building) is made of meshes
- We used `Vec<Vertex>` for flexibility - it can hold any number of vertices
- The `Drop` trait ensures GPU memory is freed when `Mesh` goes out of scope
- Later, we'll add:
  - Index buffers (EBO) for efficiency
  - Texture coordinates (UVs)
  - Normal vectors for lighting
  - Tangent/bitangent for normal mapping
- The `Vertex` struct layout affects **cache performance** - keep it small!
- We use `gl::STATIC_DRAW` because mesh data doesn't change each frame
  - For animated meshes, use `gl::DYNAMIC_DRAW` or `gl::STREAM_DRAW`
- Professional engines often have multiple mesh types:
  - `StaticMesh` - Never changes (buildings, terrain)
  - `SkinnedMesh` - For character animation
  - `ProceduralMesh` - Generated at runtime

**Memory layout visualization:**

```
Vertex struct in memory:
[position.x][position.y][position.z][color.r][color.g][color.b]
    0           4           8          12       16       20      (bytes)

Total size: 24 bytes (6 floats � 4 bytes)
Stride: 24 bytes
Position offset: 0
Color offset: 12
```

This matches our `VertexAttribPointer` calls perfectly!
