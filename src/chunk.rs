use crossbeam_utils::CachePadded;
use arrayvec::ArrayVec;
use std::{cell::UnsafeCell, mem::MaybeUninit};

#[derive(Debug, Clone, Copy)]
pub struct Chunk<T, const N: usize> (CachePadded<[T; N]>);

#[derive(Debug)]
pub struct UnsafeChunk<T, const N: usize> (UnsafeCell<Chunk<MaybeUninit<T>, N>>);

#[derive(Debug, Clone)]
pub struct ChunkedVec<T, const CHUNK_SIZE: usize> {
    chunks: Vec<Chunk<T, CHUNK_SIZE>>,
    remainder: ArrayVec<T, CHUNK_SIZE>,
}

unsafe impl<T, const N: usize> Sync for UnsafeChunk<T, N> {}

impl<T, const N: usize> UnsafeChunk<T, N> {
    #[inline]
    pub unsafe fn get_mut(&self) -> &mut Chunk<MaybeUninit<T>, N> {
        &mut *self.0.get()
    }
}

impl<T, const N: usize> Default for UnsafeChunk<T, N> {
    fn default() -> Self {
        let arr: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };
        UnsafeChunk(UnsafeCell::from(Chunk(CachePadded::from(arr))))
    }
}

impl<T, const CHUNK_SIZE: usize> ChunkedVec<T, CHUNK_SIZE> {
    pub fn new(chunks: Vec<Chunk<T, CHUNK_SIZE>>, remainder: ArrayVec<T, CHUNK_SIZE>) -> Self {
        ChunkedVec { chunks, remainder }
    }

    #[inline]
    pub fn len(&self) -> usize { self.remainder.len() + self.chunks.len() * CHUNK_SIZE }
}

impl<I: Into<usize>, T, const CHUNK_SIZE: usize> std::ops::Index<I> for Chunk<T, CHUNK_SIZE> {
    type Output = T;
    fn index(&self, ix: I) -> &T { &self.0[ix.into()] }
}

impl<I: Into<usize>, T, const CHUNK_SIZE: usize> std::ops::IndexMut<I> for Chunk<T, CHUNK_SIZE> {
    fn index_mut(&mut self, ix: I) -> &mut T { &mut self.0[ix.into()] }
}

impl<I: Into<usize>, T, const CHUNK_SIZE: usize> std::ops::Index<I> for ChunkedVec<T, CHUNK_SIZE> {
    type Output = T;

    fn index(&self, ix: I) -> &T {
        let ix = ix.into();
        let chunk_ix = ix / CHUNK_SIZE;
        if chunk_ix < self.chunks.len() {
            let chunk = &self.chunks[chunk_ix];
            &chunk[ix % CHUNK_SIZE]
        } else {
            &self.remainder[ix % CHUNK_SIZE]
        }
    }
}

impl<I: Into<usize>, T, const CHUNK_SIZE: usize> std::ops::IndexMut<I> for ChunkedVec<T, CHUNK_SIZE> {
    fn index_mut(&mut self, ix: I) -> &mut T { 
        let ix = ix.into();
        let chunk_ix = ix / CHUNK_SIZE;
        if chunk_ix < self.chunks.len() {
            let chunk = &mut self.chunks[chunk_ix];
            &mut chunk[ix % CHUNK_SIZE]
        } else {
            &mut self.remainder[ix % CHUNK_SIZE]
        }
    }
}