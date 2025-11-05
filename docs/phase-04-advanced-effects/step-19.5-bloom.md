# Step 19.5: Bloom Effect (Intermediate)

## Overview
**Bloom** is a post-processing effect that makes bright areas of your scene "glow" by bleeding light into surrounding pixels. It's one of the most popular and impactful effects in modern games, creating a sense of intense brightness that monitors can't actually display. Your orbiting light sphere is about to look amazing!

## What You'll Learn
- **Bright pass extraction**: Isolating pixels above a brightness threshold
- **Gaussian blur**: Creating smooth, photorealistic blur
- **Two-pass blur**: Efficient horizontal + vertical blur technique
- **Additive blending**: Combining bloom with original scene
- **Multi-framebuffer pipeline**: Chaining multiple render passes

## Concepts

### What is Bloom?

Bloom simulates how bright lights overwhelm camera sensors (or eyes), causing light to "bleed" beyond the bright source.

**Real-world examples:**
- Sun's rays around its edge
- Bright headlights at night
- Neon signs in dark environments
- Your orbiting white light sphere!

**Technical definition:** Extract bright pixels, blur them heavily, add them back to the scene.

### The Bloom Pipeline

```
Original Scene
    ↓
[Pass 1: Bright Pass]  ← Extract pixels above threshold
    ↓
Bright-only Texture
    ↓
[Pass 2: Horizontal Blur]  ← Blur left-right
    ↓
Blurred Horizontal
    ↓
[Pass 3: Vertical Blur]  ← Blur up-down
    ↓
Fully Blurred Bloom
    ↓
[Pass 4: Composite]  ← Add bloom to original
    ↓
Final Scene with Bloom
```

**Why two blur passes?**
- Gaussian blur is separable: 2D blur = horizontal blur + vertical blur
- 2D blur of radius R: R² samples per pixel (expensive!)
- Separable blur: 2R samples per pixel (much faster!)
- Example: 13×13 blur = 169 samples vs 13+13 = 26 samples!

### Brightness Threshold

Not all pixels should bloom - only bright ones.

**Common thresholds:**
- **Low (0.8)**: Lots of bloom, dreamlike
- **Medium (1.0)**: Only pixels brighter than white
- **High (1.5-2.0)**: Only very bright lights bloom

**Brightness calculation:**
```glsl
float brightness = dot(color.rgb, vec3(0.2126, 0.7152, 0.0722));  // Perceptual luminance
if (brightness > threshold) {
    brightColor = color;
}
```

### Gaussian Blur

A bell-curve weighted blur for smooth, natural-looking results.

**1D Gaussian weights (5-tap):**
```
[0.0545, 0.2442, 0.4026, 0.2442, 0.0545]
     ↑      ↑       ↑      ↑      ↑
  offset -2  -1     0     +1     +2
```

**Larger blur = more glow spread**

## Implementation Plan

### 1. Update Framebuffer Module

We need multiple framebuffers for the bloom pipeline. Add a helper method:

```rust
// Add to framebuffer.rs

impl Framebuffer {
    // ... existing methods ...

    /// Get the framebuffer ID (for binding as read framebuffer)
    pub fn fbo(&self) -> GLuint {
        self.fbo
    }

    /// Get dimensions
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
```

### 2. Create Bright Pass Shader

Extracts bright pixels above threshold.

#### `shader/bright_pass.frag`
```glsl
#version 410 core

out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D screenTexture;
uniform float threshold;  // Brightness threshold (default: 1.0)

void main()
{
    vec3 color = texture(screenTexture, TexCoords).rgb;

    // Calculate perceptual brightness (weighted RGB)
    float brightness = dot(color, vec3(0.2126, 0.7152, 0.0722));

    // Only output if above threshold
    if (brightness > threshold) {
        FragColor = vec4(color, 1.0);
    } else {
        FragColor = vec4(0.0, 0.0, 0.0, 1.0);  // Black
    }
}
```

**Note:** Uses the same `screen.vert` as the screen quad.

### 3. Create Blur Shader

Gaussian blur shader that can blur horizontally or vertically.

#### `shader/blur.frag`
```glsl
#version 410 core

out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D image;
uniform bool horizontal;  // true = horizontal blur, false = vertical blur

// Gaussian blur weights (5-tap)
float weights[5] = float[] (0.227027, 0.1945946, 0.1216216, 0.054054, 0.016216);

void main()
{
    vec2 tex_offset = 1.0 / textureSize(image, 0);  // Size of single texel
    vec3 result = texture(image, TexCoords).rgb * weights[0];  // Current fragment

    if (horizontal) {
        // Horizontal blur (sample left and right)
        for (int i = 1; i < 5; ++i) {
            result += texture(image, TexCoords + vec2(tex_offset.x * i, 0.0)).rgb * weights[i];
            result += texture(image, TexCoords - vec2(tex_offset.x * i, 0.0)).rgb * weights[i];
        }
    } else {
        // Vertical blur (sample up and down)
        for (int i = 1; i < 5; ++i) {
            result += texture(image, TexCoords + vec2(0.0, tex_offset.y * i)).rgb * weights[i];
            result += texture(image, TexCoords - vec2(0.0, tex_offset.y * i)).rgb * weights[i];
        }
    }

    FragColor = vec4(result, 1.0);
}
```

**Key techniques:**
- `textureSize(image, 0)` gets texture dimensions
- `weights[0]` is the center sample (highest weight)
- Samples 4 pixels in each direction (9 total per pass)

### 4. Create Bloom Composite Shader

Adds bloom to original scene.

#### `shader/bloom_composite.frag`
```glsl
#version 410 core

out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D scene;          // Original scene
uniform sampler2D bloomBlur;      // Blurred bright areas
uniform float bloomStrength;      // How much bloom to add (default: 1.0)

void main()
{
    vec3 sceneColor = texture(scene, TexCoords).rgb;
    vec3 bloomColor = texture(bloomBlur, TexCoords).rgb;

    // Additive blending with strength control
    vec3 result = sceneColor + bloomColor * bloomStrength;

    FragColor = vec4(result, 1.0);
}
```

**Bloom strength:**
- `0.0`: No bloom
- `0.5`: Subtle bloom
- `1.0`: Normal bloom
- `2.0+`: Intense, dreamlike bloom

### 5. Update Main.rs

Create additional framebuffers and implement the bloom pipeline:

```rust
// In main(), after creating the first framebuffer:

let (fb_width, fb_height) = window.get_framebuffer_size();
let mut framebuffer = Framebuffer::new(fb_width as u32, fb_height as u32);

// Create additional framebuffers for bloom pipeline
let mut bright_pass_fbo = Framebuffer::new(fb_width as u32, fb_height as u32);
let mut blur_fbo1 = Framebuffer::new(fb_width as u32, fb_height as u32);
let mut blur_fbo2 = Framebuffer::new(fb_width as u32, fb_height as u32);

// Load bloom shaders
let bright_pass_shader = Shader::new("shader/screen.vert", "shader/bright_pass.frag");
let blur_shader = Shader::new("shader/screen.vert", "shader/blur.frag");
let bloom_composite_shader = Shader::new("shader/screen.vert", "shader/bloom_composite.frag");

// Bloom parameters
let bloom_threshold = 1.0;       // Brightness threshold
let bloom_strength = 1.0;        // Bloom intensity
let blur_iterations = 5;         // More iterations = more blur
```

**In the render loop, replace the simple two-pass rendering:**

```rust
// PASS 1: Render scene to framebuffer
framebuffer.bind();
render_scene(&mut window, &scene, &shader, &texture, &camera, wireframe_mode, use_texture);

// PASS 2: Extract bright areas
bright_pass_fbo.bind();
unsafe {
    gl::Clear(gl::COLOR_BUFFER_BIT);
    bright_pass_shader.use_program();
    gl::ActiveTexture(gl::TEXTURE0);
    gl::BindTexture(gl::TEXTURE_2D, framebuffer.texture());
    bright_pass_shader.set_int("screenTexture", 0);
    bright_pass_shader.set_float("threshold", bloom_threshold);
    screen_quad.draw();
}

// PASS 3 & 4: Ping-pong blur (alternate between two framebuffers)
let mut horizontal = true;
let mut first_iteration = true;

for _ in 0..blur_iterations * 2 {  // *2 because we do horizontal then vertical
    if horizontal {
        blur_fbo1.bind();
    } else {
        blur_fbo2.bind();
    }

    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
        blur_shader.use_program();
        gl::ActiveTexture(gl::TEXTURE0);

        // First iteration reads from bright pass, subsequent read from previous blur
        let source_texture = if first_iteration {
            bright_pass_fbo.texture()
        } else if horizontal {
            blur_fbo2.texture()
        } else {
            blur_fbo1.texture()
        };

        gl::BindTexture(gl::TEXTURE_2D, source_texture);
        blur_shader.set_int("image", 0);
        blur_shader.set_bool("horizontal", horizontal);
        screen_quad.draw();
    }

    horizontal = !horizontal;
    if first_iteration {
        first_iteration = false;
    }
}

// PASS 5: Composite bloom onto original scene
Framebuffer::unbind();
unsafe {
    gl::Clear(gl::COLOR_BUFFER_BIT);
    bloom_composite_shader.use_program();

    // Bind original scene
    gl::ActiveTexture(gl::TEXTURE0);
    gl::BindTexture(gl::TEXTURE_2D, framebuffer.texture());
    bloom_composite_shader.set_int("scene", 0);

    // Bind blurred bloom
    gl::ActiveTexture(gl::TEXTURE1);
    gl::BindTexture(gl::TEXTURE_2D, blur_fbo2.texture());  // Final blur result
    bloom_composite_shader.set_int("bloomBlur", 1);

    bloom_composite_shader.set_float("bloomStrength", bloom_strength);

    screen_quad.draw();
}

window.swap_buffers();
```

### 6. Handle Window Resize

Update all framebuffers when window resizes:

```rust
glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
    gl::Viewport(0, 0, width, height);
    framebuffer.resize(width as u32, height as u32);
    bright_pass_fbo.resize(width as u32, height as u32);
    blur_fbo1.resize(width as u32, height as u32);
    blur_fbo2.resize(width as u32, height as u32);
},
```

### 7. Add Runtime Controls (Optional)

Add keyboard controls to tweak bloom in real-time:

```rust
// In handle_window_event():

glfw::WindowEvent::Key(Key::Num3, _, Action::Press, _) => {
    bloom_threshold += 0.1;
    println!("Bloom threshold: {:.1}", bloom_threshold);
}
glfw::WindowEvent::Key(Key::Num4, _, Action::Press, _) => {
    bloom_threshold = (bloom_threshold - 0.1).max(0.0);
    println!("Bloom threshold: {:.1}", bloom_threshold);
}
glfw::WindowEvent::Key(Key::Num5, _, Action::Press, _) => {
    bloom_strength += 0.1;
    println!("Bloom strength: {:.1}", bloom_strength);
}
glfw::WindowEvent::Key(Key::Num6, _, Action::Press, _) => {
    bloom_strength = (bloom_strength - 0.1).max(0.0);
    println!("Bloom strength: {:.1}", bloom_strength);
}
```

**Controls:**
- `3/4`: Increase/decrease bloom threshold
- `5/6`: Increase/decrease bloom strength

## Performance Optimization

Bloom can be expensive. Here are optimization strategies:

### 1. Downsample for Blur
Blur at half or quarter resolution:

```rust
// Create smaller blur framebuffers
let blur_width = fb_width / 2;
let blur_height = fb_height / 2;
let mut blur_fbo1 = Framebuffer::new(blur_width, blur_height);
let mut blur_fbo2 = Framebuffer::new(blur_width, blur_height);
```

**Benefits:**
- 4x fewer pixels to blur at half resolution
- Larger "perceived" blur for same iteration count
- Can look softer/dreamier (often desirable for bloom)

**Tradeoffs:**
- Slightly blockier bloom edges
- Need to handle different resolutions when compositing

### 2. Fewer Blur Iterations
Start with 3-5 iterations instead of 10. More isn't always better!

### 3. Smaller Blur Kernel
Use 3-tap instead of 5-tap Gaussian:
```glsl
float weights[3] = float[] (0.382928, 0.241732, 0.060626);
```

## Testing & Tweaking

After implementation, experiment with parameters:

### Bloom Threshold
- `0.5`: Almost everything glows (dreamlike, fantasy)
- `1.0`: Only very bright things glow (realistic)
- `2.0`: Only your light sphere and specular highlights (subtle)

### Bloom Strength
- `0.5`: Subtle glow
- `1.0`: Normal glow
- `2.0`: Intense, dramatic glow
- `5.0`: Overwhelming, ethereal

### Blur Iterations
- `3`: Tight glow, closer to source
- `5`: Medium glow (recommended)
- `10`: Wide, diffuse glow

### What to Expect

Your orbiting light sphere should now:
- ✅ Have a bright glowing core
- ✅ Emit light that bleeds into surrounding areas
- ✅ Create a halo effect
- ✅ Make the scene feel more dynamic

Specular highlights on your metal/chrome objects will also glow!

## Common Issues

### Issue: Everything is glowing
**Solution:** Increase bloom threshold (try 1.5-2.0)

### Issue: Bloom is barely visible
**Solutions:**
- Lower bloom threshold (try 0.8)
- Increase bloom strength (try 2.0)
- Increase blur iterations
- Make light sphere emission brighter

### Issue: Performance drop
**Solutions:**
- Reduce blur iterations
- Downsample blur framebuffers (half or quarter resolution)
- Use 3-tap blur instead of 5-tap

### Issue: Blocky/pixelated bloom
**Solutions:**
- Increase blur iterations
- Use full-resolution blur framebuffers
- Check texture filtering is LINEAR, not NEAREST

## Extensions

Once you have basic bloom working:

1. **HDR Bloom**: Use floating-point textures (GL_RGB16F) for better bright range
2. **Lens Dirt**: Multiply bloom by a "lens dirt" texture for realistic camera imperfections
3. **Lens Flares**: Add directional flares from bright lights
4. **Bloom Mip Chain**: Blur at multiple resolutions and combine (industry standard)
5. **Physically Based Bloom**: Use exposure-relative thresholds

## Summary

**Pipeline:**
1. Render scene → Scene texture
2. Extract bright pixels → Bright texture
3. Blur horizontally → Blurred H texture
4. Blur vertically → Blurred HV texture
5. Composite bloom + scene → Final image

**Shaders:**
- `bright_pass.frag`: Extracts pixels above threshold
- `blur.frag`: Gaussian blur (horizontal/vertical)
- `bloom_composite.frag`: Adds bloom to scene

**Parameters:**
- Threshold: What brightness level blooms
- Strength: How much bloom to add
- Iterations: How blurred/wide the glow is

**Result:**
Your orbiting light will finally **glow** like a real light source! ✨

Time estimate: 45-60 minutes

This effect will make your scene look significantly more professional and dynamic. Ready to make things glow? 🌟
