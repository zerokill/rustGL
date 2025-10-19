# Step 01: Hello Rust

**Phase:** 1 - Foundation
**Difficulty:** Beginner
**Estimated Time:** 30 minutes

## Goal

Write and run your first Rust program to verify your development environment is set up correctly.

## What You'll Learn

- How to install Rust
- Running Rust programs with `rustc`
- Basic Rust syntax
- The `println!` macro
- String literals

## Background

Before diving into game engine development, you need to ensure Rust is properly installed and you understand the absolute basics. This step is intentionally simple - we're just making sure everything works!

In Rust:
- Programs start executing in the `main` function
- `println!` is a macro (note the `!`) that prints to the console
- Strings are written in double quotes
- Statements end with semicolons

## Task

### 1. Install Rust

If you haven't already, install Rust using rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the installation prompts. After installation, verify it worked:

```bash
rustc --version
```

You should see something like `rustc 1.XX.X (...)`.

### 2. Write Your First Program

Create a file called `hello.rs` with the following content:

```rust
fn main() {
    println!("Hello, Rust!");
}
```

### 3. Compile and Run

Compile your program:

```bash
rustc hello.rs
```

This creates an executable called `hello` (or `hello.exe` on Windows).

Run it:

```bash
./hello
```

You should see: `Hello, Rust!`

## Challenges

Once you've got the basic version working, try these modifications:

1. **Multiple Lines:** Print 3 different messages
2. **Variables:** Create a variable to store your name and print it:
   ```rust
   let name = "Your Name";
   println!("Hello, {}!", name);
   ```
3. **Formatting:** Print a calculation result:
   ```rust
   let result = 2 + 2;
   println!("2 + 2 = {}", result);
   ```

## Success Criteria

- [ ] Rust is installed and `rustc --version` works
- [ ] You've written a `hello.rs` file
- [ ] The program compiles without errors
- [ ] Running the program prints "Hello, Rust!" to the console
- [ ] (Optional) You've completed at least one challenge

## Common Issues

**"rustc: command not found"**
- Restart your terminal after installing Rust
- Or manually add Rust to your PATH: `source $HOME/.cargo/env`

**"Permission denied"**
- On Unix systems, you may need to make the executable: `chmod +x hello`

**Syntax errors**
- Check that you have a semicolon after `println!(...)`
- Make sure strings are in double quotes, not single quotes
- Verify `println!` has an exclamation mark (it's a macro)

## Next Step

Once your program runs successfully and you're comfortable with the basics, you're ready for [Step 02: Cargo Project](./step-02-cargo-project.md), where you'll learn about Rust's project management tool!

## Notes

- Don't worry if Rust syntax looks unfamiliar - you'll learn as you go
- The `!` in `println!` means it's a macro, not a regular function
- Rust files use the `.rs` extension
- We're compiling directly with `rustc` for learning purposes, but real projects use Cargo (next step!)
