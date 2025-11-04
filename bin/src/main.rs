//! tc - Token Counter CLI
//!
//! A command-line utility for counting LLM tokens in files, similar to Unix `wc`.

use anyhow::{Context, Result};
use clap::Parser;
use std::io::{self, IsTerminal};
use std::path::PathBuf;
use token_counter_lib::{
    count_tokens_from_reader, count_tokens_in_file, load_tokenizer, load_tokenizer_from_bytes,
    TokenStats,
};

/// Embedded GPT-2 tokenizer (default)
const DEFAULT_TOKENIZER: &[u8] = include_bytes!("../assets/gpt2-tokenizer.json");

/// Token counter - count LLM tokens in files (similar to wc for words)
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input files (reads from stdin if not provided)
    #[arg(value_name = "FILE")]
    files: Vec<PathBuf>,

    /// Path to custom tokenizer JSON file (uses embedded GPT-2 tokenizer by default)
    #[arg(short = 't', long, value_name = "PATH")]
    tokenizer: Option<PathBuf>,

    /// Show only token count
    #[arg(long)]
    tokens_only: bool,

    /// Show line count
    #[arg(short = 'l', long)]
    lines: bool,

    /// Show byte count
    #[arg(short = 'c', long)]
    bytes: bool,
}

struct OutputConfig {
    show_tokens: bool,
    show_lines: bool,
    show_bytes: bool,
}

impl OutputConfig {
    fn from_args(args: &Args) -> Self {
        // If no specific flags are set, show all
        let nothing_specified = !args.tokens_only && !args.lines && !args.bytes;

        Self {
            show_tokens: args.tokens_only || nothing_specified,
            show_lines: args.lines || nothing_specified,
            show_bytes: args.bytes || nothing_specified,
        }
    }

    fn format_stats(&self, stats: &TokenStats, name: Option<&str>) -> String {
        let mut parts = Vec::new();

        if self.show_tokens {
            parts.push(format!("{:8}", stats.tokens));
        }
        if self.show_lines {
            parts.push(format!("{:8}", stats.lines));
        }
        if self.show_bytes {
            parts.push(format!("{:8}", stats.bytes));
        }

        let counts = parts.join(" ");

        if let Some(name) = name {
            format!("{} {}", counts, name)
        } else {
            counts
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let output_config = OutputConfig::from_args(&args);

    // Load tokenizer (use embedded GPT-2 by default, or custom if specified)
    let tokenizer = if let Some(tokenizer_path) = &args.tokenizer {
        load_tokenizer(tokenizer_path)
            .with_context(|| format!("Failed to load tokenizer from {:?}", tokenizer_path))?
    } else {
        load_tokenizer_from_bytes(DEFAULT_TOKENIZER)
            .context("Failed to load embedded GPT-2 tokenizer")?
    };

    // Process input
    if args.files.is_empty() {
        // Read from stdin
        let stdin = io::stdin();

        // Check if stdin is actually a terminal (no piped input)
        if stdin.is_terminal() {
            eprintln!("tc: reading from stdin (use --help for usage information)");
        }

        let stats = count_tokens_from_reader(stdin.lock(), &tokenizer)
            .context("Failed to count tokens from stdin")?;

        println!("{}", output_config.format_stats(&stats, None));
    } else if args.files.len() == 1 {
        // Single file
        let file = &args.files[0];
        let stats = count_tokens_in_file(file, &tokenizer)
            .with_context(|| format!("Failed to count tokens in {:?}", file))?;

        println!(
            "{}",
            output_config.format_stats(&stats, Some(&file.display().to_string()))
        );
    } else {
        // Multiple files - show each file and a total
        let mut total = TokenStats::new();

        for file in &args.files {
            match count_tokens_in_file(file, &tokenizer) {
                Ok(stats) => {
                    println!(
                        "{}",
                        output_config.format_stats(&stats, Some(&file.display().to_string()))
                    );
                    total.add(&stats);
                }
                Err(e) => {
                    eprintln!("tc: {}: {}", file.display(), e);
                    // Continue processing other files
                }
            }
        }

        // Print total
        println!("{}", output_config.format_stats(&total, Some("total")));
    }

    Ok(())
}
