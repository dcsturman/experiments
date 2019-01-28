#[macro_use]
extern crate criterion;

use criterion::Criterion;
use nth_prime as np;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("nth-prime 100", |b| b.iter(|| np::nth(100)));
    c.bench_function("nth-prime 1000", |b| b.iter(|| np::nth(1000)));
    c.bench_function("nth-prime 1200", |b| b.iter(|| np::nth(1200)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
