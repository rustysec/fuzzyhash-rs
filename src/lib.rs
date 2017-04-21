mod constants;
mod roll;
mod blockhash;

pub struct Hasher {
    bh_start: u32,
    bh_end: u32,
    bh: Vec<blockhash::Context>,
    total_size: u32,
    roll: roll::Roll
}

impl Hasher {
    pub fn new() -> Hasher {
        let mut h = Hasher {
            bh_start: 0,
            bh_end: 1,
            bh: vec![blockhash::Context::new(); constants::NUM_BLOCKHASHES as usize],
            total_size: 0,
            roll: roll::Roll::new()
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
        } else if self.bh_end == constants::NUM_BLOCKHASHES {
            self.bh[self.bh_end as usize].h = self.bh[(self.bh_end -1) as usize].h;
        }
    }

    pub fn try_reduce_blockhash(&mut self) {
        if self.bh_end - self.bh_start < 2 {
            return;
        }

        if (constants::MIN_BLOCK_SIZE << self.bh_start) * constants::SPAM_SUM_LENGTH >= self.total_size {
            return;
        }

        if self.bh[(self.bh_start + 1) as usize].d_len < constants::SPAM_SUM_LENGTH / 2 {
            return
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
            self.bh[j as usize].digest[pos] = constants::get_base64_char((self.bh[j as usize].h % 64) as usize);
            self.bh[j as usize].half_digest = constants::get_base64_char((self.bh[j as usize].half_h % 64) as usize);

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
        let blocksize_chars = blocksize_string.clone().into_bytes();
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
            },
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
                _ => true 
            } 
            || i < 3
            || base64val != result[pos - 1]
            || base64val != result[pos - 2]
            || base64val != result[pos - 3] {
                pos += 1;
            }
        }
        else if self.bh[bi as usize].digest[i as usize] != 0 {
            let base64val = self.bh[bi as usize].digest[i as usize];
            result[pos as usize] = base64val;
            if match flags {
                constants::Modes::EliminateSequences => false,
                _ => true
            } 
            || i < 3
            || base64val != result[pos - 1]
            || base64val != result[pos - 2]
            || base64val != result[pos - 3] {
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
                _ => true
            } && i > ((constants::SPAM_SUM_LENGTH / 2) - 1) as usize {
                i = ((constants::SPAM_SUM_LENGTH / 2) - 1) as usize;
            }
            
            match flags {
                constants::Modes::EliminateSequences => {
                    i = Hasher::memcpy_eliminate_sequences();
                },
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
                    _ => self.bh[bi as usize].half_h
                };
                let base64val = constants::get_base64_char((h % 64) as usize);
                result[pos] = base64val;
                if match flags {
                    constants::Modes::EliminateSequences => false,
                    _ => true
                } 
                || i < 3
                || base64val != result[pos - 1]
                || base64val != result[pos - 2]
                || base64val != result[pos - 3] {
                    pos += 1;
                }
            }
            else {
                i = match flags {
                    constants::Modes::DoNotTruncate => self.bh[bi as usize].digest[self.bh[bi as usize].d_len as usize],
                    _ => self.bh[bi as usize].half_digest
                } as usize;

                if i != 0 {
                    result[pos] = i as u8;
                    if match flags {
                        constants::Modes::EliminateSequences => false,
                        _ => true
                    } 
                    || i < 3
                    || i != result[pos - 1] as usize
                    || i != result[pos - 2] as usize
                    || i != result[pos - 3] as usize {
                        pos += 1;
                    }
                }
            }
        }
        else if h != 0 {
            result[pos] = constants::get_base64_char((self.bh[bi as usize].h % 64) as usize);
        }
        unsafe {
            result.set_len(pos);
        }
        String::from_utf8(result).unwrap()
    }
}

/// Returns the fuzzy hash of arbitrary data
///
/// # Arguments
/// * `buf` - a Vec<u8> containing the data to hash
///
/// # Example
/// ```
/// use fuzzyhash::{hash_buffer};
/// let data = "this is our test data!".to_string().as_bytes().to_vec();
/// println!("Fuzzy Hash: {}", hash_buffer(data));
/// ```
pub fn hash_buffer(buf: Vec<u8>) -> String {
    let mut hasher = Hasher::new();
    hasher.update(&buf, buf.len());
    hasher.digest(constants::Modes::None)
}

#[cfg(test)]
mod tests {
    use super::{hash_buffer};

    #[test]
    fn random_data1() {
        let data = r#"fcc55a724745b7efbf9a54908aec300d01b9830dff4ee435a667330a7fc56ca9
8913e11dc9cd172efc57c13083b24bd4eb44ef06a9c760431c0b45edf5ea76e3
ee53bde1e736f9c11383433351b98314cbada4742b1b46103a838c7d31a79b7e
20b30357d3b308721d1c5e1159eb7fe0a79e11368e06e7a1d3a52f222516e520
07354874a2790f82c62aa61a21d6335bb1b683d75e3f41d7c209bf88d4a06a6b
ceb774b75d0242badee796f39d2d6769ace96184bfc3a80081d94fc6eeda497d
b5c10e8667b9f786f20de678ad72d76c3f59cc1f75fa74b7bf4fdbd56b457689
999484ab64fd5458774610dd9c76dfaa8020d14cf1cf415deceda52db4111ac7
11fdf7e67ee5f9fe0c4e17f8a5d2a4df32ee32d6efe6dbfd5d2429ce52d56a62
6d198ffd7dddf4ad741d0c8b8aadf77cd338bf12320d0ffac2c12fd344fb8107
7431a7baa0c02b4231b16adc3438aa5569c7e86eba6401eabf3c1f1cf37587e6
86e9d0a7b49cb0f84bc356339f8aed56a281ce30c27a612b5cc0b2440198f1ca
4f81ca391ffe0f9c80f04090cf30ba167c20adecca24c865a5120604d21e383a
21c4fe0df1886d1f648e54c773a6084b1332f8ad51edc0233796384c821e078f
c0ff7e11eaca91b772fb68d670c2af6563e7740ad3eb900c18905468fe0b2635
ef82dc8ef62b14f3ccfac603c016f2a4e2231af30fb8c2c188ac9209a836e171
93e0c375b23d1271ecec237445746169d845ffd84dc40b20b704d23016d37d7d
25b1b76c3572db64b9caae2772c183ccda0cc0b42986b7f100861c8dab62313b
5e18dfd466610c8d944534a2644df922de1283e945be46a09b40edf0330601cb
124e4d572a885d89630070f393e3fff83b82b9cae94cfbf6dec9b5df6b71bede
a2229ee50dc32599c06e81b6a44f472c1dd331e39a3aba8a439c23e618307d85
98a88ccb694d9ddc82940f9273113b9154029ed0279ddb472ea5bce35790c4a4
7fde1d2b20119e4cc8eb028767592ada24bf311c535654d9273a6470479b5d51
62172924f022b3fa464ec5e20b057c49f217507ed538f6c4be4541b2a46cb00c
7b4c2c0634d2d40594d5874fb4c313d1efac95a0416eea5781c0d1a3474cee54
61fedb07a07602d62ae5f92ef5e1ed8f13c9ebccfb42f630293596937ceebf70
f943607f87d75afe4ecb1a0cb9879a408e0aef3415dce30e7ff6efdc975cfcfe
72c6d526b115aea7c2492a7f74b26703b57f2ea9827058edcee0732f37be309a
9550e7a282097f8ca5e31fc18a0626a35afa4105d2f7a8e3eaa7185fa019b52c
8534851a79683d1f9abfa0442712ed3bc42762f743c2ec2513d352806902ca18
fe4519b09a054635d86913aed6cdd601c56fee079c8b6fffb68434d46172453a
729eb2d6195a03c094373243ee4f60c920f1240dfaa6d2e84f79c84132f084c7
fdf4ba40c75cb6f24d3a11edfac45867fa083ac8cf1049f4505f644ace90aec6
d2b6a8c7847c8fde9a0c494717fc164673cdb430a73ace426cdba126fbd1053e
d6db1ceff62a14aa518b88f4d84905bc17271359380bf514decec55a172d2b61
93e254aacdb5bea463c49bd95a1abe2e9b3905ee6086bdaa6f45ff03615345f7
701c2b3839ba83afeb6b5126b2a282fc12c9242bef26dc26aa0a60b6c9a53273
f1d2bc3247fafce6b777de977824f1400a4cf70c4c31b6e944b4db7d0e146a53
8b9a26247f3f071649d4c78559299952b9ba7cf7bfd502dce197c3c16ac8b6e1
67b98e714de060cfe027cbfe7dc15932622abbe93499cfa0f6c03bffad050f5e
2b6c24781cf16e4794312c6b18b907164e218fd2db26cd0a8321243244e1f5ac
8dc80c7d4aab2e4b66cf2f4b52e11b527dd32ad837d353e81ce01969783e56ac
ad27c131afb9d5778ce9170eb58f8576d8d1f9e90d0f00f1a3351b337bc58815
ebef0d4c8f0d924b2a51d05d7c91f367d37b87973cb2de472bd0deb6c79d87d9
9a1c55d5af4d70b966fbc09ee531c3de6272af925958ec66dd385be208f5c270
5d1a62e55f9e9ec782d57e3635d4cc3b0a1c6b52e365d1eb485ccab246a3338a
daa530699a8a4c7dc072e9716373bfc248995c9bb467159d269e289d62aa7404
64300155a799ffd9ee362b4124aa1331c08648000b201528ebea173adf979634
69eb4e216dfb546d0f6bf62bc1195e0783369c09305237558b5c3913f6f25fd6
3b8b07019cdbe8ee366d82dea26903ff1cbd28dd8a301bd6367681e26f81f1ca
13351a1e922feb0794ac634eede9a29384287d909db2d2095bb9d250a022d10e
cdebaddc6d2904e9fa4e0148009c40fbda4f09c02f9d3f640bfa871b8231559a
74bb3ce0455f7b497343b8b336ab22af61bf42ece31c1e776aa813b11c667de2
bf003b7031ba665aa6a6e9513cbf12fa2f12133811f18ab35525c1d678621dfa
9bd6bb3ecdf3891b27f997f6eb6914903a5e068f774545aeff034a562d0ed310
392d7e64e37c8bf7cb2c9282aedfc19318c5223a28c1e1014005302fac671e03
cc4fd33987c76cf97c38ca710a7bfdf3bf4f74ab51f7dcd42a05dd1b26970ecd
cec48c10375432ff24e96b89cfe4b7e5bd7ae7603a66f4a039a9d55165a8b413
a80dc414f2bda7055abfc085eab6e0fa031d2d64821a00724dae9121957b8e80
7c4f6c1f8dcc6bc46665a7fee886ac9ad3bfa5d21d11580d75d3c01eadad073b
a7e8d819321b981496ddc2a10142394750c1e3eb0ea33cf86b9ac4be7bf8cc82
3285c7baab410c41d48b7f1ecd479316cf0f08abd87e2caf6ff70a95e173fad0
387c58b21c0aba8a8d2ab3a9c641b94d6dc84e4714f51c07c9fce745c05202a5
cebdc80fa6ddfea8b6b3e50cbfd73e00c0413820b007b243f2f4b1d12adefcb0
75170da30a9ead02a97c92f9c59bc9185c02a52f38671fa362e3025289ffa427
b55510b5ce5b54ebb5834b23dc0548738e9beb7495f7fb596a68a7f27d69f879
847123e04aeaec29c18040c7ed3e77a7ab88c9f3c03020035726b9b861ab14d0
0c8641b0fa6069c07688f00370b9b5793225301da6de7daf1fb85ddb660059a9
f65665489f8f0e9c551ff94aea8d19193ac4f8d654dcb9fb75497e42640a37e2
a5500b26b3fd3f77c433c0d85978c667898832f12709d5d79b1d90f62510e109
"#;

        let bytes = data.as_bytes().to_vec();
        assert_eq!(
            hash_buffer(bytes),
            "96:S+AQXqxdOnBKd+jHwAznNFzxt2HJwDX9oWZiaK0ld7vVmS85mbaN+MmFRz/jiJ:ZXqxdO8YDnN1SHJiqLaK0lbFbbaN1mFs".to_string()
        );
    }
}
