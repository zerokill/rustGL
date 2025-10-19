# Step 02: Cargo Project

**Phase:** 1 - Foundation
**Difficulty:** Beginner
**Estimated Time:** 30 minutes

## Goal

Create a proper Rust project using Cargo, Rust's build system and package manager.

## What You'll Learn

- What Cargo is and why it's important
- How to create a new Cargo project
- Understanding `Cargo.toml` (the manifest file)
- Project structure conventions
- The difference between `cargo build` and `cargo run`

## Background

In Step 01, you compiled a single Rust file with `rustc`. This works for tiny programs, but real projects need:

- **Dependency management** - Using external libraries (crates)
- **Build system** - Compiling multiple files together
- **Project organization** - Standard directory structure
- **Testing** - Running tests easily
- **Documentation** - Generating docs

Cargo handles all of this! It's similar to:
- npm/yarn (JavaScript)
- pip (Python)
- maven/gradle (Java)
- Swift Package Manager (Swift)

## Task

### 1. Verify Cargo is Installed

Cargo comes with Rust, so it should already be available:

```bash
cargo --version
```

You should see something like `cargo 1.XX.X (...)`.

### 2. Create a New Project

Navigate to where you want to create your RustGL project (probably the parent directory of this `docs` folder), then create a new project:

```bash
cargo new rustgl
cd rustgl
```

This creates a directory structure:

```
rustgl/
├── Cargo.toml      # Project configuration (like package.json)
├── src/
│   └── main.rs     # Your main source file
└── .gitignore      # Git ignore file
```

### 3. Examine Cargo.toml

Open `Cargo.toml` and examine its contents:

```toml
[package]
name = "rustgl"
version = "0.1.0"
edition = "2021"

[dependencies]
```

- `[package]` - Metadata about your project
- `name` - The project name
- `version` - Semantic version (major.minor.patch)
- `edition` - Rust edition (2021 is the latest stable)
- `[dependencies]` - External crates (empty for now)

### 4. Examine main.rs

Open `src/main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

Cargo has already created a Hello World program for you!

### 5. Build and Run

**Option A: Build then run separately**
```bash
cargo build
./target/debug/rustgl
```

**Option B: Build and run in one command** (recommended)
```bash
cargo run
```

You should see: `Hello, world!`

### 6. Understand the Build Output

After building, notice the new directories:

```
rustgl/
├── Cargo.lock      # Dependency lock file (auto-generated)
├── target/
│   └── debug/
│       └── rustgl  # Your compiled executable
```

- `Cargo.lock` - Locks dependency versions for reproducibility
- `target/debug/` - Debug build output (unoptimized, fast to compile)
- `target/release/` - Release build output (optimized, slower to compile)

## Challenges

1. **Modify the message:** Change "Hello, world!" to "Hello, RustGL!"
2. **Release build:** Try `cargo build --release` and compare the executable sizes:
   ```bash
   ls -lh target/debug/rustgl
   ls -lh target/release/rustgl
   ```
   Release builds are much smaller and faster!

3. **Clean build:** Run `cargo clean` to remove the `target/` directory, then rebuild

## Success Criteria

- [ ] You've created a new Cargo project
- [ ] You understand what `Cargo.toml` contains
- [ ] You can run your program with `cargo run`
- [ ] You see "Hello, world!" (or your custom message) printed
- [ ] You understand the difference between debug and release builds

## Common Issues

**"cargo: command not found"**
- Restart your terminal after installing Rust
- Or run: `source $HOME/.cargo/env`

**Permission errors**
- Make sure you have write permissions in the directory
- Avoid creating projects in system directories

## Next Step

Now that you have a proper Rust project, you're ready for [Step 03: Dependencies](./step-03-dependencies.md), where you'll learn to use external libraries!

## Notes

- Always use `cargo run` for development (it's faster than building then running)
- Use `cargo build --release` only when you need maximum performance
- The `Cargo.lock` file should be committed to git for applications (but not for libraries)
- Cargo automatically manages the `target/` directory - you can delete it anytime
