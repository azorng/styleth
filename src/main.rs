use secp256k1::{ PublicKey, SecretKey };
use sha3::{ Digest, Keccak256 };
use rand::Rng;

struct KeyPair {
    private_key: SecretKey,
    address: String
}

impl KeyPair {
    fn get_hex_private_key(&self) -> String {
        hex::encode(self.private_key.serialize())
    }

    fn get_full_address(&self) -> String {
        format!("{}{}", "0x", self.address)
    }
}

fn main() {
    println!("Working...");
    loop {
        let pair = generate_pair();
        if pair.address.starts_with("00000") {
            println!("{}", pair.get_hex_private_key());
            println!("{}", pair.get_full_address());
            break;
        }
    }
}

fn generate_pair() -> KeyPair {
    let random_bytes = rand::thread_rng().gen::<[u8; 32]>();

    let private_key = SecretKey::parse(&random_bytes).unwrap();
    let public_key = PublicKey::from_secret_key(&private_key);

    let key_hash = Keccak256::digest(&public_key.serialize()[1..]);
    let address = &key_hash[12..];

    KeyPair {
        private_key,
        address: hex::encode(address)
    }
}
