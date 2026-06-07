use std::path::Path;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DashboardSnapshot {
    pub height: u64,
    pub latest_hash: String,
    pub mempool_txs: usize,
    pub avg_fee: u64,
    pub avg_tx_size: usize,
    pub db_size_bytes: u64,
}

pub fn snapshot(data_dir: impl AsRef<Path>) -> anyhow::Result<DashboardSnapshot> {
    let store = dust_store::DustStore::open(data_dir.as_ref());
    let chain = store.load_chain()?;
    let mempool = store.pending_txs()?;
    let latest_hash = chain.tip().map(|b| b.header_hash().short()).unwrap_or_else(|| "000000".to_string());
    let avg_fee = if mempool.is_empty() { 0 } else { 1 };
    let avg_tx_size = if mempool.is_empty() { 0 } else { mempool.iter().map(|(_, tx)| tx.encoded_size).sum::<usize>() / mempool.len() };
    Ok(DashboardSnapshot { height: chain.height(), latest_hash, mempool_txs: mempool.len(), avg_fee, avg_tx_size, db_size_bytes: store.db_size_bytes()? })
}

pub fn run_once(data_dir: impl AsRef<Path>) -> anyhow::Result<String> {
    let s = snapshot(data_dir)?;
    Ok(format!(
        "dustchain\nheight: {}\nlatest: {}\nmempool: {}\navg_fee: {} dust\navg_tx_size: {} bytes\ndb_size: {} bytes\n",
        s.height, s.latest_hash, s.mempool_txs, s.avg_fee, s.avg_tx_size, s.db_size_bytes
    ))
}
