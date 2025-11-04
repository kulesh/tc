//! Basic usage example for token counting

use token_counter_lib::{count_stats, count_tokens, load_tokenizer_from_bytes};

/// Embedded GPT-2 tokenizer for the example
const DEFAULT_TOKENIZER: &[u8] = include_bytes!("../../bin/assets/gpt2-tokenizer.json");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the embedded tokenizer
    let tokenizer = load_tokenizer_from_bytes(DEFAULT_TOKENIZER)?;

    // Example 1: Count tokens in a simple string
    let text = "Hello, world! This is a token counting example.";
    let count = count_tokens(text, &tokenizer)?;
    println!("Text: \"{}\"", text);
    println!("Token count: {}\n", count);

    // Example 2: Get full statistics
    let longer_text = "The quick brown fox jumps over the lazy dog.\n\
                       This is a second line to demonstrate line counting.";
    let stats = count_stats(longer_text, &tokenizer)?;
    println!("Full statistics:");
    println!("  Tokens: {}", stats.tokens);
    println!("  Lines:  {}", stats.lines);
    println!("  Bytes:  {}", stats.bytes);

    Ok(())
}
