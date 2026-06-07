use dust_core::{Address, Block, BlockHeader, Hash, SignedTransaction, Transaction};

use crate::{
    encode::signed_tx_payload,
    magic::{BLOCK_MAGIC, TX_MAGIC, WIRE_VERSION},
    varint::{get_bytes, get_varint, take, take_array, take_u64, take_u8},
    Result, WireError,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TxFileInfo {
    pub magic: String,
    pub version: u8,
    pub payload_len: usize,
    pub tx_hash: Hash,
    pub from: Address,
    pub to: Address,
    pub amount: u64,
    pub nonce: u64,
    pub max_fee: u64,
    pub priority_fee: u64,
    pub memo_bytes: usize,
    pub checksum_valid: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlockFileInfo {
    pub magic: String,
    pub version: u8,
    pub payload_len: usize,
    pub height: u64,
    pub previous: Hash,
    pub state_root: Hash,
    pub tx_root: Hash,
    pub tx_count: usize,
    pub producer: Address,
    pub checksum_valid: bool,
}

pub fn inspect_tx_file(bytes: &[u8]) -> Result<TxFileInfo> {
    let (version, payload) = read_framed(bytes, TX_MAGIC)?;
    let signed = decode_signed_transaction_payload(payload)?;
    Ok(TxFileInfo {
        magic: "DUSTTX".to_string(),
        version,
        payload_len: payload.len(),
        tx_hash: signed.hash,
        from: signed.tx.from,
        to: signed.tx.to,
        amount: signed.tx.amount,
        nonce: signed.tx.nonce,
        max_fee: signed.tx.max_fee,
        priority_fee: signed.tx.priority_fee,
        memo_bytes: signed.tx.memo.len(),
        checksum_valid: true,
    })
}

pub fn inspect_block_file(bytes: &[u8]) -> Result<BlockFileInfo> {
    let (version, payload) = read_framed(bytes, BLOCK_MAGIC)?;
    let block = decode_block_payload(payload)?;
    Ok(BlockFileInfo {
        magic: "DUSTBLK".to_string(),
        version,
        payload_len: payload.len(),
        height: block.header.height,
        previous: block.header.previous_block_hash,
        state_root: block.header.state_root,
        tx_root: block.header.tx_root,
        tx_count: block.transactions.len(),
        producer: block.header.producer,
        checksum_valid: true,
    })
}

pub fn decode_signed_tx_file(bytes: &[u8]) -> Result<SignedTransaction> {
    let (_, payload) = read_framed(bytes, TX_MAGIC)?;
    decode_signed_transaction_payload(payload)
}

pub fn decode_block_file(bytes: &[u8]) -> Result<Block> {
    let (_, payload) = read_framed(bytes, BLOCK_MAGIC)?;
    decode_block_payload(payload)
}

pub fn decode_signed_transaction_payload(mut input: &[u8]) -> Result<SignedTransaction> {
    let tx = get_transaction(&mut input)?;
    let public_key = take_array::<32>(&mut input)?;
    let signature = take_array::<64>(&mut input)?;
    if !input.is_empty() {
        return Err(WireError::TrailingBytes(input.len()));
    }
    let encoded_size = signed_tx_payload_raw_len(&tx);
    let mut probe = SignedTransaction::new(tx, public_key, signature, encoded_size, Hash::ZERO);
    let payload = signed_tx_payload(&probe);
    let hash = Hash::digest(&payload);
    probe.encoded_size = payload.len();
    probe.hash = hash;
    Ok(probe)
}

pub fn decode_block_payload(mut input: &[u8]) -> Result<Block> {
    let header = get_block_header(&mut input)?;
    let tx_count = get_varint(&mut input)? as usize;
    let mut transactions = Vec::with_capacity(tx_count);
    for _ in 0..tx_count {
        let bytes = get_bytes(&mut input)?;
        transactions.push(decode_signed_transaction_payload(&bytes)?);
    }
    if !input.is_empty() {
        return Err(WireError::TrailingBytes(input.len()));
    }
    Ok(Block { header, transactions })
}

fn read_framed<'a>(bytes: &'a [u8], magic: &[u8]) -> Result<(u8, &'a [u8])> {
    let mut input = bytes;
    let got_magic = take(&mut input, magic.len())?;
    if got_magic != magic {
        return Err(WireError::InvalidMagic { expected: String::from_utf8_lossy(magic).into(), received: String::from_utf8_lossy(got_magic).into() });
    }
    let version = take_u8(&mut input)?;
    if version != WIRE_VERSION {
        return Err(WireError::UnsupportedVersion(version));
    }
    let payload_len = get_varint(&mut input)? as usize;
    let header_len = bytes.len() - input.len();
    let payload = take(&mut input, payload_len)?;
    let checksum = take_array::<32>(&mut input)?;
    if !input.is_empty() {
        return Err(WireError::TrailingBytes(input.len()));
    }
    let expected = Hash::digest(&bytes[..header_len + payload_len]);
    if checksum != *expected.as_bytes() {
        return Err(WireError::BadChecksum);
    }
    Ok((version, payload))
}

fn get_transaction(input: &mut &[u8]) -> Result<Transaction> {
    let version = take_u8(input)?;
    let chain_id = String::from_utf8(get_bytes(input)?).map_err(|_| WireError::InvalidUtf8)?;
    let from = Address::try_from(take(input, 32)?).map_err(|_| WireError::InvalidAddress)?;
    let to = Address::try_from(take(input, 32)?).map_err(|_| WireError::InvalidAddress)?;
    let amount = take_u64(input)?;
    let nonce = take_u64(input)?;
    let max_fee = take_u64(input)?;
    let priority_fee = take_u64(input)?;
    let memo = get_bytes(input)?;
    Ok(Transaction { version, chain_id, from, to, amount, nonce, max_fee, priority_fee, memo })
}

fn get_block_header(input: &mut &[u8]) -> Result<BlockHeader> {
    let version = take_u8(input)?;
    let chain_id = String::from_utf8(get_bytes(input)?).map_err(|_| WireError::InvalidUtf8)?;
    let height = take_u64(input)?;
    let previous_block_hash = Hash::try_from(take(input, 32)?).map_err(|_| WireError::InvalidHash)?;
    let state_root = Hash::try_from(take(input, 32)?).map_err(|_| WireError::InvalidHash)?;
    let tx_root = Hash::try_from(take(input, 32)?).map_err(|_| WireError::InvalidHash)?;
    let timestamp = take_u64(input)?;
    let producer = Address::try_from(take(input, 32)?).map_err(|_| WireError::InvalidAddress)?;
    let difficulty_or_slot = take_u64(input)?;
    let nonce_or_round = take_u64(input)?;
    Ok(BlockHeader { version, chain_id, height, previous_block_hash, state_root, tx_root, timestamp, producer, difficulty_or_slot, nonce_or_round })
}

fn signed_tx_payload_raw_len(tx: &Transaction) -> usize {
    // Exact enough to initialize the struct before re-encoding for the definitive hash.
    1 + tx.chain_id.len() + 10 + 32 + 32 + 8 + 8 + 8 + 8 + tx.memo.len() + 10 + 32 + 64
}
