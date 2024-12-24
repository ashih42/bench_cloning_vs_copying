use cloning_vs_copying::{
    bench_cloneable_vector2d_by_cloning, bench_copyable_vector2d_by_cloning,
    bench_copyable_vector2d_by_copying,
};

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_method1(c: &mut Criterion) {
    c.bench_function("bench_cloneable_vector2d_by_cloning", |b| {
        b.iter(bench_cloneable_vector2d_by_cloning)
    });
}

fn bench_method2(c: &mut Criterion) {
    c.bench_function("bench_copyable_vector2d_by_cloning", |b| {
        b.iter(bench_copyable_vector2d_by_cloning)
    });
}

fn bench_method3(c: &mut Criterion) {
    c.bench_function("bench_copyable_vector2d_by_copying", |b| {
        b.iter(bench_copyable_vector2d_by_copying)
    });
}

criterion_group!(benches, bench_method1, bench_method2, bench_method3);
criterion_main!(benches);
