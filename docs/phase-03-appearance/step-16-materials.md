# Step 16: Materials

**Phase 3: Appearance** | **Estimated Time:** 1-2 hours

## Goals

In this step, you will:
- Create a **Material** struct in Rust to represent object surface properties
- Define material properties: **ambient**, **diffuse**, **specular**, and **shininess**
- Pass material properties to shaders as uniforms
- See different objects with different materials (shiny metal vs rough plastic)

## Why Materials?

Right now, your lighting values are **hardcoded** in the fragment shader:
- `ambientStrength = 0.1` (line 29)
- `specularStrength = 0.5` (line 35)
- `shininess = 32` (line 38)

This means **every object** looks the same! A metal sphere and a plastic cube have identical surface properties.

**Materials solve this** by letting you define surface properties **per-object**:
- **Shiny metal**: High specular, high shininess (64+)
- **Rough plastic**: Low specular, low shininess (8-16)
- **Matte surface**: No specular, any shininess
- **Glowing object**: High ambient (emissive-like)

## Material Properties Explained

A material has **four key properties** that control how light interacts with a surface:

### 1. Ambient (vec3)
How much **ambient light** the material reflects. Think of this as the "base color in shadow."
- High ambient = bright even without direct light
- Low ambient = dark in shadows
- Typical: `vec3(0.1, 0.1, 0.1)` for most materials

### 2. Diffuse (vec3)
The **main color** of the material under direct light. This is what we usually think of as the object's color.
- Usually matches the object's base color
- Controls how much light scatters in all directions
- Typical: `vec3(0.8, 0.5, 0.2)` for orange plastic

### 3. Specular (vec3)
The **color of highlights** (shiny spots). For most materials, this is white or grayish.
- Metals: Specular matches diffuse (colored highlights)
- Non-metals: Specular is white/gray (white highlights)
- Typical: `vec3(1.0, 1.0, 1.0)` for plastic, `vec3(0.8, 0.5, 0.2)` for copper

### 4. Shininess (float)
How **focused** the specular highlight is. Higher = smaller, sharper highlight.
- Low (2-8): Very rough, broad highlight (rubber, clay)
- Medium (16-32): Somewhat shiny (plastic)
- High (64-128): Very shiny (polished metal, glass)
- Very High (256+): Mirror-like (chrome)

## Current State Check

 **Already implemented**:
- Phong lighting calculation in fragment shader (basic.frag)
- Lighting uniforms: lightPos, viewPos, lightColor
- Shader utility methods: set_vec3, set_float

L **Still needed**:
1. Create Material struct in Rust
2. Update fragment shader to use material uniforms
3. Set material uniforms in render loop
4. Create different materials for different objects

## Tasks

### Task 1: Create Material Struct in Rust

Create a new file `rustgl/src/material.rs` to define material properties.

**Create `rustgl/src/material.rs`:**

```rust
use nalgebra_glm as glm;

/// Represents material surface properties for Phong lighting
#[derive(Clone, Copy, Debug)]
pub struct Material {
    /// Ambient color - how much ambient light the material reflects
    pub ambient: glm::Vec3,

    /// Diffuse color - the main color of the material under direct light
    pub diffuse: glm::Vec3,

    /// Specular color - the color of shiny highlights
    pub specular: glm::Vec3,

    /// Shininess - controls how focused the specular highlight is (higher = sharper)
    pub shininess: f32,
}

impl Material {
    /// Creates a new material with specified properties
    pub fn new(ambient: glm::Vec3, diffuse: glm::Vec3, specular: glm::Vec3, shininess: f32) -> Self {
        Material {
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    /// Creates a shiny plastic material (medium shininess, white highlights)
    pub fn plastic(color: glm::Vec3) -> Self {
        Material {
            ambient: color * 0.1,          // 10% ambient
            diffuse: color,                // Main color
            specular: glm::vec3(0.5, 0.5, 0.5),  // White-ish highlights
            shininess: 32.0,               // Medium shine
        }
    }

    /// Creates a metallic material (high shininess, colored highlights)
    pub fn metal(color: glm::Vec3) -> Self {
        Material {
            ambient: color * 0.2,          // 20% ambient (metals are brighter)
            diffuse: color * 0.8,          // Slightly darker main color
            specular: color,               // Colored highlights (metals reflect their color)
            shininess: 64.0,               // High shine
        }
    }

    /// Creates a rough/matte material (low shininess, minimal highlights)
    pub fn matte(color: glm::Vec3) -> Self {
        Material {
            ambient: color * 0.1,
            diffuse: color,
            specular: glm::vec3(0.1, 0.1, 0.1),  // Very dim highlights
            shininess: 8.0,                // Low shine (rough surface)
        }
    }

    /// Creates a rubber-like material (very low shininess, soft highlights)
    pub fn rubber(color: glm::Vec3) -> Self {
        Material {
            ambient: color * 0.05,         // Low ambient
            diffuse: color,
            specular: glm::vec3(0.3, 0.3, 0.3),
            shininess: 4.0,                // Very low shine
        }
    }

    /// Creates a shiny material like polished chrome (very high shininess)
    pub fn chrome() -> Self {
        Material {
            ambient: glm::vec3(0.25, 0.25, 0.25),
            diffuse: glm::vec3(0.4, 0.4, 0.4),
            specular: glm::vec3(0.77, 0.77, 0.77),
            shininess: 128.0,              // Very high shine
        }
    }
}
```

**What's happening:**
- Material struct holds the four Phong properties
- Helper methods create common material types (plastic, metal, matte, rubber, chrome)
- Each preset has carefully chosen values for realistic appearance

**Add to `main.rs`:**
```rust
mod material;  // Add this with other mod declarations at the top
use material::Material;  // Add this with other use statements
```

### Task 2: Add Material Uniforms to Fragment Shader

Update the fragment shader to receive material properties as uniforms instead of using hardcoded values.

**Update `shader/basic.frag`:**

Replace the hardcoded lighting calculation with material-based lighting.

**Current code (lines 29-39):**
```glsl
float ambientStrength = 0.1;
vec3 ambient = ambientStrength * lightColor;

float diff = max(dot(norm, lightDir), 0.0);
vec3 diffuse = diff * lightColor;

float specularStrength = 0.5;
vec3 viewDir = normalize(viewPos - fragPos);
vec3 reflectDir = reflect(-lightDir, norm);
float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
vec3 specular = specularStrength * spec * lightColor;
```

**Replace with:**
```glsl
// Material properties
uniform vec3 material_ambient;
uniform vec3 material_diffuse;
uniform vec3 material_specular;
uniform float material_shininess;

void main() {
    vec3 objectColor;
    if (useTexture) {
        objectColor = texture(textureSampler, ourTexCoord).rgb;
    } else {
        objectColor = ourColor;
    }

    vec3 norm = normalize(ourNormal);
    vec3 lightDir = normalize(lightPos - fragPos);

    // Ambient - uses material ambient property
    vec3 ambient = material_ambient * lightColor;

    // Diffuse - uses material diffuse property
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * material_diffuse * lightColor;

    // Specular - uses material specular property and shininess
    vec3 viewDir = normalize(viewPos - fragPos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material_shininess);
    vec3 specular = spec * material_specular * lightColor;

    // Combine lighting with object color
    vec3 result = (ambient + diffuse + specular) * objectColor;

    FragColor = vec4(result, 1.0);
}
```

**What changed:**
- Added material uniforms at the top
- `ambient` now uses `material_ambient` instead of hardcoded `0.1`
- `diffuse` uses `material_diffuse` (multiplied by light color)
- `specular` uses `material_specular` and `material_shininess`
- This lets each object have different surface properties!

### Task 3: Create Helper Method to Set Material Uniforms

Add a convenience method to the Shader struct to set all material properties at once.

**In `rustgl/src/shader.rs`**, add this method to the `impl Shader` block:

```rust
pub fn set_material(&self, material: &Material) {
    self.set_vec3("material_ambient", &material.ambient);
    self.set_vec3("material_diffuse", &material.diffuse);
    self.set_vec3("material_specular", &material.specular);
    self.set_float("material_shininess", material.shininess);
}
```

**Don't forget to import Material at the top of shader.rs:**
```rust
use crate::material::Material;  // Add this near the top with other imports
```

**What's happening:**
- Instead of calling `set_vec3` four times, you can call `set_material(mat)` once
- This makes the render code cleaner and less error-prone

### Task 4: Create Materials and Use Them in Render Loop

Now create different materials for your objects and apply them before drawing.

**In `main.rs`, in the `main()` function after creating primitives:**

```rust
// Create materials (after creating meshes, before the render loop)
let plastic_material = Material::plastic(glm::vec3(0.3, 0.7, 1.0));  // Blue plastic
let metal_material = Material::metal(glm::vec3(1.0, 0.5, 0.2));     // Orange metal
let matte_material = Material::matte(glm::vec3(0.2, 1.0, 0.3));     // Green matte
let rubber_material = Material::rubber(glm::vec3(1.0, 0.3, 0.7));   // Pink rubber
let chrome_material = Material::chrome();                            // Chrome
```

**In the `render()` function, set materials before drawing each object:**

Update the render function signature to accept materials:
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
    plastic_mat: &Material,    // NEW
    metal_mat: &Material,      // NEW
    matte_mat: &Material,      // NEW
    rubber_mat: &Material,     // NEW
    chrome_mat: &Material,     // NEW
) {
    // ... existing setup code ...
```

**Update the render call in main() to pass materials:**
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
    &plastic_material,   // NEW
    &metal_material,     // NEW
    &matte_material,     // NEW
    &rubber_material,    // NEW
    &chrome_material,    // NEW
);
```

**In the render function, before each object draw:**

```rust
// Draw sphere (left, rotating) - BLUE PLASTIC
shader.set_material(plastic_mat);
let mut model = glm::Mat4::identity();
model = glm::translate(&model, &glm::vec3(-4.0, 0.0, 0.0));
// ... rest of sphere drawing code ...

// Draw cube (center-left, rotating) - ORANGE METAL
shader.set_material(metal_mat);
let mut model = glm::Mat4::identity();
model = glm::translate(&model, &glm::vec3(-2.0, 0.0, 0.0));
// ... rest of cube drawing code ...

// Draw cylinder (center, rotating) - GREEN MATTE
shader.set_material(matte_mat);
let mut model = glm::Mat4::identity();
model = glm::translate(&model, &glm::vec3(0.0, 0.0, 0.0));
// ... rest of cylinder drawing code ...

// Draw torus (center-right, rotating) - PINK RUBBER
shader.set_material(rubber_mat);
let mut model = glm::Mat4::identity();
model = glm::translate(&model, &glm::vec3(2.0, 0.0, 0.0));
// ... rest of torus drawing code ...

// Draw another sphere (right, different rotation) - CHROME
shader.set_material(chrome_mat);
let mut model = glm::Mat4::identity();
model = glm::translate(&model, &glm::vec3(4.0, 0.0, 0.0));
// ... rest of sphere drawing code ...

// Draw plane (ground) - MATTE (reuse)
shader.set_material(matte_mat);
let mut model = glm::Mat4::identity();
model = glm::translate(&model, &glm::vec3(0.0, -2.0, 0.0));
// ... rest of plane drawing code ...
```

**What's happening:**
- Each object now has its own material
- Before drawing each object, call `shader.set_material()` to update the uniforms
- Objects will now look visually distinct!

## Success Criteria

You have completed this step when:

-  Material struct created with ambient, diffuse, specular, shininess
-  Helper methods (plastic, metal, matte, rubber, chrome) work
-  Fragment shader uses material uniforms instead of hardcoded values
-  Each object has a different material
-  Objects look visually distinct:
  - Sphere (plastic): Medium shine, white highlights
  - Cube (metal): High shine, colored highlights
  - Cylinder (matte): Dull, minimal highlights
  - Torus (rubber): Very dull, soft highlights
  - Sphere 2 (chrome): Mirror-like, very sharp highlights
-  No compilation errors
-  No OpenGL errors

## Testing

Run your program and observe:

1. **Different shininess levels**:
   - Chrome sphere (right) should have **tiny, super-bright** highlights
   - Metal cube should have **medium-sized, colored** highlights
   - Plastic sphere (left) should have **medium white** highlights
   - Rubber torus should have **large, soft** highlights
   - Matte cylinder should have **barely visible** highlights

2. **Different colors**:
   - Metal cube: Orange with orange highlights (metals have colored specular)
   - Plastic sphere: Blue with white highlights (plastics have white specular)
   - Chrome sphere: Gray/silver with very bright white highlights

3. **Rotation reveals highlights**:
   - As objects rotate, highlights move across their surfaces
   - Different materials show different highlight behaviors

**Tip:** Disable texture (Key 2) to see materials more clearly without texture colors interfering.

## Common Issues

### Issue 1: All objects look the same

**Problem:** Materials not being set before drawing.

**Solution:**
- Make sure you call `shader.set_material()` **before** each draw call
- Check that the material uniforms are spelled correctly in the shader

### Issue 2: Objects are too dark or too bright

**Problem:** Material properties are too extreme.

**Solution:**
- Ambient should usually be low (0.05 - 0.2)
- Diffuse should be medium-high (0.5 - 1.0)
- Adjust values to taste

### Issue 3: No specular highlights visible

**Problem:** Light position, camera position, or material alignment issue.

**Solution:**
- Move the camera around to find the specular reflection angle
- Try increasing specular strength: `glm::vec3(1.0, 1.0, 1.0)`
- Lower shininess to make highlights bigger and easier to see: try 8.0

### Issue 4: Compile error: "cannot find type `Material`"

**Problem:** Forgot to add `mod material;` or `use material::Material;`

**Solution:**
- Add `mod material;` at the top of main.rs with other mod declarations
- Add `use material::Material;` with other use statements

### Issue 5: Shader compile error: "undeclared identifier 'material_ambient'"

**Problem:** Forgot to add material uniforms to fragment shader.

**Solution:**
- Make sure you added the four material uniforms at the top of basic.frag
- Spelling must match exactly: `material_ambient`, `material_diffuse`, etc.

## Understanding Check

Before moving on, make sure you understand:

1. **What are the four material properties?**
   - Ambient, diffuse, specular, shininess

2. **What's the difference between diffuse and specular?**
   - Diffuse: main color, scatters in all directions
   - Specular: highlight color, reflects toward viewer

3. **Why do metals have colored specular but plastics have white specular?**
   - Metals reflect their color in highlights (colored specular)
   - Non-metals reflect white light (white/gray specular)

4. **What does shininess control?**
   - How focused/sharp the specular highlight is
   - Higher = smaller, sharper highlight

5. **Why set materials before each draw call?**
   - Each object can have different surface properties
   - Uniforms persist until changed, so we update them per-object

## Challenges

Want to experiment? Try these:

### Challenge 1: Gold Material

Create a gold material with appropriate colors:
- Ambient: `vec3(0.24725, 0.1995, 0.0745)`
- Diffuse: `vec3(0.75164, 0.60648, 0.22648)`
- Specular: `vec3(0.628281, 0.555802, 0.366065)`
- Shininess: `51.2`

### Challenge 2: Emerald Material

Create an emerald (green gem) material:
- Ambient: `vec3(0.0215, 0.1745, 0.0215)`
- Diffuse: `vec3(0.07568, 0.61424, 0.07568)`
- Specular: `vec3(0.633, 0.727811, 0.633)`
- Shininess: `76.8`

### Challenge 3: Material Cycling

Add keyboard controls to cycle through materials:
- Key 3: Cycle to next material preset
- Apply to all objects or just one object

### Challenge 4: Material Animation

Animate shininess over time:
```rust
let shininess = 4.0 + (time.sin() * 0.5 + 0.5) * 124.0;  // 4 to 128
```
Watch highlights shrink and grow!

### Challenge 5: Custom Material Presets

Create your own material presets:
- Wood
- Stone
- Glass
- Ceramic

Research realistic values or experiment to get the look you want!

## What You've Learned

In this step, you've learned:

-  What materials are and why they're important
-  The four Phong material properties (ambient, diffuse, specular, shininess)
-  How to create a Material struct in Rust
-  How to pass material properties to shaders as uniforms
-  The difference between metallic and non-metallic materials
-  How shininess affects the appearance of highlights
-  How to create material preset functions for common surface types

## Next Steps

In **Step 17: Multiple Lights**, you will:
- Support multiple light sources in your scene
- Implement different light types (point lights, directional lights)
- Add light attenuation (falloff with distance)
- See your materials lit by multiple colored lights!

---

**Ready to make your objects look like different materials?** Implement the material system and watch your scene come alive with variety!

When you're done, let me know and I'll review your implementation! >�(
