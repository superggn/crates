use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};

// 定义基准测试方法
fn bench_fib_recursive(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci");

    for &n in &[10, 20, 30] {
        group.bench_with_input(
            BenchmarkId::new("Recursive", black_box(n)),
            black_box(&n),
            |b, &n| b.iter(|| fibonacci_recursive(black_box(n))),
        );
    }

    group.finish();
}

fn bench_fib_iterative(c: &mut Criterion) {
    c.bench_function("iter", |b| b.iter(|| fibonacci_iterative(black_box(10))));
    c.bench_function("iter", |b| b.iter(|| fibonacci_iterative(black_box(20))));
    // let mut group = c.benchmark_group("Fibonacci");

    // for &n in &[10, 20, 30] {
    //     group.bench_with_input(
    //         BenchmarkId::new("Iterative", black_box(n)),
    //         black_box(&n),
    //         |b, &n| b.iter(|| fibonacci_iterative(n)),
    //     );
    // }

    // group.finish();
}

// 递归实现
fn fibonacci_recursive(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
    }
}

// 迭代实现
fn fibonacci_iterative(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let temp = a;
        a = b;
        b = temp + b;
    }
    a
}

// 组合多个基准测试
// criterion_group!(benches, bench_fib_recursive, bench_fib_iterative);
criterion_group!(benches, bench_fib_iterative);
criterion_main!(benches);
