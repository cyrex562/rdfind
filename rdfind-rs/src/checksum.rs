// Ported from: orig_src/Checksum.hh
// Ported from: orig_src/Checksum.cc
//
// This file implements the Checksum functionality in Rust.
// Ported on 2025-05-05.

use std::io;
use sha1::Digest as Sha1Digest;


// Enum for supported checksum types
pub enum ChecksumType {
    SHA1,
    SHA256,
    SHA512,
    MD5,
    XXH128,
}

// State for each checksum type
pub enum ChecksumState {
    SHA1(sha1::Sha1),
    SHA256(sha2::Sha256),
    SHA512(sha2::Sha512),
    MD5(md5::Context),
    XXH128(xxhash_rust::xxh3::Xxh3),
}

pub struct Checksum {
    checksum_type: ChecksumType,
    state: ChecksumState,
}

impl Checksum {
    pub fn new(checksum_type: ChecksumType) -> Self {
        let state = match &checksum_type {
            ChecksumType::SHA1 => ChecksumState::SHA1(sha1::Sha1::new()),
            ChecksumType::SHA256 => ChecksumState::SHA256(sha2::Sha256::new()),
            ChecksumType::SHA512 => ChecksumState::SHA512(sha2::Sha512::new()),
            ChecksumType::MD5 => ChecksumState::MD5(md5::Context::new()),
            ChecksumType::XXH128 => ChecksumState::XXH128(xxhash_rust::xxh3::Xxh3::new()),
        };
        Self {
            checksum_type,
            state,
        }
    }

    pub fn update(&mut self, buffer: &[u8]) -> io::Result<()> {
        match &mut self.state {
            ChecksumState::SHA1(hasher) => hasher.update(buffer),
            ChecksumState::SHA256(hasher) => hasher.update(buffer),
            ChecksumState::SHA512(hasher) => hasher.update(buffer),
            ChecksumState::MD5(hasher) => hasher.consume(buffer),
            ChecksumState::XXH128(hasher) => hasher.update(buffer),
        }
        Ok(())
    }

    pub fn get_digest_length(&self) -> usize {
        match self.checksum_type {
            ChecksumType::SHA1 => 20,
            ChecksumType::SHA256 => 32,
            ChecksumType::SHA512 => 64,
            ChecksumType::MD5 => 16,
            ChecksumType::XXH128 => 16,
        }
    }

    pub fn finalize_to_vec(self) -> Vec<u8> {
        match self.state {
            ChecksumState::SHA1(hasher) => hasher.finalize().to_vec(),
            ChecksumState::SHA256(hasher) => hasher.finalize().to_vec(),
            ChecksumState::SHA512(hasher) => hasher.finalize().to_vec(),
            ChecksumState::MD5(hasher) => hasher.compute().to_vec(),
            ChecksumState::XXH128(hasher) => hasher.digest128().to_le_bytes().to_vec(),
        }
    }
}
