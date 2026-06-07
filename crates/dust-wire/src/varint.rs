use crate::{Result, WireError};

pub fn put_varint(out: &mut Vec<u8>, mut value: u64) {
    while value >= 0x80 {
        out.push((value as u8) | 0x80);
        value >>= 7;
    }
    out.push(value as u8);
}

pub fn get_varint(input: &mut &[u8]) -> Result<u64> {
    let mut result = 0u64;
    let mut shift = 0u32;

    for _ in 0..10 {
        let byte = take_u8(input)?;
        result |= ((byte & 0x7f) as u64) << shift;
        if byte & 0x80 == 0 {
            return Ok(result);
        }
        shift += 7;
    }

    Err(WireError::VarintTooLarge)
}

pub fn put_bytes(out: &mut Vec<u8>, bytes: &[u8]) {
    put_varint(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}

pub fn get_bytes(input: &mut &[u8]) -> Result<Vec<u8>> {
    let len = get_varint(input)? as usize;
    let bytes = take(input, len)?;
    Ok(bytes.to_vec())
}

pub fn take_u8(input: &mut &[u8]) -> Result<u8> {
    if input.is_empty() {
        return Err(WireError::UnexpectedEof);
    }
    let byte = input[0];
    *input = &input[1..];
    Ok(byte)
}

pub fn take_u64(input: &mut &[u8]) -> Result<u64> {
    let bytes = take(input, 8)?;
    let mut out = [0u8; 8];
    out.copy_from_slice(bytes);
    Ok(u64::from_le_bytes(out))
}

pub fn take_array<const N: usize>(input: &mut &[u8]) -> Result<[u8; N]> {
    let bytes = take(input, N)?;
    let mut out = [0u8; N];
    out.copy_from_slice(bytes);
    Ok(out)
}

pub fn take<'a>(input: &mut &'a [u8], n: usize) -> Result<&'a [u8]> {
    if input.len() < n {
        return Err(WireError::UnexpectedEof);
    }
    let (head, tail) = input.split_at(n);
    *input = tail;
    Ok(head)
}
