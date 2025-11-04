# Step 18: Skybox

## Overview
A **skybox** creates the illusion of a vast, distant environment surrounding your scene. It's a cube that wraps around the camera, textured with a panoramic image that appears infinitely far away. Skyboxes are essential for creating immersive 3D environments - whether it's a realistic sky, space scene, or indoor environment.

## What You'll Learn
- **Cubemap textures**: 6-sided textures that form a cube
- **Skybox rendering**: Special depth and culling techniques
- **Texture coordinates**: How cubemaps differ from 2D textures
- **Shader techniques**: Removing translation from view matrix

## Concepts

### What is a Skybox?

A skybox is a large cube centered on the camera that appears infinitely distant. Key properties:

1. **Always centered on camera**: Moves with the camera but never gets closer
2. **Rendered behind everything**: Uses depth testing tricks
3. **Uses cubemap texture**: 6 images mapped to cube faces
4. **No perspective distortion**: Special rendering technique maintains realism

### Cubemap Textures

Unlike regular 2D textures, cubemaps consist of **6 separate images** arranged as cube faces:

```
    +----+
    | +Y | (top)
+---+----+----+----+
| -X | +Z | +X | -Z | (left, front, right, back)
+---+----+----+----+
    | -Y | (bottom)
    +----+
```

**Coordinate system:**
- **+X**: Right face
- **-X**: Left face
- **+Y**: Top face
- **-Y**: Bottom face
- **+Z**: Front face
- **-Z**: Back face

**Sampling:** Instead of 2D UV coordinates, cubemaps use a 3D direction vector:
```glsl
vec4 color = texture(skybox, directionVector);
```

The GPU automatically picks the correct face and samples it based on which direction the vector points.

### Why Cubemaps?

**Advantages over spherical/cylindrical panoramas:**
- No polar distortion (spherical maps have pinching at poles)
- Efficient GPU sampling (hardware-optimized)
- Easy to author (6 flat images instead of complex unwrapping)
- Works perfectly for environment mapping (reflections/refractions later)

## Implementation Plan

### 1. Create Skybox Shaders

We need separate shaders for the skybox because it has unique requirements.

#### `shader/skybox.vert`
```glsl
#version 410 core

layout (location = 0) in vec3 aPos;

out vec3 TexCoords;

uniform mat4 projection;
uniform mat4 view;

void main()
{
    TexCoords = aPos;  // Use position as texture coordinates

    // Remove translation from view matrix
    // We only want rotation, not position changes
    mat4 viewNoTranslation = mat4(mat3(view));

    vec4 pos = projection * viewNoTranslation * vec4(aPos, 1.0);

    // Trick: Set z = w so that after perspective division, z/w = 1.0 (max depth)
    // This ensures skybox is always rendered behind everything
    gl_Position = pos.xyww;
}
```

**Key techniques:**
- `mat4(mat3(view))`: Removes translation, keeps only rotation
- `gl_Position = pos.xyww`: Forces depth to 1.0 (furthest possible)
- `TexCoords = aPos`: Position doubles as cubemap lookup direction

#### `shader/skybox.frag`
```glsl
#version 410 core

out vec4 FragColor;

in vec3 TexCoords;

uniform samplerCube skybox;

void main()
{
    FragColor = texture(skybox, TexCoords);
}
```

**Simplicity:** Just sample the cubemap using the interpolated direction vector.

### 2. Extend Texture Module for Cubemaps

Add cubemap support to `texture.rs`:

```rust
// Add to texture.rs imports
use gl::types::*;

// Add new texture type enum
pub enum TextureType {
    Texture2D,
    Cubemap,
}

// Update Texture struct
pub struct Texture {
    pub id: GLuint,
    pub width: u32,
    pub height: u32,
    pub texture_type: TextureType,  // NEW
}

impl Texture {
    // Existing new() method for 2D textures...

    /// Load a cubemap texture from 6 separate image files
    /// Order: right, left, top, bottom, front, back (+X, -X, +Y, -Y, +Z, -Z)
    pub fn new_cubemap(faces: [&str; 6]) -> Result<Self, String> {
        let mut texture_id = 0;

        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, texture_id);

            // Load each face
            for (i, face_path) in faces.iter().enumerate() {
                let img = image::open(face_path)
                    .map_err(|e| format!("Failed to load cubemap face {}: {}", face_path, e))?;

                let img = img.flipv(); // Flip vertically for OpenGL
                let data = img.to_rgb8();
                let (width, height) = img.dimensions();

                // GL_TEXTURE_CUBE_MAP_POSITIVE_X + i gives us each face
                gl::TexImage2D(
                    gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
                    0,
                    gl::RGB as i32,
                    width as i32,
                    height as i32,
                    0,
                    gl::RGB,
                    gl::UNSIGNED_BYTE,
                    data.as_ptr() as *const _,
                );
            }

            // Cubemap texture parameters
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);

            gl::BindTexture(gl::TEXTURE_CUBE_MAP, 0);
        }

        Ok(Texture {
            id: texture_id,
            width: 0,  // Not really relevant for cubemaps
            height: 0,
            texture_type: TextureType::Cubemap,
        })
    }

    /// Update existing bind() method to handle both types
    pub fn bind(&self, unit: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            match self.texture_type {
                TextureType::Texture2D => gl::BindTexture(gl::TEXTURE_2D, self.id),
                TextureType::Cubemap => gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.id),
            }
        }
    }
}

// Update Drop implementation
impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
```

**Important notes:**
- Cubemap faces must be loaded in specific order: +X, -X, +Y, -Y, +Z, -Z
- `CLAMP_TO_EDGE` prevents seams between cube faces
- `gl::TEXTURE_CUBE_MAP_POSITIVE_X + i` addresses each face

### 3. Create Skybox Mesh

The skybox is just a cube, but we only need positions (no normals, no UVs):

Add to `mesh.rs`:

```rust
/// Create a skybox cube (positions only, no normals or UVs)
pub fn skybox_cube() -> Self {
    // Simple cube centered at origin
    // We only need positions since we use them as texture coordinates
    let vertices = vec![
        // Positions only - no normals, no UVs needed
        // Back face
        Vertex::new([-1.0, -1.0, -1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
        Vertex::new([ 1.0,  1.0, -1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
        Vertex::new([ 1.0, -1.0, -1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
        Vertex::new([ 1.0,  1.0, -1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
        Vertex::new([-1.0, -1.0, -1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
        Vertex::new([-1.0,  1.0, -1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),

        // Front face
        Vertex::new([-1.0, -1.0,  1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
        Vertex::new([ 1.0, -1.0,  1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
        Vertex::new([ 1.0,  1.0,  1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
        Vertex::new([ 1.0,  1.0,  1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
        Vertex::new([-1.0,  1.0,  1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
        Vertex::new([-1.0, -1.0,  1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),

        // Left face
        Vertex::new([-1.0,  1.0,  1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
        Vertex::new([-1.0,  1.0, -1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
        Vertex::new([-1.0, -1.0, -1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
        Vertex::new([-1.0, -1.0, -1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
        Vertex::new([-1.0, -1.0,  1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
        Vertex::new([-1.0,  1.0,  1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),

        // Right face
        Vertex::new([ 1.0,  1.0,  1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
        Vertex::new([ 1.0, -1.0, -1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
        Vertex::new([ 1.0,  1.0, -1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
        Vertex::new([ 1.0, -1.0, -1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
        Vertex::new([ 1.0,  1.0,  1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
        Vertex::new([ 1.0, -1.0,  1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),

        // Bottom face
        Vertex::new([-1.0, -1.0, -1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
        Vertex::new([ 1.0, -1.0, -1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
        Vertex::new([ 1.0, -1.0,  1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
        Vertex::new([ 1.0, -1.0,  1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
        Vertex::new([-1.0, -1.0,  1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
        Vertex::new([-1.0, -1.0, -1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),

        // Top face
        Vertex::new([-1.0,  1.0, -1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
        Vertex::new([ 1.0,  1.0,  1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
        Vertex::new([ 1.0,  1.0, -1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
        Vertex::new([ 1.0,  1.0,  1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
        Vertex::new([-1.0,  1.0, -1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
        Vertex::new([-1.0,  1.0,  1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0]),
    ];

    Mesh::from_vertices(vertices)
}
```

**Note:** We reuse the `Vertex` structure even though we only care about positions. The vertex shader will ignore the other attributes.

### 4. Get Skybox Textures

You need 6 images for your skybox. Here are some options:

#### Option A: Free Skybox Resources
- **learnopengl.com**: https://learnopengl.com/Advanced-OpenGL/Cubemaps (example skybox)
- **OpenGameArt**: https://opengameart.org/ (search "skybox" or "cubemap")
- **Poly Haven**: https://polyhaven.com/hdris (HDRIs you can convert to cubemaps)

#### Option B: Simple Test Skybox
For testing, you can create 6 solid-color images (512x512 each):
- Right (+X): Red
- Left (-X): Cyan
- Top (+Y): White
- Bottom (-Y): Black
- Front (+Z): Green
- Back (-Z): Blue

This helps verify the faces are loading correctly.

#### File Organization
Create a directory structure:
```
resources/textures/skybox/
   right.jpg   (+X)
   left.jpg    (-X)
   top.jpg     (+Y)
   bottom.jpg  (-Y)
   front.jpg   (+Z)
   back.jpg    (-Z)
```

### 5. Integrate into Main

Update `main.rs`:

```rust
// Load skybox texture
let skybox_texture = Texture::new_cubemap([
    "resources/textures/skybox/right.jpg",
    "resources/textures/skybox/left.jpg",
    "resources/textures/skybox/top.jpg",
    "resources/textures/skybox/bottom.jpg",
    "resources/textures/skybox/front.jpg",
    "resources/textures/skybox/back.jpg",
]).expect("Failed to load skybox");

// Create skybox mesh and shader
let skybox_mesh = Mesh::skybox_cube();
let skybox_shader = Shader::new("shader/skybox.vert", "shader/skybox.frag");
```

Then in your render function, render the skybox **first** (before scene objects):

```rust
fn render(
    window: &mut glfw::Window,
    scene: &Scene,
    shader: &Shader,
    texture: &Texture,
    camera: &Camera,
    wireframe_mode: bool,
    use_texture: bool,
    skybox_mesh: &Mesh,           // NEW
    skybox_shader: &Shader,       // NEW
    skybox_texture: &Texture,     // NEW
) {
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::ClearColor(0.1, 0.1, 0.2, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

        // Set polygon mode
        if wireframe_mode {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        } else {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
        }

        let view = camera.get_view_matrix();
        let projection = glm::perspective(1024.0 / 768.0, camera.zoom.to_radians(), 0.1, 100.0);

        // ===== RENDER SKYBOX FIRST =====
        // Change depth function so depth test passes when values are equal to depth buffer's content
        gl::DepthFunc(gl::LEQUAL);

        skybox_shader.use_program();
        skybox_shader.set_mat4("view", &view);
        skybox_shader.set_mat4("projection", &projection);

        skybox_texture.bind(0);
        skybox_shader.set_int("skybox", 0);

        skybox_mesh.draw();

        // Restore default depth function
        gl::DepthFunc(gl::LESS);

        // ===== RENDER SCENE OBJECTS =====
        shader.use_program();
        shader.set_vec3("viewPos", &camera.position);
        texture.bind(0);
        shader.set_int("textureSampler", 0);
        shader.set_bool("useTexture", use_texture);

        scene.render(shader, &view, &projection);
    }
    window.swap_buffers();
}
```

**Important rendering order:**
1. Clear buffers
2. Set `gl::DepthFunc(gl::LEQUAL)` - allows skybox to render at max depth
3. Render skybox
4. Restore `gl::DepthFunc(gl::LESS)` - normal depth testing
5. Render scene objects

The skybox renders first but appears behind everything due to the depth trick in the vertex shader (`gl_Position.z = w`).

## Testing

After implementation, you should see:
1. A skybox surrounding your entire scene
2. The skybox rotates with camera rotation
3. The skybox doesn't move when camera translates
4. All scene objects render in front of the skybox

**Debugging tips:**
- If skybox appears black: Check texture loading order
- If skybox is inside-out: Check face winding order in mesh
- If skybox has seams: Verify `CLAMP_TO_EDGE` texture parameters
- If skybox moves with camera: Check view matrix translation removal

## Common Issues and Solutions

### Issue: Skybox has visible seams
**Solution:** Ensure texture wrap mode is `CLAMP_TO_EDGE`:
```rust
gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);
```

### Issue: Faces are in wrong positions
**Solution:** Double-check the face loading order. OpenGL expects: +X, -X, +Y, -Y, +Z, -Z

### Issue: Skybox appears in front of objects
**Solution:** Verify depth function changes:
- Before skybox: `gl::DepthFunc(gl::LEQUAL)`
- After skybox: `gl::DepthFunc(gl::LESS)`

### Issue: Skybox moves with camera position
**Solution:** Check vertex shader - make sure you're removing translation:
```glsl
mat4 viewNoTranslation = mat4(mat3(view));
```

## Extensions (Optional)

Once you have the basic skybox working:

1. **Dynamic Skyboxes**: Load different skyboxes and switch between them
2. **Day/Night Cycle**: Blend between two skyboxes (day and night)
3. **Animated Skyboxes**: Rotate the skybox slowly for moving clouds/stars
4. **HDR Skyboxes**: Use HDR images for more realistic lighting (Phase 6)

## Next Steps

After completing this step, you'll have:
-  A fully functional skybox system
-  Understanding of cubemap textures
-  Foundation for environment mapping (reflections in Step 20)
-  More immersive 3D scenes

The skybox is fundamental for the next steps:
- **Step 19: Framebuffers** - Render to texture (needed for post-processing)
- **Step 20: Reflections** - Use skybox for realistic reflections
- **Step 21: Refractions** - Use skybox for glass effects

## Summary

**Key concepts:**
- Cubemaps: 6-sided textures forming a cube
- Skybox rendering: Depth tricks and view matrix manipulation
- Texture sampling: 3D direction vectors instead of 2D UVs
- Rendering order: Skybox first, objects second

**Time estimate:** 45-60 minutes

Ready to add that infinite environment to your scene? <
