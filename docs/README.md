# RustGL - Learning Rust Through Game Engine Development

Welcome to RustGL, a comprehensive learning journey where you'll rebuild a full-featured 3D game engine in Rust!

## About This Project

This project is designed to teach you Rust by recreating the SwiftGL game engine. You'll progress from writing your first "Hello, World!" in Rust to implementing advanced features like:

- Real-time terrain generation with Perlin noise
- Water with realistic reflections and refractions
- Volumetric cloud rendering with raymarching
- Physics simulation with collision detection
- Instanced rendering for thousands of objects
- And much more!

## Learning Approach

This is a **hands-on, guided learning experience**:

1. Each step includes a detailed explanation of what you need to implement
2. You write all the code yourself
3. When you complete a step, you can request a code review
4. Once approved, you move to the next step

The goal is to learn Rust through practical application, not just theory.

## Project Structure

```
rustgl/
├── docs/               # Learning materials (you are here!)
│   ├── phase-01-foundation/
│   ├── phase-02-core-rendering/
│   ├── phase-03-appearance/
│   ├── phase-04-advanced-effects/
│   ├── phase-05-procedural/
│   ├── phase-06-optimization/
│   ├── phase-07-volumetric/
│   └── phase-08-polish/
├── src/                # Your Rust source code
├── shaders/            # GLSL shader files
├── resources/          # Textures and assets
└── Cargo.toml          # Rust package configuration
```

## The Learning Path

The project is divided into 8 phases with 40+ progressive steps:

### Phase 1: Foundation (Steps 1-5)
Get comfortable with Rust basics and set up your development environment.

### Phase 2: Core Rendering (Steps 6-12)
Learn to render basic 3D graphics: triangles, meshes, and primitives.

### Phase 3: Appearance (Steps 13-17)
Add textures, materials, and lighting to make your scenes look good.

### Phase 4: Advanced Effects (Steps 18-23)
Implement skybox, water with reflections/refractions, and transparency.

### Phase 5: Procedural Generation (Steps 24-28)
Generate terrain using Perlin noise and fractal algorithms.

### Phase 6: Optimization (Steps 29-33)
Render thousands of objects efficiently with instancing and physics.

### Phase 7: Volumetric Rendering (Steps 34-37)
Create realistic volumetric clouds using raymarching techniques.

### Phase 8: Polish & Integration (Steps 38-42)
Build a complete engine with scene management, input, UI, and debugging tools.

## Getting Started

1. Start with the [ROADMAP.md](./ROADMAP.md) to see the complete feature list
2. Begin with [Phase 1, Step 1: Hello Rust](./phase-01-foundation/step-01-hello-rust.md)
3. Work through each step sequentially
4. Test your code after each step
5. Request a code review before moving on

## Prerequisites

- A computer (macOS, Linux, or Windows)
- Willingness to learn and experiment
- No prior Rust experience required!
- Basic programming knowledge helpful but not required

## Reference

This project is based on the SwiftGL engine, which demonstrates:
- Modern OpenGL 3.3+ rendering
- Advanced shader techniques
- Procedural generation
- Real-time physics simulation
- Complex visual effects

You'll learn to implement all of these features in Rust!

## Philosophy

Learning happens through:
1. **Understanding** - Read the step description
2. **Implementation** - Write the code yourself
3. **Testing** - Run and verify your code works
4. **Review** - Get feedback on your implementation
5. **Iteration** - Refine and improve

Don't rush! Take time to understand each concept before moving forward.

## Let's Begin!

Ready to start your Rust journey? Head to [Phase 1, Step 1: Hello Rust](./phase-01-foundation/step-01-hello-rust.md) and let's write some code!
