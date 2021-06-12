use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pmap::pmap;

#[inline]
pub fn sample_fun(s: String) -> md5::Digest {
    md5::compute(s.as_bytes())
}

pub fn random_data(size: usize, el_size: usize) -> Vec<String> {
    use std::iter;
    use rand::{Rng, thread_rng};
    use rand::distributions::Alphanumeric;

    let mut r = Vec::with_capacity(size);
    let mut rng = thread_rng();
    for _ in 0..size {
        let s = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .map(char::from)
            .take(el_size)
            .collect();
        r.push(s);
    }
    r
}

#[inline]
pub fn baseline_bench(mut data: Vec<String>) {
    data.drain(..).map(sample_fun).for_each(drop);
}

#[inline]
pub fn pmap_bench<const N: usize>(data: Vec<String>) {
    pmap::<_, _, N>(data, sample_fun);
}

pub fn criterion_benchmark(c: &mut Criterion) {
    pretty_env_logger::try_init().unwrap_or(());
    let data = random_data(100 * 1000, 4096);
    rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build_global()
        .expect("err init rayon thread pool");
    log::info!("rayon threads: {}", rayon::current_num_threads());

    c.bench_function("baseline", |b| b.iter(|| baseline_bench(black_box(data.clone()))));
    c.bench_function("pmap 512", |b| b.iter(|| pmap_bench::<512>(black_box(data.clone()))));
    c.bench_function("pmap 1024", |b| b.iter(|| pmap_bench::<1024>(black_box(data.clone()))));
    c.bench_function("pmap 2048", |b| b.iter(|| pmap_bench::<2048>(black_box(data.clone()))));
    c.bench_function("pmap 4096", |b| b.iter(|| pmap_bench::<4096>(black_box(data.clone()))));
    c.bench_function("pmap 8192", |b| b.iter(|| pmap_bench::<8192>(black_box(data.clone()))));
    c.bench_function("pmap 16384", |b| b.iter(|| pmap_bench::<16384>(black_box(data.clone()))));
    c.bench_function("pmap 32768", |b| b.iter(|| pmap_bench::<32768>(black_box(data.clone()))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);