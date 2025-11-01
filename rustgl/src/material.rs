use nalgebra_glm as glm;

/// Represents material surface properties for Phong lighting
#[derive(Clone, Copy, Debug)]
pub struct Material {
    /// Ambient color - how much ambient light the material reflects
    pub ambient: glm::Vec3,

    /// Diffuse color - the main color of the material under direct light
    pub diffuse: glm::Vec3,

    /// Specular color - the color of shiny highlights
    pub specular: glm::Vec3,

    /// Shininess - controls how focused the specular highlight is (higher = sharper)
    pub shininess: f32,
}

impl Material {
    /// Creates a new material with specified properties
    pub fn new(ambient: glm::Vec3, diffuse: glm::Vec3, specular: glm::Vec3, shininess: f32) -> Self {
        Material {
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    /// Creates a shiny plastic material (medium shininess, white highlights)
    pub fn plastic(color: glm::Vec3) -> Self {
        Material {
            ambient: color * 0.1,          // 10% ambient
            diffuse: color,                // Main color
            specular: glm::vec3(0.5, 0.5, 0.5),  // White-ish highlights
            shininess: 32.0,               // Medium shine
        }
    }

    /// Creates a metallic material (high shininess, colored highlights)
    pub fn metal(color: glm::Vec3) -> Self {
        Material {
            ambient: color * 0.2,          // 20% ambient (metals are brighter)
            diffuse: color * 0.8,          // Slightly darker main color
            specular: color,               // Colored highlights (metals reflect their color)
            shininess: 64.0,               // High shine
        }
    }

    /// Creates a rough/matte material (low shininess, minimal highlights)
    pub fn matte(color: glm::Vec3) -> Self {
        Material {
            ambient: color * 0.1,
            diffuse: color,
            specular: glm::vec3(0.1, 0.1, 0.1),  // Very dim highlights
            shininess: 8.0,                // Low shine (rough surface)
        }
    }

    /// Creates a rubber-like material (very low shininess, soft highlights)
    pub fn rubber(color: glm::Vec3) -> Self {
        Material {
            ambient: color * 0.05,         // Low ambient
            diffuse: color,
            specular: glm::vec3(0.3, 0.3, 0.3),
            shininess: 4.0,                // Very low shine
        }
    }

    /// Creates a shiny material like polished chrome (very high shininess)
    pub fn chrome() -> Self {
        Material {
            ambient: glm::vec3(0.25, 0.25, 0.25),
            diffuse: glm::vec3(0.4, 0.4, 0.4),
            specular: glm::vec3(0.77, 0.77, 0.77),
            shininess: 128.0,              // Very high shine
        }
    }
}
