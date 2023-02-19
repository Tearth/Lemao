use std::hash::{BuildHasher, Hasher};

pub struct StorageHasher {
    state: u64,
}

pub struct StorageHasherBuilder;

impl Hasher for StorageHasher {
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.state = self.state.rotate_left(8) ^ (byte as u64);
        }
    }

    fn finish(&self) -> u64 {
        self.state
    }
}

impl BuildHasher for StorageHasherBuilder {
    type Hasher = StorageHasher;

    fn build_hasher(&self) -> StorageHasher {
        StorageHasher { state: 0 }
    }
}
