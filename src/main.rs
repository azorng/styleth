mod cli;
mod eth;
mod matcher;

use atomic_counter::AtomicCounter;
use cli::{Cli, ProgressBar};
use eth::KeyPair;
use matcher::Matcher;
use rayon::iter::ParallelIterator;

fn main() {
    let cli_args = Cli::new();
    let mode = cli_args.get_mode();
    let progress_bar = ProgressBar::new();
    let matcher = Matcher::new(mode);

    rayon::iter::repeat(KeyPair::generate).for_each(|generate| {
        let pair = generate();
        progress_bar.tick();

        if matcher.is_match(&pair.address) {
            if matcher.score.get() != 0 {
                println!("Score: {}", matcher.score.get());
            }

            println!("Private key: {}", pair.get_private_key_as_hex());
            println!("Address: {}", pair.get_address_with_prefix());
            println!("\n");
        }
    })
}
