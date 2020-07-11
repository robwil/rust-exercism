use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sieve 1000 - my optimized version using only single vec", |b| {
        b.iter(|| sieve::primes_up_to(black_box(1000)))
    });
    c.bench_function("sieve 1000 - my original solution", |b| {
        b.iter(|| sieve::primes_up_to_retain(black_box(1000)))
    });
    c.bench_function("sieve 1000 - vec retain from exercism", |b| {
        b.iter(|| sieve::primes_up_to_retain(black_box(1000)))
    });
    c.bench_function("sieve 1000 - most starred from exercism", |b| {
        b.iter(|| sieve::primes_up_to_starred(black_box(1000)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
