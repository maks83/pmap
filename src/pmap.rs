use super::chunk::{ChunkedVec, Chunk, UnsafeChunk};
use std::mem::{transmute, MaybeUninit};
use rayon;

pub fn pmap<T: Send, R: Send, const TH: usize>(input: Vec<T>, mapf: fn(T) -> R) -> ChunkedVec<R, TH> {
    let chunks_count = input.len() / TH;
    let mut chunks: Vec<UnsafeChunk<R, TH>> = Vec::with_capacity(chunks_count);
    let remainder = rayon::scope(|s| {
        let mut input = input;
        let mut remainder = Vec::split_off(&mut input, chunks_count * TH);
        for _ in 0..chunks_count {
            chunks.push(UnsafeChunk::default());
        }
        for ix in 0..chunks_count {
            let rest = Vec::split_off(&mut input, TH);
            let ichunk = input;
            let ochunk = &chunks[ix];
            input = rest;
            s.spawn(move |_| {
                unsafe {
                    map_chunk(ichunk, mapf, ochunk.get_mut());
                }
            });
        }
        remainder.drain(..).map(mapf).collect()
    });
    let chunks = unsafe {
        transmute::<_, Vec<Chunk<R, TH>>>(chunks)
    };
    ChunkedVec::new(chunks, remainder)
}

fn map_chunk<T, R, const TH: usize>(mut input: Vec<T>, mapf: fn(T) -> R, output: &mut Chunk<MaybeUninit<R>, TH>) {
    for (ix, el) in input.drain(..).enumerate() {
        let r = mapf(el);
        output[ix].write(r);
    }
}