use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

pub struct Wallet {
    signing_key: SigningKey,
}

impl Wallet {
    pub fn new() -> Self {
        let mut csprng = OsRng;
        Wallet {
            signing_key: SigningKey::generate(&mut csprng),
        }
    }

    pub fn from_secret_hex(secret: &str) -> Option<Self> {
        let bytes: [u8; 32] = hex::decode(secret).ok()?.try_into().ok()?;
        Some(Wallet {
            signing_key: SigningKey::from_bytes(&bytes),
        })
    }

    pub fn secret_hex(&self) -> String {
        hex::encode(self.signing_key.to_bytes())
    }

    pub fn address(&self) -> String {
        hex::encode(self.signing_key.verifying_key().to_bytes())
    }

    pub fn sign(&self, message: &[u8]) -> String {
        hex::encode(self.signing_key.sign(message).to_bytes())
    }
}

pub fn verify_signature(address: &str, message: &[u8], signature: &str) -> bool {
    let key_bytes: [u8; 32] = match hex::decode(address).ok().and_then(|b| b.try_into().ok()) {
        Some(bytes) => bytes,
        None => return false,
    };
    let sig_bytes: [u8; 64] = match hex::decode(signature).ok().and_then(|b| b.try_into().ok()) {
        Some(bytes) => bytes,
        None => return false,
    };
    let public_key = match VerifyingKey::from_bytes(&key_bytes) {
        Ok(key) => key,
        Err(_) => return false,
    };
    public_key
        .verify(message, &Signature::from_bytes(&sig_bytes))
        .is_ok()
}

