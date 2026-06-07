use dust_core::{Address, Hash, SignedTransaction, Transaction};
use dust_crypto::{sign, verify_signed_transaction};
use dust_store::DustStore;
use dust_wire::{inspect_block_file, inspect_tx_file, read_file, signed_tx_payload, transaction_signing_payload};

pub fn init_store(store: &DustStore) -> anyhow::Result<String> {
    store.init()?;
    Ok(format!("initialized {}", store.root().display()))
}

pub fn create_wallet(store: &DustStore, name: &str) -> anyhow::Result<String> {
    let wallet = store.create_wallet(name.trim())?;
    Ok(format!("created wallet {} · {}", wallet.name, wallet.address.short()))
}

pub fn faucet(store: &DustStore, wallet_or_address: &str, amount: u64) -> anyhow::Result<String> {
    let address = store.resolve_address(wallet_or_address.trim())?;
    let mut state = store.load_state()?;
    state.credit(address, amount);
    store.save_state(&state)?;

    let mut genesis = store.load_genesis_state()?;
    genesis.credit(address, amount);
    store.save_genesis_state(&genesis)?;

    Ok(format!("credited {} dust to {}", amount, address.short()))
}

pub fn send_transfer(
    store: &DustStore,
    from: &str,
    to: &str,
    amount: u64,
    priority_fee: u64,
    memo: &str,
) -> anyhow::Result<String> {
    let cfg = store.config()?;
    let wallet = store.wallet(from.trim())?;
    let to = store.resolve_address(to.trim())?;
    let state = store.load_state()?;
    let nonce = state.get(&wallet.address).nonce;

    let mut tx = Transaction::transfer(
        &cfg.chain_id,
        wallet.address,
        to,
        amount,
        nonce,
        0,
        priority_fee,
        memo.as_bytes().to_vec(),
    );

    let first_sig = sign(&wallet.key, &transaction_signing_payload(&tx));
    let first = SignedTransaction::new(tx.clone(), wallet.key.public_key, first_sig, 0, Hash::ZERO);
    let first_size = signed_tx_payload(&first).len();
    let required_fee = cfg.policy.required_fee(first_size);
    tx.max_fee = required_fee;

    let signature = sign(&wallet.key, &transaction_signing_payload(&tx));
    let mut signed = SignedTransaction::new(tx, wallet.key.public_key, signature, 0, Hash::ZERO);
    let payload = signed_tx_payload(&signed);
    signed.encoded_size = payload.len();
    signed.hash = Hash::digest(&payload);

    let verifier = |s: &SignedTransaction| verify_signed_transaction(s, |inner| transaction_signing_payload(&inner.tx));
    let mut probe_state = state.clone();
    probe_state.apply_transaction(&signed, &cfg.policy, verifier)?;

    let path = store.save_pending_tx(&signed)?;
    let fee = signed.fee_breakdown(&cfg.policy);
    Ok(format!(
        "queued {} · {} bytes · fee {} dust · {}",
        signed.hash.short(),
        fee.encoded_size,
        fee.paid_fee,
        path.display()
    ))
}

pub fn mine_block(store: &DustStore) -> anyhow::Result<String> {
    let mut chain = store.load_chain()?;
    let pending = store.pending_txs()?;
    let txs: Vec<_> = pending.into_iter().map(|(_, tx)| tx).collect();
    let verifier = |s: &SignedTransaction| verify_signed_transaction(s, |inner| transaction_signing_payload(&inner.tx));
    let block = chain.mine(Address::ZERO, txs, verifier)?;
    let mined_hashes: Vec<_> = block.transactions.iter().map(|tx| tx.hash).collect();
    let path = store.append_block(&block)?;
    store.save_state(&chain.state)?;
    store.clear_pending(&mined_hashes)?;
    Ok(format!(
        "mined block #{} · {} txs · {}",
        block.header.height,
        block.transactions.len(),
        path.display()
    ))
}

pub fn clear_mempool(store: &DustStore) -> anyhow::Result<String> {
    store.clear_all_pending()?;
    Ok("cleared local mempool".to_string())
}

pub fn inspect_path(path: &str) -> anyhow::Result<String> {
    let trimmed = path.trim();
    if trimmed.ends_with(".dblk") {
        let info = inspect_block_file(&read_file(trimmed)?)?;
        return Ok(info.render());
    }
    if trimmed.ends_with(".dtx") {
        let info = inspect_tx_file(&read_file(trimmed)?)?;
        return Ok(info.render());
    }
    Err(anyhow::anyhow!("expected a .dblk or .dtx file"))
}

pub fn short_status(store: &DustStore) -> String {
    match store.db_stats() {
        Ok(stats) => format!(
            "height {} · {} pending · {} wallets · {} bytes",
            stats.height, stats.mempool_files, stats.wallet_files, stats.total_bytes
        ),
        Err(err) => format!("store unavailable: {err}"),
    }
}
