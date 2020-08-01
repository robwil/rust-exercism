use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut huge_string = String::new();
    for i in 0..10000 {
        if i < 1000 {
            huge_string.push('a');
        } else if i < 2500 {
            huge_string.push('b');
        } else {
            if i % 2 == 0 {
                huge_string.push('c');
            } else if i % 3 == 0 {
                huge_string.push('d');
            } else if i % 4 == 0 {
                huge_string.push('c');
            }
        }
    }
    c.bench_function("encode - my original solution", |b| {
        b.iter(|| {
            rle::encode_original(black_box(
                &huge_string
            ))
        })
    });
    c.bench_function("encode - functional solution", |b| {
        b.iter(|| {
            rle::encode_functional(black_box(
                &huge_string
            ))
        })
    });
    c.bench_function("encode - clean iterative solution", |b| {
        b.iter(|| {
            rle::encode_cleaner(black_box(
                &huge_string
            ))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
