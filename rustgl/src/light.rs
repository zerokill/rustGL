use nalgebra_glm as glm;

#[derive(Clone, Copy, Debug)]
pub struct Light {
    pub position: glm::Vec3,
    pub color: glm::Vec3,
    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
}

impl Light {
    pub fn new(
            position: glm::Vec3,
            color: glm::Vec3,
            constant: f32,
            linear: f32,
            quadratic: f32,
        ) -> Self {
        Light {
            position,
            color,
            constant,
            linear,
            quadratic,
        }
    }

    pub fn short_range(position: glm::Vec3, color: glm::Vec3) -> Self {
        Light {
            position,
            color,
            constant: 1.0,
            linear: 0.7,
            quadratic: 1.8,
        }
    }

    pub fn medium_range(position: glm::Vec3, color: glm::Vec3) -> Self {
        Light {
            position,
            color,
            constant: 1.0,
            linear: 0.35,
            quadratic: 0.44,
        }
    }

    pub fn long_range(position: glm::Vec3, color: glm::Vec3) -> Self {
        Light {
            position,
            color,
            constant: 1.0,
            linear: 0.14,
            quadratic: 0.07,
        }
    }

    pub fn very_long_range(position: glm::Vec3, color: glm::Vec3) -> Self {
        Light {
            position,
            color,
            constant: 1.0,
            linear: 0.045,
            quadratic: 0.0075,
        }
    }
}
