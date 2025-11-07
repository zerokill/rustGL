use std::f32::consts::PI;

pub struct PerlinNoise {
    permutation: [u8; 512],
}

impl PerlinNoise {
    pub fn new(seed: u32) -> Self {
        let mut perm = [0u8; 256];

        // Initialize permutations table with values 0-255
        for i in 0..256 {
            perm[i] = i as u8;
        }

        // Shuffle using seed (simple LCG random)
        let mut rng_state = seed;
        for i in (1..256).rev() {
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

    fn fade(t: f32) -> f32 {
        t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
    }

    fn lerp(t: f32, a: f32, b: f32) -> f32 {
        a + t * (b - a)
    }

    fn grad2d(hash: u8, x: f32, y: f32) -> f32 {
        // use bottom 2 bits to select one of 4 gradiant directions
        let h = hash & 3;
        match h {
            0 => x + y,
            1 => -x + y,
            2 => x - y,
            _ => -x - y,
        }
    }

    pub fn noise2d(&self, x: f32, y: f32) -> f32 {
        // find unit grid cell containing the point
        let xi = (x.floor() as i32) & 255;
        let yi = (y.floor() as i32) & 255;

        // relative position within cell (0.0 to 1.0)
        let xf = x - x.floor();
        let yf = y - y.floor();

        let u = Self::fade(xf);
        let v = Self::fade(yf);

        // Hash coordinates of the 4 cube corners
        let aa = self.permutation[(self.permutation[xi as usize] as usize + yi as usize) % 512];
        let ab = self.permutation[(self.permutation[xi as usize] as usize + yi as usize + 1) % 512];
        let ba = self.permutation[(self.permutation[xi as usize + 1] as usize + yi as usize) % 512];
        let bb = self.permutation[(self.permutation[xi as usize + 1] as usize + yi as usize + 1) % 512];

        // Calculate gradiant influences at 4 corners
        let g1 = Self::grad2d(aa, xf, yf);
        let g2 = Self::grad2d(ba, xf - 1.0, yf);
        let g3 = Self::grad2d(ab, xf, yf - 1.0);
        let g4 = Self::grad2d(bb, xf - 1.0, yf - 1.0);

        // Interpolate horizontal
        let x1 = Self::lerp(u, g1, g2);
        let x2 = Self::lerp(u, g3, g4);

        // Interpolate vertical
        let result = Self::lerp(v, x1, x2);

        result
    }

    pub fn noise2d_01(&self, x: f32, y: f32) -> f32 {
        (self.noise2d(x, y) + 1.0) * 0.5
    }
}
