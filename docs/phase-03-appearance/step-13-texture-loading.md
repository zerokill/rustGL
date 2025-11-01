# Step 13: Texture Loading

**Phase 3: Appearance** | **Estimated Time:** 1-2 hours

## Goals

In this step, you will:
- Learn how to load image files (PNG, JPG) from disk
- Create OpenGL textures and upload image data to the GPU
- Understand texture parameters (filtering, wrapping)
- Build a reusable `Texture` struct

## Background

### What are Textures?

Textures are images that we apply to 3D surfaces to add detail without increasing geometry complexity. Instead of modeling every brick in a wall, we can use a brick texture image.

**Key Concepts:**
- **Texture Units**: OpenGL has multiple texture slots (usually 16+) that can be bound simultaneously
- **Texture Coordinates (UV)**: 2D coordinates (0.0 to 1.0) that map texture pixels to mesh vertices
- **Mipmaps**: Pre-calculated lower resolution versions for distant objects
- **Filtering**: How texture pixels are sampled (nearest, linear, etc.)
- **Wrapping**: What happens at texture edges (repeat, clamp, mirror)

### OpenGL Texture Pipeline

```
1. Load image from disk (CPU)
2. Generate texture ID with glGenTextures
3. Bind texture with glBindTexture
4. Upload data with glTexImage2D
5. Set parameters (filtering, wrapping)
6. Optionally generate mipmaps
7. Use in shaders by binding to texture units
```

## Prerequisites

You should have completed:
-  Step 12: Primitives (sphere, cube, pyramid)
-  Basic shader and mesh rendering working

## Tasks

### Task 1: Add Image Loading Dependency

Add the `image` crate to your `Cargo.toml`:

```toml
[dependencies]
glfw = "0.54"
gl = "0.14"
glam = "0.24"
image = "0.24"  # NEW: For loading PNG/JPG/BMP files
```

The `image` crate handles decoding various image formats into raw pixel data.

**Run** `cargo build` to download the dependency.

### Task 2: Create a Texture Struct

Create a new file `src/texture.rs` (or `src/graphics/texture.rs` if you're using subdirectories):

```rust
use gl::types::*;
use std::path::Path;
use image::{DynamicImage, GenericImageView};

pub struct Texture {
    pub id: GLuint,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    pub fn new(path: &str) -> Result<Self, String> {
        // TODO: Load image from file
        // TODO: Generate OpenGL texture
        // TODO: Upload data to GPU
        unimplemented!()
    }

    pub fn bind(&self, texture_unit: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + texture_unit);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
```

**Key Points:**
- `id`: OpenGL texture handle
- `bind()`: Activates texture on a specific unit
- `Drop`: Cleans up GPU resources when texture goes out of scope

### Task 3: Implement Image Loading

Fill in the `new()` method:

```rust
pub fn new(path: &str) -> Result<Self, String> {
    // 1. Load image from disk
    let img = image::open(Path::new(path))
        .map_err(|e| format!("Failed to load texture {}: {}", path, e))?;

    // 2. Convert to RGBA8 format (required by OpenGL)
    let img = img.to_rgba8();
    let (width, height) = img.dimensions();
    let data = img.into_raw();

    // 3. Generate OpenGL texture
    let mut id: GLuint = 0;
    unsafe {
        gl::GenTextures(1, &mut id);
        gl::BindTexture(gl::TEXTURE_2D, id);

        // 4. Upload pixel data to GPU
        gl::TexImage2D(
            gl::TEXTURE_2D,           // Target
            0,                        // Mipmap level (0 = base)
            gl::RGBA as GLint,        // Internal format
            width as GLint,
            height as GLint,
            0,                        // Border (must be 0)
            gl::RGBA,                 // Format of data
            gl::UNSIGNED_BYTE,        // Type of data
            data.as_ptr() as *const _, // Pointer to data
        );

        // 5. Set texture parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

        // 6. Generate mipmaps
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }

    Ok(Texture { id, width, height })
}
```

**Understanding the Parameters:**

- **TEXTURE_WRAP_S/T**: What happens at edges
  - `REPEAT`: Tiles the texture (default)
  - `CLAMP_TO_EDGE`: Stretches edge pixels
  - `MIRRORED_REPEAT`: Mirrors the texture

- **TEXTURE_MIN_FILTER**: How to sample when texture is far away
  - `LINEAR`: Smooth interpolation
  - `LINEAR_MIPMAP_LINEAR`: Trilinear filtering (best quality)

- **TEXTURE_MAG_FILTER**: How to sample when texture is close up
  - `NEAREST`: Pixelated look
  - `LINEAR`: Smooth

### Task 4: Declare Module and Test

In your `main.rs` (or `lib.rs`):

```rust
mod texture;
use texture::Texture;
```

### Task 5: Find or Create Test Textures

You'll need some test images. Options:

1. **Download free textures** from:
   - [OpenGameArt.org](https://opengameart.org/)
   - [Textures.com](https://www.textures.com/) (free with signup)

2. **Create simple test images** using any image editor (save as PNG)

3. **Use a placeholder pattern** - I can help you generate one programmatically

Create a `resources/textures/` directory:

```bash
mkdir -p resources/textures
```

Download or place a test texture there (e.g., `resources/textures/test.png`).

### Task 6: Load and Verify Texture

In your `main()` function, try loading a texture:

```rust
fn main() {
    // ... your existing window/OpenGL setup ...

    // Load a test texture
    let texture = Texture::new("resources/textures/test.png")
        .expect("Failed to load texture");

    println!("Loaded texture: {}x{} (ID: {})",
        texture.width, texture.height, texture.id);

    // ... your render loop ...
}
```

**Expected Output:**
```
Loaded texture: 512x512 (ID: 1)
```

If it loads successfully, you're ready to apply it in the next step!

## Success Criteria

You have completed this step when:

-  The `image` crate is added to dependencies
-  You have a `Texture` struct with `new()` and `bind()` methods
-  You can load a PNG or JPG file from disk
-  OpenGL texture is created and data uploaded
-  No OpenGL errors (check with `glGetError()` if unsure)
-  Texture info is printed correctly

## Testing

**Check for OpenGL Errors:**

Add this helper function:

```rust
fn check_gl_error(location: &str) {
    unsafe {
        let err = gl::GetError();
        if err != gl::NO_ERROR {
            eprintln!("OpenGL error at {}: 0x{:x}", location, err);
        }
    }
}
```

Call it after texture creation:
```rust
let texture = Texture::new("resources/textures/test.png")?;
check_gl_error("After texture loading");
```

## Common Issues

### Issue 1: "No such file or directory"

**Problem:** Image path is wrong.

**Solution:**
- Use relative paths from your project root
- Check the file actually exists: `ls resources/textures/`
- Try an absolute path for testing

### Issue 2: "Failed to load texture: Unsupported image format"

**Problem:** Image format not supported.

**Solution:**
- Use PNG or JPG (most reliable)
- Check the file isn't corrupted
- Try opening it in an image viewer first

### Issue 3: Black texture or no visible texture

**Problem:** Texture might be loading but not applied yet.

**Solution:**
- This is expected! We haven't modified shaders to sample textures yet
- That comes in Step 14: Texture Mapping
- For now, just verify it loads without errors

### Issue 4: Upside-down texture later

**Problem:** OpenGL's Y-axis is inverted compared to most image formats.

**Solution:**
- Add `.flipv()` after loading: `let img = img.to_rgba8().flipv();`
- Or handle in texture coordinates (next step)

## Understanding Check

Before moving on, make sure you understand:

1. **What is a texture?**
   - An image stored in GPU memory that can be applied to geometry

2. **What are texture units?**
   - OpenGL has multiple slots (TEXTURE0, TEXTURE1, etc.) for binding different textures

3. **What are mipmaps?**
   - Pre-generated lower resolution versions used for distant objects (improves performance and quality)

4. **What's the difference between MIN and MAG filtering?**
   - MIN: When texture is smaller on screen (far away)
   - MAG: When texture is larger on screen (close up)

5. **Why use `to_rgba8()`?**
   - OpenGL expects consistent format; RGBA8 is widely supported and predictable

## Next Steps

In **Step 14: Texture Mapping**, you will:
- Add UV coordinates to your mesh vertices
- Modify shaders to sample textures
- Apply textures to your 3D primitives
- See your spheres, cubes, and pyramids with real textures!

## Resources

- [OpenGL Texture Tutorial](https://learnopengl.com/Getting-started/Textures)
- [Rust `image` crate docs](https://docs.rs/image/latest/image/)
- [Free Texture Sources](https://opengameart.org/)

---

**Ready to proceed?** Once you've successfully loaded a texture, ask for a code review and we'll move on to Step 14: Texture Mapping!
