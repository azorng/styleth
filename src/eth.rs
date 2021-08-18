use rand::Rng;
use secp256k1::{PublicKey, SecretKey};
use sha3::{Digest, Keccak256};

pub struct KeyPair {
    private_key: SecretKey,
    pub address: String,
}

impl KeyPair {
    pub fn generate() -> KeyPair {
        let random_bytes = rand::thread_rng().gen::<[u8; 32]>();

        let private_key = SecretKey::parse(&random_bytes).unwrap();
        let public_key = PublicKey::from_secret_key(&private_key);

        let public_key_hash = Keccak256::digest(&public_key.serialize()[1..]);
        let address = &public_key_hash[12..];

        KeyPair {
            private_key,
            address: hex::encode(address),
        }
    }

    pub fn get_private_key_as_hex(&self) -> String {
        hex::encode(self.private_key.serialize())
    }

    pub fn get_address_with_prefix(&self) -> String {
        format!("0x{}", self.address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_check_valid_address() {
        let key_pair = KeyPair::generate();
        let is_every_char_hex = key_pair
            .address
            .chars()
            .all(|c| "0123456789abcdef".chars().any(|hex_char| hex_char == c));

        assert_eq!(40, key_pair.address.len());
        assert!(is_every_char_hex);
    }
}
