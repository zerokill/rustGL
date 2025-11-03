use crate::light::Light;
use crate::material::Material;
use crate::mesh::Mesh;
use crate::shader::Shader;
use crate::transform::Transform;
use nalgebra_glm as glm;

pub struct SceneObject {
    pub mesh: Mesh,
    pub material: Material,
    pub transform: Transform,
}

impl SceneObject {
    pub fn new(mesh: Mesh, material: Material, transform: Transform) -> Self {
        SceneObject {
            mesh,
            material,
            transform,
        }
    }
}

pub struct Scene {
    objects: Vec<SceneObject>,
    lights: Vec<Light>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn add_object(&mut self, mesh: Mesh, material: Material, transform: Transform) {
        self.objects
            .push(SceneObject::new(mesh, material, transform));
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn lights(&self) -> &[Light] {
        &self.lights
    }

    pub fn get_object_mut(&mut self, index: usize) -> Option<&mut SceneObject> {
        self.objects.get_mut(index)
    }

    pub fn object_count(&self) -> usize {
        self.objects.len()
    }

    /// Update the position of a specific light by index
    pub fn update_light_position(&mut self, index: usize, position: glm::Vec3) {
        if let Some(light) = self.lights.get_mut(index) {
            light.position = position;
        }
    }

    pub fn render(&self, shader: &Shader, view: &glm::Mat4, projection: &glm::Mat4) {
        shader.use_program();
        shader.set_mat4("view", view);
        shader.set_mat4("projection", projection);

        shader.set_lights(&self.lights);

        for object in &self.objects {
            shader.set_material(&object.material);
            shader.set_mat4("model", &object.transform.to_matrix());

            object.mesh.draw();
        }
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
