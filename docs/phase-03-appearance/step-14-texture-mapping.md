# Step 14: Texture Mapping

**Phase 3: Appearance** | **Estimated Time:** 30-60 minutes

## Goals

In this step, you will:
- Update the fragment shader to sample textures
- Add utility methods to the Shader struct (`set_int`, `set_bool`)
- Bind textures in the render loop
- See textures applied to your 3D primitives!

## Current State Check

Great news! Most of the work is already done:

✅ **Vertex struct** - Already has UV coordinates (mesh.rs:11)
✅ **VAO attributes** - UV attribute configured at location 3 (mesh.rs:443-452)
✅ **All primitives** - Sphere, cube, cylinder, torus, plane all have UVs
✅ **Vertex shader** - Already receives and passes UVs as `ourTexCoord`
✅ **Fragment shader** - Receives UVs but doesn't use them yet (basic.frag:3)

So we just need to:
1. Update fragment shader to sample texture
2. Add shader utility methods
3. Bind texture in render loop

## Tasks

### Task 1: Update Fragment Shader

Your current fragment shader (basic.frag) just uses vertex color. Let's update it to sample textures:

**Current `shader/basic.frag`:**
```glsl
#version 330 core
in vec3 ourColor;
in vec2 ourTexCoord;
out vec4 FragColor;

void main() {
    // For now, just use the color. UV coordinates are available as ourTexCoord
    // They can be used later for texturing
    FragColor = vec4(ourColor, 1.0);
}
```

**Updated `shader/basic.frag`:**
```glsl
#version 330 core
in vec3 ourColor;
in vec3 ourNormal;
in vec2 ourTexCoord;
out vec4 FragColor;

uniform sampler2D textureSampler;  // NEW: Texture sampler
uniform bool useTexture;           // NEW: Toggle texture on/off

void main() {
    if (useTexture) {
        // Sample texture and multiply by vertex color for tinting
        vec4 texColor = texture(textureSampler, ourTexCoord);
        FragColor = texColor * vec4(ourColor, 1.0);
    } else {
        // Use vertex color only (current behavior)
        FragColor = vec4(ourColor, 1.0);
    }
}
```

**What changed:**
- Added `uniform sampler2D textureSampler` - receives texture from CPU
- Added `uniform bool useTexture` - allows toggling textures on/off
- Added `in vec3 ourNormal` (received from vertex shader but not used yet)
- `texture(textureSampler, ourTexCoord)` - samples the texture at UV coordinates
- Multiplying by `ourColor` allows tinting (use white [1,1,1] for pure texture)

### Task 2: Add Shader Utility Methods

You need to add two new methods to your `Shader` struct for setting int and bool uniforms.

**In `shader.rs`**, add these methods (after `set_float` around line 84):

```rust
pub fn set_int(&self, name: &str, value: i32) {
    unsafe {
        let c_name = CString::new(name).unwrap();
        let location = gl::GetUniformLocation(self.id, c_name.as_ptr());
        gl::Uniform1i(location, value);
    }
}

pub fn set_bool(&self, name: &str, value: bool) {
    self.set_int(name, value as i32);
}
```

**Why these are needed:**
- `set_int()` - Sets integer uniforms (like texture unit numbers: 0, 1, 2...)
- `set_bool()` - Sets boolean uniforms (GLSL bools are just ints: 0 = false, 1 = true)

### Task 3: Update Render Function Signature

Your `render()` function needs to receive the texture.

**In `main.rs`**, update the render function signature (around line 195):

```rust
fn render(
    window: &mut glfw::Window,
    sphere: &Mesh,
    cube: &Mesh,
    cylinder: &Mesh,
    torus: &Mesh,
    plane: &Mesh,
    shader: &Shader,
    texture: &Texture,  // NEW: Add texture parameter
    camera: &Camera,
    time: f32,
) {
    // ... rest of function
}
```

### Task 4: Bind Texture and Set Uniforms

**In the `render()` function** (around line 206-212), add texture binding and uniforms:

```rust
fn render(
    window: &mut glfw::Window,
    sphere: &Mesh,
    cube: &Mesh,
    cylinder: &Mesh,
    torus: &Mesh,
    plane: &Mesh,
    shader: &Shader,
    texture: &Texture,
    camera: &Camera,
    time: f32,
) {
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::ClearColor(0.1, 0.1, 0.2, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        check_gl_error("clear");

        shader.use_program();

        // NEW: Bind texture and set shader uniforms
        texture.bind(0);                        // Bind to texture unit 0
        shader.set_int("textureSampler", 0);    // Tell shader to use texture unit 0
        shader.set_bool("useTexture", true);    // Enable texture sampling

        let view = camera.get_view_matrix();
        shader.set_mat4("view", &view);

        let projection = glm::perspective(
            1024.0 / 768.0,
            camera.zoom.to_radians(),
            0.1,
            100.0,
        );
        shader.set_mat4("projection", &projection);

        // ... rest of rendering (all your draw calls)
    }
    window.swap_buffers();
}
```

**What's happening:**
1. `texture.bind(0)` - Binds your texture to texture unit 0 (GL_TEXTURE0)
2. `shader.set_int("textureSampler", 0)` - Tells the shader which texture unit to read from
3. `shader.set_bool("useTexture", true)` - Enables texture sampling in the shader

### Task 5: Update Render Call in main()

**In `main.rs`** (around line 116-127), update the render call to pass the texture:

```rust
render(
    &mut window,
    &sphere,
    &cube,
    &cylinder,
    &torus,
    &plane,
    &shader,
    &texture,  // NEW: Pass texture reference
    &camera,
    time,
);
```

## Success Criteria

You have completed this step when:

- ✅ Fragment shader samples the texture
- ✅ `set_int()` and `set_bool()` methods added to Shader
- ✅ Texture is bound before rendering
- ✅ You can see the texture applied to all your 3D objects
- ✅ No OpenGL errors
- ✅ Objects are not black (if they are, texture isn't binding correctly)

## Testing

Run your program and you should see:

1. **All objects textured** - Your "livia.png" texture should appear on all primitives
2. **Sphere** - Texture wrapped around sphere (spherical mapping)
3. **Cube** - Each face shows the texture
4. **Cylinder** - Texture wrapped around cylinder body
5. **Torus** - Texture follows the torus surface
6. **Plane** - Texture mapped flat on the ground

**Expected behavior:**
- Texture should be visible and recognizable
- No black objects (if black, texture isn't binding)
- Texture may look stretched on some primitives (that's normal)

## Common Issues

### Issue 1: Objects are completely black

**Problem:** Texture not binding or shader uniform not set.

**Solution:**
- Check `texture.bind(0)` is called before draws
- Verify `set_int("textureSampler", 0)` matches texture unit
- Make sure texture loaded successfully (check Step 13 output)
- Try temporarily setting `useTexture = false` to see vertex colors

### Issue 2: Compile error: "cannot find function `set_int`"

**Problem:** Forgot to add utility methods to Shader.

**Solution:**
- Add `set_int()` and `set_bool()` to shader.rs
- Make sure they're `pub` methods

### Issue 3: Texture is upside down

**Problem:** OpenGL texture origin is bottom-left, images are usually top-left.

**Solution:**
- In `texture.rs`, flip image when loading: `let img = img.to_rgba8().flipv();`
- Or flip V coordinate in primitives: `v = 1.0 - v`

### Issue 4: "Unused variable: ourNormal"

**Problem:** Warning about unused normal in fragment shader.

**Solution:**
- This is fine! We'll use normals in Step 15 (Lighting)
- You can ignore the warning for now

### Issue 5: Only first object is textured

**Problem:** Texture binding might be getting reset.

**Solution:**
- Make sure texture bind and uniform sets are BEFORE the draw loop, not inside it
- They only need to be set once per frame

## Understanding Check

Before moving on, make sure you understand:

1. **What is a sampler2D?**
   - A GLSL type that represents a texture, used in shaders to sample texture data

2. **What does texture() do?**
   - Samples the texture at given UV coordinates, returns a vec4 color

3. **What is a texture unit?**
   - A slot (TEXTURE0, TEXTURE1, etc.) where textures are bound, allows multiple textures

4. **Why multiply by ourColor?**
   - Allows tinting the texture with vertex color (use white [1,1,1] for pure texture)

5. **What's the range of UV coordinates?**
   - Typically 0.0 to 1.0, but can go beyond with texture wrapping modes (REPEAT, CLAMP, etc.)

## Challenges

Want to experiment? Try these:

### Challenge 1: Toggle Textures with Keyboard
- Add keyboard input to toggle `useTexture` on/off
- Press T to switch between textured and colored mode
- See both rendering modes

### Challenge 2: Pure Texture (No Tinting)
- Change vertex colors to white `[1.0, 1.0, 1.0]` for some primitives
- See the texture without color tinting
- Compare tinted vs pure texture

### Challenge 3: Load Multiple Textures
- Load a second texture in main
- Apply different textures to different objects
- Use texture unit 1 for the second texture

### Challenge 4: Texture Tiling
- Multiply UVs by 2.0 in one primitive: `u * 2.0, v * 2.0`
- See the texture repeat 2x2 times
- Experiment with different tiling amounts

## What You've Learned

In this step, you've learned:

- ✅ How to sample textures in fragment shaders
- ✅ The role of `sampler2D` uniforms
- ✅ How texture units work (TEXTURE0, TEXTURE1, etc.)
- ✅ How to toggle rendering modes with uniforms
- ✅ The complete texture pipeline: load → bind → sample → render

## Next Steps

In **Step 15: Lighting Basics**, you will:
- Use those normal vectors you've been passing around
- Implement Phong lighting (ambient, diffuse, specular)
- Add light sources to your scene
- Combine textures with lighting for photorealistic results!

Right now your objects have textures but look flat. With lighting, they'll have depth, shadows, and realistic shading!

---

**Ready to see textured 3D objects?** Make the changes and run your program!

When you're done, show me:
1. A screenshot of your textured scene
2. Any issues you encountered
3. The output showing successful texture application

Good luck! 🦀
