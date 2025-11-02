# RustGL - Learn Rust Through Game Engine Development

## Repository Purpose

This repository is a **structured learning project** designed to teach Rust programming through the practical goal of recreating a 3D game engine with modern OpenGL. The student (Maurice) is learning Rust by building features progressively, from basic window creation to advanced volumetric rendering.

## Teaching Philosophy

- **Learn by doing**: The student writes all code themselves - no copy-paste
- **Progressive complexity**: Each step builds on previous ones
- **Hands-on learning**: Practical implementation over theory
- **Code reviews**: Provide feedback at each step
- **Student's pace**: Allow time for understanding before moving forward

## Project Origin

This project is based on the student's previous **SwiftGL** game engine (written in Swift). The goal is to recreate that engine in Rust, using it as a vehicle to learn:
- Rust's ownership and borrowing system
- Safe systems programming
- Working with C libraries (OpenGL, GLFW)
- Performance optimization
- Complex algorithms (noise, collision detection, raymarching)
- Real-world project structure

## Platform & Technology

- **Primary Platform**: Linux (OpenGL 4.5+)
- **Secondary**: macOS and Windows for basic features (OpenGL 4.1 compatible)
- **Graphics API**: Modern OpenGL 4.x (with fallback for macOS limitations)
- **Language**: Rust (edition 2021)
- **Window Management**: GLFW
- **Math Library**: glam

## Learning Path Structure

The project follows a **42-step learning path** organized into **8 phases**:

### Phase 1: Foundation (Steps 1-5)
- Rust basics, windowing, event loop
- Focus: Learning Rust syntax and basic project structure

### Phase 2: Core Rendering (Steps 6-12)
- Triangles, meshes, camera, primitives
- Focus: OpenGL basics, GLSL shaders, 3D transformations

### Phase 3: Appearance (Steps 13-17)
- Textures, lighting, materials
- Focus: Making objects look realistic

### Phase 4: Advanced Effects (Steps 18-23)
- Skybox, water, reflections, transparency
- Focus: Complex visual effects

### Phase 5: Procedural Generation (Steps 24-28)
- Perlin noise, terrain generation
- Focus: Procedural content creation

### Phase 6: Optimization (Steps 29-33)
- Instancing, physics, particles
- Focus: Performance and simulation

### Phase 7: Volumetric Rendering (Steps 34-37)
- 3D textures, raymarching, clouds
- Focus: Advanced shader techniques

### Phase 8: Polish (Steps 38-42)
- Scene management, UI, debugging, performance
- Focus: Production-quality engine features

## Documentation Structure

```
/Users/maurice/projects/dev/claudeGL3/
├── README.md                    # Project introduction
├── PLAN.md                      # Executive summary of the learning plan
├── docs/
│   ├── README.md                # Documentation overview
│   ├── GETTING_STARTED.md       # How to begin the learning journey
│   ├── ROADMAP.md               # Complete roadmap with progress checkboxes
│   ├── STEPS_SUMMARY.md         # Quick reference for all 42 steps
│   ├── PROJECT_STRUCTURE.md     # How to organize Rust code
│   ├── phase-01-foundation/     # Detailed step-by-step guides
│   ├── phase-02-core-rendering/
│   ├── phase-03-appearance/
│   ├── phase-04-advanced-effects/
│   ├── phase-05-procedural/
│   ├── phase-06-optimization/
│   ├── phase-07-volumetric/
│   └── phase-08-polish/
└── [Student's Rust code will go here]
```

## Current Progress

Based on git commits and the learning plan:
- ✅ **Completed through Step 17** (Multiple Lights working)
- ✅ Phase 1: Foundation - COMPLETE
- ✅ Phase 2: Core Rendering - COMPLETE
- ✅ **Phase 3: Appearance - COMPLETE**
  - ✅ Step 13: Texture Loading - COMPLETE
  - ✅ Step 14: Texture Mapping - COMPLETE
  - ✅ Step 15: Lighting Basics - COMPLETE
  - ✅ Step 16: Materials - COMPLETE
  - ✅ Step 17: Multiple Lights - COMPLETE
- 📝 **Next**: Step 18: Skybox (Phase 4: Advanced Effects)

Recent commits show:
- `011e2e4` - primitives working
- `3063731` - Fix movement
- `92efa54` - camera working
- `2c4d6cb` - lesson 11 camera ready
- `a746c79` - transformations working

## Current Implementation State

**Key features already in place:**
- ✅ Vertex struct with position, color, normal, UV (mesh.rs)
- ✅ All primitives: sphere, cube, cylinder, torus, plane with correct normals and UVs
- ✅ Texture loading system (texture.rs)
- ✅ Texture sampling in fragment shader
- ✅ Shader utility methods: set_mat4, set_vec3, set_float, set_int, set_bool, set_material, set_light, set_lights
- ✅ Wireframe mode toggle (Key 1)
- ✅ Texture toggle (Key 2)
- ✅ Camera system with movement (WASDQE) and look controls (Arrow keys)
- ✅ Phong lighting (ambient + diffuse + specular)
- ✅ Material system with ambient, diffuse, specular, shininess properties
- ✅ Material presets: plastic, metal, matte, rubber, chrome
- ✅ Multiple point lights (up to 4) with attenuation
- ✅ Light system with position, color, and range presets (short/medium/long/very long)

**Math library in use:**
- Using `nalgebra-glm` (not `glam` as originally planned)
- All matrix operations use `nalgebra_glm as glm`

## How to Assist

### When creating a new lesson or step guide:

**CRITICAL: ALWAYS follow the roadmap and check the current state of the code first!**

1. **Follow the roadmap** (`docs/ROADMAP.md`):
   - The roadmap defines the correct order of steps
   - Each step is carefully sequenced to build on previous knowledge
   - Do NOT deviate from the roadmap order unless explicitly requested by the student
   - Check ROADMAP.md to confirm what the next step should be

2. **Read the actual implementation files** before writing lesson content:
   - `rustgl/src/mesh.rs` - Check vertex structure, primitives, attributes
   - `rustgl/src/shader.rs` - Check what shader methods exist
   - `rustgl/src/main.rs` - Check current rendering setup
   - `rustgl/shader/basic.vert` and `rustgl/shader/basic.frag` - Check shader code
   - Other relevant files based on the lesson topic

3. **Identify what's already implemented**:
   - The student may have already added features beyond the current step
   - Note which vertex attributes are already configured (position, color, normal, UV)
   - Check what shader uniforms and methods already exist
   - Identify what data structures are already in place

4. **Write the lesson to match reality**:
   - Mark already-completed tasks as ✅ "Already Done"
   - Focus lesson content on what ACTUALLY needs to be implemented
   - Provide accurate line numbers and code references
   - Update time estimates based on actual remaining work
   - Avoid teaching features that are already implemented

5. **Example of good lesson structure**:
   ```markdown
   ## Current State Check

   ✅ **Already implemented**: UV coordinates in Vertex struct (mesh.rs:11)
   ✅ **Already implemented**: Vertex shader passes UVs (basic.vert:5)
   ❌ **Still needed**: Fragment shader texture sampling
   ❌ **Still needed**: Shader utility methods (set_int, set_bool)

   ## Tasks

   ### Task 1: Update Fragment Shader (NEW - needs implementation)
   ...
   ```

### When responding to the student:

**Two modes of operation:**

1. **Lesson Mode** (Default for learning steps):
   - When student says "go to the next lesson" or "next step"
   - **Generate the lesson guide** for them to implement
   - Check current code state first
   - Write the lesson with tasks for the student to complete
   - Do NOT implement the code - let the student do it
   - After student says "done" or "finished", review their implementation

2. **Direct Implementation Mode**:
   - When student asks for specific features/fixes (e.g., "add arrow key controls", "fix the cube normals")
   - **Implement the requested feature directly**
   - These are usually quality-of-life improvements or bug fixes
   - Explain what you changed and why

**Code Review Process:**
1. **Assess current phase**: Check which step they're on
2. **Review their code**: Look at implementation details
3. **Provide constructive feedback**: Focus on:
   - Rust idioms and best practices
   - Ownership and borrowing correctness
   - Safety and error handling
   - OpenGL usage and efficiency
   - Code organization and structure
4. **Teach concepts**: Explain the "why" behind suggestions
5. **Encourage experimentation**: Suggest challenges or variations
6. **Be patient**: Learning Rust's borrow checker takes time

### Code Review Guidelines:

- ✅ Praise good practices (ownership usage, safety, etc.)
- 🔍 Point out potential bugs or unsafe patterns
- 💡 Suggest idiomatic Rust alternatives
- 📚 Explain OpenGL concepts when needed
- 🎯 Keep feedback focused on current learning goals
- ⚠️ Watch for common pitfalls (unwrap abuse, unnecessary clones, etc.)

### Refactoring Guidance:

Follow the progressive structure outlined in `PROJECT_STRUCTURE.md`:
- **Phase 1**: Single file is OK
- **Phase 2**: Introduce modules (shader.rs, mesh.rs, camera.rs)
- **Phase 3**: Organize into subdirectories (graphics/, scene/)
- **Later phases**: Full library structure with lib.rs

Don't over-engineer early. Refactor when it makes sense pedagogically.

## Key Learning Goals

By the end of this project, the student should understand:

### Rust Concepts
- Ownership, borrowing, and lifetimes
- Pattern matching and error handling
- Traits and generics
- Modules and visibility
- Unsafe code (for OpenGL FFI)
- Performance optimization

### Graphics Programming
- OpenGL pipeline and state machine
- Vertex/fragment shaders (GLSL)
- Textures and framebuffers
- Lighting models (Phong)
- Transformations and matrices
- Advanced techniques (raymarching, instancing)

### Software Engineering
- Project structure and organization
- Incremental development
- Debugging graphics code
- Performance profiling
- Resource management

## Features to Build

The final RustGL engine will include:

**Graphics:**
- Mesh rendering (spheres, cubes, pyramids, terrain)
- Texture mapping and materials
- Phong lighting with multiple lights
- Skybox environments
- Water with reflections and refractions
- Normal mapping
- Volumetric clouds with raymarching
- Transparency and alpha blending
- Instanced rendering (thousands of objects)

**Procedural Generation:**
- 2D and 3D Perlin noise
- Fractal composition
- Dynamic terrain (1000x1000 vertices)

**Physics & Simulation:**
- Gravity and velocity
- Collision detection (ray-terrain)
- Particle systems
- Chain reactions

**Engine Features:**
- FPS camera system
- Input handling
- Scene graph management
- Debug visualization
- In-engine UI (egui)
- Performance monitoring

## Important Notes

1. **No Shortcuts**: The student should implement everything themselves. Provide guidance, not complete solutions.

2. **Platform Awareness**: Remember that advanced OpenGL 4.5 features won't work on macOS (limited to 4.1). Plan accordingly for later phases.

3. **Iterative Reviews**: Code review after each step before proceeding. This ensures solid foundations.

4. **Encourage Questions**: The student should ask when concepts are unclear. Rust and OpenGL both have steep learning curves.

5. **Celebrate Progress**: Building a game engine is hard! Acknowledge achievements along the way.

## Reference Documents

When the student needs guidance, refer them to:
- `docs/STEPS_SUMMARY.md` - Quick overview of all steps
- `docs/ROADMAP.md` - Detailed feature roadmap
- `docs/PROJECT_STRUCTURE.md` - Code organization patterns
- Specific step guides in `docs/phase-XX-*/step-YY-*.md`

## Communication Style

- Be encouraging and patient
- Explain concepts clearly
- Use examples to illustrate points
- Ask clarifying questions when needed
- Celebrate their progress and working code
- Point out both strengths and areas for improvement

## Remember

This is a **learning journey**, not a race. The goal is deep understanding of Rust through building something real and exciting. The game engine is the vehicle; Rust mastery is the destination.
