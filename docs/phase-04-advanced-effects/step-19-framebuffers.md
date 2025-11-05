# Step 19: Framebuffers (Render to Texture)

## Overview
**Framebuffers** allow you to render to textures instead of directly to the screen. This fundamental technique enables post-processing effects, mirrors, portals, shadow mapping, and much more. Instead of your scene going straight to the window, you can capture it, manipulate it, and then display it.

## What You'll Learn
- **Framebuffer Objects (FBO)**: Off-screen rendering targets
- **Render targets**: Color attachments and depth attachments
- **Render passes**: Multi-stage rendering pipeline
- **Screen-space quads**: Full-screen rendering technique
- **Post-processing foundation**: The basis for bloom, blur, color grading, etc.

## Concepts

### What is a Framebuffer?

By default, OpenGL renders to the **default framebuffer** (the window). A **Framebuffer Object (FBO)** is a custom render target that can have:

1. **Color attachment(s)**: Textures that store color data
2. **Depth attachment**: Texture/renderbuffer for depth testing
3. **Stencil attachment**: For stencil testing (optional)

**Key idea**: Render your scene to a texture, then use that texture for further processing.

### Why Framebuffers?

**Without framebuffers:**
```
Scene � Screen (that's it)
```

**With framebuffers:**
```
Scene � Texture � Post-Process � Texture � Screen
        �
    Can be reused for reflections, shadows, etc.
```

**Common uses:**
- **Post-processing**: Bloom, motion blur, color grading, vignette
- **Mirrors/reflections**: Render scene from mirror's perspective
- **Shadow mapping**: Render depth from light's perspective
- **Portals**: Render scene from portal's perspective
- **Minimap**: Render top-down view
- **Picture-in-picture**: Multiple views simultaneously

### Framebuffer Components

#### 1. Color Attachment
A texture that stores the rendered image:
```rust
gl::TexImage2D(
    gl::TEXTURE_2D,
    0,
    gl::RGB as i32,      // Store RGB color
    width, height,
    0,
    gl::RGB,
    gl::UNSIGNED_BYTE,
    std::ptr::null(),    // No initial data
);
```

#### 2. Depth Attachment
Either a texture or renderbuffer for depth testing:
```rust
// Option A: Renderbuffer (if you don't need to read depth)
gl::GenRenderbuffers(1, &mut rbo);
gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, width, height);

// Option B: Depth texture (if you need to read depth for effects)
gl::TexImage2D(
    gl::TEXTURE_2D,
    0,
    gl::DEPTH_COMPONENT as i32,
    width, height,
    0,
    gl::DEPTH_COMPONENT,
    gl::FLOAT,
    std::ptr::null(),
);
```

#### 3. Framebuffer Object
Container that holds attachments:
```rust
gl::GenFramebuffers(1, &mut fbo);
gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, texture, 0);
gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::RENDERBUFFER, rbo);

// Check if framebuffer is complete
if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
    panic!("Framebuffer is not complete!");
}
```

### Rendering Pipeline with Framebuffers

**Two-pass rendering:**

```rust
// PASS 1: Render scene to framebuffer
gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
render_scene();  // Scene is captured to texture

// PASS 2: Render texture to screen
gl::BindFramebuffer(gl::FRAMEBUFFER, 0);  // 0 = default framebuffer (screen)
gl::Clear(gl::COLOR_BUFFER_BIT);
render_screen_quad(texture);  // Display the captured texture
```

### Screen-Space Quad

To display a texture full-screen, render a quad that covers the entire screen:

```rust
// Quad vertices in NDC (Normalized Device Coordinates)
// Covers entire screen from (-1,-1) to (1,1)
let vertices = vec![
    // positions     // texCoords
    -1.0,  1.0,      0.0, 1.0,  // Top-left
    -1.0, -1.0,      0.0, 0.0,  // Bottom-left
     1.0, -1.0,      1.0, 0.0,  // Bottom-right

    -1.0,  1.0,      0.0, 1.0,  // Top-left
     1.0, -1.0,      1.0, 0.0,  // Bottom-right
     1.0,  1.0,      1.0, 1.0,  // Top-right
];
```

## Implementation Plan

### 1. Create Framebuffer Module

Create a new file `src/framebuffer.rs`:

```rust
use gl::types::*;

pub struct Framebuffer {
    fbo: GLuint,
    color_texture: GLuint,
    rbo: GLuint,  // Renderbuffer for depth/stencil
    width: u32,
    height: u32,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let mut fbo = 0;
        let mut color_texture = 0;
        let mut rbo = 0;

        unsafe {
            // Create framebuffer
            gl::GenFramebuffers(1, &mut fbo);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

            // Create color texture attachment
            gl::GenTextures(1, &mut color_texture);
            gl::BindTexture(gl::TEXTURE_2D, color_texture);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                width as i32,
                height as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                std::ptr::null(),
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            // Attach color texture to framebuffer
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                color_texture,
                0,
            );

            // Create renderbuffer for depth and stencil
            gl::GenRenderbuffers(1, &mut rbo);
            gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
            gl::RenderbufferStorage(
                gl::RENDERBUFFER,
                gl::DEPTH24_STENCIL8,
                width as i32,
                height as i32,
            );

            // Attach renderbuffer to framebuffer
            gl::FramebufferRenderbuffer(
                gl::FRAMEBUFFER,
                gl::DEPTH_STENCIL_ATTACHMENT,
                gl::RENDERBUFFER,
                rbo,
            );

            // Check if framebuffer is complete
            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                panic!("Framebuffer is not complete!");
            }

            // Unbind framebuffer
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        Framebuffer {
            fbo,
            color_texture,
            rbo,
            width,
            height,
        }
    }

    /// Bind this framebuffer for rendering
    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);
            gl::Viewport(0, 0, self.width as i32, self.height as i32);
        }
    }

    /// Unbind framebuffer (bind default framebuffer)
    pub fn unbind() {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    /// Get the color texture ID for rendering to screen
    pub fn texture(&self) -> GLuint {
        self.color_texture
    }

    /// Resize the framebuffer (useful for window resizing)
    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;

        unsafe {
            // Resize color texture
            gl::BindTexture(gl::TEXTURE_2D, self.color_texture);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                width as i32,
                height as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                std::ptr::null(),
            );

            // Resize renderbuffer
            gl::BindRenderbuffer(gl::RENDERBUFFER, self.rbo);
            gl::RenderbufferStorage(
                gl::RENDERBUFFER,
                gl::DEPTH24_STENCIL8,
                width as i32,
                height as i32,
            );
        }
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.fbo);
            gl::DeleteTextures(1, &self.color_texture);
            gl::DeleteRenderbuffers(1, &self.rbo);
        }
    }
}
```

**Key methods:**
- `new()`: Creates framebuffer with color texture and depth/stencil renderbuffer
- `bind()`: Switch to rendering to this framebuffer
- `unbind()`: Switch back to default framebuffer (screen)
- `texture()`: Get the rendered texture for display
- `resize()`: Handle window resizing

### 2. Create Screen Quad

Add to `mesh.rs`:

```rust
/// Create a full-screen quad for post-processing
/// Positions in NDC (-1 to 1), texture coords (0 to 1)
pub fn screen_quad() -> Self {
    let vertices = vec![
        // positions (NDC)        // colors (unused)      // normals (unused)    // texCoords
        Vertex::new([-1.0,  1.0, 0.0], [1.0, 1.0, 1.0], [0.0, 0.0, 1.0], [0.0, 1.0]),  // Top-left
        Vertex::new([-1.0, -1.0, 0.0], [1.0, 1.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0]),  // Bottom-left
        Vertex::new([ 1.0, -1.0, 0.0], [1.0, 1.0, 1.0], [0.0, 0.0, 1.0], [1.0, 0.0]),  // Bottom-right

        Vertex::new([-1.0,  1.0, 0.0], [1.0, 1.0, 1.0], [0.0, 0.0, 1.0], [0.0, 1.0]),  // Top-left
        Vertex::new([ 1.0, -1.0, 0.0], [1.0, 1.0, 1.0], [0.0, 0.0, 1.0], [1.0, 0.0]),  // Bottom-right
        Vertex::new([ 1.0,  1.0, 0.0], [1.0, 1.0, 1.0], [0.0, 0.0, 1.0], [1.0, 1.0]),  // Top-right
    ];

    Mesh::new(&vertices)
}
```

**Note:** The quad positions are in NDC (Normalized Device Coordinates), so they don't need transformation matrices.

### 3. Create Screen Shader

The screen shader is very simple - just sample the texture:

#### `shader/screen.vert`
```glsl
#version 410 core

layout (location = 0) in vec3 aPos;
layout (location = 3) in vec2 aTexCoord;

out vec2 TexCoords;

void main()
{
    gl_Position = vec4(aPos.x, aPos.y, 0.0, 1.0);
    TexCoords = aTexCoord;
}
```

**Key points:**
- No transformation matrices needed (already in NDC)
- Pass through texture coordinates

#### `shader/screen.frag`
```glsl
#version 410 core

out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D screenTexture;

void main()
{
    // Simple passthrough - just display the texture
    vec3 col = texture(screenTexture, TexCoords).rgb;
    FragColor = vec4(col, 1.0);
}
```

**For now:** This is a simple passthrough. In later steps, you'll add post-processing effects here (bloom, color grading, etc.).

### 4. Update Main.rs

Add the framebuffer module and update rendering:

```rust
mod framebuffer;
use framebuffer::Framebuffer;

fn main() {
    // ... existing setup ...

    // Create framebuffer
    let mut framebuffer = Framebuffer::new(1024, 768);

    // Create screen quad and shader for post-processing
    let screen_quad = Mesh::screen_quad();
    let screen_shader = Shader::new("shader/screen.vert", "shader/screen.frag");

    // ... scene setup ...

    // Main loop
    while !window.should_close() {
        // ... input handling ...

        // PASS 1: Render scene to framebuffer
        framebuffer.bind();
        render_scene(&mut window, &scene, &shader, &texture, &camera, wireframe_mode, use_texture);

        // PASS 2: Render framebuffer texture to screen
        Framebuffer::unbind();
        render_to_screen(&mut window, &screen_quad, &screen_shader, framebuffer.texture());
    }
}

fn render_scene(
    window: &mut glfw::Window,
    scene: &Scene,
    shader: &Shader,
    texture: &Texture,
    camera: &Camera,
    wireframe_mode: bool,
    use_texture: bool,
) {
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::ClearColor(0.1, 0.1, 0.2, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

        if wireframe_mode {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        } else {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
        }

        let view = camera.get_view_matrix();
        let projection = glm::perspective(1024.0 / 768.0, camera.zoom.to_radians(), 0.1, 100.0);

        shader.use_program();
        shader.set_vec3("viewPos", &camera.position);
        texture.bind(0);
        shader.set_int("textureSampler", 0);
        shader.set_bool("useTexture", use_texture);

        scene.render(&shader, &view, &projection);
    }
}

fn render_to_screen(
    window: &mut glfw::Window,
    screen_quad: &Mesh,
    screen_shader: &Shader,
    texture_id: GLuint,
) {
    unsafe {
        gl::Disable(gl::DEPTH_TEST);  // No depth test for screen quad
        gl::ClearColor(1.0, 1.0, 1.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        screen_shader.use_program();
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);
        screen_shader.set_int("screenTexture", 0);

        screen_quad.draw();
    }
    window.swap_buffers();
}
```

**Two render functions:**
1. `render_scene()`: Renders 3D scene to framebuffer (same as before)
2. `render_to_screen()`: Renders framebuffer texture to screen quad

### 5. Handle Window Resizing

Update the window resize handler:

```rust
glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
    gl::Viewport(0, 0, width, height);
    framebuffer.resize(width as u32, height as u32);  // Resize framebuffer too!
},
```

**Important:** When window resizes, framebuffer must resize to match.

## Testing

After implementation:

1. **Basic test**: Scene should look exactly the same (framebuffer is transparent right now)
2. **Verify framebuffer works**: Try rendering the texture upside-down in screen shader to confirm it's working
3. **Try simple post-processing**: Add a color tint in screen.frag to verify the pipeline works

### Simple Post-Processing Test

In `screen.frag`, try these effects:

```glsl
// Grayscale
float average = (col.r + col.g + col.b) / 3.0;
FragColor = vec4(vec3(average), 1.0);

// Invert colors
FragColor = vec4(1.0 - col, 1.0);

// Kernel effects (blur, edge detection) - see extensions below
```

## Common Issues

### Issue: Black screen
**Solutions:**
- Check `gl::CheckFramebufferStatus()` - is it FRAMEBUFFER_COMPLETE?
- Verify you're binding framebuffer before rendering scene
- Check unbind (bind 0) before rendering screen quad
- Ensure texture coordinates are correct on screen quad

### Issue: Upside-down image
**Solution:** Framebuffer textures are Y-flipped. Either:
- Flip texture coords in screen quad
- Flip in screen vertex shader: `TexCoords.y = 1.0 - aTexCoord.y;`

### Issue: Window resize causes artifacts
**Solution:** Call `framebuffer.resize()` in window resize handler

### Issue: Scene appears stretched/squashed
**Solution:** Ensure framebuffer dimensions match window dimensions

## Performance Notes

- **Multiple render passes**: Framebuffers require rendering scene multiple times (small overhead)
- **Texture size**: Framebuffer resolution affects performance and quality
- **Overdraw**: Screen quad is full-screen - expensive fragment shader operations multiply

**Optimization tips:**
- Use lower resolution framebuffers for expensive effects
- Minimize post-processing passes
- Use MRT (Multiple Render Targets) to output to multiple textures in one pass

## Extensions (Future Steps)

Once you have framebuffers working, you can add:

1. **Bloom** (Step 19.5): Extract bright areas, blur, add back
2. **Motion Blur**: Store previous frames, blend
3. **Depth of Field**: Use depth buffer to blur based on distance
4. **Screen-Space Reflections**: Ray-march through depth buffer
5. **Temporal Anti-Aliasing (TAA)**: Blend with previous frames
6. **Color Grading**: LUT-based color correction

## Next Steps

After this step:
-  Render to texture working
-  Post-processing pipeline established
-  Foundation for bloom, blur, and other effects
-  Ready for **Step 20: Reflections** (render skybox from mirror perspective)

## Summary

**Core concepts:**
- Framebuffers: Off-screen render targets
- Two-pass rendering: Scene � Texture � Screen
- Screen quad: Full-screen texture display
- Post-processing foundation: All effects start here

**Implementation checklist:**
-  Framebuffer module with color + depth attachments
-  Screen quad mesh
-  Simple screen shader
-  Two-pass rendering in main loop
-  Window resize handling

**Time estimate:** 60-75 minutes

Framebuffers are the foundation of modern graphics effects. Once you have this working, a whole world of visual effects opens up! <�