use criterion::{criterion_group, criterion_main, Criterion};
use fast_utc::Timestamp;

fn bench_now_coarsetime(c: &mut Criterion) {
    c.bench_function("Timestamp::now() (coarsetime)", |b| {
        b.iter(|| {
            Timestamp::now();
        })
    });
}

fn bench_chrono_now(c: &mut Criterion) {
    c.bench_function("chrono::Utc::now()", |b| {
        b.iter(|| {
            chrono::Utc::now();
        })
    });
}

criterion_group!(benches, bench_now_coarsetime, bench_chrono_now);
criterion_main!(benches);