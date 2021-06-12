use super::pmap::*;

#[test]
fn test() {
    pretty_env_logger::try_init().unwrap_or(());
    let sample_data = random_data(100 * 1024, 1024);
    rayon::ThreadPoolBuilder::default().build_global().expect("err init rayon");
    log::info!("rayon threads: {}", rayon::current_num_threads());

    let r1 = default_impl(sample_data.clone(), sample_fun);
    let r2 = pmap::<_, _, 32344>(sample_data.clone(), sample_fun);
    assert_eq!(r1.len(), sample_data.len());
    assert_eq!(r2.len(), r1.len());
    for ix in 0..sample_data.len() {
        assert_eq!(r2[ix], r1[ix]);
    }
}

pub fn default_impl<T, R>(mut input: Vec<T>, mapf: fn(T) -> R) -> Vec<R> {
    input.drain(..).map(mapf).collect()
}

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