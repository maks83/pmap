use super::chunk::{ChunkedVec, Chunk, UnsafeChunk};
use arrayvec::ArrayVec;
use std::mem::{transmute, MaybeUninit};
use rayon;

pub fn pmap<T: Sync, R: Send, const TH: usize>(input: &Vec<T>, mapf: fn(&T) -> R) -> ChunkedVec<R, TH> {
    let (input_chunks, input_remainder) = input.as_chunks::<TH>();
    let mut remainder = ArrayVec::new();
    if !input_chunks.is_empty() {
        let mut chunks: Vec<UnsafeChunk<R, TH>> = Vec::with_capacity(input_chunks.len());
        for _ in 0..input_chunks.len() {
            chunks.push(UnsafeChunk::default());
        }
        rayon::scope(|s| {
            for (ix, ichunk) in input_chunks.iter().enumerate() {
                let ochunk = &chunks[ix];
                s.spawn(move |_| {
                    unsafe {
                        map_chunk(ichunk, mapf, ochunk.get_mut());
                    }
                });
            }
            for el in input_remainder {
                unsafe {
                    remainder.push_unchecked(mapf(el));
                }
            }
        });
        let chunks = unsafe {
            transmute::<_, Vec<Chunk<R, TH>>>(chunks)
        };
        ChunkedVec::new(chunks, remainder)
    } else {
        for el in input_remainder {
            unsafe {
                remainder.push_unchecked(mapf(el));
            }
        }
        ChunkedVec::new(Vec::default(), remainder)
    }
}

fn map_chunk<T, R, const TH: usize>(input: &[T; TH], mapf: fn(&T) -> R, output: &mut Chunk<MaybeUninit<R>, TH>) {
    for (ix, el) in input.iter().enumerate() {
        let r = mapf(el);
        output[ix].write(r);
    }
}
