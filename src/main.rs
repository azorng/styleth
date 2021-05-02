mod eth;
mod cli;

use std::str::FromStr;
use eth::KeyPair;
use cli::{Cli, ProgressBar, Mode};
use rayon::iter::ParallelIterator;
use regex::Regex;

fn main() {
    let cli_args = Cli::new();
    let mode = cli_args.get_mode();
    let progress_bar = ProgressBar::new();

    rayon::ThreadPoolBuilder::new()
        .num_threads(num_cpus::get())
        .build()
        .unwrap()
        .install(|| {
            rayon::iter::repeat(KeyPair::generate)
                .for_each(|generate| {
                    let pair = generate();
                    progress_bar.tick();

                    let is_match = match &mode {
                        (Mode::StartsWith, input_value) => &pair.address[..input_value.len()] == input_value,
                        (Mode::Match, pattern) => Regex::from_str(&pattern).unwrap().is_match(&pair.address)
                    };

                    if is_match {
                        println!("Private key: {}", pair.get_private_key_as_hex());
                        println!("Address: {}", pair.get_address_with_prefix());
                        println!("\n");
                    }
                })
        });
}

