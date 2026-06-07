use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

use crate::{DustError, Result};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Address(pub [u8; 32]);

impl Address {
    pub const ZERO: Self = Self([0u8; 32]);

    pub fn from_public_key(public_key: &[u8; 32]) -> Self {
        Self(*blake3::hash(public_key).as_bytes())
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn short(&self) -> String {
        format!("dust:{}", hex::encode(&self.0[..5]))
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "dust1{}", hex::encode(self.0))
    }
}

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Default for Address {
    fn default() -> Self {
        Self::ZERO
    }
}

impl TryFrom<&[u8]> for Address {
    type Error = DustError;

    fn try_from(value: &[u8]) -> Result<Self> {
        if value.len() != 32 {
            return Err(DustError::InvalidAddressLength { received: value.len() });
        }
        let mut out = [0u8; 32];
        out.copy_from_slice(value);
        Ok(Self(out))
    }
}

impl FromStr for Address {
    type Err = DustError;

    fn from_str(s: &str) -> Result<Self> {
        let raw = s.strip_prefix("dust1").unwrap_or(s);
        let bytes = hex::decode(raw).map_err(|_| DustError::InvalidAddressLength { received: raw.len() / 2 })?;
        Address::try_from(bytes.as_slice())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Account {
    pub address: Address,
    pub balance: u64,
    pub nonce: u64,
}

impl Account {
    pub fn new(address: Address) -> Self {
        Self { address, balance: 0, nonce: 0 }
    }
}
