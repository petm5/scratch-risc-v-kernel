fn main() {
    println!("cargo:rustc-link-arg-bins=-Tlink_script.ld");
    println!("cargo:rustc-link-arg-bins=--oformat=binary")
}
