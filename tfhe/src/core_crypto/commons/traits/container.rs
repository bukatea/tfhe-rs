//! Module with traits pertaining to container manipulation.

use rayon::prelude::*;

/// A trait to manipulate various immutable container types transparently.
pub trait Container: AsRef<[Self::Element]> {
    type Element;

    fn container_len(&self) -> usize {
        self.as_ref().len()
    }
}

/// A trait to manipulate various mutable container types transparently.
pub trait ContainerMut: Container + AsMut<[<Self as Container>::Element]> {}

impl<T> Container for [T] {
    type Element = T;
}

impl<T> ContainerMut for [T] {}

impl<T> Container for Vec<T> {
    type Element = T;
}

impl<T> ContainerMut for Vec<T> {}

impl<T> Container for &[T] {
    type Element = T;
}

impl<T> Container for &mut [T] {
    type Element = T;
}

impl<T> ContainerMut for &mut [T] {}

impl<T> Container for aligned_vec::ABox<[T]> {
    type Element = T;
}

impl<T> ContainerMut for aligned_vec::ABox<[T]> {}

impl<T> Container for Box<[T]> {
    type Element = T;
}

impl<T> ContainerMut for Box<[T]> {}

impl<T> Container for aligned_vec::AVec<T> {
    type Element = T;
}

impl<T> ContainerMut for aligned_vec::AVec<T> {}

pub trait IntoContainerOwned: Container + AsMut<[Self::Element]> {
    fn collect<I: Iterator<Item = Self::Element>>(iter: I) -> Self;
}

impl<T> IntoContainerOwned for aligned_vec::ABox<[T]> {
    fn collect<I: Iterator<Item = Self::Element>>(iter: I) -> Self {
        aligned_vec::AVec::<T, _>::from_iter(0, iter).into_boxed_slice()
    }
}

pub trait Split: Sized {
    type Chunks: DoubleEndedIterator<Item = Self> + ExactSizeIterator<Item = Self>;

    fn into_chunks(self, chunk_size: usize) -> Self::Chunks;
    fn split_into(self, chunk_count: usize) -> Self::Chunks;
    fn split_at(self, mid: usize) -> (Self, Self);
}

impl<'a, T> Split for &'a [T] {
    type Chunks = core::slice::ChunksExact<'a, T>;

    #[inline]
    fn into_chunks(self, chunk_size: usize) -> Self::Chunks {
        debug_assert_eq!(self.len() % chunk_size, 0);
        self.chunks_exact(chunk_size)
    }
    #[inline]
    fn split_into(self, chunk_count: usize) -> Self::Chunks {
        if chunk_count == 0 {
            debug_assert_eq!(self.len(), 0);
            self.chunks_exact(1)
        } else {
            debug_assert_eq!(self.len() % chunk_count, 0);
            self.chunks_exact(self.len() / chunk_count)
        }
    }
    #[inline]
    fn split_at(self, mid: usize) -> (Self, Self) {
        self.split_at(mid)
    }
}

impl<'a, T> Split for &'a mut [T] {
    type Chunks = core::slice::ChunksExactMut<'a, T>;

    #[inline]
    fn into_chunks(self, chunk_size: usize) -> Self::Chunks {
        debug_assert_eq!(self.len() % chunk_size, 0);
        self.chunks_exact_mut(chunk_size)
    }
    #[inline]
    fn split_into(self, chunk_count: usize) -> Self::Chunks {
        if chunk_count == 0 {
            debug_assert_eq!(self.len(), 0);
            self.chunks_exact_mut(1)
        } else {
            debug_assert_eq!(self.len() % chunk_count, 0);
            self.chunks_exact_mut(self.len() / chunk_count)
        }
    }
    #[inline]
    fn split_at(self, mid: usize) -> (Self, Self) {
        self.split_at_mut(mid)
    }
}

pub trait ParSplit: Sized {
    type Chunks: IndexedParallelIterator<Item = Self>;

    fn into_par_chunks(self, chunk_size: usize) -> Self::Chunks;
    fn par_split_into(self, chunk_count: usize) -> Self::Chunks;
    fn par_split_at(self, mid: usize) -> (Self, Self);
}

impl<'a, T: Sync> ParSplit for &'a [T] {
    type Chunks = rayon::slice::Chunks<'a, T>;

    #[inline]
    fn into_par_chunks(self, chunk_size: usize) -> Self::Chunks {
        self.par_chunks(chunk_size)
    }
    #[inline]
    fn par_split_into(self, chunk_count: usize) -> Self::Chunks {
        if chunk_count == 0 {
            self.par_chunks(1)
        } else {
            self.par_chunks(self.len() / chunk_count)
        }
    }
    #[inline]
    fn par_split_at(self, mid: usize) -> (Self, Self) {
        self.split_at(mid)
    }
}

impl<'a, T: Send> ParSplit for &'a mut [T] {
    type Chunks = rayon::slice::ChunksMut<'a, T>;

    #[inline]
    fn into_par_chunks(self, chunk_size: usize) -> Self::Chunks {
        self.par_chunks_mut(chunk_size)
    }
    #[inline]
    fn par_split_into(self, chunk_count: usize) -> Self::Chunks {
        if chunk_count == 0 {
            self.par_chunks_mut(1)
        } else {
            self.par_chunks_mut(self.len() / chunk_count)
        }
    }
    #[inline]
    fn par_split_at(self, mid: usize) -> (Self, Self) {
        self.split_at_mut(mid)
    }
}
