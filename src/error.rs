#![allow(missing_docs)]

use std::fmt;

/// Errors pertaining to processing fuzzy hashes
#[derive(Debug)]
pub enum Error {
    /// Fuzzy hashes must contain at least one common substring for comparison
    NoCommonSubstrings,

    /// At least one input string is in the wrong format
    MalformedInput,

    /// Cannot parse the block size of the string
    BlockSizeParse,

    /// Two strings have incompatible block sizes. Sizes must be equal, a multiple or a multiple of
    /// 2 from each other.
    IncompatibleBlockSizes,

    /// String contains too many blocks for comparison
    TooManyBlocks,

    /// Unable to produce a valid hash string
    InvalidHashString(std::string::FromUtf8Error),
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::NoCommonSubstrings => "No common substrings were found between two fuzzy hashes",
            Error::MalformedInput => "Strings are not in proper fuzzy hash format",
            Error::BlockSizeParse => "Could not parse block sizes in string(s)",
            Error::IncompatibleBlockSizes => "Fuzzy hashes have incompatible block sizes",
            Error::TooManyBlocks => "Total number of blocks exceeds limit",
            Error::InvalidHashString(_) => "Unable to produce a valid hash string",
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        match self {
            Error::InvalidHashString(e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error processing fuzzy hash(es)")
    }
}
