# Step 24: Perlin Noise 2D

**Phase 5: Procedural Generation** | **Estimated Time:** 2-3 hours

## Goals

In this step, you will:
- Understand **Perlin noise** and why it's essential for procedural generation
- Implement a **2D Perlin noise generator** from scratch in Rust
- Learn the core concepts: **gradient vectors**, **fade function**, and **linear interpolation**
- Visualize noise output as a **test plane** or **texture**
- Create the foundation for terrain generation in later steps

## What is Perlin Noise?

**Perlin noise** is a gradient-based noise algorithm invented by Ken Perlin in 1983. It generates **smooth, natural-looking random patterns** that are perfect for:
- Terrain height maps
- Cloud patterns
- Procedural textures
- Organic-looking randomness

### Why not use `rand()`?

Regular random numbers are **discontinuous** (jerky):
```
Random: 0.1 � 0.9 � 0.2 � 0.8  (harsh jumps)
```

Perlin noise is **continuous** (smooth):
```
Perlin: 0.1 � 0.3 � 0.5 � 0.4  (gradual changes)
```

This smoothness makes Perlin noise look **natural** instead of random static.

### Key Properties

1. **Deterministic**: Same input coordinates always give same output
2. **Continuous**: Neighboring points have similar values
3. **Gradient-based**: Uses random gradient vectors, not random values
4. **Range**: Outputs roughly between -1.0 and 1.0
5. **Tileable**: Can be made to wrap seamlessly

## How Perlin Noise Works

Here's the high-level algorithm:

1. **Grid of gradients**: Create a grid where each corner has a random gradient vector
2. **Interpolation**: For any point (x, y):
   - Find the 4 surrounding grid corners
   - Calculate influence from each corner's gradient
   - Interpolate between the 4 influences using a smooth fade function
3. **Result**: A smooth value between -1.0 and 1.0

### Visual Explanation

```
Grid cell with gradients at corners:

  G1--------G2     G = gradient vector at corner
   |   P    |      P = point we're sampling
   |        |
  G3--------G4

Steps:
1. Get gradients G1, G2, G3, G4 (from permutation table)
2. Calculate distance vectors from P to each corner
3. Dot product: gradient � distance for each corner
4. Interpolate horizontally: lerp(G1�d1, G2�d2) and lerp(G3�d3, G4�d4)
5. Interpolate vertically: final = lerp(top, bottom)
```

## Current State Check

 **Already implemented**:
- Vertex structure with position, color, normal, UV (mesh.rs)
- Mesh rendering system with various primitives
- Scene management system (scene.rs) for organizing objects
- Transform system for position, rotation, and scale
- Camera and view/projection matrices
- Material and lighting system (Phong shading)
- Shader utilities for setting uniforms

L **Still needed**:
1. Create `noise.rs` module for Perlin noise implementation
2. Implement permutation table for pseudo-random gradients
3. Implement fade function (smoothstep)
4. Implement linear interpolation (lerp)
5. Implement 2D Perlin noise function
6. Add `noise_test_plane()` method to Mesh
7. Add noise plane to the scene using `scene.add_object()`

## Tasks

### Task 1: Create Noise Module Structure

Create a new file `rustgl/src/noise.rs` that will hold all noise generation code.

**Create `rustgl/src/noise.rs`:**

```rust
use std::f32::consts::PI;

/// Perlin noise generator for 2D and 3D procedural content
pub struct PerlinNoise {
    /// Permutation table for pseudo-random gradient selection
    permutation: [u8; 512],
}

impl PerlinNoise {
    /// Creates a new Perlin noise generator with a seed
    pub fn new(seed: u32) -> Self {
        let mut perm = [0u8; 256];

        // Initialize permutation table with values 0-255
        for i in 0..256 {
            perm[i] = i as u8;
        }

        // Shuffle using seed (simple LCG random)
        let mut rng_state = seed;
        for i in (1..256).rev() {
            // Linear congruential generator
            rng_state = rng_state.wrapping_mul(1664525).wrapping_add(1013904223);
            let j = (rng_state as usize) % (i + 1);
            perm.swap(i, j);
        }

        // Duplicate permutation table to avoid overflow wrapping
        let mut permutation = [0u8; 512];
        for i in 0..512 {
            permutation[i] = perm[i % 256];
        }

        PerlinNoise { permutation }
    }

    // We'll add noise functions here in the next tasks
}
```

**What's happening:**
- `permutation` table: Maps grid coordinates to pseudo-random gradient indices
- Seeded shuffle: Different seeds produce different noise patterns
- Duplicated to 512: Avoids modulo operations during noise calculation
- Linear Congruential Generator: Simple but effective pseudo-random shuffle

**Add to `main.rs`:**
```rust
mod noise;  // Add this with other mod declarations at the top
use noise::PerlinNoise;  // Add this with other use statements
```

Note: Your project should already have `Transform` imported from the transform module (`use transform::Transform;`).

### Task 2: Implement Helper Functions

Add these mathematical helper functions to the `PerlinNoise` implementation.

**Add to `impl PerlinNoise` in `noise.rs`:**

```rust
    /// Fade function for smooth interpolation (6t^5 - 15t^4 + 10t^3)
    /// This is smoother than linear and avoids visible grid artifacts
    fn fade(t: f32) -> f32 {
        t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
    }

    /// Linear interpolation between a and b
    fn lerp(t: f32, a: f32, b: f32) -> f32 {
        a + t * (b - a)
    }

    /// Calculate gradient influence in 2D
    /// hash: determines which gradient vector to use
    /// x, y: distance from grid point to sample point
    fn grad2d(hash: u8, x: f32, y: f32) -> f32 {
        // Use bottom 2 bits to select one of 4 gradient directions
        let h = hash & 3;
        match h {
            0 => x + y,   // Gradient: (1, 1)
            1 => -x + y,  // Gradient: (-1, 1)
            2 => x - y,   // Gradient: (1, -1)
            _ => -x - y,  // Gradient: (-1, -1)
        }
    }
```

**What each function does:**

1. **`fade(t)`**: Smoothstep function that creates smooth transitions
   - Input: 0.0 to 1.0
   - Output: 0.0 to 1.0, but with ease-in and ease-out
   - The formula `6t^5 - 15t^4 + 10t^3` has zero first and second derivatives at t=0 and t=1
   - This prevents visible seams in the noise

2. **`lerp(t, a, b)`**: Linear interpolation
   - Blend between values `a` and `b` using parameter `t`
   - t=0.0 returns `a`, t=1.0 returns `b`

3. **`grad2d(hash, x, y)`**: Gradient dot product
   - Uses hash to select a gradient vector
   - Computes dot product: gradient � (x, y)
   - 4 diagonal gradients: (1,1), (-1,1), (1,-1), (-1,-1)
   - Returns the influence of that gradient at the point

### Task 3: Implement 2D Perlin Noise

Now implement the main Perlin noise function that combines everything.

**Add to `impl PerlinNoise` in `noise.rs`:**

```rust
    /// Generate 2D Perlin noise at coordinates (x, y)
    /// Returns a value roughly in the range [-1.0, 1.0]
    pub fn noise2d(&self, x: f32, y: f32) -> f32 {
        // Find unit grid cell containing the point
        let xi = (x.floor() as i32) & 255;
        let yi = (y.floor() as i32) & 255;

        // Relative position within cell (0.0 to 1.0)
        let xf = x - x.floor();
        let yf = y - y.floor();

        // Compute fade curves for x and y
        let u = Self::fade(xf);
        let v = Self::fade(yf);

        // Hash coordinates of the 4 cube corners
        let aa = self.permutation[(self.permutation[xi as usize] as usize + yi as usize) % 512];
        let ab = self.permutation[(self.permutation[xi as usize] as usize + yi as usize + 1) % 512];
        let ba = self.permutation[(self.permutation[xi as usize + 1] as usize + yi as usize) % 512];
        let bb = self.permutation[(self.permutation[xi as usize + 1] as usize + yi as usize + 1) % 512];

        // Calculate gradient influences at 4 corners
        let g1 = Self::grad2d(aa, xf, yf);
        let g2 = Self::grad2d(ba, xf - 1.0, yf);
        let g3 = Self::grad2d(ab, xf, yf - 1.0);
        let g4 = Self::grad2d(bb, xf - 1.0, yf - 1.0);

        // Interpolate horizontally
        let x1 = Self::lerp(u, g1, g2);
        let x2 = Self::lerp(u, g3, g4);

        // Interpolate vertically
        let result = Self::lerp(v, x1, x2);

        result
    }
```

**How it works:**

1. **Find grid cell** (`xi`, `yi`): Which grid square contains point (x, y)?
2. **Find position in cell** (`xf`, `yf`): Where within the square? (0.0 to 1.0)
3. **Apply fade**: Smooth the interpolation parameters using the fade function
4. **Hash corners**: Use permutation table to get pseudo-random gradient for each corner
5. **Calculate gradients**: Compute influence from each corner's gradient
6. **Interpolate**: Blend the 4 corner influences smoothly

The result is a smooth value that varies naturally across 2D space.

### Task 4: Add Convenience Method for Positive Range

Often we want noise in the range [0.0, 1.0] instead of [-1.0, 1.0].

**Add to `impl PerlinNoise` in `noise.rs`:**

```rust
    /// Generate 2D Perlin noise in the range [0.0, 1.0]
    pub fn noise2d_01(&self, x: f32, y: f32) -> f32 {
        (self.noise2d(x, y) + 1.0) * 0.5
    }
```

This simply shifts and scales the output to be positive.

### Task 5: Create a Test Plane to Visualize Noise

Now let's visualize the noise! Create a plane mesh where the Y coordinate (height) is determined by Perlin noise.

**Add to `mesh.rs`, inside `impl Mesh`:**

```rust
    /// Creates a test plane with heights driven by a noise function
    /// Used to visualize 2D noise patterns
    pub fn noise_test_plane(noise_fn: &dyn Fn(f32, f32) -> f32, size: usize, scale: f32) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let step = 1.0 / size as f32;

        // Generate vertices in a grid
        for y in 0..=size {
            for x in 0..=size {
                let xf = (x as f32 - size as f32 / 2.0) * step * 10.0;
                let zf = (y as f32 - size as f32 / 2.0) * step * 10.0;

                // Sample noise at this position
                let height = noise_fn(xf * scale, zf * scale);

                // Color based on height (green = low, white = high)
                let color_val = (height + 1.0) * 0.5; // Map -1..1 to 0..1
                let color = [color_val, color_val, color_val];

                // For now, use simple upward normal (we'll calculate proper normals in Step 27)
                let normal = [0.0, 1.0, 0.0];

                let u = x as f32 / size as f32;
                let v = y as f32 / size as f32;

                vertices.push(Vertex::new(
                    [xf, height, zf],
                    color,
                    normal,
                    [u, v],
                ));
            }
        }

        // Generate indices for triangles
        for y in 0..size {
            for x in 0..size {
                let top_left = (y * (size + 1) + x) as u32;
                let top_right = top_left + 1;
                let bottom_left = ((y + 1) * (size + 1) + x) as u32;
                let bottom_right = bottom_left + 1;

                // First triangle (top-left, bottom-left, top-right)
                indices.push(top_left);
                indices.push(bottom_left);
                indices.push(top_right);

                // Second triangle (top-right, bottom-left, bottom-right)
                indices.push(top_right);
                indices.push(bottom_left);
                indices.push(bottom_right);
            }
        }

        Mesh::new_indexed(&vertices, &indices)
    }
```

**What's happening:**
- Creates a grid of `size � size` vertices
- For each vertex, samples the noise function at that position
- Height is determined by the noise value
- Color is grayscale based on height (visualizes the noise pattern)
- Generates triangle indices to form the mesh

### Task 6: Add Noise Plane to Scene

Now add code to create a Perlin noise generator and add the test plane to your scene.

**In `main.rs`, after creating the Perlin noise generator (should already exist around line 169):**

```rust
// Create Perlin noise generator
let perlin = PerlinNoise::new(42);  // Seed = 42 for reproducible results

// Create a test plane showing the noise pattern
let noise_scale = 0.5;  // Frequency of the noise (lower = larger features)
let noise_plane = Mesh::noise_test_plane(
    &|x, y| perlin.noise2d(x, y),
    100,  // 100x100 grid
    noise_scale,
);
```

**Add the noise plane to your scene (after creating the scene, before or instead of other objects):**

```rust
// Add noise test plane to scene
scene.add_object(
    noise_plane,
    Material::matte(glm::vec3(0.5, 0.7, 0.3)),  // Greenish terrain material
    Transform::from_position_scale(
        glm::vec3(0.0, 0.0, 0.0),      // Center position
        glm::vec3(5.0, 2.0, 5.0)       // Scale: 5x in XZ, 2x in Y for visible height variation
    )
);
```

**What's happening:**
- Creates the Perlin noise generator with a fixed seed (42)
- Generates a 100×100 mesh where heights are driven by the noise function
- Adds it to the scene with a greenish matte material
- Scales it up to make it visible, with extra Y scaling to emphasize height variation
- The scene's `render()` method will automatically draw it with proper transforms

**Optional: Hide other objects temporarily**

To see the noise plane clearly, you can temporarily comment out the other `scene.add_object()` calls in main.rs, or move the camera to get a better view of the terrain.

## Success Criteria

You have completed this step when:

-  `noise.rs` module created with `PerlinNoise` struct
-  Permutation table correctly initialized with seed
-  Fade function, lerp, and grad2d implemented
-  `noise2d()` function generates smooth noise values
-  Test plane renders with noise-driven heights
-  Different seeds produce different patterns
-  Noise pattern is smooth and continuous (no harsh edges)
-  No compilation errors
-  Pattern is consistent (same input � same output)

## Testing

Run your program and observe:

1. **Smooth variation**: The terrain should have smooth hills and valleys, not random spikes

2. **Continuity**: No visible grid lines or artifacts

3. **Different scales**: Try different `noise_scale` values:
   - `0.1`: Very large, gentle features
   - `0.5`: Medium-sized hills
   - `1.0`: Smaller, more frequent variations
   - `2.0`: High-frequency detail

4. **Different seeds**: Change the seed value and see completely different patterns

5. **Color visualization**: Should see grayscale gradient (dark = valleys, bright = peaks)

**Expected appearance**: Rolling hills and valleys, like a smooth heightmap.

## Common Issues

### Issue 1: Harsh grid pattern visible

**Problem:** Fade function not applied correctly.

**Solution:**
- Make sure you're using `Self::fade()` on both `u` and `v`
- Verify the fade formula: `t * t * t * (t * (t * 6.0 - 15.0) + 10.0)`

### Issue 2: Flat plane (no variation)

**Problem:** Noise always returning 0 or similar value.

**Solution:**
- Check that permutation table is properly shuffled
- Verify `grad2d()` is returning non-zero values
- Try increasing the scale: multiply x and y by 10 before passing to noise

### Issue 3: Pattern changes every frame

**Problem:** Seed is different each frame.

**Solution:**
- Create `PerlinNoise` outside the render loop
- Use a fixed seed (like 42) during testing

### Issue 4: Compile error: "cannot find function `noise_test_plane`"

**Problem:** Method added to wrong place or not marked `pub`.

**Solution:**
- Ensure `noise_test_plane` is inside `impl Mesh { ... }`
- Ensure it's marked `pub`

### Issue 5: Values out of range or NaN

**Problem:** Mathematical error in noise calculation.

**Solution:**
- Add bounds checking: `xi & 255` ensures values stay within permutation table
- Check array indices don't go out of bounds
- Verify `x.floor()` is being cast to `i32` properly

## Understanding Check

Before moving on, make sure you understand:

1. **What makes Perlin noise different from random()?**
   - Perlin is smooth and continuous; random is discontinuous

2. **What is the permutation table for?**
   - Maps grid coordinates to pseudo-random gradient vectors deterministically

3. **Why do we need a fade function?**
   - Linear interpolation creates visible grid artifacts
   - Fade creates smooth, artifact-free transitions

4. **What does the gradient do?**
   - Defines the direction of change at each grid point
   - Dot product with distance vector gives influence

5. **Why is noise deterministic?**
   - Same input always gives same output
   - Essential for consistent terrain/textures across frames

## Challenges

Want to experiment? Try these:

### Challenge 1: Adjustable Scale

Add a keyboard control to adjust noise scale in real-time:
- Key 'Z': Decrease scale (larger features)
- Key 'X': Increase scale (smaller features)

### Challenge 2: Multiple Octaves (Preview of Step 25)

Add multiple noise samples at different frequencies:
```rust
let mut height = 0.0;
height += perlin.noise2d(x * 1.0, y * 1.0) * 1.0;   // Base octave
height += perlin.noise2d(x * 2.0, y * 2.0) * 0.5;   // Finer detail
height += perlin.noise2d(x * 4.0, y * 4.0) * 0.25;  // Even finer
height /= 1.75;  // Normalize
```

### Challenge 3: Color by Height

Instead of grayscale, color the terrain:
- Blue: Low (water)
- Green: Medium (grass)
- Brown: High (mountains)
- White: Very high (snow)

### Challenge 4: Animated Noise

Add a time offset to see the noise pattern flow:
```rust
let height = perlin.noise2d(x + time * 0.1, y);
```

### Challenge 5: Domain Warping

Use noise to distort the input coordinates:
```rust
let offset_x = perlin.noise2d(x, y) * 0.5;
let offset_y = perlin.noise2d(x + 5.3, y + 2.7) * 0.5;
let height = perlin.noise2d(x + offset_x, y + offset_y);
```

This creates more organic, natural-looking patterns.

## What You've Learned

In this step, you've learned:

-  What Perlin noise is and why it's essential for procedural generation
-  How gradient-based noise works (permutation, fade, interpolation)
-  How to implement a seeded pseudo-random number generator
-  The mathematics of smooth interpolation (fade function)
-  How to visualize noise as a 3D mesh
-  The importance of deterministic algorithms for consistent results
-  How scale/frequency affects noise appearance

## Next Steps

In **Step 25: Fractal Noise**, you will:
- Combine multiple octaves of Perlin noise
- Learn about **persistence** and **lacunarity**
- Create more complex, natural-looking patterns
- Generate realistic terrain heightmaps
- Understand fractal composition techniques

---

**Ready to generate your first procedural patterns?** Implement the Perlin noise system and watch smooth, organic noise emerge from pure mathematics!

When you're done, let me know and I'll review your implementation!
