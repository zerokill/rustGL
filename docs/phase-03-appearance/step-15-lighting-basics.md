# Step 15: Lighting Basics

**Phase 3: Appearance** | **Estimated Time:** 1-2 hours

## Goals

In this step, you will:
- Implement **Phong lighting model** (ambient + diffuse + specular)
- Transform normals to world space properly
- Add a point light to your scene
- See your 3D objects come to life with realistic shading!

## Current State Check

Good news - you already have most of the infrastructure in place:

 **Vertex struct** - Already has normals (mesh.rs)
 **VAO attributes** - Normal attribute configured at location 2
 **All primitives** - Sphere, cube, cylinder, torus, plane all have normals
 **Vertex shader** - Already receives and passes normals (basic.vert:4,8,18)
 **Shader utility methods** - set_vec3, set_float already exist (shader.rs:70-84)

L **Still needed**:
1. Transform normals to world space in vertex shader
2. Pass fragment position (world space) from vertex shader
3. Add lighting uniforms (light position, view position, material properties)
4. Implement Phong lighting calculation in fragment shader

## Phong Lighting Model - Quick Theory

The **Phong lighting model** has three components:

1. **Ambient** - Base lighting, simulates indirect light bouncing everywhere
   - Constant light on all surfaces
   - Prevents objects from being completely black in shadows

2. **Diffuse** - Directional lighting based on surface normal
   - Brighter when light hits surface directly (perpendicular)
   - Darker at grazing angles
   - Uses dot product: `max(dot(normal, lightDir), 0.0)`

3. **Specular** - Shiny highlights based on view angle
   - Bright spots where light reflects toward camera
   - Controlled by "shininess" (higher = smaller, sharper highlights)
   - Uses reflection vector and view direction

**Formula**: `Final = Ambient + Diffuse + Specular`

## Tasks

### Task 1: Update Vertex Shader

We need to pass **world space** positions and normals to the fragment shader.

**Update `shader/basic.vert`:**

```glsl
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;
layout (location = 2) in vec3 aNormal;
layout (location = 3) in vec2 aTexCoord;

out vec3 ourColor;
out vec3 ourNormal;
out vec2 ourTexCoord;
out vec3 fragPos;        // NEW: Fragment position in world space

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    // Transform position to clip space (for rendering)
    gl_Position = projection * view * model * vec4(aPos, 1.0);

    // Pass through color and texture coordinates
    ourColor = aColor;
    ourTexCoord = aTexCoord;

    // NEW: Calculate fragment position in world space
    fragPos = vec3(model * vec4(aPos, 1.0));

    // NEW: Transform normal to world space
    // Normal matrix = transpose(inverse(model)) to handle non-uniform scaling
    // For now, we'll use mat3(model) which works for uniform scaling/rotation
    ourNormal = mat3(model) * aNormal;
}
```

**What changed:**
- Added `out vec3 fragPos` - Fragment position in world space (needed for lighting calculations)
- `fragPos = vec3(model * vec4(aPos, 1.0))` - Transform vertex to world space
- `ourNormal = mat3(model) * aNormal` - Transform normal to world space
  - `mat3(model)` extracts rotation/scale (drops translation)
  - This works correctly for uniform transformations
  - For non-uniform scaling, you'd need `transpose(inverse(model))`

### Task 2: Update Fragment Shader with Phong Lighting

Now we implement the full Phong lighting model in the fragment shader.

**Update `shader/basic.frag`:**

```glsl
#version 330 core
in vec3 ourColor;
in vec3 ourNormal;
in vec2 ourTexCoord;
in vec3 fragPos;         // NEW: Fragment position in world space

out vec4 FragColor;

uniform sampler2D textureSampler;
uniform bool useTexture;

// NEW: Lighting uniforms
uniform vec3 lightPos;      // Position of the light in world space
uniform vec3 viewPos;       // Position of the camera in world space
uniform vec3 lightColor;    // Color/intensity of the light

void main() {
    // Get base color (from texture or vertex color)
    vec3 objectColor;
    if (useTexture) {
        objectColor = texture(textureSampler, ourTexCoord).rgb;
    } else {
        objectColor = ourColor;
    }

    // Normalize the normal vector (interpolation can denormalize it)
    vec3 norm = normalize(ourNormal);

    // Calculate light direction (from fragment to light)
    vec3 lightDir = normalize(lightPos - fragPos);

    // 1. AMBIENT - constant base lighting
    float ambientStrength = 0.1;
    vec3 ambient = ambientStrength * lightColor;

    // 2. DIFFUSE - angle-dependent lighting
    float diff = max(dot(norm, lightDir), 0.0);  // 0.0 to 1.0 based on angle
    vec3 diffuse = diff * lightColor;

    // 3. SPECULAR - shiny highlights
    float specularStrength = 0.5;
    vec3 viewDir = normalize(viewPos - fragPos);           // Direction to camera
    vec3 reflectDir = reflect(-lightDir, norm);            // Reflection of light
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);  // 32 = shininess
    vec3 specular = specularStrength * spec * lightColor;

    // Combine all three components
    vec3 result = (ambient + diffuse + specular) * objectColor;

    FragColor = vec4(result, 1.0);
}
```

**What's happening:**

1. **Get base color** - From texture or vertex color
2. **Normalize normal** - Interpolation between vertices can denormalize normals
3. **Light direction** - Vector from fragment to light source
4. **Ambient** - `0.1 * lightColor` = 10% ambient light
5. **Diffuse** - `dot(normal, lightDir)` measures how directly light hits surface
   - Perpendicular = 1.0 (brightest)
   - Parallel = 0.0 (no diffuse)
   - `max(..., 0.0)` prevents negative values (back-facing surfaces)
6. **Specular** - `reflect(-lightDir, norm)` calculates reflection vector
   - Compare to view direction with dot product
   - `pow(..., 32)` creates small, sharp highlight (32 = shininess factor)
7. **Combine** - `(ambient + diffuse + specular) * objectColor`

### Task 3: Set Lighting Uniforms in Render Loop

Now we need to provide the lighting parameters to the shader.

**In `main.rs`**, update the `render()` function to set lighting uniforms.

**After shader.use_program() (around line 241)**, add:

```rust
shader.use_program();

// NEW: Set lighting uniforms
let light_pos = glm::vec3(5.0, 5.0, 5.0);           // Light position in world space
let light_color = glm::vec3(1.0, 1.0, 1.0);         // White light
shader.set_vec3("lightPos", &light_pos);
shader.set_vec3("viewPos", &camera.position);        // Camera position
shader.set_vec3("lightColor", &light_color);

// Texture binding (already there)
texture.bind(0);
shader.set_int("textureSampler", 0);
shader.set_bool("useTexture", use_texture);
```

**What's happening:**
- `lightPos` - Position of the light source (top-right-front in this case)
- `viewPos` - Camera position (needed for specular calculations)
- `lightColor` - White light (1.0, 1.0, 1.0) at full intensity

You already have `camera.position` available since it's passed to the render function.

## Success Criteria

You have completed this step when:

-  Shaders compile without errors
-  Objects have realistic shading (darker on sides, brighter facing light)
-  Specular highlights visible on rotating objects
-  Objects look 3D and solid (not flat)
-  Lighting changes as you move the camera (specular follows view)
-  No OpenGL errors

## Testing

Run your program and you should see:

1. **Dramatic improvement in 3D appearance** - Objects look solid and realistic
2. **Shading based on light position** - Surfaces facing (5, 5, 5) are brighter
3. **Specular highlights** - Shiny spots on rotating sphere and torus
4. **Ambient prevents pitch black** - All surfaces have at least 10% brightness
5. **Dynamic lighting** - As camera moves, highlights shift (specular is view-dependent)

**Try this:**
- Disable texture (Key 2) to see lighting on pure colors
- Move camera around - notice specular highlights follow your view
- Objects at different positions have different lighting intensity

## Common Issues

### Issue 1: Objects are too dark or completely black

**Problem:** Normals might be incorrect or not normalized.

**Solution:**
- Check that primitives have correct normals (should point outward)
- Ensure `normalize(ourNormal)` is called in fragment shader
- Try increasing `ambientStrength` to 0.3 to see if it's a lighting issue
- Add debug output: `FragColor = vec4(norm, 1.0);` to visualize normals as colors

### Issue 2: No specular highlights visible

**Problem:** Shininess too high, or light/camera positions don't align.

**Solution:**
- Lower shininess: try `pow(..., 8)` instead of `pow(..., 32)`
- Increase `specularStrength` to 1.0
- Move camera to align view direction with light reflection
- Check that `viewPos` is being set correctly (use camera.position)

### Issue 3: Lighting doesn't change with camera movement

**Problem:** Using wrong space for calculations (e.g., view space instead of world space).

**Solution:**
- Ensure `fragPos` is in world space: `vec3(model * vec4(aPos, 1.0))`
- Ensure `ourNormal` is in world space: `mat3(model) * aNormal`
- Ensure `viewPos` is camera position in world space (not view matrix)
- All lighting calculations should be in the same space (world space)

### Issue 4: Normals look wrong on scaled objects

**Problem:** Non-uniform scaling distorts normals.

**Solution:**
- For now, stick with uniform scaling (same scale for X, Y, Z)
- Later, you can implement normal matrix: `transpose(inverse(mat3(model)))`
- This requires matrix inversion on CPU, passing as separate uniform

### Issue 5: Compile error: "undeclared identifier 'fragPos'"

**Problem:** Forgot to add `in vec3 fragPos` in fragment shader.

**Solution:**
- Add `in vec3 fragPos;` at the top of basic.frag
- Make sure vertex shader has `out vec3 fragPos;`

## Understanding Check

Before moving on, make sure you understand:

1. **What is Phong lighting?**
   - A lighting model with three components: ambient, diffuse, specular

2. **Why transform normals with mat3(model)?**
   - Normals are directions (not positions), so we drop translation
   - Rotation and scaling still apply to normals
   - `mat3(model)` extracts the 3x3 rotation/scale part

3. **What is the dot product doing in diffuse calculation?**
   - Measures alignment between normal and light direction
   - 1.0 = perpendicular (brightest), 0.0 = parallel (no light)

4. **Why normalize the normal after interpolation?**
   - Interpolation between vertices can change vector length
   - Lighting calculations require unit vectors (length = 1.0)

5. **What does the shininess parameter (32) control?**
   - Size of specular highlight
   - Higher = smaller, sharper highlight (metal, glass)
   - Lower = larger, softer highlight (plastic, skin)

6. **Why is specular view-dependent but diffuse is not?**
   - Diffuse: light scatters equally in all directions
   - Specular: light reflects like a mirror, depends on view angle

## Challenges

Want to experiment? Try these:

### Challenge 1: Moving Light

- Make the light orbit around the scene using `sin(time)` and `cos(time)`
- Watch the lighting dynamically change on your objects
- Example: `let light_pos = glm::vec3(5.0 * time.cos(), 5.0, 5.0 * time.sin());`

### Challenge 2: Colored Lights

- Change `lightColor` to `vec3(1.0, 0.5, 0.3)` (warm orange)
- Or `vec3(0.3, 0.5, 1.0)` (cool blue)
- See how light color affects object appearance

### Challenge 3: Material Properties

- Add keyboard controls to adjust shininess (keys 3/4 to increase/decrease)
- Pass shininess as a uniform
- See the difference between shiny (64) and rough (8) surfaces

### Challenge 4: Multiple Lights

- Add a second light source with a different color
- Calculate lighting for both and add them together
- This is the foundation for multi-light rendering!

### Challenge 5: Visualize Light Position

- Draw a small sphere at `lightPos`
- Use emissive color (no lighting) so it glows
- Helps debug light positioning

## What You've Learned

In this step, you've learned:

-  The Phong lighting model (ambient + diffuse + specular)
-  How to transform normals to world space
-  The difference between diffuse (view-independent) and specular (view-dependent) lighting
-  How to use dot product and reflection for lighting calculations
-  Why normals must be normalized before use
-  The role of shininess in controlling specular highlights

## Next Steps

In **Step 16: Multiple Lights**, you will:
- Support multiple light sources in a scene
- Implement directional lights (sun) and point lights (bulbs)
- Add attenuation (light falloff with distance)
- Create a proper Light struct in Rust

But first, let's get basic Phong lighting working beautifully!

---

**Ready to bring your scene to life?** Make the changes and run your program!

When you're done, show me:
1. A screenshot of your lit scene (especially specular highlights!)
2. Any issues you encountered
3. What you think of the difference lighting makes

This is one of the most visually rewarding steps - your objects will finally look 3D! >�
