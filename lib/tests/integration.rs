//! Integration tests for token counting functionality

use std::io::Cursor;
use token_counter_lib::{
    count_stats, count_tokens, count_tokens_from_reader, load_tokenizer_from_bytes, TokenStats,
};

/// Embedded GPT-2 tokenizer for tests
const DEFAULT_TOKENIZER: &[u8] = include_bytes!("../../bin/assets/gpt2-tokenizer.json");

#[test]
fn test_count_tokens_simple() {
    let tokenizer = load_tokenizer_from_bytes(DEFAULT_TOKENIZER).unwrap();
    let count = count_tokens("Hello, world!", &tokenizer).unwrap();
    assert!(count > 0, "Token count should be greater than 0");
    // GPT-2 typically encodes "Hello, world!" as 4 tokens
    assert!((3..=5).contains(&count), "Expected around 4 tokens");
}

#[test]
fn test_count_stats() {
    let tokenizer = load_tokenizer_from_bytes(DEFAULT_TOKENIZER).unwrap();
    let text = "Line 1\nLine 2\nLine 3";
    let stats = count_stats(text, &tokenizer).unwrap();

    assert_eq!(stats.lines, 3, "Should count 3 lines");
    assert_eq!(
        stats.bytes,
        text.len(),
        "Byte count should match text length"
    );
    assert!(stats.tokens > 0, "Should have tokens");
}

#[test]
fn test_count_tokens_from_reader() {
    let tokenizer = load_tokenizer_from_bytes(DEFAULT_TOKENIZER).unwrap();
    let text = "Reading from a reader!";
    let reader = Cursor::new(text);

    let stats = count_tokens_from_reader(reader, &tokenizer).unwrap();
    assert!(stats.tokens > 0, "Should count tokens from reader");
    assert_eq!(stats.bytes, text.len(), "Byte count should match");
}

#[test]
fn test_token_stats_accumulation() {
    let mut total = TokenStats::new();
    assert_eq!(total.tokens, 0);

    let stats1 = TokenStats {
        tokens: 10,
        lines: 2,
        bytes: 50,
    };
    let stats2 = TokenStats {
        tokens: 15,
        lines: 3,
        bytes: 75,
    };

    total.add(&stats1);
    assert_eq!(total.tokens, 10);
    assert_eq!(total.lines, 2);
    assert_eq!(total.bytes, 50);

    total.add(&stats2);
    assert_eq!(total.tokens, 25);
    assert_eq!(total.lines, 5);
    assert_eq!(total.bytes, 125);
}

#[test]
fn test_empty_text() {
    let tokenizer = load_tokenizer_from_bytes(DEFAULT_TOKENIZER).unwrap();
    let stats = count_stats("", &tokenizer).unwrap();

    assert_eq!(stats.tokens, 0, "Empty text should have 0 tokens");
    assert_eq!(stats.lines, 0, "Empty text should have 0 lines");
    assert_eq!(stats.bytes, 0, "Empty text should have 0 bytes");
}
