use constants;

pub struct Roll {
    pub h1: u32,
    pub h2: u32,
    pub h3: u32,
    pub n: u32,
    pub window: Vec<u8>
}

impl Roll {

    pub fn sum(&mut self) -> u32 {
        self.h3.wrapping_add(self.h1.wrapping_add(self.h2))
    }

    pub fn hash(&mut self, c: u8) {
        self.h2 -= self.h1;
        self.h2 += constants::ROLLING_WINDOW as u32 * c as u32;

        self.h1 += c as u32;
        self.h1 -= self.window[(self.n as usize % constants::ROLLING_WINDOW)] as u32;

        self.window[(self.n as usize % constants::ROLLING_WINDOW)] = c;
        self.n += 1;

        self.h3 <<= 5;
        self.h3 ^= c as u32;
    }

    pub fn new() -> Roll {
        Roll {
            h1: 0,
            h2: 0,
            h3: 0,
            n: 0,
            window: vec![0; constants::ROLLING_WINDOW]
        }
    }
}
