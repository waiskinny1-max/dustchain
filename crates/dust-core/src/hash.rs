use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

use crate::{DustError, Result};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Hash(pub [u8; 32]);

impl Hash {
    pub const ZERO: Self = Self([0u8; 32]);

    pub fn digest(bytes: impl AsRef<[u8]>) -> Self {
        Self(*blake3::hash(bytes.as_ref()).as_bytes())
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn short(&self) -> String {
        hex::encode(&self.0[..6])
    }
}

impl Default for Hash {
    fn default() -> Self {
        Self::ZERO
    }
}

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

impl From<[u8; 32]> for Hash {
    fn from(value: [u8; 32]) -> Self {
        Self(value)
    }
}

impl TryFrom<&[u8]> for Hash {
    type Error = DustError;

    fn try_from(value: &[u8]) -> Result<Self> {
        if value.len() != 32 {
            return Err(DustError::InvalidHashLength { received: value.len() });
        }
        let mut out = [0u8; 32];
        out.copy_from_slice(value);
        Ok(Self(out))
    }
}

impl FromStr for Hash {
    type Err = DustError;

    fn from_str(s: &str) -> Result<Self> {
        let bytes = hex::decode(s).map_err(|_| DustError::InvalidHashLength { received: s.len() / 2 })?;
        Self::try_from(bytes.as_slice())
    }
}
