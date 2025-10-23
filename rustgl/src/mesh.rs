use std::mem;
use std::ptr;

/// Represents a single vertex with position and color
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],  // x, y, z
    pub color: [f32; 3],     // r, g, b
    pub normal: [f32; 3],    // nx, ny, nz
}

impl Vertex {
    /// Creates a new vertex with position and color
    pub fn new(position: [f32; 3], color: [f32; 3], normal: [f32; 3]) -> Self {
        Vertex { position, color, normal }
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
        let normal = [0.0, 0.0, 1.0];  // Facing camera
        let vertices = vec![
            Vertex::new([-0.5, -0.5, 0.0], color, normal),
            Vertex::new([0.5, -0.5, 0.0], color, normal),
            Vertex::new([0.0, 0.5, 0.0], color, normal),
        ];
        Mesh::new(&vertices)
    }

    /// Creates a quad mesh using indexed rendering
    pub fn quad(color: [f32; 3]) -> Self {
        let normal = [0.0, 0.0, 1.0];
        let vertices = vec![
            Vertex::new([-0.5, -0.5, 0.0], color, normal),  // 0: Bottom left
            Vertex::new([0.5, -0.5, 0.0], color, normal),   // 1: Bottom right
            Vertex::new([0.5, 0.5, 0.0], color, normal),    // 2: Top right
            Vertex::new([-0.5, 0.5, 0.0], color, normal),   // 3: Top left
        ];
        let indices = vec![
            0, 1, 2,  // First triangle
            2, 3, 0,  // Second triangle
        ];
        Mesh::new_indexed(&vertices, &indices)
    }

    /// Creates a gradient quad (different color per corner)
    pub fn quad_gradient() -> Self {
        let normal = [0.0, 0.0, 1.0];
        let vertices = vec![
            Vertex::new([-0.5, -0.5, 0.0], [1.0, 0.0, 0.0], normal),  // Red
            Vertex::new([0.5, -0.5, 0.0], [0.0, 1.0, 0.0], normal),   // Green
            Vertex::new([0.5, 0.5, 0.0], [0.0, 0.0, 1.0], normal),    // Blue
            Vertex::new([-0.5, 0.5, 0.0], [1.0, 1.0, 0.0], normal),   // Yellow
        ];
        let indices = vec![0, 1, 2, 2, 3, 0];
        Mesh::new_indexed(&vertices, &indices)
    }

    /// Creates a colored triangle mesh at a specific position
    pub fn triangle_at(color: [f32; 3], offset_x: f32, offset_y: f32) -> Self {
        let normal = [0.0, 0.0, 1.0];
        let vertices = vec![
            Vertex::new([-0.3 + offset_x, -0.3 + offset_y, 0.0], color, normal),
            Vertex::new([0.3 + offset_x, -0.3 + offset_y, 0.0], color, normal),
            Vertex::new([0.0 + offset_x, 0.3 + offset_y, 0.0], color, normal),
        ];
        Mesh::new(&vertices)
    }

    /// Creates a quad mesh at a specific position using indexed rendering
    pub fn quad_at(color: [f32; 3], offset_x: f32, offset_y: f32) -> Self {
        let normal = [0.0, 0.0, 1.0];
        let vertices = vec![
            Vertex::new([-0.3 + offset_x, -0.3 + offset_y, 0.0], color, normal),  // Bottom left
            Vertex::new([0.3 + offset_x, -0.3 + offset_y, 0.0], color, normal),   // Bottom right
            Vertex::new([0.3 + offset_x, 0.3 + offset_y, 0.0], color, normal),    // Top right
            Vertex::new([-0.3 + offset_x, 0.3 + offset_y, 0.0], color, normal),   // Top left
        ];
        let indices = vec![0, 1, 2, 2, 3, 0];
        Mesh::new_indexed(&vertices, &indices)
    }

    /// Creates a gradient quad at a specific position
    pub fn quad_gradient_at(offset_x: f32, offset_y: f32) -> Self {
        let normal = [0.0, 0.0, 1.0];
        let vertices = vec![
            Vertex::new([-0.3 + offset_x, -0.3 + offset_y, 0.0], [1.0, 0.0, 0.0], normal),  // Red
            Vertex::new([0.3 + offset_x, -0.3 + offset_y, 0.0], [0.0, 1.0, 0.0], normal),   // Green
            Vertex::new([0.3 + offset_x, 0.3 + offset_y, 0.0], [0.0, 0.0, 1.0], normal),    // Blue
            Vertex::new([-0.3 + offset_x, 0.3 + offset_y, 0.0], [1.0, 1.0, 0.0], normal),   // Yellow
        ];
        let indices = vec![0, 1, 2, 2, 3, 0];
        Mesh::new_indexed(&vertices, &indices)
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
                0,                                    // location
                3,                                    // size (x, y, z)
                gl::FLOAT,                            // type
                gl::FALSE,                            // normalized
                mem::size_of::<Vertex>() as i32,      // stride (size of entire Vertex)
                ptr::null(),                          // offset (0 for position)
            );
            gl::EnableVertexAttribArray(0);

            // Color attribute (location = 1)
            gl::VertexAttribPointer(
                1,                                                 // location
                3,                                                 // size (r, g, b)
                gl::FLOAT,                                         // type
                gl::FALSE,                                         // normalized
                mem::size_of::<Vertex>() as i32,                   // stride
                (3 * mem::size_of::<f32>()) as *const std::ffi::c_void,  // offset (3 floats)
            );
            gl::EnableVertexAttribArray(1);

            // Normal attribute (location = 2)
            gl::VertexAttribPointer(
                2,                                                 // location
                3,                                                 // size (nx, ny, nz)
                gl::FLOAT,                                         // type
                gl::FALSE,                                         // normalized
                mem::size_of::<Vertex>() as i32,                   // stride
                (6 * mem::size_of::<f32>()) as *const std::ffi::c_void,  // offset (6 floats)
            );
            gl::EnableVertexAttribArray(2);

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
//    pub fn index_count(&self) -> i32 {
//        self.index_count
//    }

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
