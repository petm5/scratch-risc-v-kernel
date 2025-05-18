fn main() {
    println!("cargo:rustc-link-arg-bins=-Tlink_script.ld");
}
