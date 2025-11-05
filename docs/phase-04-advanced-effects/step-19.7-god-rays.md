# Step 19.7: God Rays (Volumetric Light Scattering)

**Goal:** Add atmospheric god rays (light shafts) emanating from bright light sources.

**Builds on:** Step 19.5 (Bloom), BloomRenderer architecture

---

## What Are God Rays?

God rays (also called volumetric light scattering, crepuscular rays, or light shafts) simulate the visible scattering of light through a medium like fog, dust, or atmosphere. They create dramatic beams of light radiating from bright sources.

**Key Characteristics:**
- Light appears to "beam" from the source
- Blocked by occluding geometry
- Intensity fades with distance from source
- Creates atmospheric depth and drama

---

## The Algorithm

God rays use a **screen-space radial blur** technique:

### Overview
```
1. Render occlusion mask (light source vs. geometry)
2. Apply radial blur from light's screen position
3. Composite with scene
```

### Pass 1: Occlusion Mask
Render the scene showing only what the light "illuminates":
- Light source = white (1.0)
- Everything else = black (0.0)
- This creates a silhouette showing where light can pass through

### Pass 2: Radial Blur
Apply a directional blur **toward** the light's screen-space position:
- Sample the occlusion mask multiple times
- Each sample moves closer to the light position
- Apply decay to simulate light scattering through atmosphere
- Weight samples to create smooth falloff

### Pass 3: Composite
Blend the god rays with the scene:
- Additive blending (like bloom)
- Adjustable intensity/exposure

---

## Implementation Steps

### Step 1: Create God Rays Shaders

**`shader/occlusion.vert`** (reuse screen.vert)
```glsl
#version 410 core

layout(location = 0) in vec3 aPos;
layout(location = 1) in vec3 aColor;
layout(location = 2) in vec3 aNormal;
layout(location = 3) in vec2 aTexCoord;

out vec2 TexCoords;

void main()
{
    gl_Position = vec4(aPos.x, aPos.y, 0.0, 1.0);
    TexCoords = aTexCoord;
}
```

**`shader/occlusion.frag`**
```glsl
#version 410 core

out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D sceneTexture;
uniform vec3 lightColor;        // Color of the light source
uniform float luminanceThreshold; // Threshold to detect light

void main()
{
    vec3 color = texture(sceneTexture, TexCoords).rgb;

    // Calculate luminance
    float luminance = dot(color, vec3(0.2126, 0.7152, 0.0722));

    // If pixel is bright enough (likely the light source), output white
    // Otherwise, output black (occlusion)
    if (luminance > luminanceThreshold) {
        FragColor = vec4(1.0, 1.0, 1.0, 1.0);
    } else {
        FragColor = vec4(0.0, 0.0, 0.0, 1.0);
    }
}
```

**`shader/radial_blur.frag`**
```glsl
#version 410 core

out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D occlusionTexture;
uniform vec2 lightScreenPos;    // Light position in screen space [0-1]
uniform float exposure;         // Overall intensity
uniform float decay;            // Light decay factor (0.95-0.99)
uniform float density;          // Sample density (0.5-1.0)
uniform float weight;           // Sample weight (0.1-0.5)
uniform int numSamples;         // Number of samples (typically 100)

void main()
{
    // Calculate vector from current position to light position
    vec2 deltaTexCoord = TexCoords - lightScreenPos;

    // Divide by number of samples and multiply by density
    deltaTexCoord *= 1.0 / float(numSamples) * density;

    // Store initial sample
    vec3 color = texture(occlusionTexture, TexCoords).rgb;

    // Set up illumination decay factor
    float illuminationDecay = 1.0;

    // Accumulate samples along ray from pixel to light
    for (int i = 0; i < numSamples; i++) {
        // Step towards light
        TexCoords -= deltaTexCoord;

        // Sample occlusion texture
        vec3 sample = texture(occlusionTexture, TexCoords).rgb;

        // Apply decay and weight
        sample *= illuminationDecay * weight;

        // Accumulate
        color += sample;

        // Decay illumination
        illuminationDecay *= decay;
    }

    // Apply exposure
    FragColor = vec4(color * exposure, 1.0);
}
```

**`shader/godray_composite.frag`**
```glsl
#version 410 core

out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D scene;        // Original scene
uniform sampler2D godRays;      // Radial blur result
uniform float godRayStrength;   // Blending strength

void main()
{
    vec3 sceneColor = texture(scene, TexCoords).rgb;
    vec3 godRayColor = texture(godRays, TexCoords).rgb;

    // Additive blending
    vec3 result = sceneColor + godRayColor * godRayStrength;

    FragColor = vec4(result, 1.0);
}
```

---

### Step 2: Create GodRayRenderer

Create `rustgl/src/godray_renderer.rs`:

```rust
use crate::framebuffer::Framebuffer;
use crate::mesh::Mesh;
use crate::shader::Shader;
use gl::types::*;
use nalgebra_glm as glm;

pub struct GodRayRenderer {
    // Framebuffers
    occlusion_fbo: Framebuffer,
    radial_blur_fbo: Framebuffer,

    // Shaders
    occlusion_shader: Shader,
    radial_blur_shader: Shader,
    composite_shader: Shader,

    // Geometry
    screen_quad: Mesh,

    // Parameters
    pub exposure: f32,
    pub decay: f32,
    pub density: f32,
    pub weight: f32,
    pub num_samples: i32,
    pub luminance_threshold: f32,
}

impl GodRayRenderer {
    pub fn new(width: u32, height: u32) -> Self {
        GodRayRenderer {
            occlusion_fbo: Framebuffer::new(width, height),
            radial_blur_fbo: Framebuffer::new(width, height),

            occlusion_shader: Shader::new("shader/screen.vert", "shader/occlusion.frag"),
            radial_blur_shader: Shader::new("shader/screen.vert", "shader/radial_blur.frag"),
            composite_shader: Shader::new("shader/screen.vert", "shader/godray_composite.frag"),

            screen_quad: Mesh::screen_quad(),

            // Default parameters
            exposure: 0.5,
            decay: 0.97,
            density: 0.8,
            weight: 0.3,
            num_samples: 100,
            luminance_threshold: 0.9,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.occlusion_fbo.resize(width, height);
        self.radial_blur_fbo.resize(width, height);
    }

    /// Apply god rays effect
    /// light_world_pos: Light position in world space
    /// view: View matrix
    /// projection: Projection matrix
    /// scene_texture: The rendered scene texture
    /// strength: God ray intensity
    pub fn apply(
        &mut self,
        scene_texture: GLuint,
        light_world_pos: glm::Vec3,
        view: &glm::Mat4,
        projection: &glm::Mat4,
        strength: f32,
        window_width: i32,
        window_height: i32,
    ) {
        // Calculate light position in screen space
        let light_screen_pos = self.world_to_screen(light_world_pos, view, projection);

        // Pass 1: Generate occlusion mask
        self.generate_occlusion_mask(scene_texture);

        // Pass 2: Apply radial blur
        self.apply_radial_blur(light_screen_pos);

        // Pass 3: Composite with scene
        self.composite(scene_texture, strength, window_width, window_height);
    }

    fn world_to_screen(&self, world_pos: glm::Vec3, view: &glm::Mat4, projection: &glm::Mat4) -> glm::Vec2 {
        // Transform to clip space
        let clip_space = projection * view * glm::vec4(world_pos.x, world_pos.y, world_pos.z, 1.0);

        // Perspective divide to get NDC
        let ndc = glm::vec3(
            clip_space.x / clip_space.w,
            clip_space.y / clip_space.w,
            clip_space.z / clip_space.w,
        );

        // Convert from NDC [-1, 1] to screen space [0, 1]
        glm::vec2(
            (ndc.x + 1.0) * 0.5,
            (ndc.y + 1.0) * 0.5,
        )
    }

    fn generate_occlusion_mask(&mut self, scene_texture: GLuint) {
        self.occlusion_fbo.bind();
        unsafe {
            gl::Disable(gl::DEPTH_TEST);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            self.occlusion_shader.use_program();
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, scene_texture);
            self.occlusion_shader.set_int("sceneTexture", 0);
            self.occlusion_shader.set_float("luminanceThreshold", self.luminance_threshold);
            self.screen_quad.draw();
        }
    }

    fn apply_radial_blur(&mut self, light_screen_pos: glm::Vec2) {
        self.radial_blur_fbo.bind();
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            self.radial_blur_shader.use_program();
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.occlusion_fbo.texture());
            self.radial_blur_shader.set_int("occlusionTexture", 0);
            self.radial_blur_shader.set_vec2("lightScreenPos", &light_screen_pos);
            self.radial_blur_shader.set_float("exposure", self.exposure);
            self.radial_blur_shader.set_float("decay", self.decay);
            self.radial_blur_shader.set_float("density", self.density);
            self.radial_blur_shader.set_float("weight", self.weight);
            self.radial_blur_shader.set_int("numSamples", self.num_samples);
            self.screen_quad.draw();
        }
    }

    fn composite(&self, scene_texture: GLuint, strength: f32, window_width: i32, window_height: i32) {
        Framebuffer::unbind();
        unsafe {
            gl::Viewport(0, 0, window_width, window_height);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            self.composite_shader.use_program();
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, scene_texture);
            self.composite_shader.set_int("scene", 0);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, self.radial_blur_fbo.texture());
            self.composite_shader.set_int("godRays", 1);
            self.composite_shader.set_float("godRayStrength", strength);
            self.screen_quad.draw();
        }
    }
}
```

---

### Step 3: Add to AppState

```rust
struct AppState {
    // ... existing fields
    godray_enabled: bool,
    godray_strength: f32,
    godray_exposure: f32,
    godray_decay: f32,
}

impl AppState {
    fn new() -> Self {
        AppState {
            // ... existing fields
            godray_enabled: true,
            godray_strength: 1.0,
            godray_exposure: 0.5,
            godray_decay: 0.97,
        }
    }
}
```

---

### Step 4: Integrate with Main

```rust
// In main()
let mut godray_renderer = GodRayRenderer::new(fb_width as u32, fb_height as u32);

// In render loop - after bloom
if state.godray_enabled {
    // Get light position (orbiting light sphere - index 3)
    let light_pos = scene.lights()[3].position;
    let view = camera.get_view_matrix();
    let projection = glm::perspective(fb_width as f32 / fb_height as f32, camera.zoom.to_radians(), 0.1, 100.0);

    godray_renderer.apply(
        bloom_renderer.scene_texture(),  // You'll need to expose this
        light_pos,
        &view,
        &projection,
        state.godray_strength,
        fb_width,
        fb_height,
    );
}
```

---

### Step 5: Add Keyboard Controls

```rust
// In handle_window_event()
glfw::WindowEvent::Key(Key::Num9, _, Action::Press, _) => {
    state.godray_enabled = !state.godray_enabled;
    println!("God rays: {}", if state.godray_enabled { "ON" } else { "OFF" });
}
glfw::WindowEvent::Key(Key::O, _, Action::Press, _) => {
    state.godray_exposure += 0.1;
    println!("God ray exposure: {:.2}", state.godray_exposure);
}
glfw::WindowEvent::Key(Key::P, _, Action::Press, _) => {
    state.godray_exposure = (state.godray_exposure - 0.1).max(0.0);
    println!("God ray exposure: {:.2}", state.godray_exposure);
}
```

---

## Expected Result

You should see:
- **Bright shafts of light** radiating from the orbiting sphere
- Rays are **blocked by geometry** (objects cast "shadows" in the rays)
- Intensity **fades with distance** from the light
- **Atmospheric effect** that enhances scene depth

### Tuning Parameters

- **`exposure`**: Overall brightness (0.3-1.0)
- **`decay`**: How quickly light fades (0.95-0.99) - higher = longer rays
- **`density`**: Sample spacing (0.5-1.0) - higher = smoother
- **`weight`**: Sample contribution (0.1-0.5) - higher = brighter
- **`num_samples`**: Ray quality (50-150) - higher = smoother but slower
- **`luminance_threshold`**: What counts as "light" (0.8-1.0)

---

## Optimization Notes

**Performance Impact:**
- Radial blur with 100 samples = expensive!
- Consider rendering at half resolution
- Skip effect when light is off-screen

**Quality vs Speed:**
```rust
// High quality (slower)
num_samples: 150,
density: 1.0,

// Balanced (recommended)
num_samples: 100,
density: 0.8,

// Fast (lower quality)
num_samples: 50,
density: 0.5,
```

---

## Debug Tips

**If rays don't appear:**
1. Check light screen position - print it!
2. View occlusion mask (set it as final output)
3. Lower `luminance_threshold` to 0.5
4. Increase `exposure` to 1.0

**If rays are too subtle:**
- Increase `exposure`
- Increase `weight`
- Increase `godray_strength`

**If rays are blocky:**
- Increase `num_samples`
- Increase `density`

---

## Architecture Benefits

Using the same pattern as BloomRenderer:
- ✅ Clean separation of concerns
- ✅ Easy to enable/disable
- ✅ Self-contained resize handling
- ✅ Tunable parameters
- ✅ Can be combined with bloom!

---

## Next Steps

After completing this step, you could:
1. Add multiple light sources (array of god rays)
2. Add color to rays (tinted god rays)
3. Animate parameters for pulsing effects
4. Add volumetric fog for full atmospheric rendering

Or continue with **Step 20** for the next main lesson!
