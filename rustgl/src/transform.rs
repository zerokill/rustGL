use nalgebra_glm as glm;

#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub position: glm::Vec3,
    pub rotation: glm::Vec3,
    pub scale: glm::Vec3,
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            position: glm::vec3(0.0, 0.0, 0.0),
            rotation: glm::vec3(0.0, 0.0, 0.0),
            scale: glm::vec3(1.0, 1.0, 1.0),
        }
    }

    pub fn from_position(position: glm::Vec3) -> Self {
        Transform {
            position,
            rotation: glm::vec3(0.0, 0.0, 0.0),
            scale: glm::vec3(1.0, 1.0, 1.0),
        }
    }

    pub fn from_position_scale(position: glm::Vec3, scale: glm::Vec3) -> Self {
        Transform {
            position,
            rotation: glm::vec3(0.0, 0.0, 0.0),
            scale,
        }
    }

    /// Convert the transform to a 4x4 model matrix
    pub fn to_matrix(&self) -> glm::Mat4 {
        let mut matrix = glm::Mat4::identity();

        // Apply transformations in TRS order: Translate -> Rotate -> Scale
        matrix = glm::translate(&matrix, &self.position);

        // Apply rotations (order matters: typically Y -> X -> Z)
        if self.rotation.x != 0.0 {
            matrix = glm::rotate(&matrix, self.rotation.x, &glm::vec3(1.0, 0.0, 0.0));
        }
        if self.rotation.y != 0.0 {
            matrix = glm::rotate(&matrix, self.rotation.y, &glm::vec3(0.0, 1.0, 0.0));
        }
        if self.rotation.z != 0.0 {
            matrix = glm::rotate(&matrix, self.rotation.z, &glm::vec3(0.0, 0.0, 1.0));
        }

        matrix = glm::scale(&matrix, &self.scale);

        matrix
    }

    /// Rotate around the Y axis (yaw)
    pub fn rotate_y(&mut self, angle: f32) {
        self.rotation.y += angle;
    }

    /// Rotate around the X axis (pitch)
    pub fn rotate_x(&mut self, angle: f32) {
        self.rotation.x += angle;
    }

    /// Rotate around the Z axis (roll)
    pub fn rotate_z(&mut self, angle: f32) {
        self.rotation.z += angle;
    }

    /// Apply multiple rotations at once
    pub fn rotate(&mut self, x: f32, y: f32, z: f32) {
        self.rotation.x += x;
        self.rotation.y += y;
        self.rotation.z += z;
    }

    /// Translate by a delta vector
    pub fn translate(&mut self, delta: glm::Vec3) {
        self.position += delta;
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}
