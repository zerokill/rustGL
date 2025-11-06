use crate::light::Light;
use crate::material::Material;
use crate::mesh::Mesh;
use crate::shader::Shader;
use crate::texture::Texture;
use crate::transform::Transform;
use nalgebra_glm as glm;

pub struct SceneObject {
    pub mesh: Mesh,
    pub material: Material,
    pub transform: Transform,
}

pub struct Skybox {
    pub mesh: Mesh,
    pub shader: Shader,
    pub texture: Texture,
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
    skybox: Option<Skybox>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
            lights: Vec::new(),
            skybox: None,
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

    pub fn get_object(&self, index: usize) -> Option<&SceneObject> {
        self.objects.get(index)
    }

    pub fn get_object_mut(&mut self, index: usize) -> Option<&mut SceneObject> {
        self.objects.get_mut(index)
    }

    pub fn object_count(&self) -> usize {
        self.objects.len()
    }

    pub fn objects_iter(&self) -> std::slice::Iter<SceneObject> {
        self.objects.iter()
    }

    /// Update the position of a specific light by index
    pub fn update_light_position(&mut self, index: usize, position: glm::Vec3) {
        if let Some(light) = self.lights.get_mut(index) {
            light.position = position;
        }
    }

    /// Set the skybox for the scene
    pub fn set_skybox(&mut self, mesh: Mesh, shader: Shader, texture: Texture) {
        self.skybox = Some(Skybox {
            mesh,
            shader,
            texture,
        });
    }

    pub fn render(&self, shader: &Shader, view: &glm::Mat4, projection: &glm::Mat4, skybox_enabled: bool) {
        // Render skybox first (if present and enabled)
        if skybox_enabled {
            if let Some(skybox) = &self.skybox {
            unsafe {
                gl::DepthFunc(gl::LEQUAL);

                skybox.shader.use_program();
                skybox.shader.set_mat4("view", view);
                skybox.shader.set_mat4("projection", projection);
                skybox.texture.bind(0);
                skybox.shader.set_int("skybox", 0);
                skybox.mesh.draw();

                gl::DepthFunc(gl::LESS);
            }
            }
        }

        // Render scene objects
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
