mod eth;
mod cli;

use eth::KeyPair;
use cli::Cli;

fn main() {
    println!("Working...");

    let cli_args = Cli::new();

    loop {
        let pair = KeyPair::generate();
        if &pair.address[..cli_args.starts_with.len()] == cli_args.starts_with {
            println!("\n");
            println!("Private key: {}", pair.get_private_key_as_hex());
            println!("Address: {}", pair.get_address_with_prefix());
        }
    }
}
