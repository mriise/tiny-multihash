use crate::hasher::{Digest, Hasher};
use blake2b_simd::{Params, State};
use core::marker::PhantomData;
use generic_array::typenum::{U32, U64};
use generic_array::{ArrayLength, GenericArray};

/// Blake2b hasher.
pub struct Blake2bHasher<Size: ArrayLength<u8> + core::fmt::Debug + Eq> {
    _marker: PhantomData<Size>,
    state: State,
}

impl<Size: ArrayLength<u8> + core::fmt::Debug + Eq> Default for Blake2bHasher<Size> {
    fn default() -> Self {
        let mut params = Params::new();
        params.hash_length(Size::to_usize());
        Self {
            _marker: PhantomData,
            state: params.to_state(),
        }
    }
}

impl<Size: ArrayLength<u8> + core::fmt::Debug + Eq> Hasher for Blake2bHasher<Size> {
    type Size = Size;

    fn write(&mut self, input: &[u8]) {
        self.state.update(input);
    }

    fn sum(self) -> Digest<Self::Size> {
        Digest::new(GenericArray::clone_from_slice(
            self.state.finalize().as_bytes(),
        ))
    }
}

/// 256 bit blake2b hasher.
pub type Blake2b256 = Blake2bHasher<U32>;

/// 512 bit blake2b hasher.
pub type Blake2b512 = Blake2bHasher<U64>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake2b256() {
        let hash = Blake2b256::digest(b"hello world");
        let mut hasher = Blake2b256::default();
        hasher.write(b"hello world");
        let hash2 = hasher.sum();
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_blake2b512() {
        let hash = Blake2b512::digest(b"hello world");
        let mut hasher = Blake2b512::default();
        hasher.write(b"hello world");
        let hash2 = hasher.sum();
        assert_eq!(hash, hash2);
    }
}