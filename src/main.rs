mod eth;
mod cli;
mod matcher;

use eth::KeyPair;
use cli::{Cli, ProgressBar, Mode};
use rayon::iter::ParallelIterator;
use atomic_counter::{AtomicCounter, RelaxedCounter};
use matcher::Matcher;

fn main() {
    let cli_args = Cli::new();
    let mode = cli_args.get_mode();
    let progress_bar = ProgressBar::new();
    let score = RelaxedCounter::new(1);
    let matcher = Matcher::new(mode.0, mode.1);

    rayon::iter::repeat(KeyPair::generate)
        .for_each(|generate| {
            let pair = generate();
            progress_bar.tick();

            if matcher.is_match(&pair.address) {
                if progress_bar.get_attempts() > 1000 {
                    println!("Score: {}", score.get());
                    println!("Private key: {}", pair.get_private_key_as_hex());
                    println!("Address: {}", pair.get_address_with_prefix());
                    println!("\n");
                }
                score.inc();
            }
        })
}

