// // =============================================================================
// // Exercise 075 — Foreign Function Interface (FFI)
// // =============================================================================
// //
// // This exercise demonstrates how Rust can call functions written in C.
// //
// // KEY CONCEPTS COVERED:
// //   1. Declaring extern "C" functions
// //   2. Passing scalar types (i32, f64)
// //   3. Passing C strings  (CString → *const c_char)
// //   4. Passing #[repr(C)] structs by pointer
// //   5. Passing slices as (pointer, length) pairs
// //   6. Passing Rust functions as C callbacks (function pointers)
// //   7. Reading global mutable state from C
// //
// // SAFETY NOTE:
// //   Every call to a C function is `unsafe` because Rust cannot verify:
// //     - null pointer dereference won't happen
// //     - buffer overflows won't happen
// //     - data races won't happen
// //   We must uphold these invariants ourselves.
// // =============================================================================

// use std::ffi::{CStr, CString};
// use std::os::raw::c_char;

// // ---------------------------------------------------------------------------
// // Step 1: Declare the C functions we want to call
// // ---------------------------------------------------------------------------
// // `extern "C"` tells Rust to use the C calling convention (cdecl).
// // Every signature here must EXACTLY match the C header.

// unsafe extern "C" {
//     // --- Scalars ---
//     fn add(a: i32, b: i32) -> i32;
//     fn multiply(a: i32, b: i32) -> i32;
//     fn divide_safe(numerator: f64, denominator: f64) -> f64;

//     // --- Strings ---
//     fn string_length(s: *const c_char) -> i32;
//     fn greet(name: *const c_char, out_buf: *mut c_char, buf_size: i32);

//     // --- Structs ---
//     fn point_distance(a: *const Point2D, b: *const Point2D) -> f64;
//     fn point_midpoint(a: *const Point2D, b: *const Point2D) -> Point2D;

//     // --- Arrays ---
//     fn array_sum(arr: *const i32, len: i32) -> i32;
//     fn array_double(arr: *mut i32, len: i32);

//     // --- Callbacks ---
//     fn array_transform(arr: *mut i32, len: i32, f: extern "C" fn(i32) -> i32);

//     // --- Global state ---
//     fn get_call_count() -> i32;
// }

// // ---------------------------------------------------------------------------
// // Step 2: Mirror the C struct layout in Rust
// // ---------------------------------------------------------------------------
// // `#[repr(C)]` guarantees the struct has the same memory layout as C.
// // Without it, Rust is free to reorder fields for optimal packing.

// #[repr(C)]
// #[derive(Debug, Clone, Copy)]
// struct Point2D {
//     x: f64,
//     y: f64,
// }

// // ---------------------------------------------------------------------------
// // Step 3: A Rust function with C calling convention (used as a callback)
// // ---------------------------------------------------------------------------
// // `extern "C"` on a Rust function makes it callable from C.

// extern "C" fn square(x: i32) -> i32 {
//     x * x
// }

// extern "C" fn negate(x: i32) -> i32 {
//     -x
// }

// // ---------------------------------------------------------------------------
// // Helper to print section banners
// // ---------------------------------------------------------------------------
// fn banner(title: &str) {
//     println!();
//     println!("╔══════════════════════════════════════════════════════════╗");
//     println!("║  {:<55}║", title);
//     println!("╚══════════════════════════════════════════════════════════╝");
// }

// // ---------------------------------------------------------------------------
// // Main — demonstrate every FFI pattern
// // ---------------------------------------------------------------------------
// fn main() {
//     println!("🦀 Rust ↔ C  Foreign Function Interface Demo");
//     println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

//     // -----------------------------------------------------------------------
//     // 1. Scalar arithmetic
//     // -----------------------------------------------------------------------
//     banner("1. Scalar Arithmetic");

//     unsafe {
//         let sum = add(40, 2);
//         println!("  add(40, 2)            = {sum}");

//         let product = multiply(6, 7);
//         println!("  multiply(6, 7)        = {product}");

//         let quotient = divide_safe(22.0, 7.0);
//         println!("  divide_safe(22, 7)    = {quotient:.6}");

//         let safe_div = divide_safe(1.0, 0.0);
//         println!("  divide_safe(1, 0)     = {safe_div}  (safe fallback)");
//     }

//     // -----------------------------------------------------------------------
//     // 2. C Strings
//     // -----------------------------------------------------------------------
//     banner("2. C Strings");

//     unsafe {
//         // Rust → C string: use CString (owned, null-terminated)
//         let rust_str = "Foreign Function Interface";
//         let c_str = CString::new(rust_str).expect("CString::new failed");

//         let len = string_length(c_str.as_ptr());
//         println!("  string_length(\"{rust_str}\") = {len}");

//         // C writes into a buffer we provide
//         let name = CString::new("Rustacean").unwrap();
//         let mut buf = vec![0u8; 128];
//         greet(name.as_ptr(), buf.as_mut_ptr() as *mut c_char, 128);

//         // C → Rust string: use CStr (borrowed, null-terminated)
//         let greeting = CStr::from_ptr(buf.as_ptr() as *const c_char);
//         println!("  greet(\"Rustacean\")    = \"{}\"", greeting.to_str().unwrap());
//     }

//     // -----------------------------------------------------------------------
//     // 3. Struct interop
//     // -----------------------------------------------------------------------
//     banner("3. Struct Interop (#[repr(C)])");

//     unsafe {
//         let p1 = Point2D { x: 0.0, y: 0.0 };
//         let p2 = Point2D { x: 3.0, y: 4.0 };

//         let dist = point_distance(&p1, &p2);
//         println!("  distance({p1:?}, {p2:?}) = {dist}");

//         let mid = point_midpoint(&p1, &p2);
//         println!("  midpoint({p1:?}, {p2:?}) = {mid:?}");
//     }

//     // -----------------------------------------------------------------------
//     // 4. Arrays (pointer + length)
//     // -----------------------------------------------------------------------
//     banner("4. Arrays (pointer + length)");

//     unsafe {
//         let numbers = [1_i32, 2, 3, 4, 5];
//         let total = array_sum(numbers.as_ptr(), numbers.len() as i32);
//         println!("  array_sum([1,2,3,4,5])    = {total}");

//         let mut data = [10_i32, 20, 30];
//         println!("  before array_double:      {:?}", data);
//         array_double(data.as_mut_ptr(), data.len() as i32);
//         println!("  after  array_double:      {:?}", data);
//     }

//     // -----------------------------------------------------------------------
//     // 5. Function-pointer callbacks
//     // -----------------------------------------------------------------------
//     banner("5. Callbacks (function pointers)");

//     unsafe {
//         let mut vals = [1_i32, 2, 3, 4, 5];

//         println!("  before transform:         {:?}", vals);
//         array_transform(vals.as_mut_ptr(), vals.len() as i32, square);
//         println!("  after  square:            {:?}", vals);

//         array_transform(vals.as_mut_ptr(), vals.len() as i32, negate);
//         println!("  after  negate:            {:?}", vals);
//     }

//     // -----------------------------------------------------------------------
//     // 6. Global mutable state
//     // -----------------------------------------------------------------------
//     banner("6. Global State From C");

//     unsafe {
//         let count = get_call_count();
//         println!("  Total C function calls so far: {count}");
//     }

//     // -----------------------------------------------------------------------
//     // Summary
//     // -----------------------------------------------------------------------
//     println!();
//     println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
//     println!("✅ All FFI demos completed successfully!");
//     println!();
//     println!("📝 Key takeaways:");
//     println!("   • extern \"C\" {{ ... }} declares C functions for Rust");
//     println!("   • Every FFI call is `unsafe` — YOU guarantee safety");
//     println!("   • Use CString/CStr for string interop");
//     println!("   • Use #[repr(C)] to match C struct layouts");
//     println!("   • Pass arrays as (pointer, length) pairs");
//     println!("   • extern \"C\" fn in Rust can be used as C callbacks");
//     println!("   • build.rs + cc crate compiles C code automatically");
// }

unsafe extern "C" {
    fn add(a: i32, b: i32) -> i32;
}

fn main() {
    let a: i32 = 3;
    let b: i32 = 4;
    unsafe {
        println!("Result of add: {:?}", add(a,b));
    }

}