use crate::framebuffer::Framebuffer;
use crate::mesh::Mesh;
use crate::shader::Shader;
use gl::types::*;

pub struct BloomRenderer {
    // Framebuffers
    scene_fbo: Framebuffer,
    bright_pass_fbo: Framebuffer,
    blur_fbo1: Framebuffer,
    blur_fbo2: Framebuffer,

    // Shaders
    bright_pass_shader: Shader,
    blur_shader: Shader,
    composite_shader: Shader,
    screen_shader: Shader,

    // Geometry
    screen_quad: Mesh,

    // Settings
    blur_iterations: usize,
}

impl BloomRenderer {
    pub fn new(width: u32, height: u32) -> Self {
        BloomRenderer {
            scene_fbo: Framebuffer::new(width, height),
            bright_pass_fbo: Framebuffer::new(width, height),
            blur_fbo1: Framebuffer::new(width, height),
            blur_fbo2: Framebuffer::new(width, height),

            bright_pass_shader: Shader::new("shader/screen.vert", "shader/bright_pass.frag"),
            blur_shader: Shader::new("shader/screen.vert", "shader/blur.frag"),
            composite_shader: Shader::new("shader/screen.vert", "shader/bloom_composite.frag"),
            screen_shader: Shader::new("shader/screen.vert", "shader/screen.frag"),

            screen_quad: Mesh::screen_quad(),

            blur_iterations: 5,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.scene_fbo.resize(width, height);
        self.bright_pass_fbo.resize(width, height);
        self.blur_fbo1.resize(width, height);
        self.blur_fbo2.resize(width, height);
    }

    /// Get the scene texture for use by other post-processing effects
    pub fn scene_texture(&self) -> GLuint {
        self.scene_fbo.texture()
    }

    /// Main entry point - renders the scene with optional bloom
    pub fn render<F>(
        &mut self,
        render_scene: F,
        threshold: f32,
        strength: f32,
        enabled: bool,
        window_width: i32,
        window_height: i32,
    ) where
        F: FnOnce(),
    {
        // Pass 1: Render scene to framebuffer
        self.scene_fbo.bind();
        render_scene();

        if enabled {
            // Passes 2-5: Apply bloom effect
            self.apply_bloom(threshold, strength, window_width, window_height);
        } else {
            // Just render scene without bloom
            self.render_passthrough(window_width, window_height);
        }
    }

    /// Apply the full bloom pipeline (bright pass + blur + composite)
    fn apply_bloom(&mut self, threshold: f32, strength: f32, window_width: i32, window_height: i32) {
        // Pass 2: Extract bright areas
        self.bright_pass_fbo.bind();
        unsafe {
            gl::Disable(gl::DEPTH_TEST);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            self.bright_pass_shader.use_program();
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.scene_fbo.texture());
            self.bright_pass_shader.set_int("screenTexture", 0);
            self.bright_pass_shader.set_float("threshold", threshold);
            self.screen_quad.draw();
        }

        // Passes 3 & 4: Ping-pong blur
        let mut horizontal = true;
        let mut first_iteration = true;

        for _ in 0..self.blur_iterations * 2 {
            if horizontal {
                self.blur_fbo1.bind();
            } else {
                self.blur_fbo2.bind();
            }

            unsafe {
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);

                self.blur_shader.use_program();
                gl::ActiveTexture(gl::TEXTURE0);

                let source_texture = if first_iteration {
                    self.bright_pass_fbo.texture()
                } else if horizontal {
                    self.blur_fbo2.texture()
                } else {
                    self.blur_fbo1.texture()
                };

                gl::BindTexture(gl::TEXTURE_2D, source_texture);
                self.blur_shader.set_int("image", 0);
                self.blur_shader.set_bool("horizontal", horizontal);
                self.screen_quad.draw();
            }

            horizontal = !horizontal;
            if first_iteration {
                first_iteration = false;
            }
        }

        // Pass 5: Composite bloom with scene
        Framebuffer::unbind();
        unsafe {
            gl::Viewport(0, 0, window_width, window_height);
            gl::Disable(gl::DEPTH_TEST);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            self.composite_shader.use_program();
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.scene_fbo.texture());
            self.composite_shader.set_int("scene", 0);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, self.blur_fbo2.texture());
            self.composite_shader.set_int("bloomBlur", 1);
            self.composite_shader.set_float("bloomStrength", strength);
            self.screen_quad.draw();
        }
    }

    /// Render scene without bloom
    fn render_passthrough(&self, window_width: i32, window_height: i32) {
        Framebuffer::unbind();
        unsafe {
            gl::Viewport(0, 0, window_width, window_height);
            gl::Disable(gl::DEPTH_TEST);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            self.screen_shader.use_program();
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.scene_fbo.texture());
            self.screen_shader.set_int("screenTexture", 0);
            self.screen_quad.draw();
        }
    }
}
