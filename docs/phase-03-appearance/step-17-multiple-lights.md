# Step 17: Multiple Lights

**Phase 3: Appearance** | **Estimated Time:** 2-3 hours

## Goals

In this step, you will:
- Create a **Light** struct in Rust to represent light sources
- Support **multiple point lights** in your scene (not just one!)
- Implement **light attenuation** (lights get dimmer with distance)
- Add **different colored lights** for dramatic effects
- See your materials lit by multiple lights simultaneously

## Why Multiple Lights?

Right now, you have **one hardcoded light** at position `(5, 5, 5)` with white color. Real scenes have multiple light sources:
- **Indoor scene**: Ceiling lights, lamps, candles, screens
- **Outdoor scene**: Sun, moon, street lights, fire
- **Game scene**: Torches, magic spells, explosions, laser beams

Multiple lights make scenes look **realistic and dynamic**. Different colored lights can create mood (warm orange firelight vs cool blue moonlight).

## Light Types

There are several types of lights in graphics:

### 1. Point Light (What we'll implement)
- Emits light in **all directions** from a single point
- Gets dimmer with distance (**attenuation**)
- Examples: Light bulb, candle, torch, star

### 2. Directional Light (Optional challenge)
- Light rays are **parallel** (infinite distance)
- No attenuation (same brightness everywhere)
- Examples: Sun, moon

### 3. Spotlight (Future step)
- Light emits in a **cone** shape
- Has a direction and cutoff angle
- Examples: Flashlight, car headlights, stage spotlight

**For this step, we'll focus on point lights with attenuation.**

## Light Attenuation

In the real world, light **gets dimmer** as you move farther from the source. This is called **attenuation** or **light falloff**.

The formula is:
```
attenuation = 1.0 / (constant + linear * distance + quadratic * distance�)
```

- **Constant**: Usually 1.0 (base brightness)
- **Linear**: Linear falloff (controls how quickly light dims)
- **Quadratic**: Quadratic falloff (realistic physical falloff)

Common values:
- **Short range** (7 units): constant=1.0, linear=0.7, quadratic=1.8
- **Medium range** (13 units): constant=1.0, linear=0.35, quadratic=0.44
- **Long range** (32 units): constant=1.0, linear=0.14, quadratic=0.07
- **Very long range** (100 units): constant=1.0, linear=0.045, quadratic=0.0075

## Current State Check

 **Already implemented**:
- Single point light with position and color
- Material system with ambient, diffuse, specular properties
- Phong lighting calculation in fragment shader

L **Still needed**:
1. Create Light struct in Rust
2. Support array of lights in fragment shader
3. Calculate lighting contribution for each light
4. Add attenuation to make lights realistic
5. Set multiple light uniforms in render loop

## Tasks

### Task 1: Create Light Struct in Rust

Create a new file `rustgl/src/light.rs` to represent light sources.

**Create `rustgl/src/light.rs`:**

```rust
use nalgebra_glm as glm;

/// Represents a point light source
#[derive(Clone, Copy, Debug)]
pub struct Light {
    /// Position of the light in world space
    pub position: glm::Vec3,

    /// Color/intensity of the light (RGB, each component 0.0-1.0+)
    pub color: glm::Vec3,

    /// Attenuation constant term (usually 1.0)
    pub constant: f32,

    /// Attenuation linear term (controls linear falloff)
    pub linear: f32,

    /// Attenuation quadratic term (controls quadratic falloff - realistic physics)
    pub quadratic: f32,
}

impl Light {
    /// Creates a new light with specified properties
    pub fn new(
        position: glm::Vec3,
        color: glm::Vec3,
        constant: f32,
        linear: f32,
        quadratic: f32,
    ) -> Self {
        Light {
            position,
            color,
            constant,
            linear,
            quadratic,
        }
    }

    /// Creates a point light with short range (~7 units)
    pub fn short_range(position: glm::Vec3, color: glm::Vec3) -> Self {
        Light {
            position,
            color,
            constant: 1.0,
            linear: 0.7,
            quadratic: 1.8,
        }
    }

    /// Creates a point light with medium range (~13 units)
    pub fn medium_range(position: glm::Vec3, color: glm::Vec3) -> Self {
        Light {
            position,
            color,
            constant: 1.0,
            linear: 0.35,
            quadratic: 0.44,
        }
    }

    /// Creates a point light with long range (~32 units)
    pub fn long_range(position: glm::Vec3, color: glm::Vec3) -> Self {
        Light {
            position,
            color,
            constant: 1.0,
            linear: 0.14,
            quadratic: 0.07,
        }
    }

    /// Creates a point light with very long range (~100 units)
    pub fn very_long_range(position: glm::Vec3, color: glm::Vec3) -> Self {
        Light {
            position,
            color,
            constant: 1.0,
            linear: 0.045,
            quadratic: 0.0075,
        }
    }
}
```

**Add to `main.rs`:**
```rust
mod light;  // Add this with other mod declarations at the top
use light::Light;  // Add this with other use statements
```

### Task 2: Update Fragment Shader for Multiple Lights

Update the fragment shader to support an array of lights and calculate their combined contribution.

**Update `shader/basic.frag`:**

Replace the single light uniforms and calculation with a multiple light system.

**Current single light uniforms (lines 12-14):**
```glsl
uniform vec3 lightPos;
uniform vec3 viewPos;
uniform vec3 lightColor;
```

**Replace entire shader with:**
```glsl
#version 330 core
in vec3 ourColor;
in vec3 ourNormal;
in vec2 ourTexCoord;
in vec3 fragPos;

out vec4 FragColor;

uniform sampler2D textureSampler;
uniform bool useTexture;

uniform vec3 viewPos;

// Material properties
uniform vec3 material_ambient;
uniform vec3 material_diffuse;
uniform vec3 material_specular;
uniform float material_shininess;

// Light properties (support up to 4 lights)
#define MAX_LIGHTS 4
uniform int numLights;

struct Light {
    vec3 position;
    vec3 color;
    float constant;
    float linear;
    float quadratic;
};

uniform Light lights[MAX_LIGHTS];

// Function to calculate lighting contribution from a single point light
vec3 calculatePointLight(Light light, vec3 normal, vec3 fragPos, vec3 viewDir, vec3 objectColor) {
    vec3 lightDir = normalize(light.position - fragPos);

    // Diffuse
    float diff = max(dot(normal, lightDir), 0.0);

    // Specular
    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material_shininess);

    // Attenuation (light falloff with distance)
    float distance = length(light.position - fragPos);
    float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * distance * distance);

    // Combine results with material properties
    vec3 ambient = material_ambient * light.color;
    vec3 diffuse = diff * material_diffuse * light.color;
    vec3 specular = spec * material_specular * light.color;

    // Apply attenuation
    ambient *= attenuation;
    diffuse *= attenuation;
    specular *= attenuation;

    return (ambient + diffuse + specular) * objectColor;
}

void main() {
    vec3 objectColor;
    if (useTexture) {
        objectColor = texture(textureSampler, ourTexCoord).rgb;
    } else {
        objectColor = ourColor;
    }

    vec3 norm = normalize(ourNormal);
    vec3 viewDir = normalize(viewPos - fragPos);

    // Accumulate lighting from all lights
    vec3 result = vec3(0.0);
    for (int i = 0; i < numLights && i < MAX_LIGHTS; i++) {
        result += calculatePointLight(lights[i], norm, fragPos, viewDir, objectColor);
    }

    FragColor = vec4(result, 1.0);
}
```

**What changed:**
- **`MAX_LIGHTS` define**: Support up to 4 lights (you can increase this if needed)
- **`numLights` uniform**: How many lights are actually active
- **`Light` struct**: Holds position, color, and attenuation parameters
- **`lights[]` array**: Array of up to 4 lights
- **`calculatePointLight()` function**: Calculates lighting contribution from one light
  - Computes ambient, diffuse, specular
  - Applies attenuation based on distance
  - Returns the combined result
- **`main()` loop**: Iterates through all active lights and accumulates their contributions

### Task 3: Add Shader Methods for Setting Lights

Add helper methods to the Shader struct to set light uniforms.

**In `rustgl/src/shader.rs`**, add this import at the top:

```rust
use crate::light::Light;  // Add this near the top with other imports
```

**Add these methods to `impl Shader`:**

```rust
/// Sets a single light uniform at the specified index
pub fn set_light(&self, index: usize, light: &Light) {
    let base = format!("lights[{}]", index);
    self.set_vec3(&format!("{}.position", base), &light.position);
    self.set_vec3(&format!("{}.color", base), &light.color);
    self.set_float(&format!("{}.constant", base), light.constant);
    self.set_float(&format!("{}.linear", base), light.linear);
    self.set_float(&format!("{}.quadratic", base), light.quadratic);
}

/// Sets all lights from a slice
pub fn set_lights(&self, lights: &[Light]) {
    self.set_int("numLights", lights.len() as i32);
    for (i, light) in lights.iter().enumerate() {
        if i >= 4 {  // MAX_LIGHTS
            break;
        }
        self.set_light(i, light);
    }
}
```

**What's happening:**
- `set_light()` sets a single light at a specific array index
  - Uses format! to create uniform names like `lights[0].position`
- `set_lights()` sets all lights from a slice
  - Sets `numLights` uniform
  - Calls `set_light()` for each light (up to MAX_LIGHTS)

### Task 4: Create Multiple Lights in Your Scene

Create several lights with different colors and positions.

**In `main.rs`, in the `main()` function after creating materials:**

```rust
// Create lights (after creating materials, before the render loop)
let lights = vec![
    // White light above and to the right
    Light::long_range(
        glm::vec3(5.0, 5.0, 5.0),
        glm::vec3(1.0, 1.0, 1.0),  // White
    ),
    // Red light on the left
    Light::medium_range(
        glm::vec3(-5.0, 2.0, 0.0),
        glm::vec3(1.0, 0.2, 0.2),  // Red
    ),
    // Blue light on the right
    Light::medium_range(
        glm::vec3(5.0, 2.0, -3.0),
        glm::vec3(0.2, 0.4, 1.0),  // Blue
    ),
    // Green light in front (subtle)
    Light::short_range(
        glm::vec3(0.0, 1.0, 5.0),
        glm::vec3(0.3, 1.0, 0.3),  // Green
    ),
];
```

**What's happening:**
- **4 lights** with different positions and colors
- **White light** (main light, long range)
- **Red light** on the left side
- **Blue light** on the right side
- **Green light** in front (short range, subtle)

### Task 5: Set Lights in Render Loop

Update the render function to set lights instead of the old single light.

**Update the `render()` function signature:**

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
    wireframe_mode: bool,
    use_texture: bool,
    plastic_mat: &Material,
    metal_mat: &Material,
    matte_mat: &Material,
    rubber_mat: &Material,
    chrome_mat: &Material,
    lights: &[Light],  // NEW: Accept lights slice
) {
```

**In the render function, replace the old light code:**

Find this code (around line 280):
```rust
let light_pos = glm::vec3(5.0, 5.0, 5.0);
let light_color = glm::vec3(1.0, 1.0, 1.0);
shader.set_vec3("lightPos", &light_pos);
shader.set_vec3("viewPos", &camera.position);
shader.set_vec3("lightColor", &light_color);
```

**Replace with:**
```rust
shader.set_vec3("viewPos", &camera.position);
shader.set_lights(lights);
```

**Update the render call in main() to pass lights:**

Find the render call in the main loop and add `&lights`:
```rust
render(
    &mut window,
    &sphere,
    &cube,
    &cylinder,
    &torus,
    &plane,
    &shader,
    &texture,
    &camera,
    time,
    wireframe_mode,
    use_texture,
    &plastic_material,
    &metal_material,
    &matte_material,
    &rubber_material,
    &chrome_material,
    &lights,  // NEW: Pass lights
);
```

## Success Criteria

You have completed this step when:

-  Light struct created with position, color, and attenuation parameters
-  Fragment shader supports multiple lights (up to 4)
-  Light attenuation implemented (lights dim with distance)
-  Helper methods created: `set_light()`, `set_lights()`
-  Multiple colored lights in your scene
-  Objects show combined lighting from all lights
-  No compilation errors
-  No OpenGL errors

## Testing

Run your program and observe:

1. **Multiple colored lights**:
   - Objects should have **mixed colors** from different lights
   - Left side of objects should have a **red tint** (red light)
   - Right side should have a **blue tint** (blue light)
   - Overall scene is brighter with multiple lights

2. **Light attenuation**:
   - Objects **farther from lights** should be darker
   - Objects **close to lights** should be brighter
   - The plane should show clear light falloff

3. **Dynamic lighting**:
   - As objects rotate, they show different colored highlights
   - Materials still behave correctly (shiny vs matte)

**Try this:**
- Disable texture (Key 2) to see colored lighting more clearly
- Move camera around to see how multiple lights affect objects from different angles
- Notice how the chrome sphere picks up all the different colored lights!

## Common Issues

### Issue 1: Objects are completely black

**Problem:** No lights are reaching the fragments, or `numLights` is 0.

**Solution:**
- Check that `lights` vector is not empty
- Verify `shader.set_lights(&lights)` is called before drawing
- Print `lights.len()` to confirm it's not zero
- Check shader compilation errors

### Issue 2: Only white light, no colored lights

**Problem:** Still using old single light code.

**Solution:**
- Make sure you removed the old `lightPos` and `lightColor` uniforms
- Verify new shader code is being loaded (delete and recompile)
- Check that Light structs have non-white colors

### Issue 3: Lights are too dim or too bright

**Problem:** Attenuation values or light colors are not balanced.

**Solution:**
- Try increasing light colors: `glm::vec3(2.0, 0.5, 0.5)` (values > 1.0 make lights brighter)
- Use `long_range()` instead of `medium_range()` for wider coverage
- Adjust attenuation values to taste

### Issue 4: Shader compile error: "syntax error"

**Problem:** GLSL syntax issue with Light struct or array.

**Solution:**
- Make sure Light struct definition comes before it's used in the uniform
- Check that semicolons are in the right places
- Verify `#define MAX_LIGHTS 4` is before the uniform declaration

### Issue 5: Performance is slow

**Problem:** Too many lights or expensive attenuation calculations.

**Solution:**
- 4 lights should be fine, but more may slow down older GPUs
- Consider reducing MAX_LIGHTS if needed
- This is expected - multiple lights require more calculations per fragment

## Understanding Check

Before moving on, make sure you understand:

1. **What is a point light?**
   - Emits light in all directions from a single point
   - Dims with distance (attenuation)

2. **What is light attenuation?**
   - Light getting dimmer as distance increases
   - Formula: `1.0 / (constant + linear*d + quadratic*d�)`

3. **How do multiple lights combine?**
   - Each light calculates its ambient, diffuse, specular contribution
   - All contributions are added together
   - Result = light1 + light2 + light3 + ...

4. **Why use a loop in the shader?**
   - Process each light in the array
   - Accumulate lighting contributions
   - More flexible than hardcoding each light

5. **Why do we need a Light struct in Rust AND GLSL?**
   - Rust: Store light data on CPU, pass to shader
   - GLSL: Receive light data as uniforms, use in calculations

6. **What do constant, linear, quadratic control?**
   - How quickly light dims with distance
   - Higher linear/quadratic = faster falloff (shorter range)

## Challenges

Want to experiment? Try these:

### Challenge 1: Animated Lights

Make lights move in a circle around the scene:
```rust
// In render function
let angle = time;
lights[1].position = glm::vec3(5.0 * angle.cos(), 2.0, 5.0 * angle.sin());
```

**Note:** You'll need to make `lights` mutable in render function.

### Challenge 2: Pulsing Lights

Make light intensity pulse with time:
```rust
let pulse = (time * 2.0).sin() * 0.5 + 0.5;  // 0.0 to 1.0
lights[0].color = glm::vec3(pulse, pulse, pulse);
```

### Challenge 3: Directional Light

Add support for a directional light (sun):
- No position, just a direction
- No attenuation (same brightness everywhere)
- Add to shader as a separate uniform

### Challenge 4: More Lights

Increase MAX_LIGHTS to 8 or 16:
- Update shader `#define MAX_LIGHTS 8`
- Create more lights in your scene
- Observe performance impact

### Challenge 5: Light Visualization

Draw small spheres at light positions to visualize them:
- Create a tiny sphere mesh
- Render at each light's position
- Use the light's color for the sphere
- Set material to emissive (high ambient, no diffuse/specular)

### Challenge 6: Keyboard Light Control

Add keys to toggle lights on/off:
- Key 3: Toggle light 0
- Key 4: Toggle light 1
- Remove disabled lights from the array before passing to shader

## What You've Learned

In this step, you've learned:

-  How to create a Light struct to represent light sources
-  What light attenuation is and why it's important
-  How to support multiple lights in shaders using arrays
-  How to calculate lighting contribution from each light
-  How multiple lights combine to create complex lighting
-  The difference between short-range and long-range lights
-  How colored lights create dramatic visual effects

## Next Steps

In **Step 18: Skybox**, you will:
- Create a 360� environment background
- Learn about cubemaps (6-sided textures)
- Make your scene feel more immersive
- Add reflections in a later step

---

**Ready to light up your scene with multiple colored lights?** Implement the multiple light system and watch your materials shine under complex lighting!

When you're done, let me know and I'll review your implementation! >�(
