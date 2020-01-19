//! An implementation of fuzzyhash/ssdeep hash algorithm. The
//! original [CTPH](https://www.sciencedirect.com/science/article/pii/S1742287606000764?via%3Dihub)
//! paper describes how this fuzzy hash is computed.
//!

#![warn(missing_docs)]

mod blockhash;
mod compare;
mod constants;
mod hasher;
mod roll;

pub use constants::Modes;
pub use hasher::Hasher;
use std::ffi::{CStr, CString};
use std::fmt;
use std::io::Read;
use std::path::Path;

/// Hasher for fuzzy algorithm
pub struct FuzzyHash {
    hasher: Hasher,
    hash: Option<String>,
}

impl Default for FuzzyHash {
    fn default() -> Self {
        Self {
            hasher: Hasher::new(),
            hash: None,
        }
    }
}

impl FuzzyHash {
    /// Construct a new FuzzyHash from source data
    pub fn new<S: AsRef<[u8]>>(input: S) -> Self {
        let input = input.as_ref();
        let mut this = Self::default();
        this.hasher.update(input, input.len());
        this
    }

    /// Hash a file pointed to by `path`.
    ///
    /// # Example
    /// ```no_run
    /// use fuzzyhash::{FuzzyHash};
    /// let hash = FuzzyHash::file("/home/me/a_large_file.bin").unwrap();
    /// ```
    ///
    pub fn file<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let mut file = std::fs::File::open(path.as_ref())?;
        let mut hasher = Hasher::new();
        loop {
            let mut buffer = [0; 1024];
            let len = file.read(&mut buffer)?;
            hasher.update(&buffer, len);

            if len < 1024 {
                break;
            }
        }

        Ok(Self { hasher, hash: None })
    }

    /// Add chunk to the data source
    pub fn update<S: AsRef<[u8]>>(&mut self, input: S) {
        let input = input.as_ref();
        self.hasher.update(input, input.len());
    }

    /// Called to finalize the hashing and generate a string value
    pub fn finalize(&mut self) {
        if self.hash.is_none() {
            self.hash = Some(self.hasher.digest(constants::Modes::None));
        }
    }

    /// Compare two fuzzy hashes
    ///
    /// # Arguments
    /// * `first` - first fuzzy hash to compare
    /// * `second` - second fuzzy hash to compare
    ///
    /// # Example
    /// ```
    /// use fuzzyhash::FuzzyHash;
    /// assert_eq!(FuzzyHash::compare(
    ///            "96:U57GjXnLt9co6pZwvLhJluvrszNgMFwO6MFG8SvkpjTWf:Hj3BeoEcNJ0TspgIG8SvkpjTg",
    ///            "96:U57GjXnLt9co6pZwvLhJluvrs1eRTxYARdEallia:Hj3BeoEcNJ0TsI9xYeia3R"),
    ///     63);
    /// ```
    pub fn compare<S: AsRef<str>, T: AsRef<str>>(first: S, second: T) -> u32 {
        compare::compare(first, second)
    }

    /// Compare this fuzzy hash against another
    ///
    /// # Arguments
    /// * `other` - compare this fuzzy hash to `other`
    ///
    /// # Example
    /// ```
    /// use fuzzyhash::FuzzyHash;
    /// let mut fuzzy_hash = FuzzyHash::new("some data to hash for the purposes of running a test");
    /// fuzzy_hash.finalize();
    /// assert_eq!(fuzzy_hash.compare_to(
    ///            &"3:HEREar5MFUul0U6R9F1:knl8lql1".into()),
    ///            Some(18));
    /// ```
    pub fn compare_to(&self, other: &FuzzyHash) -> Option<u32> {
        self.hash
            .as_ref()
            .map(|ref hash| FuzzyHash::compare(hash, &other.to_string()))
    }
}

impl fmt::Display for FuzzyHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.hash.as_ref().unwrap_or(&String::new()))
    }
}

impl From<&str> for FuzzyHash {
    fn from(s: &str) -> Self {
        Self {
            hasher: Hasher::new(),
            hash: Some(s.to_string()),
        }
    }
}

impl From<String> for FuzzyHash {
    fn from(s: String) -> Self {
        Self {
            hasher: Hasher::new(),
            hash: Some(s),
        }
    }
}

/// Returns the fuzzy hash of arbitrary data. This method provides better FFI compatibility.
///
/// # Arguments
/// * `buf` - a pointer to the array containing the data to hash
/// * `length` - length of buf
///
/// # Safety
///
/// This is function is `unsafe` as it is intended to read a string from FFI
///
/// # Example
/// ```
/// use fuzzyhash::{hash_buffer_raw};
/// use std::ffi::CString;
///
/// let data = "this is our test data!".to_string();
/// let hash = unsafe { CString::from_raw(hash_buffer_raw(data.as_bytes().as_ptr(), data.len())) };
/// println!("Fuzzy Hash: {}", hash.into_string().unwrap());
/// ```
#[no_mangle]
pub unsafe extern "C" fn hash_buffer_raw(buf: *const u8, length: usize) -> *mut i8 {
    let data = std::slice::from_raw_parts(buf, length);
    let mut fuzzy_hash = FuzzyHash::new(data);
    fuzzy_hash.finalize();

    let s = CString::new(fuzzy_hash.to_string()).unwrap();

    s.into_raw()
}

/// FFI Compatible fuzzy hash comparisons.
///
/// # Arguments
/// * `first` - a C style fuzzy hash string
/// * `second` - a C style fuzzy hash string
///
/// # Safety
///
/// This is function is `unsafe` as it is intended to read strings from FFI
///
/// # Example
/// ```
/// use fuzzyhash::{compare_strings_raw};
/// use std::ffi::CString;
///
/// let first = CString::new("this is our test data!").unwrap();
/// let second = CString::new("this is my test data!").unwrap();
/// println!("Fuzzy Hash: {}", unsafe { compare_strings_raw(first.into_raw(), second.into_raw()) });
/// ```
#[no_mangle]
pub unsafe extern "C" fn compare_strings_raw(first: *const i8, second: *const i8) -> u32 {
    let f = CStr::from_ptr(first).to_string_lossy().into_owned();
    let s = CStr::from_ptr(second).to_string_lossy().into_owned();

    FuzzyHash::compare(&f, &s)
}
