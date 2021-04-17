use secp256k1::{ PublicKey, SecretKey };
use sha3::{ Digest, Keccak256 };
use rand::Rng;

pub struct KeyPair {
    private_key: SecretKey,
    pub address: String
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
            address: hex::encode(address)
        }
    }

    pub fn get_private_key_as_hex(&self) -> String {
        hex::encode(self.private_key.serialize())
    }

    pub fn get_address_with_prefix(&self) -> String {
        format!("0x{}", self.address)
    }
}
