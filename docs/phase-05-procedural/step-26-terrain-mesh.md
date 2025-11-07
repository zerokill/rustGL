# Step 26: Terrain Mesh

**Phase 5: Procedural Generation** | **Estimated Time:** 2-3 hours

## Goals

In this step, you will:
- Create a dedicated **Terrain** struct for large-scale terrain generation
- Generate **larger terrain meshes** (256�256 or more vertices)
- Implement **height sampling** for physics and collision detection
- Store terrain parameters for **regeneration** and **modification**
- Optimize terrain mesh generation for performance
- Integrate terrain properly into your scene system

## Why a Dedicated Terrain Struct?

Right now you're using `Mesh::noise_test_plane()` for terrain, which works but has limitations:

**Current limitations:**
- L Can't query terrain height at arbitrary positions (needed for collision)
- L No way to modify or regenerate terrain
- L Parameters not stored (octaves, persistence, etc.)
- L Not optimized for large terrain meshes
- L Generic mesh doesn't capture terrain-specific needs

**With a Terrain struct:**
-  Can sample height at any (x, z) position
-  Store noise parameters for regeneration
-  Easy to update/modify terrain dynamically
-  Terrain-specific optimizations
-  Foundation for features like collision, physics, LOD

## Terrain Architecture

```
Terrain struct:
  - width, depth: Terrain dimensions
  - resolution: Vertex density (e.g., 256�256)
  - noise_seed: For reproducible generation
  - octaves, persistence, lacunarity, scale: Fractal noise params
  - heights: 2D array of height values (for collision)
  - mesh: The renderable Mesh

Methods:
  - new(): Create terrain with parameters
  - generate_mesh(): Build the mesh from noise
  - sample_height(x, z): Get height at position
  - regenerate(): Rebuild mesh with new parameters
```

## Current State Check

 **Already implemented**:
- `PerlinNoise` with `fractal_noise()` (noise.rs)
- `Mesh` with indexed rendering (mesh.rs)
- Scene system with `add_object()` (scene.rs)
- Transform for positioning and scaling
- Fractal noise working from Step 25

L **Still needed**:
1. Create `Terrain` struct
2. Implement terrain mesh generation
3. Store height data for sampling
4. Add height sampling method
5. Integrate terrain into scene
6. Test with larger terrain sizes

## Tasks

### Task 1: Create Terrain Module

Create a new file for terrain-specific code.

**Create `rustgl/src/terrain.rs`:**

```rust
use crate::mesh::Mesh;
use crate::noise::PerlinNoise;
use crate::mesh::Vertex;

/// Represents a procedurally generated terrain
pub struct Terrain {
    /// Width of terrain in world units
    pub width: f32,

    /// Depth of terrain in world units
    pub depth: f32,

    /// Number of vertices along width (actual vertices = resolution + 1)
    pub resolution_x: usize,

    /// Number of vertices along depth (actual vertices = resolution + 1)
    pub resolution_z: usize,

    /// Fractal noise parameters
    pub noise_seed: u32,
    pub octaves: u32,
    pub persistence: f32,
    pub lacunarity: f32,
    pub noise_scale: f32,

    /// Height scale multiplier
    pub height_scale: f32,

    /// Stored height values for collision detection [z][x]
    heights: Vec<Vec<f32>>,
}

impl Terrain {
    /// Create a new terrain with specified parameters
    pub fn new(
        width: f32,
        depth: f32,
        resolution_x: usize,
        resolution_z: usize,
        noise_seed: u32,
        octaves: u32,
        persistence: f32,
        lacunarity: f32,
        noise_scale: f32,
        height_scale: f32,
    ) -> Self {
        Terrain {
            width,
            depth,
            resolution_x,
            resolution_z,
            noise_seed,
            octaves,
            persistence,
            lacunarity,
            noise_scale,
            height_scale,
            heights: Vec::new(),
        }
    }

    /// Create a terrain with sensible defaults
    pub fn with_defaults(width: f32, depth: f32, resolution: usize) -> Self {
        Self::new(
            width,
            depth,
            resolution,
            resolution,
            42,          // seed
            5,           // octaves
            0.5,         // persistence
            2.0,         // lacunarity
            0.3,         // noise_scale
            1.0,         // height_scale
        )
    }
}
```

**Add to `main.rs`:**
```rust
mod terrain;  // Add with other mod declarations
use terrain::Terrain;  // Add with other use statements
```

### Task 2: Implement Height Data Generation

Add the method that generates terrain heights from noise.

**Add to `impl Terrain` in `rustgl/src/terrain.rs`:**

```rust
    /// Generate terrain height data from noise
    pub fn generate(&mut self) {
        let perlin = PerlinNoise::new(self.noise_seed);

        // Initialize heights storage
        self.heights = vec![vec![0.0; self.resolution_x + 1]; self.resolution_z + 1];

        let step_x = self.width / self.resolution_x as f32;
        let step_z = self.depth / self.resolution_z as f32;

        // Generate height data
        for z in 0..=self.resolution_z {
            for x in 0..=self.resolution_x {
                // World position
                let world_x = (x as f32 * step_x) - (self.width / 2.0);
                let world_z = (z as f32 * step_z) - (self.depth / 2.0);

                // Sample fractal noise at this position
                let noise_value = perlin.fractal_noise(
                    world_x * self.noise_scale,
                    world_z * self.noise_scale,
                    self.octaves,
                    self.persistence,
                    self.lacunarity,
                );

                let height = noise_value * self.height_scale;

                // Store height for collision detection
                self.heights[z][x] = height;
            }
        }
    }

    /// Create a mesh from the current height data
    /// This is called when adding terrain to the scene
    pub fn create_mesh(&self) -> Mesh {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let step_x = self.width / self.resolution_x as f32;
        let step_z = self.depth / self.resolution_z as f32;

        // Generate vertices from stored heights
        for z in 0..=self.resolution_z {
            for x in 0..=self.resolution_x {
                // World position
                let world_x = (x as f32 * step_x) - (self.width / 2.0);
                let world_z = (z as f32 * step_z) - (self.depth / 2.0);

                // Get height from stored data
                let height = self.heights[z][x];

                // Color based on height
                let color_value = (height / self.height_scale + 1.0) * 0.5; // Map back to 0..1
                let color = [
                    color_value * 0.5 + 0.2,  // R: brownish
                    color_value * 0.7 + 0.3,  // G: greenish
                    color_value * 0.3 + 0.1,  // B: less blue
                ];

                // Temporary normal (will be calculated properly in Step 27)
                let normal = [0.0, 1.0, 0.0];

                // UV coordinates
                let u = x as f32 / self.resolution_x as f32;
                let v = z as f32 / self.resolution_z as f32;

                vertices.push(Vertex::new(
                    [world_x, height, world_z],
                    color,
                    normal,
                    [u, v],
                ));
            }
        }

        // Generate indices for triangles
        for z in 0..self.resolution_z {
            for x in 0..self.resolution_x {
                let top_left = (z * (self.resolution_x + 1) + x) as u32;
                let top_right = top_left + 1;
                let bottom_left = ((z + 1) * (self.resolution_x + 1) + x) as u32;
                let bottom_right = bottom_left + 1;

                // First triangle (counter-clockwise winding)
                indices.push(top_left);
                indices.push(bottom_left);
                indices.push(top_right);

                // Second triangle (counter-clockwise winding)
                indices.push(top_right);
                indices.push(bottom_left);
                indices.push(bottom_right);
            }
        }

        Mesh::new_indexed(&vertices, &indices)
    }
```

**What's happening:**
- **`generate()`**: Only generates and stores height data
  - Samples noise for each grid point
  - Stores heights in 2D array
  - Fast, lightweight operation

- **`create_mesh()`**: Generates a fresh Mesh on demand
  - Reads from stored height data
  - Creates vertices and indices
  - Returns ownership of Mesh to caller
  - Can be called multiple times if needed

### Task 3: Implement Height Sampling

Add a method to query terrain height at any world position.

**Add to `impl Terrain` in `rustgl/src/terrain.rs`:**

```rust
    /// Sample the terrain height at a world position (x, z)
    /// Returns None if position is outside terrain bounds
    pub fn sample_height(&self, world_x: f32, world_z: f32) -> Option<f32> {
        // Convert world position to local terrain coordinates
        let local_x = world_x + (self.width / 2.0);
        let local_z = world_z + (self.depth / 2.0);

        // Check bounds
        if local_x < 0.0 || local_x > self.width || local_z < 0.0 || local_z > self.depth {
            return None;
        }

        // Convert to grid coordinates
        let grid_x = local_x / (self.width / self.resolution_x as f32);
        let grid_z = local_z / (self.depth / self.resolution_z as f32);

        // Get integer grid cell
        let x0 = grid_x.floor() as usize;
        let z0 = grid_z.floor() as usize;
        let x1 = (x0 + 1).min(self.resolution_x);
        let z1 = (z0 + 1).min(self.resolution_z);

        // Get fractional part for interpolation
        let fx = grid_x - x0 as f32;
        let fz = grid_z - z0 as f32;

        // Bilinear interpolation of height values
        let h00 = self.heights[z0][x0];
        let h10 = self.heights[z0][x1];
        let h01 = self.heights[z1][x0];
        let h11 = self.heights[z1][x1];

        // Interpolate along x
        let h0 = h00 * (1.0 - fx) + h10 * fx;
        let h1 = h01 * (1.0 - fx) + h11 * fx;

        // Interpolate along z
        let height = h0 * (1.0 - fz) + h1 * fz;

        Some(height)
    }
```

**What's happening:**
- **Convert coordinates**: World � Local � Grid
- **Bounds checking**: Return None if outside terrain
- **Bilinear interpolation**: Smooth height between grid points
- **Returns exact height** at any position, not just grid vertices

This is crucial for:
- Physics: Keep objects above terrain
- Collision: Detect when objects hit ground
- AI: Pathfinding on terrain
- Placement: Put objects on terrain surface

### Task 4: Add Utility Methods

**Add to `impl Terrain` in `rustgl/src/terrain.rs`:**

```rust
    /// Get terrain dimensions
    pub fn dimensions(&self) -> (f32, f32) {
        (self.width, self.depth)
    }

    /// Regenerate terrain with current parameters
    pub fn regenerate(&mut self) {
        self.generate();
    }

    /// Update noise parameters and regenerate
    pub fn set_noise_params(&mut self, octaves: u32, persistence: f32, lacunarity: f32, scale: f32) {
        self.octaves = octaves;
        self.persistence = persistence;
        self.lacunarity = lacunarity;
        self.noise_scale = scale;
        self.regenerate();
    }
```

### Task 5: Create Terrain and Add to Scene

Now let's use the Terrain struct in your main application. This is clean because terrain generates heights once, then can create meshes on demand for the scene.

**In `main.rs`, replace the fractal_terrain code (around lines 169-282) with:**

```rust
// Create terrain with parameters
let mut terrain = Terrain::with_defaults(100.0, 100.0, 128);
terrain.height_scale = 10.0;  // Dramatic elevation
terrain.generate();

// Test terrain height sampling
println!("=== Terrain Height Sampling Tests ===");
println!("Terrain height at (0, 0): {:?}", terrain.sample_height(0.0, 0.0));
println!("Terrain height at (10, 10): {:?}", terrain.sample_height(10.0, 10.0));
println!("Terrain height at (-25, 30): {:?}", terrain.sample_height(-25.0, 30.0));
println!("Terrain height outside bounds: {:?}", terrain.sample_height(1000.0, 1000.0));
println!("======================================\n");
```

**Then, create the scene and add objects AFTER terrain:**

```rust
let mut scene = Scene::new();

// Set up skybox (keep existing code)
// ... skybox code ...

// Add ground plane (keep existing if you want it, or remove)
// ... ground plane code ...

// Add other objects (spheres, cubes, etc. - keep existing code)
// ... other objects ...
```

**Now add terrain to scene using the clean `create_mesh()` approach:**

```rust
// Add terrain to scene
// Call create_mesh() to generate a fresh mesh that the scene will own
// Terrain keeps the height data for collision queries
scene.add_object(
    terrain.create_mesh(),  // Generate mesh on demand
    Material::matte(glm::vec3(0.4, 0.6, 0.3)),
    Transform::from_position(glm::vec3(0.0, 0.0, 0.0))
);
```

**That's it!** Clean and simple:
- Terrain generates heights once with `generate()`
- Terrain creates a mesh when needed with `create_mesh()`
- Scene takes ownership of the mesh
- Terrain keeps height data for `sample_height()` queries
- If you need to regenerate (e.g., user changes parameters), call `terrain.regenerate()` and then add a new mesh to the scene

## Success Criteria

You have completed this step when:

-  `Terrain` struct created in terrain.rs
-  Terrain stores width, depth, resolution, and noise parameters
-  `generate()` method creates mesh from fractal noise
-  Height values stored in 2D array
-  `sample_height()` returns correct heights with bilinear interpolation
-  Terrain integrated into scene system
-  Can create larger terrains (128�128 or 256�256)
-  Height sampling test output shows correct values
-  No compilation errors
-  Terrain renders correctly

## Testing

Run your program and verify:

1. **Console output**: You should see height sampling test results:
   ```
   === Terrain Height Sampling Tests ===
   Terrain height at (0, 0): Some(4.523)
   Terrain height at (10, 10): Some(6.821)
   Terrain height at (-25, 30): Some(2.145)
   Terrain height outside bounds: None
   ======================================
   ```

2. **Larger terrain**: You should see a bigger, more detailed terrain

3. **Performance**: 128�128 (16,641 vertices, 32,768 triangles) should run smoothly
   - If smooth, try 256�256 (65,536 vertices, 131,072 triangles)

4. **Visual quality**: More detail visible with higher resolution

5. **Navigate terrain**: Fly around and observe the landscape

## Common Issues

### Issue 1: Terrain appears flat

**Problem:** `height_scale` too low or noise_scale too high.

**Solution:**
- Increase `height_scale` to 10.0 or higher
- Decrease `noise_scale` to 0.2-0.4 for larger features

### Issue 2: Terrain too spiky/noisy

**Problem:** Too many octaves or persistence too high.

**Solution:**
- Reduce octaves to 3-4
- Try `persistence = 0.4`

### Issue 3: Performance is slow

**Problem:** Resolution too high for your hardware.

**Solution:**
- Start with 64�64 resolution
- Increase to 128�128 if smooth
- Only use 256�256 on powerful hardware

### Issue 4: Height sampling returns None everywhere

**Problem:** Coordinate conversion issue or bounds checking too strict.

**Solution:**
- Print `local_x`, `local_z` values to debug
- Check that terrain width/depth matches expected values
- Verify world coordinates are within [-width/2, width/2]

### Issue 5: Height values don't match visual terrain

**Problem:** Different mesh generation between terrain and scene.

**Solution:**
- Make sure you're using the same noise parameters for both
- Verify height_scale is applied consistently

## Understanding Check

Before moving on, make sure you understand:

1. **Why do we store heights separately?**
   - For collision detection without accessing mesh vertices
   - Fast height lookups during gameplay
   - Physics calculations

2. **What is bilinear interpolation?**
   - Smoothly interpolate between 4 corner values
   - Gives accurate height between grid vertices
   - Prevents "stepping" artifacts

3. **Why make terrain larger than before?**
   - More realistic game worlds
   - Room for exploration
   - Better sense of scale

4. **What's the performance trade-off?**
   - Higher resolution = more detail but slower
   - 64�64 = 4,225 vertices (fast)
   - 128�128 = 16,641 vertices (good balance)
   - 256�256 = 65,536 vertices (detailed, may be slow)

5. **Why store terrain parameters?**
   - Can regenerate terrain with tweaked parameters
   - Reproducible terrain from seed
   - Modify terrain dynamically in real-time

## Challenges

Want to experiment? Try these:

### Challenge 1: Height-Based Coloring

Improve the terrain coloring based on elevation:

```rust
let color = if height < 0.0 {
    [0.1, 0.3, 0.8]  // Water (blue)
} else if height < 2.0 {
    [0.3, 0.7, 0.3]  // Grass (green)
} else if height < 5.0 {
    [0.5, 0.4, 0.3]  // Rock (brown)
} else {
    [0.9, 0.9, 1.0]  // Snow (white)
};
```

### Challenge 2: UI Controls for Terrain

Add egui sliders to modify and regenerate terrain:

```rust
ui.heading("Terrain Controls");
ui.add(egui::Slider::new(&mut terrain.octaves, 1..=8).text("Octaves"));
ui.add(egui::Slider::new(&mut terrain.persistence, 0.1..=0.9).text("Persistence"));
ui.add(egui::Slider::new(&mut terrain.height_scale, 1.0..=20.0).text("Height Scale"));
if ui.button("Regenerate Terrain").clicked() {
    terrain.regenerate();
    // Need to update scene mesh too
}
```

### Challenge 3: Multiple Terrain Chunks

Create a grid of terrain chunks for a larger world:

```rust
for chunk_z in -1..=1 {
    for chunk_x in -1..=1 {
        let mut chunk = Terrain::with_defaults(50.0, 50.0, 64);
        chunk.generate();
        // Add to scene with offset
    }
}
```

### Challenge 4: Terrain Query Visualization

Add a small sphere that follows the terrain height at the camera's XZ position:

```rust
if let Some(height) = terrain.sample_height(camera.position.x, camera.position.z) {
    // Add indicator sphere at (camera.x, height, camera.z)
}
```

### Challenge 5: Different Terrain Presets

Create factory methods for different biomes:

```rust
impl Terrain {
    pub fn mountains(width: f32, depth: f32, resolution: usize) -> Self {
        let mut terrain = Self::with_defaults(width, depth, resolution);
        terrain.octaves = 7;
        terrain.persistence = 0.6;
        terrain.height_scale = 20.0;
        terrain
    }

    pub fn plains(width: f32, depth: f32, resolution: usize) -> Self {
        let mut terrain = Self::with_defaults(width, depth, resolution);
        terrain.octaves = 3;
        terrain.persistence = 0.3;
        terrain.height_scale = 2.0;
        terrain
    }
}
```

## What You've Learned

In this step, you've learned:

-  How to structure terrain as a dedicated game object
-  Storing height data for non-rendering purposes
-  Bilinear interpolation for smooth height queries
-  Generating large, detailed terrain meshes efficiently
-  Managing parameters for dynamic terrain regeneration
-  The foundation for collision detection and physics
-  Performance considerations for large meshes
-  Integrating complex procedural systems into scenes

## Next Steps

In **Step 27: Terrain Normals**, you will:
- Calculate **proper normals** from neighboring vertices
- Understand **cross products** for normal calculation
- Implement **smooth vs flat shading**
- Fix terrain lighting to look realistic
- Make your terrain beautifully lit with correct shadows!

---

**Ready to build large-scale procedural worlds?** Implement the Terrain struct and create the foundation for realistic game environments!

When you're done, let me know and I'll review your implementation!
