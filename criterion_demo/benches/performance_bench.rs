use criterion::{Criterion, criterion_group, criterion_main};
// use my_module::*;

mod my_module {
    pub struct MyStruct {
        pub value: i32,
    }

    impl Clone for MyStruct {
        #[inline]
        fn clone(&self) -> Self {
            MyStruct { value: self.value }
        }
    }

    #[inline]
    pub fn add_one(x: i32) -> i32 {
        x + 1
    }

    pub fn add_two(x: i32) -> i32 {
        x + 2
    }

    #[inline]
    pub fn performance_critical_function(x: i32) -> i32 {
        let y = add_one(x);
        let z = add_two(y);
        z
    }

    pub fn performance_critical_function_no_inline(x: i32) -> i32 {
        let y = add_one(x);
        let z = add_two(y);
        z
    }
}

fn bench_performance_critical_function(c: &mut Criterion) {
    c.bench_function("performance_critical_function", |b| {
        b.iter(|| my_module::performance_critical_function(10))
    });
}

fn bench_performance_critical_function_no_inline(c: &mut Criterion) {
    c.bench_function("performance_critical_function_no_inline", |b| {
        b.iter(|| my_module::performance_critical_function_no_inline(10))
    });
}

criterion_group!(
    benches,
    bench_performance_critical_function,
    bench_performance_critical_function_no_inline
);

criterion_main!(benches);
