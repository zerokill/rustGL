# Step 07: First Triangle

**Phase:** 2 - Core Rendering
**Difficulty:** Intermediate
**Estimated Time:** 1.5 hours

## Goal

Render your first triangle using modern OpenGL (VAO, VBO, shaders).

## What You'll Learn

- Vertex Array Objects (VAO)
- Vertex Buffer Objects (VBO)
- Writing simple GLSL shaders
- The OpenGL rendering pipeline
- Vertex attributes

## Background

Modern OpenGL uses a pipeline approach:

```
Vertices → Vertex Shader → Rasterization → Fragment Shader → Screen
```

**Vertex Shader**: Transforms 3D positions
**Fragment Shader**: Determines pixel colors

You need to:
1. Store vertex data in GPU memory (VBO)
2. Describe the data layout (VAO)
3. Write shaders to process the data
4. Issue a draw call

## Task

### 1. Create Vertex Data

A triangle needs 3 vertices. Each vertex has a position (x, y, z):

```rust
// At the top of main(), after loading OpenGL:
let vertices: [f32; 9] = [
    // x     y      z
    -0.5, -0.5,  0.0,  // Bottom left
     0.5, -0.5,  0.0,  // Bottom right
     0.0,  0.5,  0.0,  // Top center
];
```

### 2. Create Shaders

Create two strings for your shaders:

```rust
const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;

    void main() {
        gl_Position = vec4(aPos, 1.0);
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;

    void main() {
        FragColor = vec4(1.0, 0.5, 0.2, 1.0);  // Orange color
    }
"#;
```

### 3. Compile Shaders

Add a helper function to compile shaders:

```rust
fn compile_shader(source: &str, shader_type: gl::types::GLenum) -> u32 {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        let c_str = std::ffi::CString::new(source.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        // Check for compilation errors
        let mut success = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer = vec![0u8; len as usize];
            gl::GetShaderInfoLog(shader, len, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut i8);
            panic!(
                "Shader compilation failed: {}",
                String::from_utf8_lossy(&buffer)
            );
        }

        shader
    }
}
```

### 4. Link Shader Program

Add a function to create a shader program:

```rust
fn create_shader_program(vertex_src: &str, fragment_src: &str) -> u32 {
    unsafe {
        let vertex_shader = compile_shader(vertex_src, gl::VERTEX_SHADER);
        let fragment_shader = compile_shader(fragment_src, gl::FRAGMENT_SHADER);

        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);

        // Check for linking errors
        let mut success = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            let mut len = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer = vec![0u8; len as usize];
            gl::GetProgramInfoLog(program, len, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut i8);
            panic!("Program linking failed: {}", String::from_utf8_lossy(&buffer));
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        program
    }
}
```

### 5. Create VAO and VBO

In `main()`, after loading OpenGL and creating vertex data:

```rust
let (vao, vbo) = unsafe {
    // Create Vertex Array Object
    let mut vao = 0;
    gl::GenVertexArrays(1, &mut vao);
    gl::BindVertexArray(vao);

    // Create Vertex Buffer Object
    let mut vbo = 0;
    gl::GenBuffers(1, &mut vbo);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        (vertices.len() * std::mem::size_of::<f32>()) as isize,
        vertices.as_ptr() as *const _,
        gl::STATIC_DRAW,
    );

    // Configure vertex attributes
    gl::VertexAttribPointer(
        0,                                      // location = 0 in shader
        3,                                      // 3 components (x, y, z)
        gl::FLOAT,                              // type
        gl::FALSE,                              // normalized?
        (3 * std::mem::size_of::<f32>()) as i32, // stride
        std::ptr::null(),                       // offset
    );
    gl::EnableVertexAttribArray(0);

    // Unbind
    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    gl::BindVertexArray(0);

    (vao, vbo)
};

// Create shader program
let shader_program = create_shader_program(VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE);
```

### 6. Render the Triangle

Modify your `render` function:

```rust
fn render(window: &mut glfw::Window, vao: u32, shader_program: u32) {
    unsafe {
        gl::ClearColor(0.1, 0.1, 0.2, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        // Use our shader program
        gl::UseProgram(shader_program);

        // Draw the triangle
        gl::BindVertexArray(vao);
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
        gl::BindVertexArray(0);
    }

    window.swap_buffers();
}
```

### 7. Update Main Loop

Pass the necessary variables to render:

```rust
// In the main loop:
render(&mut window, vao, shader_program);
```

### 8. Cleanup

Add cleanup before main exits:

```rust
// After main loop, before end of main():
unsafe {
    gl::DeleteVertexArrays(1, &vao);
    gl::DeleteBuffers(1, &vbo);
    gl::DeleteProgram(shader_program);
}
```

## Understanding the Code

**VAO (Vertex Array Object)**:
- Stores vertex attribute configuration
- One VAO can be reused for many draw calls

**VBO (Vertex Buffer Object)**:
- Stores actual vertex data in GPU memory
- `STATIC_DRAW` = data won't change

**VertexAttribPointer**:
- Tells OpenGL how to interpret the vertex data
- Location 0, 3 floats per vertex, tightly packed

**DrawArrays**:
- `TRIANGLES` = every 3 vertices form a triangle
- Start at vertex 0, draw 3 vertices

## Challenges

1. **Different positions**: Move the triangle vertices
2. **Different color**: Change the fragment shader color
3. **Multiple triangles**: Add more vertices and draw 2 triangles (6 vertices)
4. **Wireframe mode**: Add before drawing:
   ```rust
   gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
   ```

## Success Criteria

- [ ] Program compiles without errors
- [ ] You see an orange triangle on screen
- [ ] Triangle remains visible when window is resized
- [ ] No OpenGL errors in console

## Common Issues

**Nothing renders / Black screen**:
- Check shader compilation messages
- Ensure VAO is bound before drawing
- Verify `UseProgram` is called before drawing

**Compilation errors**:
- Check GLSL syntax in shaders
- Ensure version `#version 330 core` matches your OpenGL version

**Triangle appears upside down**:
- This is normal! OpenGL's origin is bottom-left
- We'll fix this when we add a projection matrix

## Next Steps

Congratulations! You've rendered your first triangle. Continue to [Step 08: Shaders](./step-08-shaders.md) to learn more about shaders.

## Notes

- This is "modern" OpenGL (3.3+) - no immediate mode!
- VAOs are required in Core Profile
- Shaders are small programs that run on the GPU
- Every pixel on screen goes through the fragment shader
