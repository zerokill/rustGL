use crate::mesh::Mesh;
use crate::mesh::Vertex;
use crate::noise::PerlinNoise;

pub struct Terrain {
    // width of terrain in world units
    pub width: f32,

    // depth of terrain in world units
    pub depth: f32,

    // Number of vertices along width (actual vertices = resolution + 1)
    pub resolution_x: usize,

    // Number of vertices along depth (actual vertices = resolution + 1)
    pub resolution_z: usize,

    // Fractal noise parameters
    pub noise_seed: u32,
    pub octaves: u32,
    pub persistence: f32,
    pub lacunarity: f32,
    pub noise_scale: f32,

    // Height scale multiplier
    pub height_scale: f32,

    // Stored heights for collision detection [z][x]
    heights: Vec<Vec<f32>>,
}

impl Terrain {
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

    // Create terrain with sensible defaults
    pub fn with_defaults(width: f32, depth: f32, resolution: usize) -> Self {
        Self::new(
            width, depth, resolution, resolution, 42,  // seed
            5,   // octaves
            0.5, // persistence
            2.0, // lacunarity
            0.3, // noise_scale
            1.0, // height_scale
        )
    }

    pub fn generate(&mut self) {
        let perlin = PerlinNoise::new(self.noise_seed);

        // Initiate height storage
        self.heights = vec![vec![0.0; self.resolution_x + 1]; self.resolution_z + 1];

        let step_x = self.width / self.resolution_x as f32;
        let step_z = self.depth / self.resolution_z as f32;

        // generate height data
        for z in 0..=self.resolution_z {
            for x in 0..=self.resolution_x {
                // world position
                let world_x = (x as f32 * step_x) - (self.width / 2.0);
                let world_z = (z as f32 * step_z) - (self.depth / 2.0);

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

    // Create a mesh for the current height data
    // This is called when adding terrain to the scene
    pub fn create_mesh(&self) -> Mesh {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let step_x = self.width / self.resolution_x as f32;
        let step_z = self.depth / self.resolution_z as f32;

        // generate vertices from stored heights
        for z in 0..=self.resolution_z {
            for x in 0..=self.resolution_x {
                // world position
                let world_x = (x as f32 * step_x) - (self.width / 2.0);
                let world_z = (z as f32 * step_z) - (self.depth / 2.0);

                let height = self.heights[z][x];

                // Color based on height
                let color_value = (height / self.height_scale + 1.0) * 0.5; // map back to 0..1

                let color = [
                    color_value * 0.5 + 0.2, // R: brownish
                    color_value * 0.7 + 0.3, // G: greenish
                    color_value * 0.3 + 0.1, // B: less blue
                ];

                // Temporary normal.
                let normal = [0.0, 1.0, 0.0];

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

    pub fn sample_height(&self, world_x: f32, world_z: f32) -> Option<f32> {
        // Convert world position to local terrain coordinates
        let local_x = world_x + (self.width / 2.0);
        let local_z = world_z + (self.depth / 2.0);

        // check bounds
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

        // get fractional part for interpolation
        let fx = grid_x - x0 as f32;
        let fz = grid_z - z0 as f32;

        // Bilinear interpolation of height scales
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

    /// Get terrain dimensions
    pub fn dimensions(&self) -> (f32, f32) {
        (self.width, self.depth)
    }

    /// Regenerate terrain with current parameters
    pub fn regenerate(&mut self) {
        self.generate();
    }

    /// Update noise parameters and regenerate
    pub fn set_noise_params(
        &mut self,
        octaves: u32,
        persistence: f32,
        lacunarity: f32,
        scale: f32,
    ) {
        self.octaves = octaves;
        self.persistence = persistence;
        self.lacunarity = lacunarity;
        self.noise_scale = scale;
        self.regenerate();
    }
}
