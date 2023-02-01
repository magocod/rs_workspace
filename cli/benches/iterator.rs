use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{thread_rng, Rng};

const LEN: usize = 1024 * 1024;

fn sum_for(x: &[f64]) -> f64 {
    let mut result: f64 = 0.0;
    for i in 0..x.len() {
        result += x[i];
    }
    result
}

fn sum_iter(x: &[f64]) -> f64 {
    x.iter().sum::<f64>()
}

fn rand_array(cnt: u32) -> Vec<f64> {
    let mut rng = thread_rng();
    (0..cnt).map(|_| rng.gen::<f64>()).collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    let samples = rand_array(LEN as u32);
    c.bench_function("sum_for", |b| b.iter(|| sum_for(black_box(&samples))));
    c.bench_function("sum_iter", |b| b.iter(|| sum_iter(black_box(&samples))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
