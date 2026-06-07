use ed25519_dalek::{SigningKey, VerifyingKey};
use rand_core::OsRng;

use crate::{CryptoError, Result};

pub type SecretKeyBytes = [u8; 32];
pub type PublicKeyBytes = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyMaterial {
    pub secret_key: SecretKeyBytes,
    pub public_key: PublicKeyBytes,
}

impl KeyMaterial {
    pub fn generate() -> Self {
        let signing = SigningKey::generate(&mut OsRng);
        let verifying = signing.verifying_key();
        Self { secret_key: signing.to_bytes(), public_key: verifying.to_bytes() }
    }

    pub fn from_secret_hex(value: &str) -> Result<Self> {
        let bytes = hex::decode(value).map_err(|_| CryptoError::HexDecode)?;
        if bytes.len() != 32 {
            return Err(CryptoError::InvalidSecretKeyLength { received: bytes.len() });
        }
        let mut secret_key = [0u8; 32];
        secret_key.copy_from_slice(&bytes);
        let signing = SigningKey::from_bytes(&secret_key);
        let verifying = signing.verifying_key();
        Ok(Self { secret_key, public_key: verifying.to_bytes() })
    }

    pub fn secret_hex(&self) -> String {
        hex::encode(self.secret_key)
    }

    pub fn public_hex(&self) -> String {
        hex::encode(self.public_key)
    }

    pub fn signing_key(&self) -> SigningKey {
        SigningKey::from_bytes(&self.secret_key)
    }

    pub fn verifying_key(&self) -> Result<VerifyingKey> {
        VerifyingKey::from_bytes(&self.public_key).map_err(|_| CryptoError::InvalidKeyMaterial)
    }
}
