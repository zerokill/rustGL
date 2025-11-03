# Step 17.5: Scene Management (Intermediate Refactoring)

## Overview
This is an **intermediate step** between Phase 3 (Appearance) and Phase 4 (Advanced Effects) to refactor the growing complexity in `main.rs`. By introducing a `Scene` module, we'll centralize object and light management, making the codebase cleaner and preparing for future features like instanced rendering.

## Why Refactor Now?

Current problems with `main.rs`:
1. **Parameter explosion**: The `render()` function takes 15+ parameters
2. **Repetitive code**: Each object requires 4-5 lines of identical setup code
3. **Hard to extend**: Adding new objects means editing multiple places
4. **No separation of concerns**: Rendering logic mixed with scene setup

After this refactoring:
- `main.rs` will shrink from ~377 lines to ~150 lines
- Adding new objects becomes a single line
- Code is ready for instanced rendering (Phase 6)
- Better Rust practice with ownership and encapsulation

## Concepts

### Scene Graph
A **scene** is a container for all objects that need to be rendered. It stores:
- **Objects**: Meshes with their materials and transforms
- **Lights**: All light sources in the scene
- **Rendering logic**: How to draw all objects efficiently

### SceneObject
A `SceneObject` bundles together everything needed to render one object:
```rust
pub struct SceneObject {
    pub mesh: Mesh,           // Geometry
    pub material: Material,   // Surface properties
    pub transform: Transform, // Position, rotation, scale
}
```

### Transform Struct
Instead of building `glm::Mat4` manually every frame, we'll create a `Transform` struct:
```rust
pub struct Transform {
    pub position: glm::Vec3,
    pub rotation: glm::Vec3,  // Euler angles in radians
    pub scale: glm::Vec3,
}
```

This makes transforms easier to work with and prepares for animation/physics later.

## Implementation Plan

### 1. Create `transform.rs`
This module will handle position, rotation, and scale:

```rust
// rustgl/src/transform.rs
use nalgebra_glm as glm;

#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub position: glm::Vec3,
    pub rotation: glm::Vec3,  // Euler angles (x, y, z) in radians
    pub scale: glm::Vec3,
}

impl Transform {
    /// Create a new transform at the origin with default scale
    pub fn new() -> Self {
        Transform {
            position: glm::vec3(0.0, 0.0, 0.0),
            rotation: glm::vec3(0.0, 0.0, 0.0),
            scale: glm::vec3(1.0, 1.0, 1.0),
        }
    }

    /// Create a transform with a specific position
    pub fn from_position(position: glm::Vec3) -> Self {
        Transform {
            position,
            rotation: glm::vec3(0.0, 0.0, 0.0),
            scale: glm::vec3(1.0, 1.0, 1.0),
        }
    }

    /// Create a transform with position and scale
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
```

**Key Points:**
- `to_matrix()` converts to the `glm::Mat4` that shaders need
- Helper methods like `rotate_y()` make animation easier
- Derived traits (`Clone`, `Copy`, `Debug`) make it ergonomic to use
- TRS order (Translate-Rotate-Scale) is standard in graphics

### 2. Create `scene.rs`
This module manages all scene objects and lights:

```rust
// rustgl/src/scene.rs
use crate::mesh::Mesh;
use crate::material::Material;
use crate::transform::Transform;
use crate::light::Light;
use crate::shader::Shader;
use nalgebra_glm as glm;

/// A single object in the scene
pub struct SceneObject {
    pub mesh: Mesh,
    pub material: Material,
    pub transform: Transform,
}

impl SceneObject {
    /// Create a new scene object
    pub fn new(mesh: Mesh, material: Material, transform: Transform) -> Self {
        SceneObject {
            mesh,
            material,
            transform,
        }
    }
}

/// Manages all objects and lights in the scene
pub struct Scene {
    objects: Vec<SceneObject>,
    lights: Vec<Light>,
}

impl Scene {
    /// Create an empty scene
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }

    /// Add an object to the scene
    pub fn add_object(&mut self, mesh: Mesh, material: Material, transform: Transform) {
        self.objects.push(SceneObject::new(mesh, material, transform));
    }

    /// Add a light to the scene
    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    /// Get a reference to all lights
    pub fn lights(&self) -> &[Light] {
        &self.lights
    }

    /// Get a mutable reference to a specific object by index
    pub fn get_object_mut(&mut self, index: usize) -> Option<&mut SceneObject> {
        self.objects.get_mut(index)
    }

    /// Get the number of objects in the scene
    pub fn object_count(&self) -> usize {
        self.objects.len()
    }

    /// Render all objects in the scene
    pub fn render(&self, shader: &Shader, view: &glm::Mat4, projection: &glm::Mat4) {
        shader.use_program();

        // Set view and projection matrices (same for all objects)
        shader.set_mat4("view", view);
        shader.set_mat4("projection", projection);

        // Set lights (same for all objects)
        shader.set_lights(&self.lights);

        // Render each object
        for object in &self.objects {
            // Set object-specific uniforms
            shader.set_material(&object.material);
            shader.set_mat4("model", &object.transform.to_matrix());

            // Draw the mesh
            object.mesh.draw();
        }
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
```

**Key Points:**
- `Scene` owns all objects and lights (single source of truth)
- `render()` method handles the entire rendering loop
- Lights and matrices are set once, then each object is rendered
- `get_object_mut()` allows updating transforms for animation
- Prepared for future extensions (spatial partitioning, culling, etc.)

### 3. Update `main.rs`
Simplify the main rendering code dramatically:

**Changes needed:**

1. **Add new module declarations** at the top:
```rust
mod transform;
mod scene;
```

2. **Add new imports**:
```rust
use transform::Transform;
use scene::Scene;
```

3. **Replace object creation and setup** (lines 72-113):
```rust
// Create scene
let mut scene = Scene::new();

// Add ground plane
scene.add_object(
    Mesh::plane(10.0, 10.0, [0.3, 0.3, 0.3]),
    Material::matte(glm::vec3(0.2, 1.0, 0.3)),
    Transform::from_position(glm::vec3(0.0, -2.0, 0.0)),
);

// Add rotating sphere (left)
scene.add_object(
    Mesh::sphere(1.0, 32, 16, [0.3, 0.7, 1.0]),
    Material::plastic(glm::vec3(0.3, 0.7, 1.0)),
    Transform::from_position(glm::vec3(-4.0, 0.0, 0.0)),
);

// Add rotating cube (center-left)
scene.add_object(
    Mesh::cube([1.0, 0.5, 0.2]),
    Material::metal(glm::vec3(1.0, 0.5, 0.2)),
    Transform::from_position(glm::vec3(-2.0, 0.0, 0.0)),
);

// Add rotating cylinder (center)
scene.add_object(
    Mesh::cylinder(0.5, 2.0, 32, [0.2, 1.0, 0.3]),
    Material::matte(glm::vec3(0.2, 1.0, 0.3)),
    Transform::from_position(glm::vec3(0.0, 0.0, 0.0)),
);

// Add rotating torus (center-right)
scene.add_object(
    Mesh::torus(1.0, 0.3, 32, 16, [1.0, 0.3, 0.7]),
    Material::rubber(glm::vec3(1.0, 0.3, 0.7)),
    Transform::from_position(glm::vec3(2.0, 0.0, 0.0)),
);

// Add small chrome sphere (right)
scene.add_object(
    Mesh::sphere(1.0, 32, 16, [0.8, 0.8, 0.8]),
    Material::chrome(),
    Transform::from_position_scale(glm::vec3(4.0, 0.0, 0.0), glm::vec3(0.8, 0.8, 0.8)),
);

// Add lights
scene.add_light(Light::long_range(
    glm::vec3(5.0, 5.0, 5.0),
    glm::vec3(5.0, 5.0, 5.0),
));
scene.add_light(Light::medium_range(
    glm::vec3(-5.0, 2.0, 0.0),
    glm::vec3(4.0, 0.6, 0.6),
));
scene.add_light(Light::medium_range(
    glm::vec3(5.0, 2.0, -3.0),
    glm::vec3(0.6, 1.2, 4.0),
));
scene.add_light(Light::short_range(
    glm::vec3(0.0, 1.0, 5.0),
    glm::vec3(1.0, 3.0, 1.0),
));
```

4. **Update the `update()` function signature** to receive `&mut Scene`:
```rust
fn update(delta_time: f32, time: &mut f32, scene: &mut Scene) {
    *time += delta_time;

    // Animate objects by updating their transforms
    // Object indices: 0=plane, 1=sphere, 2=cube, 3=cylinder, 4=torus, 5=chrome sphere

    if let Some(sphere) = scene.get_object_mut(1) {
        sphere.transform.rotate(0.0, 0.5 * delta_time, 0.0);
        sphere.transform.rotate_x(0.3 * delta_time);
    }

    if let Some(cube) = scene.get_object_mut(2) {
        sphere.transform.rotate(0.7 * delta_time, 0.7 * delta_time, 0.0);
    }

    if let Some(cylinder) = scene.get_object_mut(3) {
        cylinder.transform.rotate(0.3 * delta_time, 0.4 * delta_time, 0.0);
    }

    if let Some(torus) = scene.get_object_mut(4) {
        torus.transform.rotate(0.0, 0.6 * delta_time, 0.0);
        torus.transform.rotate_x(0.6 * delta_time * 0.5);
    }

    if let Some(chrome_sphere) = scene.get_object_mut(5) {
        chrome_sphere.transform.rotate(0.8 * delta_time * 0.5, 0.8 * delta_time, 0.8 * delta_time * 0.5);
    }
}
```

5. **Simplify the `render()` function signature** drastically:
```rust
fn render(
    window: &mut glfw::Window,
    scene: &Scene,
    shader: &Shader,
    texture: &Texture,
    camera: &Camera,
    wireframe_mode: bool,
    use_texture: bool,
) {
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::ClearColor(0.1, 0.1, 0.2, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

        // Set polygon mode based on wireframe toggle
        if wireframe_mode {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        } else {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
        }

        shader.use_program();

        // Set camera-related uniforms
        shader.set_vec3("viewPos", &camera.position);

        // Set texture
        texture.bind(0);
        shader.set_int("textureSampler", 0);
        shader.set_bool("useTexture", use_texture);

        // Create view and projection matrices
        let view = camera.get_view_matrix();
        let projection = glm::perspective(
            1024.0 / 768.0,
            camera.zoom.to_radians(),
            0.1,
            100.0,
        );

        // Render the entire scene
        scene.render(shader, &view, &projection);
    }
    window.swap_buffers();
}
```

6. **Update function calls in the main loop**:
```rust
// In main(), change these lines:
update(delta_time, &mut time, &mut scene);
render(
    &mut window,
    &scene,
    &shader,
    &texture,
    &camera,
    wireframe_mode,
    use_texture,
);
```

7. **Don't forget to add the modules to `main.rs`**:
```rust
mod transform;  // Add after line 9
mod scene;      // Add after transform
```

And update imports:
```rust
use transform::Transform;  // Add after line 19
use scene::Scene;          // Add after transform
```

## Benefits of This Refactoring

### Before:
```rust
// main.rs: 377 lines
// render() function: 15 parameters, 105 lines
// Adding a new object: Edit 3+ locations
```

### After:
```rust
// main.rs: ~150 lines
// render() function: 7 parameters, 45 lines
// Adding a new object: 1 line
scene.add_object(Mesh::sphere(...), Material::metal(...), Transform::new());
```

### Preparation for Instancing:
When you reach **Step 29: Instanced Rendering** in Phase 6, you'll extend the Scene to support:
```rust
// Future enhancement (don't implement now)
scene.add_instanced_object(
    Mesh::cube(...),
    Material::metal(...),
    vec![transform1, transform2, ..., transform1000],  // Render 1000 cubes efficiently
);
```

The current Scene structure makes this extension natural and clean.

## Testing

After implementing, verify:
1. **Scene renders correctly**: All 6 objects appear in the same positions
2. **Animation works**: Objects rotate as before
3. **Lights work**: Same lighting as before
4. **Code is cleaner**: `main.rs` is significantly shorter

## Common Pitfalls

### 1. Module Declaration Order
Make sure modules are declared **before** they're used:
```rust
mod transform;  // Must come before scene
mod scene;      // Uses transform
```

### 2. Object Indices
After adding objects to the scene, remember their indices for animation:
```rust
// Index 0: plane (ground)
// Index 1: sphere (left)
// Index 2: cube (center-left)
// Index 3: cylinder (center)
// Index 4: torus (center-right)
// Index 5: chrome sphere (right)
```

### 3. Rotation Order
In `Transform::to_matrix()`, rotation order matters:
- Current order: X → Y → Z (standard)
- Different orders produce different results
- Consistent with most graphics engines

### 4. Mutable References
`get_object_mut()` returns `Option<&mut SceneObject>` because:
- Index might be out of bounds
- Rust's borrow checker requires explicit mutability
- Use `if let Some(obj) = ...` pattern

## Next Steps

After completing this refactoring:
1. ✅ Your code is cleaner and more maintainable
2. ✅ You're ready for **Step 18: Skybox** (Phase 4)
3. ✅ Future instancing will be much easier to implement
4. ✅ You've practiced important Rust patterns (ownership, borrowing, Options)

## Summary

This intermediate step introduces:
- **Transform struct**: Clean abstraction for position/rotation/scale
- **Scene management**: Centralized rendering logic
- **SceneObject**: Bundles mesh, material, and transform
- **Better code organization**: Smaller, more focused functions
- **Preparation for instancing**: Scene structure ready to extend

Time to implement: ~30-45 minutes

Ready to clean up that `main.rs`? 🚀
