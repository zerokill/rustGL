use std::mem;
use std::ptr;

/// Represents a single vertex with position and color
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],  // x, y, z
    pub color: [f32; 3],     // r, g, b
}

impl Vertex {
    /// Creates a new vertex with position and color
    pub fn new(position: [f32; 3], color: [f32; 3]) -> Self {
        Vertex { position, color }
    }
}

/// A mesh holds vertex data and OpenGL buffer objects
pub struct Mesh {
    vao: u32,
    vbo: u32,
    vertex_count: i32,
}

impl Mesh {
    /// Creates a new mesh from a list of vertices
    ///
    /// # Arguments
    /// * `vertices` - Slice of Vertex structs to upload to GPU
    ///
    /// # Example
    /// ```
    /// let vertices = vec![
    ///     Vertex::new([-0.5, -0.5, 0.0], [1.0, 0.0, 0.0]),
    ///     Vertex::new([0.5, -0.5, 0.0], [0.0, 1.0, 0.0]),
    ///     Vertex::new([0.0, 0.5, 0.0], [0.0, 0.0, 1.0]),
    /// ];
    /// let mesh = Mesh::new(&vertices);
    /// ```
    pub fn new(vertices: &[Vertex]) -> Self {
        let mut vao = 0;
        let mut vbo = 0;

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

            // Unbind
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Mesh {
            vao,
            vbo,
            vertex_count: vertices.len() as i32,
        }
    }

    /// Renders the mesh
    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count);
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
}

// Cleanup when Mesh is dropped
impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}
