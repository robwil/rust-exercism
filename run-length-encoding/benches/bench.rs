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
        b.iter(|| rle::encode_original(black_box(&huge_string)))
    });
    c.bench_function("encode - functional solution", |b| {
        b.iter(|| rle::encode_functional(black_box(&huge_string)))
    });
    c.bench_function("encode - clean iterative solution", |b| {
        b.iter(|| rle::encode_cleaner(black_box(&huge_string)))
    });
    c.bench_function("encode - clean & fast solution", |b| {
        b.iter(|| rle::encode_clean_fast(black_box(&huge_string)))
    });

    let encoded_str = rle::encode_original(&huge_string);

    c.bench_function("decode - my original solution", |b| {
        b.iter(|| rle::decode_original(black_box(&encoded_str)))
    });

    c.bench_function("decode - clean iterative solution", |b| {
        b.iter(|| rle::decode_cleaner(black_box(&encoded_str)))
    });

    c.bench_function("decode - clean & fast solution", |b| {
        b.iter(|| rle::decode_clean_fast(black_box(&encoded_str)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
