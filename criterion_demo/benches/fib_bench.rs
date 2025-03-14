use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn bench_fibonacci(c: &mut Criterion) {
    let inputs = vec![10, 15, 20, 25];
    for &n in &inputs {
        c.bench_function(&format!("fib {}", n), |b| {
            b.iter(|| fibonacci(black_box(n)))
        });
    }
}

criterion_group!(fib_benches, bench_fibonacci);

criterion_main!(fib_benches);
