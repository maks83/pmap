use super::chunk::{ChunkedVec, Chunk, UnsafeChunk};
use arrayvec::ArrayVec;
use std::mem::{transmute, MaybeUninit};
use crossbeam_utils::sync::WaitGroup;
use rayon;

pub fn pmap<T: Sync, R, const TH: usize>(input: &Vec<T>, mapf: fn(&T) -> R) -> ChunkedVec<R, TH> {
    let (input_chunks, input_remainder) = input.as_chunks::<TH>();
    let wg = WaitGroup::new();
    let mut chunks: Vec<UnsafeChunk<R, TH>> = Vec::with_capacity(input_chunks.len());
    for (ix, ichunk) in input_chunks.iter().enumerate() {
        chunks.push(UnsafeChunk::default());
        let ochunk = &chunks[ix];
        let wg = wg.clone();
        rayon::scope(move |_| {
            unsafe {
                map_chunk(ichunk, mapf, ochunk.get_mut());
            }
            drop(wg);
        });
    }
    let mut remainder = ArrayVec::new();
    for el in input_remainder {
        unsafe {
            remainder.push_unchecked(mapf(el));
        }
    }
    wg.wait();
    let chunks = unsafe {
        transmute::<_, Vec<Chunk<R, TH>>>(chunks)
    };
    ChunkedVec::new(chunks, remainder)
}

fn map_chunk<T, R, const TH: usize>(input: &[T; TH], mapf: fn(&T) -> R, output: &mut Chunk<MaybeUninit<R>, TH>) {
    for (ix, el) in input.iter().enumerate() {
        let r = mapf(el);
        output[ix].write(r);
    }
}
