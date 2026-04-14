use criterion::{Criterion, criterion_group, criterion_main, black_box};
use ex086_bechmarking::print_fibonacci;

fn benchmark_fibonacci(c: &mut Criterion) {
    c.bench_function("fibonacci 20", |b| b.iter(|| print_fibonacci(black_box(20))));
}

criterion_group!(benches, benchmark_fibonacci);
criterion_main!(benches);
