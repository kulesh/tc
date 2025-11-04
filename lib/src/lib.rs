//! # Token Counter Library
//!
//! This library provides token counting functionality for LLM tokenizers.

use std::fs;
use std::io::{self, Read};
use std::path::Path;
use thiserror::Error;
use tokenizers::Tokenizer;

/// Custom error type for the library
#[derive(Error, Debug)]
pub enum Error {
    /// Tokenizer loading error
    #[error("failed to load tokenizer: {0}")]
    TokenizerLoad(String),

    /// IO error
    #[error("io error: {0}")]
    Io(#[from] io::Error),

    /// Encoding error
    #[error("failed to encode text: {0}")]
    Encoding(String),
}

/// Result type alias for the library
pub type Result<T> = std::result::Result<T, Error>;

/// Statistics for token counting
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenStats {
    /// Number of tokens
    pub tokens: usize,
    /// Number of lines
    pub lines: usize,
    /// Number of bytes
    pub bytes: usize,
}

impl TokenStats {
    /// Creates a new TokenStats with zero counts
    pub fn new() -> Self {
        Self {
            tokens: 0,
            lines: 0,
            bytes: 0,
        }
    }

    /// Adds another TokenStats to this one
    pub fn add(&mut self, other: &TokenStats) {
        self.tokens += other.tokens;
        self.lines += other.lines;
        self.bytes += other.bytes;
    }
}

impl Default for TokenStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Loads a tokenizer from a JSON file
///
/// # Arguments
///
/// * `path` - Path to the tokenizer JSON file
///
/// # Errors
///
/// Returns an error if the tokenizer file cannot be loaded or parsed
pub fn load_tokenizer<P: AsRef<Path>>(path: P) -> Result<Tokenizer> {
    Tokenizer::from_file(path.as_ref()).map_err(|e| Error::TokenizerLoad(format!("{}", e)))
}

/// Loads a tokenizer from bytes (e.g., embedded tokenizer data)
///
/// # Arguments
///
/// * `bytes` - Tokenizer JSON data as bytes
///
/// # Errors
///
/// Returns an error if the tokenizer data cannot be parsed
///
/// # Examples
///
/// ```
/// use token_counter_lib::load_tokenizer_from_bytes;
///
/// // Example with valid tokenizer JSON bytes
/// let tokenizer_json = br#"{"version":"1.0","model":{"type":"BPE","vocab":{}}}"#;
/// // This would fail with real data, but demonstrates the API
/// let result = load_tokenizer_from_bytes(tokenizer_json);
/// assert!(result.is_err()); // Invalid tokenizer format
/// ```
pub fn load_tokenizer_from_bytes(bytes: &[u8]) -> Result<Tokenizer> {
    Tokenizer::from_bytes(bytes).map_err(|e| Error::TokenizerLoad(format!("{}", e)))
}

/// Counts tokens in a string
///
/// # Arguments
///
/// * `text` - The text to count tokens in
/// * `tokenizer` - The tokenizer to use
///
/// # Errors
///
/// Returns an error if encoding fails
///
/// # Examples
///
/// ```no_run
/// use token_counter_lib::{load_tokenizer, count_tokens};
///
/// let tokenizer = load_tokenizer("tokenizer.json").unwrap();
/// let count = count_tokens("Hello, world!", &tokenizer).unwrap();
/// assert!(count > 0);
/// ```
pub fn count_tokens(text: &str, tokenizer: &Tokenizer) -> Result<usize> {
    let encoding = tokenizer
        .encode(text, false)
        .map_err(|e| Error::Encoding(format!("{}", e)))?;
    Ok(encoding.len())
}

/// Counts tokens and other statistics for text
///
/// # Arguments
///
/// * `text` - The text to analyze
/// * `tokenizer` - The tokenizer to use
///
/// # Errors
///
/// Returns an error if encoding fails
pub fn count_stats(text: &str, tokenizer: &Tokenizer) -> Result<TokenStats> {
    let tokens = count_tokens(text, tokenizer)?;
    let lines = text.lines().count();
    let bytes = text.len();

    Ok(TokenStats {
        tokens,
        lines,
        bytes,
    })
}

/// Counts tokens in a file
///
/// # Arguments
///
/// * `path` - Path to the file
/// * `tokenizer` - The tokenizer to use
///
/// # Errors
///
/// Returns an error if the file cannot be read or encoding fails
///
/// # Examples
///
/// ```no_run
/// use token_counter_lib::{load_tokenizer, count_tokens_in_file};
/// use std::path::Path;
///
/// let tokenizer = load_tokenizer("tokenizer.json").unwrap();
/// let stats = count_tokens_in_file(Path::new("example.txt"), &tokenizer).unwrap();
/// println!("Tokens: {}", stats.tokens);
/// ```
pub fn count_tokens_in_file<P: AsRef<Path>>(path: P, tokenizer: &Tokenizer) -> Result<TokenStats> {
    let text = fs::read_to_string(path)?;
    count_stats(&text, tokenizer)
}

/// Counts tokens from a reader (e.g., stdin)
///
/// # Arguments
///
/// * `reader` - The reader to read from
/// * `tokenizer` - The tokenizer to use
///
/// # Errors
///
/// Returns an error if reading fails or encoding fails
pub fn count_tokens_from_reader<R: Read>(
    mut reader: R,
    tokenizer: &Tokenizer,
) -> Result<TokenStats> {
    let mut text = String::new();
    reader.read_to_string(&mut text)?;
    count_stats(&text, tokenizer)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests require a tokenizer file to be present
    // For now, we'll test the basic structure

    #[test]
    fn test_token_stats_new() {
        let stats = TokenStats::new();
        assert_eq!(stats.tokens, 0);
        assert_eq!(stats.lines, 0);
        assert_eq!(stats.bytes, 0);
    }

    #[test]
    fn test_token_stats_add() {
        let mut stats1 = TokenStats {
            tokens: 10,
            lines: 2,
            bytes: 50,
        };
        let stats2 = TokenStats {
            tokens: 5,
            lines: 1,
            bytes: 25,
        };
        stats1.add(&stats2);
        assert_eq!(stats1.tokens, 15);
        assert_eq!(stats1.lines, 3);
        assert_eq!(stats1.bytes, 75);
    }
}
