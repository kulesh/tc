//! Benchmarks for token counting performance
//!
//! Run with: cargo bench

use std::hint::black_box;
use token_counter_lib::{count_stats, count_tokens, load_tokenizer_from_bytes};

/// Embedded GPT-2 tokenizer for benchmarks
const DEFAULT_TOKENIZER: &[u8] = include_bytes!("../../bin/assets/gpt2-tokenizer.json");

fn main() {
    let tokenizer =
        load_tokenizer_from_bytes(DEFAULT_TOKENIZER).expect("Failed to load embedded tokenizer");

    println!("=== Token Counter Benchmarks ===\n");

    // Benchmark 1: Short text
    benchmark_short_text(&tokenizer);

    // Benchmark 2: Medium text
    benchmark_medium_text(&tokenizer);

    // Benchmark 3: Long text
    benchmark_long_text(&tokenizer);
}

fn benchmark_short_text(tokenizer: &tokenizers::Tokenizer) {
    let text = "Hello, world!";
    let iterations = 100_000;
    let start = std::time::Instant::now();

    for _ in 0..iterations {
        let _ = black_box(count_tokens(black_box(text), tokenizer));
    }

    let duration = start.elapsed();
    println!("Short text (\"{}\")", text);
    println!("  Iterations: {}", iterations);
    println!("  Total time: {:?}", duration);
    println!("  Average:    {:?}\n", duration / iterations);
}

fn benchmark_medium_text(tokenizer: &tokenizers::Tokenizer) {
    let text = "The quick brown fox jumps over the lazy dog. \
                This is a medium-length text that contains several sentences. \
                It's designed to benchmark token counting performance on typical input.";
    let iterations = 50_000;
    let start = std::time::Instant::now();

    for _ in 0..iterations {
        let _ = black_box(count_stats(black_box(text), tokenizer));
    }

    let duration = start.elapsed();
    println!("Medium text ({} chars)", text.len());
    println!("  Iterations: {}", iterations);
    println!("  Total time: {:?}", duration);
    println!("  Average:    {:?}\n", duration / iterations);
}

fn benchmark_long_text(tokenizer: &tokenizers::Tokenizer) {
    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
                Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. \
                Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris. "
        .repeat(10);
    let iterations = 10_000;
    let start = std::time::Instant::now();

    for _ in 0..iterations {
        let _ = black_box(count_stats(black_box(&text), tokenizer));
    }

    let duration = start.elapsed();
    println!("Long text ({} chars)", text.len());
    println!("  Iterations: {}", iterations);
    println!("  Total time: {:?}", duration);
    println!("  Average:    {:?}\n", duration / iterations);
}
