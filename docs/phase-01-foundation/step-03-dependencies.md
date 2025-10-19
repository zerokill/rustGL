# Step 03: Dependencies

**Phase:** 1 - Foundation
**Difficulty:** Beginner
**Estimated Time:** 30 minutes

## Goal

Learn to add and use external crates (Rust libraries) in your project.

## What You'll Learn

- How to find Rust crates on crates.io
- Adding dependencies to `Cargo.toml`
- Importing and using external code
- Understanding semantic versioning
- The Rust module system basics

## Background

One of Rust's strengths is its ecosystem of libraries (called "crates"). Rather than building everything from scratch, you'll use existing crates for:

- **Window creation:** `glfw` or `winit`
- **OpenGL bindings:** `gl`
- **Math:** `glam` or `nalgebra`
- **Image loading:** `image`
- **UI:** `egui`

Finding crates:
- **crates.io** - The official Rust package registry
- **docs.rs** - Automatic documentation for all crates
- **lib.rs** - Alternative crate search with better UI

## Task

### 1. Choose a Simple Crate to Practice

Let's use `rand` (random number generation) as a practice example.

Visit [crates.io](https://crates.io/) and search for "rand". You'll see:
- The crate name: `rand`
- The current version: e.g., `0.8.5`
- Documentation link
- Number of downloads

### 2. Add the Dependency

Edit your `Cargo.toml` file and add `rand` to the `[dependencies]` section:

```toml
[package]
name = "rustgl"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8"
```

**Version syntax:**
- `"0.8"` - Use the latest 0.8.x version (recommended)
- `"0.8.5"` - Use exactly version 0.8.5
- `"*"` - Use the absolute latest (not recommended!)

### 3. Use the Crate

Modify `src/main.rs` to use the `rand` crate:

```rust
use rand::Rng;  // Import the Rng trait

fn main() {
    let mut rng = rand::thread_rng();
    let number: u32 = rng.gen_range(1..=100);

    println!("Random number: {}", number);
}
```

**What's happening:**
- `use rand::Rng` - Imports the `Rng` trait (needed for the `gen_range` method)
- `rand::thread_rng()` - Creates a random number generator
- `gen_range(1..=100)` - Generates a number between 1 and 100 (inclusive)

### 4. Build and Run

```bash
cargo run
```

The first time you run this, Cargo will:
1. Download the `rand` crate and its dependencies
2. Compile them
3. Build your program
4. Run it

You should see a random number printed!

Run it a few more times to verify you get different numbers.

### 5. Examine Cargo.lock

After building, open `Cargo.lock`. You'll see exact versions of `rand` and all its dependencies. This file ensures everyone building your project gets the same versions.

## Understanding the Module System

Rust organizes code into modules. When you write:

```rust
use rand::Rng;
```

You're saying: "From the `rand` crate, import the `Rng` trait into scope."

Common patterns:
```rust
use std::collections::HashMap;  // Import a specific type
use rand::Rng;                   // Import a trait
use glam::{Vec3, Mat4};          // Import multiple items
```

## Challenges

1. **Multiple random numbers:** Generate and print 5 random numbers
2. **Random float:** Generate a random floating-point number between 0.0 and 1.0
   ```rust
   let float_num: f32 = rng.gen();
   ```
3. **Add another crate:** Try adding `colored` to print colored text:
   ```toml
   [dependencies]
   rand = "0.8"
   colored = "2.0"
   ```

   ```rust
   use colored::Colorize;

   fn main() {
       println!("{}", "Hello, RustGL!".green());
       println!("{}", "This is colored!".red().bold());
   }
   ```

## Success Criteria

- [ ] You've added `rand` to your `Cargo.toml`
- [ ] Your program compiles and runs
- [ ] You see a random number printed
- [ ] You understand how `use` statements work
- [ ] (Optional) You've tried the challenges

## Common Issues

**"failed to resolve: use of undeclared crate or module"**
- Check that you added the dependency to `Cargo.toml`
- Run `cargo clean` and try again
- Make sure you spelled the crate name correctly

**Version conflicts**
- If you get errors about incompatible versions, try updating: `cargo update`
- Or specify exact versions in `Cargo.toml`

**Compilation takes a long time**
- This is normal for the first build - Cargo is compiling all dependencies
- Subsequent builds are much faster thanks to incremental compilation

## Next Step

Great! You now know how to use external libraries. Next up: [Step 04: Window Creation](./step-04-window-creation.md), where you'll create your first graphical window!

## Notes

- Cargo caches downloaded crates in `~/.cargo/`
- You can update dependencies with `cargo update`
- Use `cargo tree` to see the full dependency graph
- The Rust ecosystem strongly favors semantic versioning
- Most game engines use `glam` for math - we'll add it later
