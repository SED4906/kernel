fn main() {
    // Tell cargo to pass the linker script to the linker..
    println!("cargo:rustc-link-arg=-Tx86_64-unknown-none-linker.ld");
    // ..and to re-run if it changes.
    println!("cargo:rerun-if-changed=x86_64-unknown-none-linker.ld");
}