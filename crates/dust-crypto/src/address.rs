use dust_core::Address;

use crate::Result;

pub fn address_from_public_key(public_key: &[u8; 32]) -> Address {
    Address::from_public_key(public_key)
}

pub fn parse_address(value: &str) -> Result<Address> {
    value.parse().map_err(|_| crate::CryptoError::InvalidKeyMaterial)
}
