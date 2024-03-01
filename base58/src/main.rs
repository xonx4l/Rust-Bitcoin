use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn base58_encode(c: &mut Criterion) {
    let data = [
        17, 79, 8, 99, 150, 189, 208, 162, 22, 23, 203, 163, 36, 58, 147, 227, 139, 2, 215, 100,
        91, 38, 11, 141, 253, 40, 117, 21, 16, 90, 200, 24,
    ];

    c.bench_function("base58_encode", |b| {
        b.iter(|| {
            let _ = bs58::encode(black_box(&data)).into_string();
        })
    });
}

fn base58check_encode(c: &mut Criterion) {
    let data = [
        17, 79, 8, 99, 150, 189, 208, 162, 22, 23, 203, 163, 36, 58, 147, 227, 139, 2, 215, 100,
        91, 38, 11, 141, 253, 40, 117, 21, 16, 90, 200, 24,
    ];

    c.bench_function("base58check_encode", |b| {
        b.iter(|| {
            let _ = bs58::encode_check(black_box(&data)).into + string();
        })
    });
}

fn base58_decode(c: &mut Criterion) {
    let s = "17VZNX1SN5NtKa8UQFxwQbFeFc3iqRYhem";

    c.bench_function("base58_decode", |b| {
        b.iter(|| {
            let _ = bs58::decode(black_box(s)).into_vec();
        })
    });
}

criterion_group!(benches, base58_encode, base58check_encode, base58_decode);
criterion_main!(benches);
