# Step 19.8: Object-Specific God Rays (Intermediate)

**Goal:** Refine the god ray implementation to only emit rays from the glowing orb, not from all bright pixels in the scene.

**Builds on:** Step 19.7 (God Rays), Step 19.5 (Bloom)

**Time estimate:** 45-60 minutes

---

## What You'll Learn

- **Geometry-based occlusion**: Rendering specific 3D geometry to create targeted effects
- **Depth-aware masking**: Using depth buffers for proper occlusion
- **Effect isolation**: Separating bloom and god rays for independent control
- **Physically-based approach**: God rays from actual light sources, not arbitrary bright pixels

---

## The Problem

Currently, your god ray implementation uses a **luminance threshold** approach:

```glsl
// In occlusion.frag
float luminance = dot(color, vec3(0.2126, 0.7152, 0.0722));
if (luminance > luminanceThreshold) {
    FragColor = vec4(1.0, 1.0, 1.0, 1.0);  // Create rays
}
```

**This means god rays appear from:**
- ❌ Bloom effects on all objects
- ❌ Specular highlights on metal/chrome materials
- ❌ Any bright texture or surface
- ❌ The entire bright part of the scene

**What we want:**
- ✅ God rays only from the glowing orb
- ✅ Proper occlusion when orb is behind objects
- ✅ Independent from bloom and other effects

---

## Concepts

### Screen-Space vs Geometry-Based Occlusion

**Current Approach (Screen-Space Threshold):**
```
Scene Texture → Luminance Check → White/Black Mask → Radial Blur
```
- Simple and fast
- No control over which objects emit rays
- Affected by all bright pixels

**New Approach (Geometry-Based):**
```
3D Orb Mesh → Render to Occlusion Buffer → Depth Testing → Radial Blur
```
- Precise control over light source
- Proper depth-based occlusion
- Independent of scene brightness

### Why This Is Better

1. **Artistic Control**: Choose exactly which objects emit god rays
2. **Physical Accuracy**: Rays come from actual light sources
3. **Effect Independence**: Bloom and god rays don't interfere
4. **Proper Occlusion**: Objects blocking the light prevent rays naturally

### The New Pipeline

```
Pass 1: Render orb geometry to occlusion buffer (white on black)
   ↓
Pass 2: Apply radial blur toward light position
   ↓
Pass 3: Composite god rays with scene
```

The key difference: Pass 1 renders **3D geometry** instead of filtering a texture.

---

## The Solution

Instead of using a luminance threshold on the entire scene, we'll:
1. Render **only the glowing orb** to the occlusion buffer
2. Render it as a **solid white silhouette**
3. Use proper **depth testing** so occluded parts don't create rays
4. Everything else renders as black (occluded)

This way, god rays only emanate from the visible portions of the glowing orb.

---

## Implementation Steps

### Step 1: Create Simple Occlusion Vertex Shader

The occlusion pass currently uses `screen.vert` (a fullscreen quad shader). We need a proper 3D vertex shader that can render the orb geometry.

**Create:** `rustgl/shader/occlusion.vert`

```glsl
#version 410 core

layout (location = 0) in vec3 aPos;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    gl_Position = projection * view * model * vec4(aPos, 1.0);
}
```

**What it does:**
- Takes 3D vertex positions
- Transforms them through model-view-projection matrices
- Outputs clip space position for rasterization

---

### Step 2: Simplify Occlusion Fragment Shader

The fragment shader should just output white for the orb, no luminance calculations needed.

**Modify:** `rustgl/shader/occlusion.frag`

Replace the entire shader with:

```glsl
#version 410 core

out vec4 FragColor;

void main()
{
    // Render orb as solid white
    FragColor = vec4(1.0, 1.0, 1.0, 1.0);
}
```

**What it does:**
- Simply outputs white for every pixel of the orb
- No uniforms needed
- The depth buffer handles occlusion automatically

---

### Step 3: Update GodRayRenderer to Accept Scene Data

Currently, `generate_occlusion_mask()` only takes the scene texture. We need to render actual 3D geometry, so we need access to the scene.

**Modify:** `rustgl/src/godray_renderer.rs`

**Step 3a: Update the `apply()` method signature**

Find the `apply()` method (around line 53) and update its signature:

```rust
pub fn apply(
    &mut self,
    scene_texture: GLuint,
    orb_mesh: &Mesh,           // NEW: the orb geometry
    orb_transform: &Transform, // NEW: the orb's transform
    light_world_pos: glm::Vec3,
    view: &glm::Mat4,
    projection: &glm::Mat4,
    strength: f32,
    window_width: i32,
    window_height: i32,
) {
```

**Step 3b: Update the occlusion generation call**

Update the call to `generate_occlusion_mask()` inside `apply()`:

```rust
self.generate_occlusion_mask(scene_texture, orb_mesh, orb_transform, view, projection);
```

**Step 3c: Update `generate_occlusion_mask()` method**

Replace the entire method (lines 104-118):

```rust
fn generate_occlusion_mask(
    &mut self,
    scene_texture: GLuint,
    orb_mesh: &Mesh,
    orb_transform: &Transform,
    view: &glm::Mat4,
    projection: &glm::Mat4,
) {
    self.occlusion_fbo.bind();
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

        // Render only the orb as white
        self.occlusion_shader.use_program();
        self.occlusion_shader.set_mat4("model", &orb_transform.get_matrix());
        self.occlusion_shader.set_mat4("view", view);
        self.occlusion_shader.set_mat4("projection", projection);
        orb_mesh.draw();
    }
}
```

**Key changes:**
- ❌ Removed: Texture sampling and luminance threshold
- ✅ Added: 3D geometry rendering with MVP matrices
- ✅ Added: Depth testing (so occluded parts don't emit rays)
- ✅ Added: Clear depth buffer

**Step 3d: Update the shader initialization**

Find the shader initialization in the `new()` method (line 32) and update it:

```rust
occlusion_shader: Shader::new("shader/occlusion.vert", "shader/occlusion.frag"),
```

Make sure it's loading the new vertex shader.

---

### Step 4: Update Main.rs to Pass Orb Data

The render loop needs to pass the orb's mesh and transform to the god ray renderer.

**Modify:** `rustgl/src/main.rs`

Find the god ray application code (around lines 287-301) and update it:

```rust
if state.godray_enabled {
    // Get the orbiting light sphere (object index 6)
    let orb = scene.get_object(6).expect("Orb not found");
    let light_pos = scene.lights()[3].position;
    let view = camera.get_view_matrix();
    let projection = glm::perspective(
        fb_width as f32 / fb_height as f32,
        camera.zoom.to_radians(),
        0.1,
        100.0
    );

    godray_renderer.apply(
        bloom_renderer.scene_texture(),
        &orb.mesh,        // NEW: pass the orb mesh
        &orb.transform,   // NEW: pass the orb transform
        light_pos,
        &view,
        &projection,
        state.godray_strength,
        fb_width,
        fb_height,
    );
}
```

**Note:** You'll need to check if `Scene::get_object()` exists. If not, you'll need to add it to `scene.rs`:

```rust
pub fn get_object(&self, index: usize) -> Option<&Object> {
    self.objects.get(index)
}
```

---

### Step 5: Ensure Framebuffer Has Depth Buffer

The occlusion framebuffer needs a depth buffer for proper occlusion testing.

**Check:** `rustgl/src/framebuffer.rs`

Look at your `Framebuffer::new()` implementation. Ensure it creates a depth renderbuffer. It should look something like this:

```rust
// Create depth renderbuffer
let mut depth_rbo = 0;
gl::GenRenderbuffers(1, &mut depth_rbo);
gl::BindRenderbuffer(gl::RENDERBUFFER, depth_rbo);
gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT, width as i32, height as i32);
gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, depth_rbo);
```

If your framebuffer doesn't have depth buffer support, you'll need to add it.

---

## Expected Results

After implementing these changes, compile and run:

```bash
cd rustgl
cargo run
```

### What You Should See

**God rays now:**
- ✅ Only emanate from the white glowing orb
- ✅ Don't appear on specular highlights on metal/chrome objects
- ✅ Are independent of bloom effects
- ✅ Properly occluded when orb passes behind other objects
- ✅ Maintain smooth radial blur quality

**Visual comparison:**
- **Before**: Bright metal highlights and bloom create unwanted rays everywhere
- **After**: Clean, focused rays only from the light source

### Testing Your Implementation

**Test cases:**
1. Move camera around - rays should always point toward orb
2. Position orb behind the cube/sphere - only visible parts emit rays
3. Toggle bloom (key 7) - god rays remain unchanged
4. Toggle god rays (key 9) - effect cleanly enables/disables
5. Adjust bloom threshold (keys 3/4) - no effect on god rays

**Debug controls:**
- `Key 7`: Toggle bloom (shouldn't affect god rays)
- `Key 9`: Toggle god rays
- `Keys O/P`: Adjust god ray exposure
- `WASD + QE`: Move camera to test occlusion

---

## Technical Comparison

### Before: Luminance Threshold Approach
```
Rendered Scene → Sample Texture → Luminance Check → White/Black Mask → Radial Blur
```

**Pros:**
- Simple, single shader pass
- Fast (just texture sampling)

**Cons:**
- No control over which objects emit rays
- Affected by bloom, specular, bright textures
- Effects interfere with each other

### After: Geometry-Based Approach
```
Orb Mesh → Render 3D Geometry → Depth Test → White Silhouette → Radial Blur
```

**Pros:**
- Precise control over light sources
- Proper depth-based occlusion
- Independent from other effects
- Physically accurate

**Cons:**
- Slightly more complex setup
- Requires depth buffer in framebuffer

### Key Benefits

1. **Artistic Control**: Choose exactly which objects emit god rays
2. **Physical Accuracy**: Rays come from actual light sources, not arbitrary bright pixels
3. **Effect Independence**: Bloom and god rays work together without interference
4. **Proper Occlusion**: Objects naturally block rays based on depth

---

## Common Issues

### Issue 1: "No method `get_object` in Scene"

**Solution:** Add the getter method to `scene.rs`:
```rust
pub fn get_object(&self, index: usize) -> Option<&Object> {
    self.objects.get(index)
}
```

### Issue 2: God rays appear too faint

**Solution:** The orb might be too small. Try:
- Increase god ray strength (keys 5/6 or `state.godray_strength`)
- Increase orb scale in the scene setup
- Adjust exposure with O/P keys

### Issue 3: God rays have hard edges

**Solution:** This is expected with geometry rendering. The radial blur should smooth them out. If they're still too hard:
- Increase `num_samples` in GodRayRenderer (line 43)
- Increase `density` parameter
- Ensure the blur pass is working correctly

### Issue 4: Black screen

**Solution:** Depth testing issue. Check:
- Framebuffer has depth buffer attached
- `gl::Enable(gl::DEPTH_TEST)` is called in occlusion pass
- Depth buffer is cleared: `gl::Clear(gl::DEPTH_BUFFER_BIT)`

---

## Bonus Challenges

Once you have it working:

1. **Multiple Light Sources**: Extend the system to handle multiple orbs
   - Render multiple orbs to occlusion buffer
   - Apply radial blur from each light position
   - Composite all results

2. **Light Color**: Make god rays take the color of the light
   - Pass light color to occlusion shader
   - Output colored rays instead of white

3. **Volumetric Quality**: Improve the effect
   - Add noise to the rays for atmospheric scattering
   - Vary intensity based on camera-to-light distance

4. **Performance**: Reduce the resolution of the occlusion/blur passes for better FPS

---

## Summary

This intermediate lesson refines your god ray implementation from a screen-space effect to a geometry-based effect.

**What changed:**
- Occlusion pass now renders 3D orb geometry instead of filtering by luminance
- Added depth buffer for proper occlusion handling
- God rays are now independent of bloom and other bright pixels
- More physically accurate and artistically controllable

**Key techniques learned:**
- Rendering specific geometry for post-processing effects
- Using depth testing in framebuffer passes
- Isolating effects for independent control
- Transitioning from image-based to geometry-based approaches

**What's next:**
This approach can be extended to:
- Multiple light sources (render each to occlusion buffer)
- Colored god rays (use light color in shader)
- Animated/pulsing rays (vary parameters over time)
- Volumetric fog (full 3D raymarching)

---

## Success Checklist

Before moving on, verify:

- ✅ God rays only appear around the glowing orb
- ✅ No god rays from bloom or specular highlights
- ✅ Rays are properly occluded when orb is behind objects
- ✅ Effect can be toggled on/off with key 9
- ✅ Bloom and god rays work independently
- ✅ No compiler errors or warnings
- ✅ Scene renders correctly with god rays enabled/disabled
- ✅ Performance remains smooth (check FPS)

---

When you're done, let me know and I'll review your implementation!
