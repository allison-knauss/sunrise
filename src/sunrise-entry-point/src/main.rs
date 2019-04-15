fn main() {
    println!("A documentation and workspace entry point for the Sunrise project.");
    println!("");
    println!("Build with `cargo build --all`");
    println!("Test with `cargo test --all`");
    println!("");
    println!("CARGO_MANIFEST_DIR: {:?}", env!("CARGO_MANIFEST_DIR"));
}
