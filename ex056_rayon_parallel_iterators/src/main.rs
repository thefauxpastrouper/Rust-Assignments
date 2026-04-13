// Rayon Parallel Iterators: Parallelize computations using Rayon
use rayon::prelude::*;
use std::time::Instant;

fn main() {
    let num: Vec<u64> = (1..=10000000).collect();

    let start = Instant::now();
    let _sum_seq: u64 = num.iter().sum();
    let seq_elasped = start.elapsed();

    let start = Instant::now();
    let _sum_par: u64 = num.par_iter().sum();
    let par_elasped = start.elapsed();

    println!("Sequentially time taken: {:?} Parallel Time Taken: {:?}", seq_elasped, par_elasped);
}
