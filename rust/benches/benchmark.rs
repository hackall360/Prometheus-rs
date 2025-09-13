use std::collections::HashMap;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn closure_bench(c: &mut Criterion) {
    let iterations = 100_000;
    c.bench_function("closure", |b| {
        b.iter(|| {
            for _ in 0..iterations {
                (|| {
                    if !black_box(true) {
                        black_box(());
                    }
                })();
            }
        });
    });
}

fn settable_bench(c: &mut Criterion) {
    let iterations = 100_000;
    c.bench_function("settable", |b| {
        b.iter(|| {
            let mut t = HashMap::new();
            for i in 1..=iterations {
                t.insert(i.to_string(), format!("EPIC GAMER {}", i));
            }
            black_box(t);
        });
    });
}

fn gettable_bench(c: &mut Criterion) {
    let iterations = 100_000;
    c.bench_function("gettable", |b| {
        b.iter(|| {
            let mut t = HashMap::new();
            for i in 1..=iterations {
                t.insert(i.to_string(), format!("EPIC GAMER {}", i));
            }
            for i in 1..=iterations {
                let key = i.to_string();
                let value = t.get(&key);
                black_box(value);
            }
        });
    });
}

criterion_group!(benches, closure_bench, settable_bench, gettable_bench);
criterion_main!(benches);
