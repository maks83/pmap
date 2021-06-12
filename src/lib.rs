#![feature(const_generics)]
#![feature(slice_as_chunks)]
#![feature(maybe_uninit_extra)]
#![allow(incomplete_features)]

mod chunk;
mod pmap;

#[cfg(test)]
pub mod tests;

pub use self::pmap::*;
pub use self::chunk::*;