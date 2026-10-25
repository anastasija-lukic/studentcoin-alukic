use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

use crate::hash::{sha256, to_hex};

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
        to_hex(&self.signing_key.to_bytes())
    }

    pub fn public_key_hex(&self) -> String {
        to_hex(&self.signing_key.verifying_key().to_bytes())
    }

    pub fn address(&self) -> String {
        address_from_public_key(&self.signing_key.verifying_key().to_bytes())
    }

    pub fn sign(&self, hash: &[u8; 32]) -> String {
        to_hex(&self.signing_key.sign(hash).to_bytes())
    }
}

impl Default for Wallet {
    fn default() -> Self {
        Self::new()
    }
}

pub fn address_from_public_key(public_key: &[u8]) -> String {
    to_hex(&sha256(public_key))
}

pub fn verify_signature(public_key: &str, hash: &[u8; 32], signature: &str) -> bool {
    let key_bytes: [u8; 32] = match hex::decode(public_key).ok().and_then(|b| b.try_into().ok()) {
        Some(bytes) => bytes,
        None => return false,
    };
    let sig_bytes: [u8; 64] = match hex::decode(signature).ok().and_then(|b| b.try_into().ok()) {
        Some(bytes) => bytes,
        None => return false,
    };
    let verifying_key = match VerifyingKey::from_bytes(&key_bytes) {
        Ok(key) => key,
        Err(_) => return false,
    };
    verifying_key
        .verify(hash, &Signature::from_bytes(&sig_bytes))
        .is_ok()
}
