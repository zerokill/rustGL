use nalgebra_glm as glm;

pub struct Camera {
    pub position: glm::Vec3,
    pub front: glm::Vec3,
    pub up: glm::Vec3,
    pub right: glm::Vec3,
    pub world_up: glm::Vec3,

    pub pitch: f32,
    pub yaw: f32,

    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
    pub zoom: f32,
}

impl Camera {
    pub fn new(position: glm::Vec3, up: glm::Vec3, yaw: f32, pitch: f32) -> Self {
        let mut camera = Camera {
            position,
            front: glm::vec3(0.0, 0.0, -1.0),
            up: glm::vec3(0.0, 0.0, 0.0),
            right: glm::vec3(0.0, 0.0, 0.0),
            world_up: up,
            yaw,
            pitch,
            movement_speed: 2.5,
            mouse_sensitivity: 0.1,
            zoom: 45.0,
        };
        camera.update_camera_vectors();
        camera
    }

    pub fn default() -> Self {
        Camera::new(
            glm::vec3(0.0, 0.0, 3.0),  // Position
            glm::vec3(0.0, 1.0, 0.0),  // World up
            -90.0,                      // Yaw (looking down -Z)
            0.0,                        // Pitch (level)
        )
    }

    pub fn get_view_matrix(&self) -> glm::Mat4 {
        glm::look_at(&self.position, &(self.position + self.front), &self.up)
    }

    /// Processes keyboard input
    pub fn process_keyboard(&mut self, direction: CameraMovement, delta_time: f32) {
        let velocity = self.movement_speed * delta_time;
        match direction {
            CameraMovement::Forward => {
                self.position += self.front * velocity;
            }
            CameraMovement::Backward => {
                self.position -= self.front * velocity;
            }
            CameraMovement::Left => {
                self.position -= self.right * velocity;
            }
            CameraMovement::Right => {
                self.position += self.right * velocity;
            }
            CameraMovement::Up => {
                self.position += self.up * velocity;
            }
            CameraMovement::Down => {
                self.position -= self.up * velocity;
            }
        }
    }

    /// Processes mouse movement (for mouse look, which we'll add later)
    pub fn process_mouse_movement(&mut self, x_offset: f32, y_offset: f32, constrain_pitch: bool) {
        let x_offset = x_offset * self.mouse_sensitivity;
        let y_offset = y_offset * self.mouse_sensitivity;

        self.yaw += x_offset;
        self.pitch += y_offset;

        // Constrain pitch to prevent screen flip
        if constrain_pitch {
            if self.pitch > 89.0 {
                self.pitch = 89.0;
            }
            if self.pitch < -89.0 {
                self.pitch = -89.0;
            }
        }

        self.update_camera_vectors();
    }

    /// Processes mouse scroll (for zoom, which we'll add later)
    pub fn process_mouse_scroll(&mut self, y_offset: f32) {
        self.zoom -= y_offset;
        if self.zoom < 1.0 {
            self.zoom = 1.0;
        }
        if self.zoom > 45.0 {
            self.zoom = 45.0;
        }
    }

    pub fn update_camera_vectors(&mut self) {
        let front = glm::vec3(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        );
        self.front = glm::normalize(&front);

        self.right = glm::normalize(&glm::cross(&self.front, &self.world_up));
        self.up = glm::normalize(&glm::cross(&self.right, &self.front));
    }
}

/// Camera movement directions
#[derive(Debug, Clone, Copy)]
pub enum CameraMovement {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
}

