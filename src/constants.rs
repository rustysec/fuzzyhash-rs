pub const ROLLING_WINDOW: usize = 7;
pub const MIN_BLOCK_SIZE: u32 = 3;
pub const NUM_BLOCKHASHES: u32 = 31;
pub const SPAM_SUM_LENGTH: u32 = 64;
pub const MAX_RESULT_LENGTH: u32 = 2 * SPAM_SUM_LENGTH + 20;
pub const BASE64_CHARS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub enum Modes {
    None = 0,
    EliminateSequences = 1,
    DoNotTruncate = 2
}

pub fn get_base64_char(pos: usize) -> u8 { BASE64_CHARS.bytes().nth(pos).unwrap_or(0) }