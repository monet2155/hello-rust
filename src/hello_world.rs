fn main() {
    // 1. rust use 4 space without tab
    // 2. println! is macro call. If just function call, use println()
    println!("Hello, world!");
}

// rust's compile process
// 1. "rustc [filename]" to compile
// 2. "[filename].exe" and "[filename].pdb" was created
// 3. "[filename].exe" can run where not installed rust

// rustc working simple program.
// If project is getting bigger, deployment is going harder
// So we can use Cargo - Rust build system, package manager
