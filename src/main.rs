mod eth;
mod cli;

use eth::KeyPair;
use cli::{Cli, ProgressBar};
use rayon::iter::ParallelIterator;

fn main() {
    let cli_args = Cli::new();
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
                    if &pair.address[..cli_args.starts_with.len()] == cli_args.starts_with {
                        println!("Private key: {}", pair.get_private_key_as_hex());
                        println!("Address: {}", pair.get_address_with_prefix());
                        println!("\n");
                    }
                })
        });
}
