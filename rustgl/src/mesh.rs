use std::mem;
use std::ptr;

/// Represents a single vertex with position, color, normal, and UV coordinates
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3], // x, y, z
    pub color: [f32; 3],    // r, g, b
    pub normal: [f32; 3],   // nx, ny, nz
    pub uv: [f32; 2],       // u, v (texture coordinates)
}

impl Vertex {
    /// Creates a new vertex with position, color, normal, and UV coordinates
    pub fn new(position: [f32; 3], color: [f32; 3], normal: [f32; 3], uv: [f32; 2]) -> Self {
        Vertex {
            position,
            color,
            normal,
            uv,
        }
    }
}

/// A mesh holds vertex data and OpenGL buffer objects
pub struct Mesh {
    vao: u32,
    vbo: u32,
    ebo: Option<u32>,
    vertex_count: i32,
    index_count: i32,
}

impl Mesh {
    /// Creates a colored triangle mesh
    pub fn triangle(color: [f32; 3]) -> Self {
        let normal = [0.0, 0.0, 1.0]; // Facing camera
        let vertices = vec![
            Vertex::new([-0.5, -0.5, 0.0], color, normal, [0.0, 0.0]),
            Vertex::new([0.5, -0.5, 0.0], color, normal, [1.0, 0.0]),
            Vertex::new([0.0, 0.5, 0.0], color, normal, [0.5, 1.0]),
        ];
        Mesh::new(&vertices)
    }

    /// Creates a quad mesh using indexed rendering
    pub fn quad(color: [f32; 3]) -> Self {
        let normal = [0.0, 0.0, 1.0];
        let vertices = vec![
            Vertex::new([-0.5, -0.5, 0.0], color, normal, [0.0, 0.0]), // 0: Bottom left
            Vertex::new([0.5, -0.5, 0.0], color, normal, [1.0, 0.0]),  // 1: Bottom right
            Vertex::new([0.5, 0.5, 0.0], color, normal, [1.0, 1.0]),   // 2: Top right
            Vertex::new([-0.5, 0.5, 0.0], color, normal, [0.0, 1.0]),  // 3: Top left
        ];
        let indices = vec![
            0, 1, 2, // First triangle
            2, 3, 0, // Second triangle
        ];
        Mesh::new_indexed(&vertices, &indices)
    }

    /// Creates a gradient quad (different color per corner)
    pub fn quad_gradient() -> Self {
        let normal = [0.0, 0.0, 1.0];
        let vertices = vec![
            Vertex::new([-0.5, -0.5, 0.0], [1.0, 0.0, 0.0], normal, [0.0, 0.0]), // Red
            Vertex::new([0.5, -0.5, 0.0], [0.0, 1.0, 0.0], normal, [1.0, 0.0]),  // Green
            Vertex::new([0.5, 0.5, 0.0], [0.0, 0.0, 1.0], normal, [1.0, 1.0]),   // Blue
            Vertex::new([-0.5, 0.5, 0.0], [1.0, 1.0, 0.0], normal, [0.0, 1.0]),  // Yellow
        ];
        let indices = vec![0, 1, 2, 2, 3, 0];
        Mesh::new_indexed(&vertices, &indices)
    }

    /// Creates a colored triangle mesh at a specific position
    pub fn triangle_at(color: [f32; 3], offset_x: f32, offset_y: f32) -> Self {
        let normal = [0.0, 0.0, 1.0];
        let vertices = vec![
            Vertex::new(
                [-0.3 + offset_x, -0.3 + offset_y, 0.0],
                color,
                normal,
                [0.0, 0.0],
            ),
            Vertex::new(
                [0.3 + offset_x, -0.3 + offset_y, 0.0],
                color,
                normal,
                [1.0, 0.0],
            ),
            Vertex::new(
                [0.0 + offset_x, 0.3 + offset_y, 0.0],
                color,
                normal,
                [0.5, 1.0],
            ),
        ];
        Mesh::new(&vertices)
    }

    /// Creates a quad mesh at a specific position using indexed rendering
    pub fn quad_at(color: [f32; 3], offset_x: f32, offset_y: f32) -> Self {
        let normal = [0.0, 0.0, 1.0];
        let vertices = vec![
            Vertex::new(
                [-0.3 + offset_x, -0.3 + offset_y, 0.0],
                color,
                normal,
                [0.0, 0.0],
            ), // Bottom left
            Vertex::new(
                [0.3 + offset_x, -0.3 + offset_y, 0.0],
                color,
                normal,
                [1.0, 0.0],
            ), // Bottom right
            Vertex::new(
                [0.3 + offset_x, 0.3 + offset_y, 0.0],
                color,
                normal,
                [1.0, 1.0],
            ), // Top right
            Vertex::new(
                [-0.3 + offset_x, 0.3 + offset_y, 0.0],
                color,
                normal,
                [0.0, 1.0],
            ), // Top left
        ];
        let indices = vec![0, 1, 2, 2, 3, 0];
        Mesh::new_indexed(&vertices, &indices)
    }

    /// Creates a gradient quad at a specific position
    pub fn quad_gradient_at(offset_x: f32, offset_y: f32) -> Self {
        let normal = [0.0, 0.0, 1.0];
        let vertices = vec![
            Vertex::new(
                [-0.3 + offset_x, -0.3 + offset_y, 0.0],
                [1.0, 0.0, 0.0],
                normal,
                [0.0, 0.0],
            ), // Red
            Vertex::new(
                [0.3 + offset_x, -0.3 + offset_y, 0.0],
                [0.0, 1.0, 0.0],
                normal,
                [1.0, 0.0],
            ), // Green
            Vertex::new(
                [0.3 + offset_x, 0.3 + offset_y, 0.0],
                [0.0, 0.0, 1.0],
                normal,
                [1.0, 1.0],
            ), // Blue
            Vertex::new(
                [-0.3 + offset_x, 0.3 + offset_y, 0.0],
                [1.0, 1.0, 0.0],
                normal,
                [0.0, 1.0],
            ), // Yellow
        ];
        let indices = vec![0, 1, 2, 2, 3, 0];
        Mesh::new_indexed(&vertices, &indices)
    }

    /// Creates a 3D cube mesh using indexed rendering
    pub fn cube(color: [f32; 3]) -> Self {
        // For proper flat shading, each face needs its own vertices with correct normals
        // This means 24 vertices total (4 vertices × 6 faces) instead of 8 shared vertices
        let vertices = vec![
            // Front face (normal pointing +Z)
            Vertex::new([-0.5, -0.5, 0.5], color, [0.0, 0.0, 1.0], [0.0, 0.0]), // 0
            Vertex::new([0.5, -0.5, 0.5], color, [0.0, 0.0, 1.0], [1.0, 0.0]),  // 1
            Vertex::new([0.5, 0.5, 0.5], color, [0.0, 0.0, 1.0], [1.0, 1.0]),   // 2
            Vertex::new([-0.5, 0.5, 0.5], color, [0.0, 0.0, 1.0], [0.0, 1.0]),  // 3
            // Back face (normal pointing -Z)
            Vertex::new([0.5, -0.5, -0.5], color, [0.0, 0.0, -1.0], [0.0, 0.0]), // 4
            Vertex::new([-0.5, -0.5, -0.5], color, [0.0, 0.0, -1.0], [1.0, 0.0]), // 5
            Vertex::new([-0.5, 0.5, -0.5], color, [0.0, 0.0, -1.0], [1.0, 1.0]), // 6
            Vertex::new([0.5, 0.5, -0.5], color, [0.0, 0.0, -1.0], [0.0, 1.0]),  // 7
            // Right face (normal pointing +X)
            Vertex::new([0.5, -0.5, 0.5], color, [1.0, 0.0, 0.0], [0.0, 0.0]), // 8
            Vertex::new([0.5, -0.5, -0.5], color, [1.0, 0.0, 0.0], [1.0, 0.0]), // 9
            Vertex::new([0.5, 0.5, -0.5], color, [1.0, 0.0, 0.0], [1.0, 1.0]), // 10
            Vertex::new([0.5, 0.5, 0.5], color, [1.0, 0.0, 0.0], [0.0, 1.0]),  // 11
            // Left face (normal pointing -X)
            Vertex::new([-0.5, -0.5, -0.5], color, [-1.0, 0.0, 0.0], [0.0, 0.0]), // 12
            Vertex::new([-0.5, -0.5, 0.5], color, [-1.0, 0.0, 0.0], [1.0, 0.0]),  // 13
            Vertex::new([-0.5, 0.5, 0.5], color, [-1.0, 0.0, 0.0], [1.0, 1.0]),   // 14
            Vertex::new([-0.5, 0.5, -0.5], color, [-1.0, 0.0, 0.0], [0.0, 1.0]),  // 15
            // Top face (normal pointing +Y)
            Vertex::new([-0.5, 0.5, 0.5], color, [0.0, 1.0, 0.0], [0.0, 0.0]), // 16
            Vertex::new([0.5, 0.5, 0.5], color, [0.0, 1.0, 0.0], [1.0, 0.0]),  // 17
            Vertex::new([0.5, 0.5, -0.5], color, [0.0, 1.0, 0.0], [1.0, 1.0]), // 18
            Vertex::new([-0.5, 0.5, -0.5], color, [0.0, 1.0, 0.0], [0.0, 1.0]), // 19
            // Bottom face (normal pointing -Y)
            Vertex::new([-0.5, -0.5, -0.5], color, [0.0, -1.0, 0.0], [0.0, 0.0]), // 20
            Vertex::new([0.5, -0.5, -0.5], color, [0.0, -1.0, 0.0], [1.0, 0.0]),  // 21
            Vertex::new([0.5, -0.5, 0.5], color, [0.0, -1.0, 0.0], [1.0, 1.0]),   // 22
            Vertex::new([-0.5, -0.5, 0.5], color, [0.0, -1.0, 0.0], [0.0, 1.0]),  // 23
        ];

        // 36 indices for 12 triangles (6 faces × 2 triangles)
        let indices = vec![
            // Front face
            0, 1, 2, 2, 3, 0, // Back face
            4, 5, 6, 6, 7, 4, // Right face
            8, 9, 10, 10, 11, 8, // Left face
            12, 13, 14, 14, 15, 12, // Top face
            16, 17, 18, 18, 19, 16, // Bottom face
            20, 21, 22, 22, 23, 20,
        ];

        Mesh::new_indexed(&vertices, &indices)
    }

    /// Creates a UV sphere mesh using indexed rendering
    ///
    /// # Arguments
    /// * `radius` - Sphere radius
    /// * `segments` - Number of horizontal divisions (longitude)
    /// * `rings` - Number of vertical divisions (latitude)
    /// * `color` - RGB color for all vertices
    pub fn sphere(radius: f32, segments: u32, rings: u32, color: [f32; 3]) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Generate vertices
        for ring in 0..=rings {
            let theta = ring as f32 * std::f32::consts::PI / rings as f32;
            let sin_theta = theta.sin();
            let cos_theta = theta.cos();

            for seg in 0..=segments {
                let phi = seg as f32 * 2.0 * std::f32::consts::PI / segments as f32;
                let sin_phi = phi.sin();
                let cos_phi = phi.cos();

                // Spherical to Cartesian coordinates
                let x = cos_phi * sin_theta;
                let y = cos_theta;
                let z = sin_phi * sin_theta;

                // Position and normal (for a sphere, normal = normalized position)
                let position = [x * radius, y * radius, z * radius];
                let normal = [x, y, z];

                // UV coordinates
                let u = seg as f32 / segments as f32;
                let v = ring as f32 / rings as f32;

                vertices.push(Vertex::new(position, color, normal, [u, v]));
            }
        }

        // Generate indices
        for ring in 0..rings {
            for seg in 0..segments {
                let current_ring_start = ring * (segments + 1);
                let next_ring_start = (ring + 1) * (segments + 1);

                let current = current_ring_start + seg;
                let next = current_ring_start + seg + 1;
                let current_below = next_ring_start + seg;
                let next_below = next_ring_start + seg + 1;

                // First triangle
                indices.push(current);
                indices.push(current_below);
                indices.push(next);

                // Second triangle
                indices.push(next);
                indices.push(current_below);
                indices.push(next_below);
            }
        }

        Mesh::new_indexed(&vertices, &indices)
    }

    /// Creates a cylinder mesh using indexed rendering
    ///
    /// # Arguments
    /// * `radius` - Cylinder radius
    /// * `height` - Cylinder height
    /// * `segments` - Number of segments around the circumference
    /// * `color` - RGB color for all vertices
    pub fn cylinder(radius: f32, height: f32, segments: u32, color: [f32; 3]) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let half_height = height / 2.0;

        // Generate vertices for top and bottom circles
        for i in 0..=1 {
            let y = if i == 0 { -half_height } else { half_height };
            let normal_y = if i == 0 { -1.0 } else { 1.0 };
            let v = if i == 0 { 0.0 } else { 1.0 };

            for seg in 0..=segments {
                let theta = seg as f32 * 2.0 * std::f32::consts::PI / segments as f32;
                let x = theta.cos() * radius;
                let z = theta.sin() * radius;
                let u = seg as f32 / segments as f32;

                vertices.push(Vertex::new([x, y, z], color, [0.0, normal_y, 0.0], [u, v]));
            }
        }

        // Generate vertices for the side
        for i in 0..=1 {
            let y = if i == 0 { -half_height } else { half_height };
            let v = i as f32;

            for seg in 0..=segments {
                let theta = seg as f32 * 2.0 * std::f32::consts::PI / segments as f32;
                let x = theta.cos() * radius;
                let z = theta.sin() * radius;
                let nx = theta.cos();
                let nz = theta.sin();
                let u = seg as f32 / segments as f32;

                vertices.push(Vertex::new([x, y, z], color, [nx, 0.0, nz], [u, v]));
            }
        }

        // Generate indices for sides
        let side_start = (segments + 1) * 2;
        for seg in 0..segments {
            let current = side_start + seg;
            let next = side_start + seg + 1;
            let current_top = side_start + (segments + 1) + seg;
            let next_top = side_start + (segments + 1) + seg + 1;

            indices.push(current);
            indices.push(next);
            indices.push(current_top);

            indices.push(current_top);
            indices.push(next);
            indices.push(next_top);
        }

        // Generate indices for top and bottom caps
        // Bottom cap (center vertex)
        let bottom_center_idx = vertices.len() as u32;
        vertices.push(Vertex::new(
            [0.0, -half_height, 0.0],
            color,
            [0.0, -1.0, 0.0],
            [0.5, 0.5],
        ));

        for seg in 0..segments {
            indices.push(bottom_center_idx);
            indices.push(seg + 1);
            indices.push(seg);
        }

        // Top cap (center vertex)
        let top_center_idx = vertices.len() as u32;
        vertices.push(Vertex::new(
            [0.0, half_height, 0.0],
            color,
            [0.0, 1.0, 0.0],
            [0.5, 0.5],
        ));

        let top_start = segments + 1;
        for seg in 0..segments {
            indices.push(top_center_idx);
            indices.push(top_start + seg);
            indices.push(top_start + seg + 1);
        }

        Mesh::new_indexed(&vertices, &indices)
    }

    /// Creates a torus mesh using indexed rendering
    ///
    /// # Arguments
    /// * `major_radius` - Distance from center of torus to center of tube
    /// * `minor_radius` - Radius of the tube
    /// * `major_segments` - Number of segments around the major circle
    /// * `minor_segments` - Number of segments around the tube
    /// * `color` - RGB color for all vertices
    pub fn torus(
        major_radius: f32,
        minor_radius: f32,
        major_segments: u32,
        minor_segments: u32,
        color: [f32; 3],
    ) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Generate vertices
        for i in 0..=major_segments {
            let u = i as f32 * 2.0 * std::f32::consts::PI / major_segments as f32;
            let cos_u = u.cos();
            let sin_u = u.sin();

            for j in 0..=minor_segments {
                let v = j as f32 * 2.0 * std::f32::consts::PI / minor_segments as f32;
                let cos_v = v.cos();
                let sin_v = v.sin();

                let x = (major_radius + minor_radius * cos_v) * cos_u;
                let y = minor_radius * sin_v;
                let z = (major_radius + minor_radius * cos_v) * sin_u;

                let nx = cos_v * cos_u;
                let ny = sin_v;
                let nz = cos_v * sin_u;

                // UV coordinates
                let tex_u = i as f32 / major_segments as f32;
                let tex_v = j as f32 / minor_segments as f32;

                vertices.push(Vertex::new([x, y, z], color, [nx, ny, nz], [tex_u, tex_v]));
            }
        }

        // Generate indices
        for i in 0..major_segments {
            for j in 0..minor_segments {
                let first = i * (minor_segments + 1) + j;
                let second = first + minor_segments + 1;

                indices.push(first);
                indices.push(second);
                indices.push(first + 1);

                indices.push(second);
                indices.push(second + 1);
                indices.push(first + 1);
            }
        }

        Mesh::new_indexed(&vertices, &indices)
    }

    /// Creates a plane mesh using indexed rendering
    ///
    /// # Arguments
    /// * `width` - Plane width
    /// * `depth` - Plane depth
    /// * `color` - RGB color for all vertices
    pub fn plane(width: f32, depth: f32, color: [f32; 3]) -> Self {
        let half_width = width / 2.0;
        let half_depth = depth / 2.0;
        let normal = [0.0, 1.0, 0.0];

        let vertices = vec![
            Vertex::new([-half_width, 0.0, -half_depth], color, normal, [0.0, 0.0]),
            Vertex::new([half_width, 0.0, -half_depth], color, normal, [1.0, 0.0]),
            Vertex::new([half_width, 0.0, half_depth], color, normal, [1.0, 1.0]),
            Vertex::new([-half_width, 0.0, half_depth], color, normal, [0.0, 1.0]),
        ];

        let indices = vec![0, 1, 2, 2, 3, 0];

        Mesh::new_indexed(&vertices, &indices)
    }

    pub fn skybox_cube() -> Self {
        // Simple cube centered at origin
        // We only need positions since we use them as texture coordinates
        let vertices = vec![
            // Positions only - no normals, no UVs needed
            // Back face
            Vertex::new(
                [-1.0, -1.0, -1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            Vertex::new(
                [1.0, 1.0, -1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            Vertex::new(
                [1.0, -1.0, -1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            Vertex::new(
                [1.0, 1.0, -1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            Vertex::new(
                [-1.0, -1.0, -1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            Vertex::new(
                [-1.0, 1.0, -1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            // Front face
            Vertex::new(
                [-1.0, -1.0, 1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            Vertex::new(
                [1.0, -1.0, 1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            Vertex::new(
                [1.0, 1.0, 1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            Vertex::new(
                [1.0, 1.0, 1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            Vertex::new(
                [-1.0, 1.0, 1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            Vertex::new(
                [-1.0, -1.0, 1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            // Left face
            Vertex::new(
                [-1.0, 1.0, 1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            Vertex::new(
                [-1.0, 1.0, -1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            Vertex::new(
                [-1.0, -1.0, -1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            Vertex::new(
                [-1.0, -1.0, -1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            Vertex::new(
                [-1.0, -1.0, 1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            Vertex::new(
                [-1.0, 1.0, 1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            // Right face
            Vertex::new(
                [1.0, 1.0, 1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            Vertex::new(
                [1.0, -1.0, -1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            Vertex::new(
                [1.0, 1.0, -1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            Vertex::new(
                [1.0, -1.0, -1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            Vertex::new(
                [1.0, 1.0, 1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            Vertex::new(
                [1.0, -1.0, 1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            // Bottom face
            Vertex::new(
                [-1.0, -1.0, -1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            Vertex::new(
                [1.0, -1.0, -1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            Vertex::new(
                [1.0, -1.0, 1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            Vertex::new(
                [1.0, -1.0, 1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            Vertex::new(
                [-1.0, -1.0, 1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            Vertex::new(
                [-1.0, -1.0, -1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            // Top face
            Vertex::new(
                [-1.0, 1.0, -1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            Vertex::new(
                [1.0, 1.0, 1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            Vertex::new(
                [1.0, 1.0, -1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            Vertex::new(
                [1.0, 1.0, 1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            Vertex::new(
                [-1.0, 1.0, -1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
            Vertex::new(
                [-1.0, 1.0, 1.0],
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0],
            ),
        ];

        Mesh::new(&vertices)
    }

    pub fn new(vertices: &[Vertex]) -> Self {
        Self::new_internal(vertices, None)
    }

    pub fn new_indexed(vertices: &[Vertex], indices: &[u32]) -> Self {
        Self::new_internal(vertices, Some(indices))
    }

    pub fn new_internal(vertices: &[Vertex], indices: Option<&[u32]>) -> Self {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = None;
        let index_count;

        unsafe {
            // Generate VAO and VBO
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            // Bind VAO first
            gl::BindVertexArray(vao);

            // Upload vertex data to VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<Vertex>()) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // Position attribute (location = 0)
            gl::VertexAttribPointer(
                0,                               // location
                3,                               // size (x, y, z)
                gl::FLOAT,                       // type
                gl::FALSE,                       // normalized
                mem::size_of::<Vertex>() as i32, // stride (size of entire Vertex)
                ptr::null(),                     // offset (0 for position)
            );
            gl::EnableVertexAttribArray(0);

            // Color attribute (location = 1)
            gl::VertexAttribPointer(
                1,                                                      // location
                3,                                                      // size (r, g, b)
                gl::FLOAT,                                              // type
                gl::FALSE,                                              // normalized
                mem::size_of::<Vertex>() as i32,                        // stride
                (3 * mem::size_of::<f32>()) as *const std::ffi::c_void, // offset (3 floats)
            );
            gl::EnableVertexAttribArray(1);

            // Normal attribute (location = 2)
            gl::VertexAttribPointer(
                2,                                                      // location
                3,                                                      // size (nx, ny, nz)
                gl::FLOAT,                                              // type
                gl::FALSE,                                              // normalized
                mem::size_of::<Vertex>() as i32,                        // stride
                (6 * mem::size_of::<f32>()) as *const std::ffi::c_void, // offset (6 floats)
            );
            gl::EnableVertexAttribArray(2);

            // UV attribute (location = 3)
            gl::VertexAttribPointer(
                3,                                                      // location
                2,                                                      // size (u, v)
                gl::FLOAT,                                              // type
                gl::FALSE,                                              // normalized
                mem::size_of::<Vertex>() as i32,                        // stride
                (9 * mem::size_of::<f32>()) as *const std::ffi::c_void, // offset (9 floats: 3 pos + 3 color + 3 normal)
            );
            gl::EnableVertexAttribArray(3);

            // Handle EBO if indices are provided
            index_count = if let Some(idx) = indices {
                let mut ebo_id = 0;
                gl::GenBuffers(1, &mut ebo_id);
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo_id);
                gl::BufferData(
                    gl::ELEMENT_ARRAY_BUFFER,
                    (idx.len() * mem::size_of::<u32>()) as isize,
                    idx.as_ptr() as *const _,
                    gl::STATIC_DRAW,
                );
                ebo = Some(ebo_id);
                idx.len() as i32
            } else {
                0
            };

            // Unbind
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Mesh {
            vao,
            vbo,
            ebo,
            vertex_count: vertices.len() as i32,
            index_count,
        }
    }

    /// Renders the mesh
    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            if let Some(_) = self.ebo {
                gl::DrawElements(
                    gl::TRIANGLES,
                    self.index_count,
                    gl::UNSIGNED_INT,
                    ptr::null(),
                )
            } else {
                gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count);
            }
            gl::BindVertexArray(0);
        }
    }

    /// Returns the VAO handle (useful for debugging)
    pub fn vao(&self) -> u32 {
        self.vao
    }

    /// Returns the vertex count
    pub fn vertex_count(&self) -> i32 {
        self.vertex_count
    }

    /// Returns the index count (0 if non-indexed)
    pub fn index_count(&self) -> i32 {
        self.index_count
    }

    /// Returns true if this mesh uses indexed rendering
    pub fn is_indexed(&self) -> bool {
        self.ebo.is_some()
    }
}

// Cleanup when Mesh is dropped
impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            if let Some(ebo_id) = self.ebo {
                gl::DeleteBuffers(1, &ebo_id);
            }
        }
    }
}
