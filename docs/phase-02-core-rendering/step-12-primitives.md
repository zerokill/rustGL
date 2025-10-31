# Step 12: Geometric Primitives

**Phase:** 2 - Core Rendering
**Difficulty:** Intermediate
**Estimated Time:** 2-3 hours

## Goal

Create procedurally generated 3D geometric primitives (sphere, cylinder, torus, plane) using indexed rendering.

## What You'll Learn

- Procedural mesh generation
- UV sphere generation using spherical coordinates
- Cylinder generation with caps
- Torus generation using parametric equations
- Creating reusable primitive functions
- Understanding vertex topology and index patterns
- Proper normal calculation for primitives

## Background

So far, you've created simple shapes like triangles and cubes by manually defining vertices. But what about complex shapes like spheres with hundreds of vertices? Writing all those coordinates by hand would be tedious and error-prone.

**The problem:**
- Complex shapes like spheres require many vertices
- Manual vertex definition is time-consuming and error-prone
- Hard to adjust quality (more/fewer triangles)

**The solution: Procedural Generation**

Generate vertices mathematically using formulas:
- **Sphere** - Use spherical coordinates (¸, Æ)
- **Cylinder** - Circular cross-sections at different heights
- **Torus** - Revolve a circle around a circle
- **Plane** - Simple quad for ground/floors

### Why Indexed Rendering?

Primitives have many shared vertices. Without indexing:
- A sphere with 32 segments needs ~6,000 vertices (lots of duplicates!)
- With indexing: Only ~500 unique vertices

```
Without indices:        With indices:
Triangle 1: A, B, C    Vertices: [A, B, C, D]
Triangle 2: C, B, D    Indices: [0,1,2, 2,1,3]
= 6 vertices           = 4 vertices + 6 indices
```

## Implementation

### 1. Sphere Generation

A UV sphere is created by "slicing" the sphere horizontally (rings/latitude) and vertically (segments/longitude).

**Mathematical foundation:**
```
Spherical coordinates:
¸ (theta) = latitude angle (0 to À, top to bottom)
Æ (phi) = longitude angle (0 to 2À, around equator)

Cartesian conversion:
x = cos(Æ) × sin(¸)
y = cos(¸)
z = sin(Æ) × sin(¸)
```

Add this to `src/mesh.rs`:

```rust
/// Creates a UV sphere mesh using indexed rendering
///
/// # Arguments
/// * `radius` - Sphere radius
/// * `segments` - Number of horizontal divisions (longitude)
/// * `rings` - Number of vertical divisions (latitude)
/// * `color` - RGB color for all vertices
pub fn sphere(radius: f32, segments: u32, rings: u32, color: [f32; 3]) -> Self {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Generate vertices
    for ring in 0..=rings {
        let theta = ring as f32 * std::f32::consts::PI / rings as f32;
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        for seg in 0..=segments {
            let phi = seg as f32 * 2.0 * std::f32::consts::PI / segments as f32;
            let sin_phi = phi.sin();
            let cos_phi = phi.cos();

            // Spherical to Cartesian coordinates
            let x = cos_phi * sin_theta;
            let y = cos_theta;
            let z = sin_phi * sin_theta;

            // Position and normal (for a sphere, normal = normalized position)
            let position = [x * radius, y * radius, z * radius];
            let normal = [x, y, z];

            vertices.push(Vertex::new(position, color, normal));
        }
    }

    // Generate indices
    for ring in 0..rings {
        for seg in 0..segments {
            let current_ring_start = ring * (segments + 1);
            let next_ring_start = (ring + 1) * (segments + 1);

            let current = current_ring_start + seg;
            let next = current_ring_start + seg + 1;
            let current_below = next_ring_start + seg;
            let next_below = next_ring_start + seg + 1;

            // First triangle
            indices.push(current);
            indices.push(current_below);
            indices.push(next);

            // Second triangle
            indices.push(next);
            indices.push(current_below);
            indices.push(next_below);
        }
    }

    Mesh::new_indexed(&vertices, &indices)
}
```

**Key points:**
- Uses `0..=rings` (inclusive) to complete the sphere top-to-bottom
- Each ring has `segments + 1` vertices (last vertex connects back to first)
- Creates 2 triangles per "quad" in the grid
- Normal = position vector (normalized) for spheres

### 2. Cylinder Generation

A cylinder consists of three parts:
1. Side (vertical surface)
2. Top cap (circle)
3. Bottom cap (circle)

```rust
/// Creates a cylinder mesh using indexed rendering
///
/// # Arguments
/// * `radius` - Cylinder radius
/// * `height` - Cylinder height
/// * `segments` - Number of segments around the circumference
/// * `color` - RGB color for all vertices
pub fn cylinder(radius: f32, height: f32, segments: u32, color: [f32; 3]) -> Self {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let half_height = height / 2.0;

    // Generate vertices for top and bottom circles
    for i in 0..=1 {
        let y = if i == 0 { -half_height } else { half_height };
        let normal_y = if i == 0 { -1.0 } else { 1.0 };

        for seg in 0..=segments {
            let theta = seg as f32 * 2.0 * std::f32::consts::PI / segments as f32;
            let x = theta.cos() * radius;
            let z = theta.sin() * radius;

            vertices.push(Vertex::new([x, y, z], color, [0.0, normal_y, 0.0]));
        }
    }

    // Generate vertices for the side
    for i in 0..=1 {
        let y = if i == 0 { -half_height } else { half_height };

        for seg in 0..=segments {
            let theta = seg as f32 * 2.0 * std::f32::consts::PI / segments as f32;
            let x = theta.cos() * radius;
            let z = theta.sin() * radius;
            let nx = theta.cos();
            let nz = theta.sin();

            vertices.push(Vertex::new([x, y, z], color, [nx, 0.0, nz]));
        }
    }

    // Generate indices for sides
    let side_start = (segments + 1) * 2;
    for seg in 0..segments {
        let current = side_start + seg;
        let next = side_start + seg + 1;
        let current_top = side_start + (segments + 1) + seg;
        let next_top = side_start + (segments + 1) + seg + 1;

        indices.push(current);
        indices.push(next);
        indices.push(current_top);

        indices.push(current_top);
        indices.push(next);
        indices.push(next_top);
    }

    // Generate indices for top and bottom caps
    // Bottom cap (center vertex)
    let bottom_center_idx = vertices.len() as u32;
    vertices.push(Vertex::new([0.0, -half_height, 0.0], color, [0.0, -1.0, 0.0]));

    for seg in 0..segments {
        indices.push(bottom_center_idx);
        indices.push(seg + 1);
        indices.push(seg);
    }

    // Top cap (center vertex)
    let top_center_idx = vertices.len() as u32;
    vertices.push(Vertex::new([0.0, half_height, 0.0], color, [0.0, 1.0, 0.0]));

    let top_start = segments + 1;
    for seg in 0..segments {
        indices.push(top_center_idx);
        indices.push(top_start + seg);
        indices.push(top_start + seg + 1);
    }

    Mesh::new_indexed(&vertices, &indices)
}
```

**Key points:**
- Separate vertices for caps (normals point up/down) and sides (normals point outward)
- Center vertex for each cap to create triangle fan
- Side normals = horizontal direction (y = 0)

### 3. Torus Generation

A torus is created by revolving a circle (minor radius) around another circle (major radius).

```rust
/// Creates a torus mesh using indexed rendering
///
/// # Arguments
/// * `major_radius` - Distance from center of torus to center of tube
/// * `minor_radius` - Radius of the tube
/// * `major_segments` - Number of segments around the major circle
/// * `minor_segments` - Number of segments around the tube
/// * `color` - RGB color for all vertices
pub fn torus(major_radius: f32, minor_radius: f32, major_segments: u32, minor_segments: u32, color: [f32; 3]) -> Self {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Generate vertices
    for i in 0..=major_segments {
        let u = i as f32 * 2.0 * std::f32::consts::PI / major_segments as f32;
        let cos_u = u.cos();
        let sin_u = u.sin();

        for j in 0..=minor_segments {
            let v = j as f32 * 2.0 * std::f32::consts::PI / minor_segments as f32;
            let cos_v = v.cos();
            let sin_v = v.sin();

            let x = (major_radius + minor_radius * cos_v) * cos_u;
            let y = minor_radius * sin_v;
            let z = (major_radius + minor_radius * cos_v) * sin_u;

            let nx = cos_v * cos_u;
            let ny = sin_v;
            let nz = cos_v * sin_u;

            vertices.push(Vertex::new([x, y, z], color, [nx, ny, nz]));
        }
    }

    // Generate indices
    for i in 0..major_segments {
        for j in 0..minor_segments {
            let first = i * (minor_segments + 1) + j;
            let second = first + minor_segments + 1;

            indices.push(first);
            indices.push(second);
            indices.push(first + 1);

            indices.push(second);
            indices.push(second + 1);
            indices.push(first + 1);
        }
    }

    Mesh::new_indexed(&vertices, &indices)
}
```

**Key points:**
- Two parameters: `u` (around major circle), `v` (around tube)
- Position = major circle center + tube circle offset
- Similar index pattern to sphere

### 4. Plane Generation

A simple quad for ground/floors.

```rust
/// Creates a plane mesh using indexed rendering
///
/// # Arguments
/// * `width` - Plane width
/// * `depth` - Plane depth
/// * `color` - RGB color for all vertices
pub fn plane(width: f32, depth: f32, color: [f32; 3]) -> Self {
    let half_width = width / 2.0;
    let half_depth = depth / 2.0;
    let normal = [0.0, 1.0, 0.0];

    let vertices = vec![
        Vertex::new([-half_width, 0.0, -half_depth], color, normal),
        Vertex::new([half_width, 0.0, -half_depth], color, normal),
        Vertex::new([half_width, 0.0, half_depth], color, normal),
        Vertex::new([-half_width, 0.0, half_depth], color, normal),
    ];

    let indices = vec![0, 1, 2, 2, 3, 0];

    Mesh::new_indexed(&vertices, &indices)
}
```

### 5. Update main.rs

Create a scene showcasing all primitives:

```rust
// Create all primitive shapes
let sphere = Mesh::sphere(1.0, 32, 16, [0.3, 0.7, 1.0]);  // Blue sphere
let cube = Mesh::cube([1.0, 0.5, 0.2]);  // Orange cube
let cylinder = Mesh::cylinder(0.5, 2.0, 32, [0.2, 1.0, 0.3]);  // Green cylinder
let torus = Mesh::torus(1.0, 0.3, 32, 16, [1.0, 0.3, 0.7]);  // Pink torus
let plane = Mesh::plane(10.0, 10.0, [0.3, 0.3, 0.3]);  // Gray plane
```

Update the render function:

```rust
fn render(
    window: &mut glfw::Window,
    sphere: &Mesh,
    cube: &Mesh,
    cylinder: &Mesh,
    torus: &Mesh,
    plane: &Mesh,
    shader: &Shader,
    camera: &Camera,
    time: f32,
) {
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::ClearColor(0.1, 0.1, 0.2, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

        shader.use_program();

        let view = camera.get_view_matrix();
        shader.set_mat4("view", &view);

        let projection = glm::perspective(
            1024.0 / 768.0,
            camera.zoom.to_radians(),
            0.1,
            100.0,
        );
        shader.set_mat4("projection", &projection);

        // Draw plane (ground)
        let mut model = glm::Mat4::identity();
        model = glm::translate(&model, &glm::vec3(0.0, -2.0, 0.0));
        shader.set_mat4("model", &model);
        plane.draw();

        // Draw sphere (left, rotating)
        let mut model = glm::Mat4::identity();
        model = glm::translate(&model, &glm::vec3(-4.0, 0.0, 0.0));
        model = glm::rotate(&model, time * 0.5, &glm::vec3(0.0, 1.0, 0.0));
        model = glm::rotate(&model, time * 0.3, &glm::vec3(1.0, 0.0, 0.0));
        shader.set_mat4("model", &model);
        sphere.draw();

        // Draw cube (center-left, rotating)
        let mut model = glm::Mat4::identity();
        model = glm::translate(&model, &glm::vec3(-2.0, 0.0, 0.0));
        model = glm::rotate(&model, time * 0.7, &glm::vec3(1.0, 1.0, 0.0));
        shader.set_mat4("model", &model);
        cube.draw();

        // Draw cylinder (center, rotating)
        let mut model = glm::Mat4::identity();
        model = glm::translate(&model, &glm::vec3(0.0, 0.0, 0.0));
        model = glm::rotate(&model, time * 0.4, &glm::vec3(0.0, 1.0, 0.0));
        model = glm::rotate(&model, time * 0.3, &glm::vec3(1.0, 0.0, 0.0));
        shader.set_mat4("model", &model);
        cylinder.draw();

        // Draw torus (center-right, rotating)
        let mut model = glm::Mat4::identity();
        model = glm::translate(&model, &glm::vec3(2.0, 0.0, 0.0));
        model = glm::rotate(&model, time * 0.6, &glm::vec3(1.0, 0.5, 0.0));
        shader.set_mat4("model", &model);
        torus.draw();

        // Draw another sphere (right, different rotation)
        let mut model = glm::Mat4::identity();
        model = glm::translate(&model, &glm::vec3(4.0, 0.0, 0.0));
        model = glm::rotate(&model, time * 0.8, &glm::vec3(0.5, 1.0, 0.5));
        model = glm::scale(&model, &glm::vec3(0.8, 0.8, 0.8));
        shader.set_mat4("model", &model);
        sphere.draw();
    }
    window.swap_buffers();
}
```

## Testing

Build and run:

```bash
cargo build
cargo run
```

You should see:
- A gray plane as the ground
- Five rotating primitives in a row
- Use WASD + Q/E to fly around the scene
- Each primitive should be smooth and properly 3D

## Common Issues

### Sphere looks like a saddle
- Check theta range is 0 to À (not -À/2 to À/2)
- Ensure you're using `sin_theta` for x and z components
- Verify index winding order

### Cylinder caps missing
- Make sure you're creating center vertices for caps
- Check indices reference correct vertex ranges
- Verify normals point up/down for caps

### Torus looks wrong
- Ensure major_radius > minor_radius
- Check both parameter ranges go 0 to 2À
- Verify the position formula includes both radii

### Missing depth testing
You must enable depth testing or back faces will render in front:
```rust
gl::Enable(gl::DEPTH_TEST);
gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
```

## Understanding the Math

### Sphere: Spherical Coordinates
```
¸ (theta): Latitude angle
  - 0 = North pole (top)
  - À/2 = Equator
  - À = South pole (bottom)

Æ (phi): Longitude angle
  - 0 = Prime meridian
  - 2À = Full circle around
```

### Cylinder: Circular Cross-Sections
```
For each height (y):
  For each angle (¸):
    x = cos(¸) × radius
    z = sin(¸) × radius
```

### Torus: Double Revolution
```
Major circle (u): Around center
Minor circle (v): Around tube

Position = major_center + tube_offset
```

## Quality vs Performance

Adjusting segment/ring counts:

| Primitive | Low Quality | Medium Quality | High Quality |
|-----------|-------------|----------------|--------------|
| Sphere    | 16×8        | 32×16          | 64×32        |
| Cylinder  | 16 segments | 32 segments    | 64 segments  |
| Torus     | 16×8        | 32×16          | 48×24        |

**Rule of thumb:**
- Distant objects: Low quality
- Player focus: Medium quality
- Hero objects: High quality

## Exercises

1. **Cone** - Create a cone primitive (like cylinder but top radius = 0)
2. **Capsule** - Create a capsule (cylinder with hemisphere caps)
3. **UV Coordinates** - Add texture coordinates (u, v) to primitives
4. **LOD System** - Switch quality based on distance from camera
5. **Wireframe Mode** - Add ability to render primitives as wireframes

## Next Steps

Now that you have primitives, you can:
- **Step 13: Textures** - Add images/patterns to surfaces
- **Step 14: Lighting** - Make primitives react to light sources
- Build complex scenes by combining primitives
- Create a primitive library for game objects

## Key Takeaways

- Procedural generation creates complex shapes mathematically
- Indexed rendering saves memory by reusing vertices
- Spherical coordinates convert angles to 3D positions
- Different primitives need different normal calculations
- Quality parameters let you trade performance for visual fidelity

## File Locations

- `src/mesh.rs:150` - Sphere generation
- `src/mesh.rs:204` - Cylinder generation
- `src/mesh.rs:288` - Torus generation
- `src/mesh.rs:342` - Plane generation
- `src/main.rs:66` - Creating primitives
- `src/main.rs:190` - Rendering primitives

---

**Completed Step 12!** You now have a library of procedural 3D primitives.
