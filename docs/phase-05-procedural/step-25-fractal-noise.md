# Step 25: Fractal Noise

**Phase 5: Procedural Generation** | **Estimated Time:** 1-2 hours

## Goals

In this step, you will:
- Understand **fractal noise** (also called **fBm** - fractal Brownian motion)
- Learn about **octaves**, **persistence**, and **lacunarity**
- Implement multi-octave noise by combining multiple frequencies
- Create more **natural, detailed terrain** with both large features and fine detail
- Add controls to adjust noise parameters in real-time

## What is Fractal Noise?

**Single-octave Perlin noise** looks smooth but somewhat boring:
-  Smooth gradients
- L Lacks detail at different scales
- L Looks too uniform and artificial

**Fractal noise** combines **multiple octaves** (frequencies) of noise:
-  Large hills (low frequency)
-  Medium bumps (medium frequency)
-  Fine detail (high frequency)
-  Looks **natural** like real terrain

### Real-World Analogy

Think of a mountain range:
1. **Large mountains** - low frequency, large amplitude (octave 1)
2. **Hills on the mountains** - medium frequency, medium amplitude (octave 2)
3. **Rocks and bumps** - high frequency, small amplitude (octave 3)
4. **Pebbles and texture** - very high frequency, tiny amplitude (octave 4)

Each octave adds detail at a different scale!

## Key Concepts

### 1. Octaves
The **number of noise layers** to combine. More octaves = more detail (but slower).
- **1 octave**: Smooth, simple
- **4 octaves**: Natural terrain
- **8 octaves**: Very detailed, expensive

### 2. Frequency
How "zoomed in" the noise is. Higher frequency = smaller features.
- Each octave typically **doubles** the frequency
- Controlled by the **lacunarity** parameter

### 3. Amplitude
How "tall" the noise features are. Lower amplitude for fine detail.
- Each octave typically **halves** the amplitude
- Controlled by the **persistence** parameter

### 4. Persistence (Amplitude Falloff)
How much each octave contributes compared to the previous one.
- **0.5**: Each octave is half the amplitude of the previous (common)
- **0.3**: Very smooth, large features dominate
- **0.7**: Very detailed, small features more prominent

### 5. Lacunarity (Frequency Multiplier)
How much the frequency increases per octave.
- **2.0**: Each octave doubles the frequency (standard)
- **1.5**: Slower frequency increase, smoother
- **3.0**: Faster frequency increase, more chaotic

## The Fractal Noise Formula

```
total = 0
amplitude = 1.0
frequency = 1.0

for each octave:
    total += noise(x * frequency, y * frequency) * amplitude
    amplitude *= persistence
    frequency *= lacunarity

return total / normalization_factor
```

## Current State Check

 **Already implemented**:
- `PerlinNoise` struct with `noise2d()` function (noise.rs)
- `noise_test_plane()` mesh generation (mesh.rs)
- Scene system for adding objects (scene.rs)
- Transform system for positioning and scaling

L **Still needed**:
1. Add fractal noise parameters to `PerlinNoise`
2. Implement multi-octave noise function
3. Create fractal terrain visualization
4. Add to scene with proper Transform

## Tasks

### Task 1: Add Fractal Noise Method to PerlinNoise

Add a new method that combines multiple octaves of noise.

**Add to `impl PerlinNoise` in `rustgl/src/noise.rs`:**

```rust
    /// Generate fractal noise (fBm) by combining multiple octaves
    ///
    /// # Parameters
    /// - `x, y`: Sample position
    /// - `octaves`: Number of noise layers (typically 4-8)
    /// - `persistence`: Amplitude multiplier per octave (typically 0.5)
    /// - `lacunarity`: Frequency multiplier per octave (typically 2.0)
    ///
    /// # Returns
    /// Noise value in approximate range [-1.0, 1.0]
    pub fn fractal_noise(
        &self,
        x: f32,
        y: f32,
        octaves: u32,
        persistence: f32,
        lacunarity: f32,
    ) -> f32 {
        let mut total = 0.0;
        let mut amplitude = 1.0;
        let mut frequency = 1.0;
        let mut max_value = 0.0; // Used for normalizing result to [-1.0, 1.0]

        for _ in 0..octaves {
            // Sample noise at this frequency
            total += self.noise2d(x * frequency, y * frequency) * amplitude;

            // Track maximum possible value for normalization
            max_value += amplitude;

            // Decrease amplitude and increase frequency for next octave
            amplitude *= persistence;
            frequency *= lacunarity;
        }

        // Normalize to approximately [-1.0, 1.0]
        total / max_value
    }
```

**What's happening:**
- **Loop through octaves**: Each iteration adds a layer of detail
- **Scale by frequency**: Higher octaves sample the noise at higher frequencies (zoomed in)
- **Scale by amplitude**: Higher octaves have less influence (smaller bumps)
- **Normalize**: Divide by max_value to keep output in roughly [-1, 1] range
- **Accumulate**: Add all octaves together for the final result

### Task 2: Add Convenience Method for Positive Range

Just like `noise2d_01()`, add a version that returns [0, 1]:

**Add to `impl PerlinNoise` in `rustgl/src/noise.rs`:**

```rust
    /// Generate fractal noise in the range [0.0, 1.0]
    pub fn fractal_noise_01(
        &self,
        x: f32,
        y: f32,
        octaves: u32,
        persistence: f32,
        lacunarity: f32,
    ) -> f32 {
        (self.fractal_noise(x, y, octaves, persistence, lacunarity) + 1.0) * 0.5
    }
```

### Task 3: Create Fractal Terrain Test Plane

Now let's create a new terrain using fractal noise and add it to the scene.

**In `main.rs`, replace or update the noise plane creation (around line 169):**

```rust
// Create Perlin noise generator
let perlin = PerlinNoise::new(42);

// Fractal noise parameters
let noise_scale = 0.3;      // Overall frequency (lower = larger features)
let octaves = 5;             // Number of detail layers
let persistence = 0.5;       // Amplitude falloff (0.5 = each octave is half as strong)
let lacunarity = 2.0;        // Frequency multiplier (2.0 = each octave is twice the frequency)

// Create fractal terrain plane
let fractal_terrain = Mesh::noise_test_plane(
    &|x, y| perlin.fractal_noise(x, y, octaves, persistence, lacunarity),
    100,  // 100x100 grid
    noise_scale,
);
```

**Then add it to your scene (replace the old noise_plane object or add as new):**

```rust
// Add fractal terrain to scene
scene.add_object(
    fractal_terrain,
    Material::matte(glm::vec3(0.4, 0.6, 0.3)),  // Terrain green
    Transform::from_position_scale(
        glm::vec3(0.0, -1.0, 0.0),      // Slightly below origin
        glm::vec3(8.0, 3.0, 8.0)        // Larger scale for dramatic terrain
    )
);
```

**What's happening:**
- Uses `fractal_noise()` instead of `noise2d()`
- 5 octaves creates nice detail without being too expensive
- `persistence = 0.5` is a natural-looking falloff
- `lacunarity = 2.0` doubles frequency each octave (standard)
- Larger scale (8x in XZ, 3x in Y) makes the terrain more dramatic

### Task 4: Compare Single vs Fractal Noise (Optional)

To really see the difference, create both terrains side-by-side:

```rust
// Simple Perlin terrain (left side)
let simple_terrain = Mesh::noise_test_plane(
    &|x, y| perlin.noise2d(x, y),
    100,
    0.5,
);

scene.add_object(
    simple_terrain,
    Material::matte(glm::vec3(0.6, 0.4, 0.3)),  // Brown
    Transform::from_position_scale(
        glm::vec3(-10.0, -1.0, 0.0),     // Left side
        glm::vec3(5.0, 2.0, 5.0)
    )
);

// Fractal terrain (right side)
let fractal_terrain = Mesh::noise_test_plane(
    &|x, y| perlin.fractal_noise(x, y, 5, 0.5, 2.0),
    100,
    0.3,
);

scene.add_object(
    fractal_terrain,
    Material::matte(glm::vec3(0.4, 0.6, 0.3)),  // Green
    Transform::from_position_scale(
        glm::vec3(10.0, -1.0, 0.0),      // Right side
        glm::vec3(5.0, 2.0, 5.0)
    )
);
```

Now you can fly between them and see the difference!

## Success Criteria

You have completed this step when:

-  `fractal_noise()` method implemented in `noise.rs`
-  Method accepts octaves, persistence, and lacunarity parameters
-  Fractal terrain displays with multiple levels of detail
-  Terrain added to scene using `scene.add_object()`
-  Transform properly positions and scales the terrain
-  Terrain looks more natural than single-octave noise
-  Can see both large features and fine detail
-  No compilation errors
-  Scene renders correctly

## Testing

Run your program and observe:

1. **Multiple scales of detail**: You should see:
   - Large rolling hills (low frequency)
   - Medium-sized bumps on the hills (medium frequency)
   - Fine texture and detail (high frequency)

2. **Natural appearance**: Should look like real terrain, not artificial

3. **Compare different parameters**:

   Try changing octaves:
   - `octaves = 1`: Smooth, boring (same as Step 24)
   - `octaves = 3`: Some detail
   - `octaves = 5`: Good balance
   - `octaves = 8`: Very detailed (may be slow)

   Try changing persistence:
   - `persistence = 0.3`: Very smooth, large features dominate
   - `persistence = 0.5`: Balanced (natural)
   - `persistence = 0.7`: Very rough, small features prominent

   Try changing lacunarity:
   - `lacunarity = 1.5`: Gentle frequency increase
   - `lacunarity = 2.0`: Standard doubling
   - `lacunarity = 3.0`: Aggressive, chaotic detail

4. **Performance**: More octaves = slower, but should still be real-time

## Common Issues

### Issue 1: Terrain too flat or too spiky

**Problem:** Amplitude scaling not right.

**Solution:**
- Adjust the Y scale in Transform: `glm::vec3(8.0, 3.0, 8.0)`
- Lower Y scale (e.g., 2.0) = flatter
- Higher Y scale (e.g., 5.0) = more dramatic

### Issue 2: Looks the same as single octave

**Problem:** Octaves set to 1, or persistence too low.

**Solution:**
- Make sure `octaves >= 3`
- Try `persistence = 0.5` for visible detail layers

### Issue 3: Too much detail, looks noisy

**Problem:** Too many octaves or persistence too high.

**Solution:**
- Reduce octaves to 4-5
- Try `persistence = 0.4` for smoother terrain

### Issue 4: Terrain appears too small/large

**Problem:** Incorrect frequency scale.

**Solution:**
- Adjust `noise_scale`: Lower = larger features, Higher = smaller features
- Try values between 0.1 (very large) and 1.0 (small features)

### Issue 5: Performance is slow

**Problem:** Too many octaves or grid resolution too high.

**Solution:**
- Reduce octaves from 8 to 4-5
- Reduce grid size from 100 to 50 (faster, but less smooth)

## Understanding Check

Before moving on, make sure you understand:

1. **What is an octave?**
   - A layer of noise at a specific frequency and amplitude

2. **Why do we need multiple octaves?**
   - To add detail at different scales for natural-looking terrain
   - Single octave looks too smooth and artificial

3. **What does persistence control?**
   - How much each octave contributes (amplitude falloff)
   - Lower = smoother, higher = more detailed

4. **What does lacunarity control?**
   - How much the frequency increases per octave
   - 2.0 means each octave is twice the frequency

5. **Why normalize by max_value?**
   - To keep the output in a consistent range [-1, 1]
   - Without it, more octaves would make values grow unbounded

6. **How does fractal noise create natural terrain?**
   - Combines large features (mountains) with small details (rocks)
   - Mimics how nature creates terrain at multiple scales

## Challenges

Want to experiment? Try these:

### Challenge 1: Adjustable Parameters

Add variables to store noise parameters and create UI sliders to adjust them in real-time:

```rust
// In AppState struct:
noise_octaves: u32,
noise_persistence: f32,
noise_lacunarity: f32,
noise_scale: f32,

// In render_ui():
ui.add(egui::Slider::new(&mut state.noise_octaves, 1..=8).text("Octaves"));
ui.add(egui::Slider::new(&mut state.noise_persistence, 0.1..=0.9).text("Persistence"));
ui.add(egui::Slider::new(&mut state.noise_lacunarity, 1.0..=4.0).text("Lacunarity"));
ui.add(egui::Slider::new(&mut state.noise_scale, 0.1..=2.0).text("Scale"));
```

Then regenerate the terrain when parameters change!

### Challenge 2: Ridged Noise

Create "ridged" terrain (like mountain ridges) by using the absolute value:

```rust
total += (1.0 - self.noise2d(x * frequency, y * frequency).abs()) * amplitude;
```

This inverts valleys into peaks, creating sharp ridges.

### Challenge 3: Turbulence

Use the absolute value of noise for "turbulent" patterns:

```rust
total += self.noise2d(x * frequency, y * frequency).abs() * amplitude;
```

Great for clouds, marble, or fire effects.

### Challenge 4: Domain Warping with Fractals

Combine fractal noise with domain warping:

```rust
let offset_x = perlin.fractal_noise(x, y, 3, 0.5, 2.0);
let offset_y = perlin.fractal_noise(x + 5.3, y + 2.7, 3, 0.5, 2.0);
let height = perlin.fractal_noise(x + offset_x, y + offset_y, 5, 0.5, 2.0);
```

Creates very organic, flowing terrain.

### Challenge 5: Terraced Terrain

Quantize the height values to create terraced/stepped terrain:

```rust
let height = perlin.fractal_noise(x, y, 5, 0.5, 2.0);
let terraced = (height * 10.0).floor() / 10.0;  // 10 terraces
```

Great for stylized or low-poly aesthetic.

### Challenge 6: Island Generation

Use distance from center to create islands:

```rust
let height = perlin.fractal_noise(x, y, 5, 0.5, 2.0);
let distance = (x * x + y * y).sqrt();
let island_mask = (1.0 - distance / 5.0).max(0.0);
let island_height = height * island_mask;
```

Creates circular islands with beaches!

## What You've Learned

In this step, you've learned:

-  What fractal noise (fBm) is and why it's essential for natural terrain
-  How octaves combine to create multi-scale detail
-  The role of persistence in controlling detail prominence
-  The role of lacunarity in controlling frequency scaling
-  How to implement multi-octave noise combination
-  How to normalize fractal noise output
-  The difference between single-octave and fractal terrain
-  How to integrate procedural terrain into your scene system
-  Parameter tuning for different terrain styles

## Next Steps

In **Step 26: Terrain Mesh**, you will:
- Create a dedicated `Terrain` struct for large-scale terrain
- Generate massive terrain meshes (e.g., 256x256 or larger)
- Optimize terrain rendering
- Add proper height sampling for physics and collision
- Prepare for dynamic terrain updates

---

**Ready to create natural, detailed procedural terrain?** Implement fractal noise and watch your terrain come alive with realistic detail at multiple scales!

When you're done, let me know and I'll review your implementation!
