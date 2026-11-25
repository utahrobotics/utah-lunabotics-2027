fn main() {
    // teensy4-bsp build script; writes Teensy 4 memory
    // map & boot header consumed by this crates binaries
    println!("cargo:rustc-link-arg-bins=-Tt4link.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}
