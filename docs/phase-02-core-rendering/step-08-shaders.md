# Step 08: Shaders (Advanced)

**Phase:** 2 - Core Rendering
**Difficulty:** Intermediate
**Estimated Time:** 1-2 hours

## Goal

Improve shader management by creating a `Shader` struct and **refactor your code into modules** - your first lesson in Rust code organization!

## What You'll Learn

- Creating a reusable `Shader` struct
- Loading shader source from files
- **Your first Rust module** (`shader.rs`)
- The `mod` keyword and module system
- Reading files with `std::fs`
- Rust error handling with `Result`

## Background

In Step 07, you embedded shaders as strings in `main.rs`. This works but has problems:
- Hard to edit shaders (no syntax highlighting)
- Can't share shaders between programs
- `main.rs` gets cluttered

**Professional approach:**
1. Store shaders as `.glsl` files
2. Create a `Shader` struct to manage them
3. **Move shader code to its own module** (Rust best practice!)

This is also your **first refactoring** - learning when and how to organize Rust code properly.

## Task Overview

1. Create shader files (`.vert` and `.frag`)
2. Create `src/shader.rs` module
3. Update `main.rs` to use the module
4. See your first colored triangle!

**Note:** See [PROJECT_STRUCTURE.md](../../PROJECT_STRUCTURE.md#phase-2-core-rendering-steps-6-12) for details on Phase 2 code organization.

## Success Criteria

- [ ] You've created `src/shader.rs`
- [ ] Shaders load from `.glsl` files
- [ ] Triangle has interpolated RGB colors
- [ ] You understand the `mod` keyword
- [ ] Code is organized into modules

## Next Step

Great work! You've learned Rust's module system. Next: [Step 09: Mesh Structure](./step-09-mesh-structure.md), where you'll create a reusable `Mesh` struct!

## Notes

- **This is your first refactoring!** You learned when to split code into modules
- Real Rust projects use modules extensively - start building this habit now
- See [PROJECT_STRUCTURE.md](../../PROJECT_STRUCTURE.md) for the complete organization strategy
- As your project grows, you'll refactor into subdirectories (Phase 3+)
