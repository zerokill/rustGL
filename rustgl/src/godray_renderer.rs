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
    pub luminance_threshold: f32,
}

impl GodRayRenderer {
    pub fn new(width: u32, height: u32) -> Self {
        GodRayRenderer {
            occlusion_fbo: Framebuffer::new(width, height),
            radial_blur_fbo: Framebuffer::new(width, height),

            occlusion_shader: Shader::new("shader/screen.vert", "shader/occlusion.frag"),
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
        light_world_pos: glm::Vec3,
        view: &glm::Mat4,
        projection: &glm::Mat4,
        strength: f32,
        window_width: i32,
        window_height: i32,
    ) {
        // Convert light position to screen space and check if visible
        let (light_screen_pos, is_visible) = self.world_to_screen(light_world_pos, view, projection);

        // Only apply god rays if light is in front of camera and on screen
        if !is_visible {
            // Just render the scene without god rays
            self.render_passthrough(scene_texture, window_width, window_height);
            return;
        }

        self.generate_occlusion_mask(scene_texture);
        self.apply_radial_blur(light_screen_pos);
        self.composite(scene_texture, strength, window_width, window_height);
    }

    pub fn world_to_screen(&self, world_pos: glm::Vec3, view: &glm::Mat4, projection: &glm::Mat4) -> (glm::Vec2, bool) {
        let clip_space = projection * view * glm::vec4(world_pos.x, world_pos.y, world_pos.z, 1.0);

        // Check if light is behind the camera (negative w or negative z after perspective divide)
        if clip_space.w <= 0.0 {
            return (glm::vec2(0.5, 0.5), false);
        }

        let ndc = glm::vec3(
            clip_space.x / clip_space.w,
            clip_space.y / clip_space.w,
            clip_space.z / clip_space.w,
        );

        // Check if light is in front of camera (NDC z should be between -1 and 1)
        // and roughly on screen (we allow some margin for off-screen rays)
        let is_visible = ndc.z >= -1.0 && ndc.z <= 1.0;

        let screen_pos = glm::vec2(
            (ndc.x + 1.0) * 0.5,
            (ndc.y + 1.0) * 0.5,
        );

        (screen_pos, is_visible)
    }

    fn generate_occlusion_mask(&mut self, scene_texture: GLuint) {
        self.occlusion_fbo.bind();
        unsafe {
            gl::Disable(gl::DEPTH_TEST);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            self.occlusion_shader.use_program();
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, scene_texture);
            self.occlusion_shader.set_int("sceneTexture", 0);
            self.occlusion_shader.set_float("luminanceThreshold", self.luminance_threshold);
            self.screen_quad.draw();
        }
    }

    fn apply_radial_blur(&mut self, light_screen_pos: glm::Vec2) {
        self.radial_blur_fbo.bind();
        unsafe {
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
}
