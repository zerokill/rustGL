use crate::framebuffer::Framebuffer;
use crate::mesh::Mesh;
use crate::shader::Shader;
use gl::types::*;
use nalgebra_glm as glm;

pub struct GodRayRenderer {
    occlusion_fbo: Framebuffer,
    radial_blur_fbo: Framebuffer,

    occlusion_shader: Shader,
    radial_blur_shader: Shader,
    composite_shader: Shader,
    screen_shader: Shader,

    screen_quad: Mesh,

    pub exposure: f32,
    pub decay: f32,
    pub density: f32,
    pub weight: f32,
    pub num_samples: i32,
    #[allow(dead_code)]
    pub luminance_threshold: f32,
}

impl GodRayRenderer {
    pub fn new(width: u32, height: u32) -> Self {
        GodRayRenderer {
            occlusion_fbo: Framebuffer::new(width, height),
            radial_blur_fbo: Framebuffer::new(width, height),

            occlusion_shader: Shader::new("shader/occlusion.vert", "shader/occlusion.frag"),
            radial_blur_shader: Shader::new("shader/screen.vert", "shader/radial_blur.frag"),
            composite_shader: Shader::new("shader/screen.vert", "shader/godray_composite.frag"),
            screen_shader: Shader::new("shader/screen.vert", "shader/screen.frag"),

            screen_quad: Mesh::screen_quad(),

            exposure: 0.5,
            decay: 0.97,
            density: 0.8,
            weight: 0.3,
            num_samples: 100,
            luminance_threshold: 0.9,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.occlusion_fbo.resize(width, height);
        self.radial_blur_fbo.resize(width, height);
    }

    pub fn apply(
        &mut self,
        scene_texture: GLuint,
        scene: &crate::scene::Scene,
        orb_index: usize,
        light_world_pos: glm::Vec3,
        view: &glm::Mat4,
        projection: &glm::Mat4,
        strength: f32,
        debug_mode: u8,  // 0 = off, 1 = occlusion, 2 = radial blur, 3 = rays only
        window_width: i32,
        window_height: i32,
    ) {
        let (light_screen_pos, is_on_screen) = self.world_to_screen_checked(light_world_pos, view, projection);

        self.generate_occlusion_mask(scene, orb_index, view, projection);

        // Debug mode 1: Show occlusion buffer
        if debug_mode == 1 {
            self.render_debug_buffer(self.occlusion_fbo.texture(), window_width, window_height);
            return;
        }

        // Only apply radial blur if light is reasonably close to screen
        // (we allow some margin for off-screen rays)
        if is_on_screen {
            self.apply_radial_blur(light_screen_pos);
        } else {
            // Clear the radial blur buffer if light is too far off-screen
            self.radial_blur_fbo.bind();
            unsafe {
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
        }

        // Debug mode 2 & 3: Show radial blur buffer (god rays only)
        if debug_mode == 2 || debug_mode == 3 {
            self.render_debug_buffer(self.radial_blur_fbo.texture(), window_width, window_height);
            return;
        }

        // Normal mode (0): Composite with scene
        self.composite(scene_texture, strength, window_width, window_height);
    }

    fn world_to_screen_checked(&self, world_pos: glm::Vec3, view: &glm::Mat4, projection: &glm::Mat4) -> (glm::Vec2, bool) {
        let clip_space = projection * view * glm::vec4(world_pos.x, world_pos.y, world_pos.z, 1.0);

        // Check if behind camera
        if clip_space.w <= 0.0 {
            return (glm::vec2(0.5, 0.5), false);
        }

        let ndc = glm::vec3(
            clip_space.x / clip_space.w,
            clip_space.y / clip_space.w,
            clip_space.z / clip_space.w,
        );

        // Check if light is reasonably close to screen
        // We allow some margin (up to 2x off-screen) for edge rays
        let margin = 2.0;
        let is_on_screen = ndc.x >= -margin && ndc.x <= margin &&
                          ndc.y >= -margin && ndc.y <= margin &&
                          ndc.z >= -1.0 && ndc.z <= 1.0;

        // Clamp to reasonable range for radial blur
        let screen_pos = glm::vec2(
            ((ndc.x + 1.0) * 0.5).clamp(-1.0, 2.0),  // Allow some off-screen
            ((ndc.y + 1.0) * 0.5).clamp(-1.0, 2.0),
        );

        (screen_pos, is_on_screen)
    }

    fn generate_occlusion_mask(
        &mut self,
        scene: &crate::scene::Scene,
        orb_index: usize,
        view: &glm::Mat4,
        projection: &glm::Mat4,
    ) {
        self.occlusion_fbo.bind();
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            self.occlusion_shader.use_program();
            self.occlusion_shader.set_mat4("view", view);
            self.occlusion_shader.set_mat4("projection", projection);

            // Render all scene objects to build depth buffer
            for (i, obj) in scene.objects_iter().enumerate() {
                self.occlusion_shader.set_mat4("model", &obj.transform.to_matrix());

                // Set uniform to indicate if this is the orb or an occluder
                let is_orb = i == orb_index;
                self.occlusion_shader.set_bool("isOrb", is_orb);

                obj.mesh.draw();
            }
        }

        // Unbind the framebuffer so we can read from its texture
        Framebuffer::unbind();
    }

    fn apply_radial_blur(&mut self, light_screen_pos: glm::Vec2) {
        self.radial_blur_fbo.bind();
        unsafe {
            gl::Disable(gl::DEPTH_TEST);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            self.radial_blur_shader.use_program();
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.occlusion_fbo.texture());
            self.radial_blur_shader.set_int("occlusionTexture", 0);
            self.radial_blur_shader.set_vec2("lightScreenPos", &light_screen_pos);
            self.radial_blur_shader.set_float("exposure", self.exposure);
            self.radial_blur_shader.set_float("decay", self.decay);
            self.radial_blur_shader.set_float("density", self.density);
            self.radial_blur_shader.set_float("weight", self.weight);
            self.radial_blur_shader.set_int("numSamples", self.num_samples);
            self.screen_quad.draw();
        }
    }

    fn composite(&self, scene_texture: GLuint, strength: f32, window_width: i32, window_height: i32) {
        Framebuffer::unbind();
        unsafe {
            gl::Disable(gl::DEPTH_TEST);
            gl::Viewport(0, 0, window_width, window_height);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            self.composite_shader.use_program();
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, scene_texture);
            self.composite_shader.set_int("scene", 0);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, self.radial_blur_fbo.texture());
            self.composite_shader.set_int("godRays", 1);
            self.composite_shader.set_float("godRayStrength", strength);
            self.screen_quad.draw();
        }
    }

    fn render_passthrough(&self, scene_texture: GLuint, window_width: i32, window_height: i32) {
        Framebuffer::unbind();
        unsafe {
            gl::Viewport(0, 0, window_width, window_height);
            gl::Disable(gl::DEPTH_TEST);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            self.screen_shader.use_program();
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, scene_texture);
            self.screen_shader.set_int("screenTexture", 0);
            self.screen_quad.draw();
        }
    }

    fn render_debug_buffer(&self, texture: GLuint, window_width: i32, window_height: i32) {
        Framebuffer::unbind();
        unsafe {
            gl::Viewport(0, 0, window_width, window_height);
            gl::Disable(gl::DEPTH_TEST);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            self.screen_shader.use_program();
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            self.screen_shader.set_int("screenTexture", 0);
            self.screen_quad.draw();
        }
    }
}
