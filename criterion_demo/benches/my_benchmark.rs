use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// 组1：测试不同 n 的 Fibonacci
fn bench_fib_20(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

fn bench_fib_25(c: &mut Criterion) {
    c.bench_function("fib 25", |b| b.iter(|| fibonacci(black_box(25))));
}

// 组2：测试 Fibonacci 迭代 vs 递归
fn fibonacci_iterative(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let temp = b;
        b = a + b;
        a = temp;
    }
    a
}

fn bench_fib_recursive(c: &mut Criterion) {
    c.bench_function("fib recursive 20", |b| b.iter(|| fibonacci(black_box(20))));
}

fn bench_fib_iterative(c: &mut Criterion) {
    c.bench_function("fib iterative 20", |b| {
        b.iter(|| fibonacci_iterative(black_box(20)))
    });
}

// 定义基准测试组
// criterion_group!(fib_group, bench_fib_20, bench_fib_25);
criterion_group!(compare_group, bench_fib_recursive, bench_fib_iterative);

// 让 `criterion_main!` 运行多个基准测试组
// criterion_main!(fib_group, compare_group);
criterion_main!(compare_group);
