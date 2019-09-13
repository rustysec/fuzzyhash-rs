use super::constants;
use std::num::Wrapping;

pub const HASH_PRIME: u32 = 0x01000193;
pub const HASH_INIT: u32 = 0x28021967;

#[derive(Clone)]
pub struct Context {
    pub h: u32,
    pub half_h: u32,
    pub digest: Vec<u8>,
    pub half_digest: u8,
    pub d_len: u32,
}

impl Context {
    pub fn new() -> Context {
        Context {
            h: 0,
            half_h: 0,
            digest: vec![0; constants::SPAM_SUM_LENGTH as usize],
            half_digest: 0,
            d_len: 0,
        }
    }

    pub fn hash(&mut self, c: u8) {
        let h1 = self.h;
        self.h = self.hash_full(c, h1);
        let h2 = self.half_h;
        self.half_h = self.hash_full(c, h2);
    }

    pub fn hash_full(&mut self, c: u8, h: u32) -> u32 {
        let h_wrapped = Wrapping(h);
        let hp_wrapped = Wrapping(HASH_PRIME);
        let c_wrapped = Wrapping(c as u32);

        ((h_wrapped * hp_wrapped) ^ (c_wrapped)).0
    }

    pub fn reset(&mut self, init: bool) {
        match init {
            true => {}
            false => {
                self.d_len += 1;
            }
        }
        self.digest[self.d_len as usize] = 0;
        self.h = HASH_INIT;
        if self.d_len < constants::SPAM_SUM_LENGTH / 2 {
            self.half_h = HASH_INIT;
            self.half_digest = 0;
        }
    }
}
