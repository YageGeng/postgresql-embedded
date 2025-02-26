use criterion::{Criterion, criterion_group, criterion_main};
use postgresql_embedded::Result;
use postgresql_embedded::blocking::PostgreSQL;
use std::time::Duration;

fn benchmarks(criterion: &mut Criterion) {
    bench_lifecycle(criterion).ok();
}

fn bench_lifecycle(criterion: &mut Criterion) -> Result<()> {
    criterion.bench_function("lifecycle", |bencher| {
        bencher.iter(|| {
            lifecycle().ok();
        });
    });

    Ok(())
}

fn lifecycle() -> Result<()> {
    let mut postgresql = PostgreSQL::default();
    postgresql.setup()?;
    postgresql.start()?;
    postgresql.stop()
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(30))
        .sample_size(10);
    targets = benchmarks
);
criterion_main!(benches);
