// use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
fn square(n: u32) -> u32 {
    n * n
}

fn bench_square(c: &mut Criterion) {
    let mut group = c.benchmark_group("square_group");
    group.bench_with_input(BenchmarkId::new("square", 10), &10, |b, &n| {
        b.iter(|| square(black_box(n)))
    });
    group.finish();
}

criterion_group!(benches, bench_square);
criterion_main!(benches);

use std::iter;

fn from_elem(c: &mut Criterion) {
    static KB: usize = 1024;

    let mut group = c.benchmark_group("from_elem");
    for size in [KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB].iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| iter::repeat(0u8).take(size).collect::<Vec<_>>());
        });
    }
    group.finish();

    let mut group = c.benchmark_group("from_elem_decimal");
    for size in [KB, 2 * KB].iter() {
        group.throughput(Throughput::BytesDecimal(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| iter::repeat(0u8).take(size).collect::<Vec<_>>());
        });
    }
    group.finish();
}

// criterion_group!(benches, from_elem);
