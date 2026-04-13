// =============================================================================
// build.rs — Cargo build script that compiles the C library and links it
// =============================================================================
//
// HOW IT WORKS:
//   Cargo runs this file *before* compiling the Rust crate.
//   We use the `cc` crate to:
//     1. Compile c_src/mathlib.c into an object file
//     2. Package it as a static library (libmathlib.a)
//     3. Tell Cargo where to find it and to link it
//
//   The `cc` crate emits the correct `cargo:rustc-link-*` directives
//   automatically, so we don't need to print them ourselves.

// fn main() {
//     // Tell cargo to re-run this build script if the C sources change
//     println!("cargo:rerun-if-changed=c_src/mathlib.c");
//     println!("cargo:rerun-if-changed=c_src/mathlib.h");

//     // Compile the C library
//     cc::Build::new()
//         .file("c_src/mathlib.c")      // source file to compile
//         .include("c_src")              // header search path
//         .warnings(true)                // enable compiler warnings
//         .flag_if_supported("-O2")      // optimise release builds
//         .compile("mathlib");           // produces libmathlib.a and links it

//     // The `cc` crate automatically emits:
//     //   cargo:rustc-link-lib=static=mathlib
//     //   cargo:rustc-link-search=native=<OUT_DIR>
// }

fn main() {

    println!("cargo:rerun-if-changed=c/math.c");
    println!("cargo:rerun-if-changed=c/math.h");

    cc::Build::new()
        .file("c/math.c")
        .include("c")
        .warnings(true)
        .flag_if_supported("-O2")
        .compile("math");
}
