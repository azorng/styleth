mod eth;

use eth::KeyPair;

fn main() {
    println!("Working...");

    loop {
        let pair = KeyPair::generate();
        if pair.address.starts_with("000") {
            println!("Private key: {}", pair.get_private_key_as_hex());
            println!("Address: {}", pair.get_address_with_prefix());
        }
    }
}
