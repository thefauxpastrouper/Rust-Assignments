/// # Exercise 087: Profile-Guided Optimization (PGO)
///
/// Profile-Guided Optimization is a compiler technique where you:
/// 1. Build an instrumented binary that collects runtime profile data
/// 2. Run the binary with representative workloads to generate profile data
/// 3. Rebuild the binary using the collected profile to guide optimizations
///
/// This results in better branch prediction, improved inlining decisions,
/// and optimized code layout based on actual usage patterns.
///
/// ## PGO Workflow (run these commands manually):
///
/// ### Step 1: Clean previous profile data
/// ```sh
/// rm -rf /tmp/pgo-data
/// ```
///
/// ### Step 2: Build with instrumentation
/// ```sh
/// RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" \
///     cargo build --release -p ex087_profile_guided_optimizations
/// ```
///
/// ### Step 3: Run the instrumented binary with representative workloads
/// ```sh
/// ./target/release/ex087_profile_guided_optimizations
/// ```
///
/// ### Step 4: Merge the raw profile data
/// First, ensure llvm-tools is installed (ships the matching llvm-profdata):
/// ```sh
/// rustup component add llvm-tools
/// ```
/// Then find and use the toolchain-bundled llvm-profdata:
/// ```sh
/// # Find the path automatically:
/// PROFDATA=$(find $(rustc --print sysroot) -name llvm-profdata)
/// $PROFDATA merge -o /tmp/pgo-data/merged.profdata /tmp/pgo-data
/// ```
///
/// ### Step 5: Rebuild using the profile data for optimization
/// ```sh
/// RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata -Cllvm-args=-pgo-warn-missing-function" \
///     cargo build --release -p ex087_profile_guided_optimizations
/// ```
///
/// ### Step 6: Run the optimized binary and compare
/// ```sh
/// ./target/release/ex087_profile_guided_optimizations
/// ```

use std::time::Instant;

// ---------------------------------------------------------------------------
// Workload functions – these simulate "hot" and "cold" paths so PGO can
// observe which branches and functions are actually used at runtime.
// ---------------------------------------------------------------------------

/// A sorting function that the profiler will observe as HOT.
fn hot_sort(data: &mut Vec<u32>) {
    // Insertion sort – intentionally simple so the compiler has room to
    // optimise layout & inlining based on profile data.
    for i in 1..data.len() {
        let key = data[i];
        let mut j = i;
        while j > 0 && data[j - 1] > key {
            data[j] = data[j - 1];
            j -= 1;
        }
        data[j] = key;
    }
}

/// A function that is rarely called – PGO should de-prioritise it.
#[inline(never)]
fn cold_path(x: u64) -> u64 {
    // Expensive recursive Fibonacci (intentionally slow)
    if x <= 1 { return x; }
    cold_path(x - 1) + cold_path(x - 2)
}

/// Simulates a realistic branch-heavy workload.
/// ~90 % of iterations take the "common" path; ~10 % take the "rare" path.
/// PGO uses this distribution to optimise branch prediction.
fn branchy_workload(iterations: usize) -> u64 {
    let mut sum: u64 = 0;
    for i in 0..iterations {
        if i % 10 != 0 {
            // Common path (~90 %)
            sum = sum.wrapping_add(i as u64);
        } else {
            // Rare path (~10 %)
            sum = sum.wrapping_add((i as u64).wrapping_mul(3));
        }
    }
    sum
}

/// Demonstrates loop-heavy computation that benefits from PGO's layout opts.
fn matrix_multiply(size: usize) -> Vec<Vec<f64>> {
    let a: Vec<Vec<f64>> = (0..size)
        .map(|i| (0..size).map(|j| (i * size + j) as f64).collect())
        .collect();
    let b = a.clone();
    let mut c = vec![vec![0.0_f64; size]; size];

    for i in 0..size {
        for k in 0..size {
            let a_ik = a[i][k];
            for j in 0..size {
                c[i][j] += a_ik * b[k][j];
            }
        }
    }
    c
}

// ---------------------------------------------------------------------------
// Benchmark helper
// ---------------------------------------------------------------------------

fn bench<F: FnOnce() -> T, T>(label: &str, f: F) -> T {
    let start = Instant::now();
    let result = f();
    let elapsed = start.elapsed();
    println!("  {label:<30} {:>10.3?}", elapsed);
    result
}

// ---------------------------------------------------------------------------
// Main – runs the representative workloads
// ---------------------------------------------------------------------------

fn main() {
    println!("=== Exercise 087: Profile-Guided Optimization (PGO) ===\n");
    println!("Running representative workloads for profiling…\n");

    // 1. Sorting (hot path)
    bench("Insertion sort (10k elems)", || {
        let mut data: Vec<u32> = (0..10_000).rev().collect();
        hot_sort(&mut data);
        assert_eq!(data[0], 0);
    });

    // 2. Branchy workload
    let sum = bench("Branchy workload (1M iters)", || {
        branchy_workload(1_000_000)
    });
    println!("    → checksum: {sum}");

    // 3. Matrix multiply
    bench("Matrix multiply (100×100)", || {
        let c = matrix_multiply(100);
        assert!(c[0][0] >= 0.0);
    });

    // 4. Cold path (called only once – PGO should deprioritise)
    let fib = bench("Cold path: fib(30)", || cold_path(30));
    println!("    → fib(30) = {fib}");

    // 5. Multiple hot-sort rounds to give PGO plenty of profile data
    bench("Hot sort ×50 rounds (1k elems)", || {
        for _ in 0..50 {
            let mut data: Vec<u32> = (0..1_000).rev().collect();
            hot_sort(&mut data);
        }
    });

    println!("\n--- Workloads complete ---");
    println!();
    println!("To see PGO benefits, compare timings between:");
    println!("  • A normal `cargo build --release` binary");
    println!("  • A PGO-optimised binary (follow the steps in the doc-comment above)");
    println!();
    println!("Typical improvements: 5–20 % on branch-heavy and loop-heavy code.");
}
