// Ported from: orig_src/EasyRandom.cc
// Ported from: orig_src/EasyRandom.hh
//
// This file implements the EasyRandom functionality in Rust.
// Ported on 2025-05-05.

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::cell::RefCell;

const ACCEPTABLE_FILENAME_CHARS: &[u8; 64] =
    b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-";

thread_local! {
    static GLOBAL_RNG: RefCell<StdRng> = RefCell::new(StdRng::seed_from_u64(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64
    ));
}

pub struct EasyRandom;

impl EasyRandom {
    pub fn new() -> Self {
        EasyRandom
    }

    pub fn make_random_file_string(&self, n: usize) -> String {
        GLOBAL_RNG.with(|rng| {
            let mut rng = rng.borrow_mut();
            (0..n)
                .map(|_| {
                    let idx = rng.random_range(0..ACCEPTABLE_FILENAME_CHARS.len());
                    ACCEPTABLE_FILENAME_CHARS[idx] as char
                })
                .collect()
        })
    }
}
