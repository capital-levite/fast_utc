use criterion::{criterion_group, criterion_main, Criterion};
use utctimestamp::UtcTimeStamp;

fn bench_now_chrono_fallback(c: &mut Criterion) {
    c.bench_function("UtcTimeStamp::now() (chrono fallback)", |b| {
        b.iter(|| {
            UtcTimeStamp::now();
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

criterion_group!(benches, bench_now_chrono_fallback, bench_chrono_now);
criterion_main!(benches);