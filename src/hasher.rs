use super::{blockhash, constants, roll};

pub struct Hasher {
    bh_start: u32,
    bh_end: u32,
    bh: Vec<blockhash::Context>,
    total_size: u32,
    roll: roll::Roll,
}

impl Hasher {
    pub fn new() -> Hasher {
        let mut h = Hasher {
            bh_start: 0,
            bh_end: 1,
            bh: vec![blockhash::Context::new(); constants::NUM_BLOCKHASHES as usize],
            total_size: 0,
            roll: roll::Roll::new(),
        };
        h.bh[0].reset(true);
        h
    }

    pub fn memcpy_eliminate_sequences() -> usize {
        // TODO
        0
    }

    pub fn try_fork_blockhash(&mut self) {
        if self.bh_end < constants::NUM_BLOCKHASHES {
            self.bh[self.bh_end as usize].h = self.bh[(self.bh_end - 1) as usize].h;
            self.bh[self.bh_end as usize].half_h = self.bh[(self.bh_end - 1) as usize].half_h;

            self.bh[self.bh_end as usize].digest[0] = 0;
            self.bh[self.bh_end as usize].half_digest = 0;
            self.bh[self.bh_end as usize].d_len = 0;
            self.bh_end += 1;
        } else if self.bh_end == constants::NUM_BLOCKHASHES - 1 {
            self.bh[self.bh_end as usize].h = self.bh[(self.bh_end - 1) as usize].h;
        }
    }

    pub fn try_reduce_blockhash(&mut self) {
        if self.bh_end - self.bh_start < 2 {
            return;
        }

        if (constants::MIN_BLOCK_SIZE << self.bh_start) * constants::SPAM_SUM_LENGTH
            >= self.total_size
        {
            return;
        }

        if self.bh[(self.bh_start + 1) as usize].d_len < constants::SPAM_SUM_LENGTH / 2 {
            return;
        }

        self.bh_start += 1;
    }

    pub fn engine_step(&mut self, c: u8) {
        self.roll.hash(c);
        let h = self.roll.sum();
        for i in self.bh_start..self.bh_end {
            self.bh[i as usize].hash(c);
        }

        let mut j = self.bh_start;
        while j < self.bh_end {
            if h % (constants::MIN_BLOCK_SIZE << j) != (constants::MIN_BLOCK_SIZE << j) - 1 {
                break;
            }

            if self.bh[j as usize].d_len == 0 {
                self.try_fork_blockhash();
            }
            let pos = self.bh[j as usize].d_len as usize;
            self.bh[j as usize].digest[pos] =
                constants::get_base64_char((self.bh[j as usize].h % 64) as usize);
            self.bh[j as usize].half_digest =
                constants::get_base64_char((self.bh[j as usize].half_h % 64) as usize);

            if self.bh[j as usize].d_len < constants::SPAM_SUM_LENGTH - 1 {
                self.bh[j as usize].reset(false);
            } else {
                self.try_reduce_blockhash();
            }
            j += 1;
        }
    }

    pub fn update(&mut self, buffer: &[u8], len: usize) {
        self.total_size += len as u32;
        for i in 0..len {
            self.engine_step(buffer[i]);
        }
    }

    pub fn digest(&mut self, flags: constants::Modes) -> String {
        let mut result = vec![0; constants::MAX_RESULT_LENGTH as usize];
        let mut pos = 0;
        let mut bi = self.bh_start;
        let mut h = self.roll.sum();

        while (constants::MIN_BLOCK_SIZE << bi) * constants::SPAM_SUM_LENGTH < self.total_size {
            bi += 1;
            if bi >= constants::NUM_BLOCKHASHES {
                println!("Too many blocks!");
            }
        }

        while bi >= self.bh_end {
            bi -= 1;
        }

        while bi > self.bh_start && self.bh[bi as usize].d_len < constants::SPAM_SUM_LENGTH / 2 {
            bi -= 1;
        }

        let actual_blocksize = constants::MIN_BLOCK_SIZE << bi;
        let blocksize_string = actual_blocksize.to_string();
        let blocksize_chars = blocksize_string.into_bytes();
        let mut i = blocksize_chars.len();

        for j in 0..i {
            result[j + pos] = blocksize_chars[j];
        }
        result[i] = ':' as u8;
        i += 1;

        pos += i;
        i = self.bh[bi as usize].d_len as usize;

        match flags {
            constants::Modes::EliminateSequences => {
                i = Hasher::memcpy_eliminate_sequences();
            }
            _ => {
                for k in 0..i {
                    result[pos + k] = self.bh[bi as usize].digest[k];
                }
            }
        }

        pos += i;
        if h != 0 {
            let base64val = constants::get_base64_char((self.bh[bi as usize].h % 64) as usize);
            result[pos] = base64val;
            if match flags {
                constants::Modes::EliminateSequences => false,
                _ => true,
            } || i < 3
                || base64val != result[pos - 1]
                || base64val != result[pos - 2]
                || base64val != result[pos - 3]
            {
                pos += 1;
            }
        } else if self.bh[bi as usize].digest[i as usize] != 0 {
            let base64val = self.bh[bi as usize].digest[i as usize];
            result[pos as usize] = base64val;
            if match flags {
                constants::Modes::EliminateSequences => false,
                _ => true,
            } || i < 3
                || base64val != result[pos - 1]
                || base64val != result[pos - 2]
                || base64val != result[pos - 3]
            {
                pos += 1;
            }
        }
        result[pos] = ':' as u8;
        pos += 1;

        if bi < self.bh_end - 1 {
            bi += 1;
            i = self.bh[bi as usize].d_len as usize;

            if match flags {
                constants::Modes::DoNotTruncate => false,
                _ => true,
            } && i > ((constants::SPAM_SUM_LENGTH / 2) - 1) as usize
            {
                i = ((constants::SPAM_SUM_LENGTH / 2) - 1) as usize;
            }

            match flags {
                constants::Modes::EliminateSequences => {
                    i = Hasher::memcpy_eliminate_sequences();
                }
                _ => {
                    for k in 0..i {
                        result[pos + k] = self.bh[bi as usize].digest[k];
                    }
                }
            }
            pos += i;

            if h != 0 {
                h = match flags {
                    constants::Modes::DoNotTruncate => self.bh[bi as usize].h,
                    _ => self.bh[bi as usize].half_h,
                };
                let base64val = constants::get_base64_char((h % 64) as usize);
                result[pos] = base64val;
                if match flags {
                    constants::Modes::EliminateSequences => false,
                    _ => true,
                } || i < 3
                    || base64val != result[pos - 1]
                    || base64val != result[pos - 2]
                    || base64val != result[pos - 3]
                {
                    pos += 1;
                }
            } else {
                i = match flags {
                    constants::Modes::DoNotTruncate => {
                        self.bh[bi as usize].digest[self.bh[bi as usize].d_len as usize]
                    }
                    _ => self.bh[bi as usize].half_digest,
                } as usize;

                if i != 0 {
                    result[pos] = i as u8;
                    if match flags {
                        constants::Modes::EliminateSequences => false,
                        _ => true,
                    } || i < 3
                        || i != result[pos - 1] as usize
                        || i != result[pos - 2] as usize
                        || i != result[pos - 3] as usize
                    {
                        pos += 1;
                    }
                }
            }
        } else if h != 0 {
            result[pos] = constants::get_base64_char((self.bh[bi as usize].h % 64) as usize);
        }
        unsafe {
            result.set_len(pos);
        }
        String::from_utf8(result).unwrap()
    }
}