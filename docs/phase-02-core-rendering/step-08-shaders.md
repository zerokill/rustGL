# Step 08: Shaders (Advanced)

**Phase:** 2 - Core Rendering
**Difficulty:** Intermediate
**Estimated Time:** 1-2 hours

## Goal

Improve shader management by creating a `Shader` struct and **refactor your code into modules** - your first lesson in Rust code organization!

## What You'll Learn

- Creating a reusable `Shader` struct
- Loading shader source from files
- **Your first Rust module** (`shader.rs`)
- The `mod` keyword and module system
- Reading files with `std::fs`
- Rust error handling with `Result`
- Struct methods and `impl` blocks
- Public vs private functions (`pub`)

## Background

In Step 07, you embedded shaders as strings in `main.rs`. This works but has problems:
- Hard to edit shaders (no syntax highlighting in Rust strings)
- Can't share shaders between different programs
- `main.rs` gets cluttered as project grows
- No syntax checking for GLSL code

**Professional approach:**
1. Store shaders as `.glsl` files (or `.vert`/`.frag`)
2. Create a `Shader` struct to manage compilation and linking
3. **Move shader code to its own module** (Rust best practice!)

This is also your **first refactoring** - learning when and how to organize Rust code properly.

## Rust Module System Basics

Before we start, let's understand Rust modules:

```rust
// In main.rs:
mod shader;  // This tells Rust: "look for src/shader.rs"

use shader::Shader;  // Import the Shader struct from shader module

fn main() {
    let shader = Shader::new("vertex.glsl", "fragment.glsl");
}
```

**Key concepts:**
- `mod shader;` declares a module (looks for `src/shader.rs`)
- `pub` makes items public (accessible from other modules)
- Without `pub`, items are private to the module
- `use` imports items to avoid fully qualified names

## Task

### Part 1: Create Shader Files

Create a `shaders/` directory in your `rustgl/` folder:

```bash
mkdir -p rustgl/shaders
```

**Create `rustgl/shaders/basic.vert`:**
```glsl
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;

out vec3 ourColor;

void main() {
    gl_Position = vec4(aPos, 1.0);
    ourColor = aColor;
}
```

**Create `rustgl/shaders/basic.frag`:**
```glsl
#version 330 core
in vec3 ourColor;
out vec4 FragColor;

void main() {
    FragColor = vec4(ourColor, 1.0);
}
```

**What changed:**
- Vertex shader now accepts **two attributes**: position (`aPos`) and color (`aColor`)
- Vertex shader **passes color to fragment shader** via `out vec3 ourColor`
- Fragment shader **receives interpolated color** via `in vec3 ourColor`
- Fragment shader uses the interpolated color instead of hardcoded orange

### Part 2: Create the Shader Module

**Create `rustgl/src/shader.rs`:**

```rust
use std::ffi::CString;
use std::fs;
use std::ptr;

/// Manages a compiled and linked OpenGL shader program
pub struct Shader {
    pub id: u32,  // OpenGL program ID
}

impl Shader {
    /// Creates a new shader program from vertex and fragment shader files
    ///
    /// # Arguments
    /// * `vertex_path` - Path to vertex shader file (e.g., "shaders/basic.vert")
    /// * `fragment_path` - Path to fragment shader file (e.g., "shaders/basic.frag")
    ///
    /// # Panics
    /// Panics if shader files can't be read or shaders fail to compile/link
    pub fn new(vertex_path: &str, fragment_path: &str) -> Self {
        // Read shader source files
        let vertex_src = fs::read_to_string(vertex_path)
            .expect(&format!("Failed to read vertex shader: {}", vertex_path));

        let fragment_src = fs::read_to_string(fragment_path)
            .expect(&format!("Failed to read fragment shader: {}", fragment_path));

        unsafe {
            // Compile shaders
            let vertex_shader = Self::compile_shader(&vertex_src, gl::VERTEX_SHADER);
            let fragment_shader = Self::compile_shader(&fragment_src, gl::FRAGMENT_SHADER);

            // Link program
            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);

            // Check for linking errors
            Self::check_link_errors(program);

            // Clean up individual shaders (no longer needed after linking)
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            Shader { id: program }
        }
    }

    /// Activates this shader program
    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    /// Compiles a shader from source code
    ///
    /// Private helper function (no `pub` keyword)
    unsafe fn compile_shader(source: &str, shader_type: gl::types::GLenum) -> u32 {
        let shader = gl::CreateShader(shader_type);
        let c_str = CString::new(source.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Check for compilation errors
        Self::check_compile_errors(shader, shader_type);

        shader
    }

    /// Checks for shader compilation errors
    unsafe fn check_compile_errors(shader: u32, shader_type: gl::types::GLenum) {
        let mut success = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

        if success == 0 {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

            let mut buffer = vec![0u8; len as usize];
            gl::GetShaderInfoLog(
                shader,
                len,
                ptr::null_mut(),
                buffer.as_mut_ptr() as *mut i8,
            );

            let shader_type_str = if shader_type == gl::VERTEX_SHADER {
                "VERTEX"
            } else {
                "FRAGMENT"
            };

            panic!(
                "{} shader compilation failed:\n{}",
                shader_type_str,
                String::from_utf8_lossy(&buffer)
            );
        }
    }

    /// Checks for program linking errors
    unsafe fn check_link_errors(program: u32) {
        let mut success = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);

        if success == 0 {
            let mut len = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);

            let mut buffer = vec![0u8; len as usize];
            gl::GetProgramInfoLog(
                program,
                len,
                ptr::null_mut(),
                buffer.as_mut_ptr() as *mut i8,
            );

            panic!(
                "Shader program linking failed:\n{}",
                String::from_utf8_lossy(&buffer)
            );
        }
    }
}

// Cleanup when Shader is dropped (goes out of scope)
impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
```

**Key Rust concepts in this code:**

1. **`pub struct Shader`** - Public struct, accessible from other modules
2. **`impl Shader { ... }`** - Implementation block, defines methods on the struct
3. **`pub fn new(...)`** - Public "constructor" function (Rust convention)
4. **`Self`** - Refers to the type being implemented (`Shader`)
5. **`&self`** - Borrows the struct (methods can read it)
6. **`fs::read_to_string()`** - Reads entire file into a `String`
7. **`.expect()`** - Unwraps `Result`, panics with message if error occurs
8. **`impl Drop`** - Destructor, runs when value goes out of scope

### Part 3: Update Main.rs

Now update your `rustgl/src/main.rs` to use the new module:

```rust
extern crate gl;
extern crate glfw;

// Declare the shader module
mod shader;

use glfw::{Action, Context, Key};
use std::time::Instant;

// Import Shader struct from shader module
use shader::Shader;

fn main() {
    // Initialize GLFW
    let mut glfw = glfw::init_no_callbacks().expect("Failed to initialize GLFW");

    // Request OpenGL version (platform-specific)
    #[cfg(target_os = "linux")]
    {
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 5));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
    }

    #[cfg(target_os = "macos")]
    {
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    }

    // Create window
    let (mut window, events) = glfw
        .create_window(
            1024,
            768,
            "RustGL by mau",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // Load OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // Print OpenGL version
    unsafe {
        let version = std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8);
        println!("OpenGL Version: {}", version.to_str().unwrap());
    }

    // NEW: Colored triangle vertices (position + color)
    let vertices: [f32; 18] = [
        // positions        // colors
        -0.5, -0.5, 0.0,   1.0, 0.0, 0.0,  // Bottom left (red)
         0.5, -0.5, 0.0,   0.0, 1.0, 0.0,  // Bottom right (green)
         0.0,  0.5, 0.0,   0.0, 0.0, 1.0,  // Top center (blue)
    ];

    // Create VAO and VBO
    let (vao, vbo) = unsafe {
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as isize,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        // Position attribute (location = 0)
        gl::VertexAttribPointer(
            0,                                           // location
            3,                                           // size (x, y, z)
            gl::FLOAT,                                   // type
            gl::FALSE,                                   // normalized
            (6 * std::mem::size_of::<f32>()) as i32,    // stride (6 floats per vertex)
            std::ptr::null(),                            // offset (0)
        );
        gl::EnableVertexAttribArray(0);

        // Color attribute (location = 1)
        gl::VertexAttribPointer(
            1,                                                              // location
            3,                                                              // size (r, g, b)
            gl::FLOAT,                                                      // type
            gl::FALSE,                                                      // normalized
            (6 * std::mem::size_of::<f32>()) as i32,                       // stride
            (3 * std::mem::size_of::<f32>()) as *const std::ffi::c_void,  // offset (3 floats)
        );
        gl::EnableVertexAttribArray(1);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        (vao, vbo)
    };

    // NEW: Use the Shader module to load shaders from files!
    let shader = Shader::new("shaders/basic.vert", "shaders/basic.frag");

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
        render(&mut window, vao, &shader);
    }
}

fn process_events(
    window: &mut glfw::Window,
    events: &glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
) {
    window.glfw.poll_events();
    for (_, event) in glfw::flush_messages(events) {
        handle_window_event(window, event);
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        glfw::WindowEvent::Key(Key::Space, _, Action::Press, _) => {
            println!("Space pressed!");
        }
        glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
            gl::Viewport(0, 0, width, height);
        },
        _ => {}
    }
}

// NEW: render() now takes a reference to Shader
fn render(window: &mut glfw::Window, vao: u32, shader: &Shader) {
    unsafe {
        gl::ClearColor(0.1, 0.1, 0.2, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        shader.use_program();  // Use the shader's method
        gl::BindVertexArray(vao);
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
        gl::BindVertexArray(0);
    }
    window.swap_buffers();
}
```

**Key changes from Step 07:**

1. **Added `mod shader;`** at top - declares the shader module
2. **Added `use shader::Shader;`** - imports the Shader struct
3. **Changed vertices array** from 9 floats to 18 floats (position + color)
4. **Added second vertex attribute** for color (location = 1)
5. **Changed stride from 3 to 6** - each vertex now has 6 floats
6. **Added offset for color attribute** - starts 3 floats into the data
7. **Replaced shader compilation code** with `Shader::new(...)`
8. **Updated render()** to take `&Shader` and call `shader.use_program()`
9. **Removed `compile_shader()` and `create_shader_program()`** - now in shader module

### Part 4: Build and Run

```bash
cd rustgl
cargo run
```

You should see a **beautiful gradient triangle**:
- Bottom left corner: **Red**
- Bottom right corner: **Green**
- Top center: **Blue**
- The colors **blend smoothly** in between (interpolation)

## Understanding Vertex Attributes

The new vertex data has **interleaved attributes**:

```
Vertex 0:  [-0.5, -0.5, 0.0,  1.0, 0.0, 0.0]
            └─────position────┘  └────color────┘

Vertex 1:  [0.5, -0.5, 0.0,   0.0, 1.0, 0.0]
            └────position────┘  └────color────┘

Vertex 2:  [0.0, 0.5, 0.0,    0.0, 0.0, 1.0]
            └───position────┘  └────color────┘
```

**Stride:** Distance between the start of one vertex and the start of the next
```
stride = 6 floats × 4 bytes = 24 bytes
```

**Offsets:**
- Position attribute: offset = 0 (starts at beginning)
- Color attribute: offset = 12 bytes (3 floats × 4 bytes)

## Understanding Color Interpolation

The GPU **automatically interpolates** vertex attributes:

1. **Vertex Shader** receives exact vertex colors (red, green, blue)
2. **Vertex Shader** passes colors to fragment shader via `out vec3 ourColor`
3. **Rasterizer** (GPU hardware) generates fragments for every pixel in the triangle
4. **Rasterizer interpolates** the color values based on distance from vertices
5. **Fragment Shader** receives interpolated color via `in vec3 ourColor`
6. **Fragment Shader** outputs final color

Example: A pixel halfway between red and green vertices gets color (0.5, 0.5, 0.0) = yellow!

## Challenges

### Challenge 1: Change Colors
Modify the vertex colors to create different gradients:
- Try all white `[1.0, 1.0, 1.0]`
- Try cyan, magenta, yellow
- Make it grayscale

### Challenge 2: Add More Triangles
Create a second triangle or a quad (two triangles). You'll need:
- More vertices in the array
- Update `gl::DrawArrays(gl::TRIANGLES, 0, 6)` to draw 6 vertices

### Challenge 3: Load Different Shaders
Create a second set of shader files (`solid.vert`, `solid.frag`) that render a solid color:
```glsl
// solid.frag
#version 330 core
out vec4 FragColor;

void main() {
    FragColor = vec4(1.0, 1.0, 0.0, 1.0);  // Yellow
}
```

Load both shaders and switch between them with a key press.

### Challenge 4: Uniform Colors
Add a uniform to your fragment shader:
```glsl
// In basic.frag:
uniform vec3 customColor;

void main() {
    FragColor = vec4(customColor, 1.0);
}
```

Then in Rust, add a method to `Shader`:
```rust
pub fn set_vec3(&self, name: &str, x: f32, y: f32, z: f32) {
    unsafe {
        let c_name = CString::new(name).unwrap();
        let location = gl::GetUniformLocation(self.id, c_name.as_ptr());
        gl::Uniform3f(location, x, y, z);
    }
}
```

Use it:
```rust
shader.use_program();
shader.set_vec3("customColor", 1.0, 0.5, 0.2);  // Orange
```

## Success Criteria

- [ ] You've created `rustgl/shaders/basic.vert` and `rustgl/shaders/basic.frag`
- [ ] You've created `rustgl/src/shader.rs` module
- [ ] You've added `mod shader;` to `main.rs`
- [ ] Shaders load from `.glsl` files
- [ ] Triangle has **interpolated RGB colors** (red, green, blue at corners)
- [ ] You understand the `mod` keyword and module system
- [ ] You understand vertex attributes with stride and offset
- [ ] Code is organized into modules
- [ ] You understand the difference between `pub` and private items

## Common Issues

**"Failed to read vertex shader: shaders/basic.vert"**
- Make sure you run `cargo run` from the `rustgl/` directory
- The path is relative to where you run the command
- Check that `rustgl/shaders/` directory exists

**"no such file or directory"**
- Did you create the `shaders/` directory?
- Are the files named correctly? (`.vert` and `.frag` extensions)

**"unresolved import `shader`"**
- Make sure `mod shader;` is at the top of `main.rs`
- Make sure `shader.rs` exists in `rustgl/src/`
- Make sure `Shader` struct has `pub` keyword

**Triangle is black or wrong colors**
- Check that you updated the vertex array to 18 floats
- Check that you added the second `VertexAttribPointer` for color
- Check that stride is `6 * sizeof(f32)`
- Check that offset for color is `3 * sizeof(f32)`

**"field `id` of struct `Shader` is private"**
- You probably tried to access `shader.id` from `main.rs`
- Use `shader.use_program()` instead (that's why we made it public)

## Next Step

Excellent work! You've learned:
- Rust's module system (`mod`, `pub`, `use`)
- Struct methods and `impl` blocks
- File I/O with `std::fs`
- The `Drop` trait for cleanup
- Multiple vertex attributes with stride and offset
- GPU color interpolation

Next: [Step 09: Mesh Structure](./step-09-mesh-structure.md), where you'll create a reusable `Mesh` struct to encapsulate VAO/VBO management!

## Notes

- **This is your first refactoring!** You learned when to split code into modules
- Real Rust projects use modules extensively - start building this habit now
- See [PROJECT_STRUCTURE.md](../../PROJECT_STRUCTURE.md) for the complete organization strategy
- The `impl Drop` trait is Rust's RAII (Resource Acquisition Is Initialization) - cleanup happens automatically
- As your project grows (Phase 3+), you'll refactor into subdirectories like `src/graphics/shader.rs`
- OpenGL's vertex attribute system is **extremely flexible** - you can pack any data you want
- Stride and offset calculations are **critical** - get them wrong and you'll see garbage or crashes
- The `unsafe` blocks are necessary because we're calling C functions (OpenGL) - Rust can't verify their safety
