pub(crate) const ROLLING_WINDOW: usize = 7;
pub(crate) const MIN_BLOCK_SIZE: u32 = 3;
pub(crate) const NUM_BLOCKHASHES: u32 = 31;
pub(crate) const SPAM_SUM_LENGTH: u32 = 64;
pub(crate) const MAX_RESULT_LENGTH: u32 = 2 * SPAM_SUM_LENGTH + 20;
pub(crate) const BASE64_CHARS: &str =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Hashing modes
pub enum Modes {
    /// No special behavior
    None = 0,
    /// Eliminate sequences of more than three identical characters
    EliminateSequences = 1,
    /// Do not to truncate the second part to SPAMSUM_LENGTH/2 characters
    DoNotTruncate = 2,
}

pub(crate) fn get_base64_char(pos: usize) -> u8 {
    BASE64_CHARS.bytes().nth(pos).unwrap_or(0)
}
