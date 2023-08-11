fn main() {
    // If any file inside the "macros" directory changes, the build script will rerun.
    println!("cargo:rerun-if-changed=macros/src/");
}
