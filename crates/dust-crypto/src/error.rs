pub type Result<T> = std::result::Result<T, CryptoError>;

#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("invalid secret key length: expected 32 bytes, received {received}")]
    InvalidSecretKeyLength { received: usize },

    #[error("invalid public key length: expected 32 bytes, received {received}")]
    InvalidPublicKeyLength { received: usize },

    #[error("invalid signature length: expected 64 bytes, received {received}")]
    InvalidSignatureLength { received: usize },

    #[error("invalid key material")]
    InvalidKeyMaterial,

    #[error("hex decode failed")]
    HexDecode,
}
