use dust_core::{Address, SignedTransaction};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

use crate::{CryptoError, KeyMaterial, Result};

pub fn sign(key: &KeyMaterial, payload: &[u8]) -> [u8; 64] {
    let signing: SigningKey = key.signing_key();
    signing.sign(payload).to_bytes()
}

pub fn verify_with_public_key(public_key: &[u8; 32], signature: &[u8; 64], payload: &[u8]) -> bool {
    let Ok(verifying) = VerifyingKey::from_bytes(public_key) else {
        return false;
    };
    let sig = Signature::from_bytes(signature);
    verifying.verify(payload, &sig).is_ok()
}

pub fn verify_signed_transaction<F>(signed: &SignedTransaction, payload_encoder: F) -> bool
where
    F: Fn(&SignedTransaction) -> Vec<u8>,
{
    if Address::from_public_key(&signed.public_key) != signed.tx.from {
        return false;
    }
    let payload = payload_encoder(signed);
    verify_with_public_key(&signed.public_key, &signed.signature, &payload)
}

pub fn signature_from_slice(value: &[u8]) -> Result<[u8; 64]> {
    if value.len() != 64 {
        return Err(CryptoError::InvalidSignatureLength { received: value.len() });
    }
    let mut out = [0u8; 64];
    out.copy_from_slice(value);
    Ok(out)
}
