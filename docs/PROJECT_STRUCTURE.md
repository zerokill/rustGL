# RustGL Project Structure Guide

This document explains how to organize your Rust code following best practices. The structure evolves as you progress through the learning path.

## Philosophy

Rust projects follow strong conventions:
- **`src/`** - All source code
- **`Cargo.toml`** - Project manifest (dependencies, metadata)
- **Modules** - Organize code into logical units
- **Visibility** - Use `pub` intentionally
- **Separation of concerns** - Each module has a clear purpose

We'll start simple and refactor as the project grows, teaching you when and how to split code into modules.

---

## Recommended Repository Structure

```
rustgl/                          # Root of your repository
├── docs/                        # Learning materials (this documentation)
│   ├── README.md
│   ├── GETTING_STARTED.md
│   ├── ROADMAP.md
│   ├── PROJECT_STRUCTURE.md    # This file
│   └── phase-XX-*/
│
├── rustgl/                      # Your Rust project (Cargo workspace)
│   ├── Cargo.toml               # Project manifest
│   ├── src/                     # Source code
│   │   ├── main.rs              # Entry point
│   │   ├── lib.rs               # Optional: library code
│   │   └── */                   # Modules (added as you progress)
│   ├── shaders/                 # GLSL shader files
│   │   ├── basic.vert
│   │   ├── basic.frag
│   │   └── */
│   ├── resources/               # Assets (textures, models)
│   │   ├── textures/
│   │   ├── skybox/
│   │   └── models/
│   ├── examples/                # Optional: example programs
│   └── tests/                   # Optional: integration tests
│
└── README.md                    # Repository overview
```

**Key Decision:** Your Rust code goes in `rustgl/` subdirectory, keeping it separate from documentation.

---

## Progressive Structure by Phase

### Phase 1: Foundation (Steps 1-5)

**Single file approach - learning the basics**

```
rustgl/
├── Cargo.toml
└── src/
    └── main.rs        # Everything in one file (OK for now!)
```

**`main.rs` structure:**
```rust
// External crates
extern crate glfw;

// Imports
use glfw::{Action, Context, Key};

// Main function
fn main() {
    // Your code here
}

// Helper functions
fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    // ...
}
```

**Why single file?**
- Focus on learning Rust syntax
- Understand ownership without module complexity
- See the whole program at once

---

### Phase 2: Core Rendering (Steps 6-12)

**Introduce modules - organize by functionality**

```
rustgl/
├── Cargo.toml
└── src/
    ├── main.rs           # Entry point, main loop
    ├── shader.rs         # Shader compilation and management
    ├── mesh.rs           # Mesh data structure and rendering
    └── camera.rs         # Camera system
```

**`main.rs`:**
```rust
mod shader;  // Declares the shader module
mod mesh;    // Declares the mesh module
mod camera;  // Declares the camera module

use shader::Shader;
use mesh::Mesh;
use camera::Camera;

fn main() {
    // Use your modules
    let shader = Shader::new("vertex.glsl", "fragment.glsl");
    let mesh = Mesh::new(vertices);
    let camera = Camera::new();
}
```

**`shader.rs`:**
```rust
// A module for shader-related code
pub struct Shader {
    pub id: u32,
}

impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Self {
        // Implementation
    }

    pub fn use_program(&self) {
        // Implementation
    }
}
```

**Why modules now?**
- Code is getting larger
- Each struct deserves its own file
- Learn Rust's module system
- Understand `pub` visibility

**Step-by-step refactoring:**
1. **Step 8 (Shaders):** Extract shader code to `shader.rs`
2. **Step 9 (Mesh):** Create `mesh.rs` for mesh structure
3. **Step 11 (Camera):** Move camera to `camera.rs`
4. **Step 12 (Primitives):** Add primitive generators to `mesh.rs`

---

### Phase 3: Appearance (Steps 13-17)

**Organize by domain - rendering subsystems**

```
rustgl/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── lib.rs            # Optional: library for reusable code
    │
    ├── graphics/         # Graphics subsystem
    │   ├── mod.rs        # Module declaration
    │   ├── shader.rs
    │   ├── mesh.rs
    │   ├── texture.rs    # NEW: Texture loading/binding
    │   └── material.rs   # NEW: Material system
    │
    ├── scene/            # Scene management
    │   ├── mod.rs
    │   └── camera.rs
    │
    └── utils/            # Utilities
        └── mod.rs
```

**`src/graphics/mod.rs`:**
```rust
// Re-export public items
pub mod shader;
pub mod mesh;
pub mod texture;
pub mod material;

// Re-export commonly used types
pub use shader::Shader;
pub use mesh::Mesh;
pub use texture::Texture;
pub use material::Material;
```

**`main.rs`:**
```rust
mod graphics;
mod scene;
mod utils;

use graphics::{Shader, Mesh, Texture, Material};
use scene::Camera;

fn main() {
    // Clean imports!
}
```

**Why subdirectories?**
- Group related functionality
- Clearer mental model
- Scales to larger projects
- Matches Rust conventions

---

### Phase 4-5: Advanced Features (Steps 18-28)

**Full subsystem organization**

```
rustgl/
├── Cargo.toml
└── src/
    ├── main.rs           # Minimal - just calls engine
    ├── lib.rs            # Core engine library
    │
    ├── graphics/         # Rendering
    │   ├── mod.rs
    │   ├── shader.rs
    │   ├── mesh.rs
    │   ├── texture.rs
    │   ├── material.rs
    │   ├── framebuffer.rs    # NEW: FBOs
    │   ├── skybox.rs         # NEW: Skybox rendering
    │   └── primitives/       # Primitive generators
    │       ├── mod.rs
    │       ├── sphere.rs
    │       ├── cube.rs
    │       └── pyramid.rs
    │
    ├── scene/            # Scene management
    │   ├── mod.rs
    │   ├── camera.rs
    │   ├── light.rs          # NEW: Lighting system
    │   └── node.rs           # NEW: Scene graph nodes
    │
    ├── procedural/       # Procedural generation
    │   ├── mod.rs
    │   ├── noise.rs          # NEW: Perlin noise
    │   └── terrain.rs        # NEW: Terrain generation
    │
    ├── water/            # Water system
    │   ├── mod.rs
    │   └── water.rs          # NEW: Water rendering
    │
    └── utils/
        ├── mod.rs
        └── resource_manager.rs
```

**`lib.rs` (engine core):**
```rust
pub mod graphics;
pub mod scene;
pub mod procedural;
pub mod water;
pub mod utils;

// Re-export commonly used types
pub mod prelude {
    pub use crate::graphics::{Shader, Mesh, Texture};
    pub use crate::scene::{Camera, Light};
    pub use crate::procedural::Terrain;
}
```

**`main.rs` (application):**
```rust
use rustgl::prelude::*;

fn main() {
    // Your application code
}
```

**Why lib.rs?**
- Separates engine from application
- Enables testing individual components
- Could be published as a crate
- Clear API boundaries

---

### Phase 6-8: Production Quality (Steps 29-42)

**Complete engine structure**

```
rustgl/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── lib.rs
    │
    ├── graphics/
    │   ├── mod.rs
    │   ├── shader.rs
    │   ├── mesh.rs
    │   ├── texture.rs
    │   ├── material.rs
    │   ├── framebuffer.rs
    │   ├── skybox.rs
    │   ├── instance.rs       # NEW: Instanced rendering
    │   ├── primitives/
    │   │   ├── mod.rs
    │   │   ├── sphere.rs
    │   │   ├── cube.rs
    │   │   └── pyramid.rs
    │   └── debug/            # NEW: Debug rendering
    │       ├── mod.rs
    │       ├── wireframe.rs
    │       └── normals.rs
    │
    ├── scene/
    │   ├── mod.rs
    │   ├── camera.rs
    │   ├── light.rs
    │   ├── node.rs
    │   └── scene_graph.rs    # NEW: Scene management
    │
    ├── procedural/
    │   ├── mod.rs
    │   ├── noise.rs
    │   ├── terrain.rs
    │   └── clouds.rs         # NEW: Volumetric clouds
    │
    ├── water/
    │   ├── mod.rs
    │   └── water.rs
    │
    ├── physics/              # NEW: Physics subsystem
    │   ├── mod.rs
    │   ├── collision.rs
    │   └── particle.rs
    │
    ├── input/                # NEW: Input system
    │   ├── mod.rs
    │   └── input_manager.rs
    │
    ├── ui/                   # NEW: UI system
    │   ├── mod.rs
    │   └── debug_ui.rs
    │
    └── utils/
        ├── mod.rs
        ├── resource_manager.rs
        └── time.rs           # NEW: Time management
```

---

## Best Practices

### 1. Module Organization

**✅ DO:**
```rust
// src/graphics/mod.rs
pub mod shader;
pub mod mesh;

pub use shader::Shader;
pub use mesh::Mesh;
```

**❌ DON'T:**
```rust
// Everything in one giant file
// No module organization
```

### 2. File Naming

**✅ DO:**
```
snake_case.rs      # resource_manager.rs
mod.rs             # Module declarations
```

**❌ DON'T:**
```
CamelCase.rs
mixedCase.rs
```

### 3. Visibility

**✅ DO:**
```rust
pub struct Shader {      // Public interface
    id: u32,             // Private field
}

impl Shader {
    pub fn new() -> Self { }      // Public method
    fn compile(&self) { }         // Private helper
}
```

**❌ DON'T:**
```rust
pub struct Shader {
    pub id: u32,         // Exposes implementation details
}
```

### 4. Use Statements

**✅ DO:**
```rust
// Group by source
use std::fs;
use std::path::Path;

// External crates
use glfw::Context;
use gl::types::*;

// Local modules
use crate::graphics::Shader;
use crate::scene::Camera;
```

**❌ DON'T:**
```rust
use std::fs;
use crate::graphics::Shader;
use glfw::Context;
use std::path::Path;  // Random order
```

### 5. Module Prelude Pattern

**✅ DO:**
```rust
// src/prelude.rs
pub use crate::graphics::{Shader, Mesh, Texture};
pub use crate::scene::Camera;

// main.rs
use rustgl::prelude::*;  // Import commonly used items
```

### 6. Error Handling

**✅ DO:**
```rust
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn load_shader(path: &str) -> Result<Shader> {
    // Returns proper errors
}
```

**❌ DON'T:**
```rust
pub fn load_shader(path: &str) -> Shader {
    // Panics on error - bad!
}
```

---

## Cargo.toml Best Practices

**Well-organized `Cargo.toml`:**

```toml
[package]
name = "rustgl"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]

[dependencies]
# Windowing
glfw = "0.54"

# Graphics
gl = "0.14"
glam = "0.24"  # Math library

# Image loading
image = "0.24"

# UI (added later)
# egui = "0.23"

[dev-dependencies]
# Development-only dependencies

[profile.release]
opt-level = 3
lto = true

[profile.dev]
opt-level = 0
```

**Comments in Cargo.toml:**
- Group dependencies by purpose
- Comment out future dependencies
- Configure release optimizations

---

## When to Refactor

Each phase teaches you when to reorganize:

| Phase | Structure | Why |
|-------|-----------|-----|
| 1 | Single file | Learning basics |
| 2 | Module per struct | Code is growing |
| 3 | Subdirectories | Related functionality |
| 4-5 | Subsystems | Multiple features |
| 6-8 | Full library | Production quality |

**Golden Rule:** Don't over-engineer early. Refactor when:
- A file exceeds ~300 lines
- You have multiple related structs
- You're repeating imports
- The mental model isn't clear

---

## Quick Reference

**Create a new module:**
```bash
# Create file
touch src/my_module.rs

# Declare in main.rs or lib.rs
mod my_module;
```

**Create a directory module:**
```bash
# Create directory and mod.rs
mkdir src/my_module
touch src/my_module/mod.rs

# Declare in parent
mod my_module;
```

**Re-export from module:**
```rust
// src/graphics/mod.rs
pub mod shader;
pub use shader::Shader;  // Re-export for convenience
```

**Use from another module:**
```rust
// src/main.rs
mod graphics;
use graphics::Shader;  // Direct path
```

---

## Summary

You'll learn Rust organization progressively:
1. **Phase 1:** Single file (focus on Rust basics)
2. **Phase 2:** Multiple files (learn modules)
3. **Phase 3:** Subdirectories (organize by domain)
4. **Phase 4-8:** Full library structure (production patterns)

Each step guide will tell you **exactly when and how to refactor**. You'll develop good habits naturally as the project grows!

The result: a well-organized, idiomatic Rust codebase that follows community best practices. 🦀
