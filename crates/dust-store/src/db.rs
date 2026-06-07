use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use dust_core::{Account, Address, Block, Chain, FeePolicy, Hash, State};
use dust_crypto::{address_from_public_key, KeyMaterial};
use dust_wire::{
    block_file_bytes,
    decode::decode_block_file,
    decode::decode_signed_tx_file,
    read_file,
    signed_tx_file_bytes,
};

use crate::{Result, StoreError};

#[derive(Clone, Debug)]
pub struct ChainConfig {
    pub chain_id: String,
    pub policy: FeePolicy,
    pub data_dir: PathBuf,
}

impl ChainConfig {
    pub fn default_for(data_dir: impl Into<PathBuf>) -> Self {
        Self { chain_id: "dust-local".to_string(), policy: FeePolicy::default(), data_dir: data_dir.into() }
    }

    pub fn render(&self) -> String {
        format!(r#"[chain]
chain_id = "{}"
target_block_time_secs = 5
max_block_size_bytes = {}
max_tx_size_bytes = {}
max_memo_bytes = {}

[fees]
base_fee = {}
fee_per_kb = {}
included_bytes = {}
max_priority_fee = {}

[mempool]
max_txs = 10000
max_txs_per_account = 64
max_txs_per_peer = 512

[node]
host = "127.0.0.1"
port = 3030
data_dir = "{}"

[logs]
level = "info"
"#,
            self.chain_id,
            self.policy.max_block_size_bytes,
            self.policy.max_tx_size_bytes,
            self.policy.max_memo_bytes,
            self.policy.base_fee,
            self.policy.fee_per_kb,
            self.policy.included_bytes,
            self.policy.max_priority_fee,
            self.data_dir.display(),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WalletRecord {
    pub name: String,
    pub key: KeyMaterial,
    pub address: Address,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoreMetadata {
    pub schema_version: u32,
    pub chain_id: String,
    pub height: u64,
    pub tip_hash: Hash,
    pub state_root: Hash,
    pub blocks: usize,
    pub updated_at_unix: u64,
}

impl StoreMetadata {
    pub fn fresh(chain_id: impl Into<String>, state: &State) -> Self {
        Self {
            schema_version: 1,
            chain_id: chain_id.into(),
            height: 0,
            tip_hash: Hash::ZERO,
            state_root: state.root_hash(),
            blocks: 0,
            updated_at_unix: now_unix(),
        }
    }

    pub fn render(&self) -> String {
        format!(
            "schema_version={}\nchain_id={}\nheight={}\ntip_hash={}\nstate_root={}\nblocks={}\nupdated_at_unix={}\n",
            self.schema_version,
            self.chain_id,
            self.height,
            self.tip_hash,
            self.state_root,
            self.blocks,
            self.updated_at_unix,
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlockIndexEntry {
    pub height: u64,
    pub hash: Hash,
    pub previous: Hash,
    pub txs: usize,
    pub bytes: u64,
    pub file_name: String,
}

impl BlockIndexEntry {
    fn from_block(block: &Block, bytes: u64, file_name: String) -> Self {
        Self {
            height: block.header.height,
            hash: block.header_hash(),
            previous: block.header.previous_block_hash,
            txs: block.transactions.len(),
            bytes,
            file_name,
        }
    }

    fn render_header() -> &'static str {
        "height\thash\tprevious\ttxs\tbytes\tfile\n"
    }

    fn render(&self) -> String {
        format!("{}\t{}\t{}\t{}\t{}\t{}\n", self.height, self.hash, self.previous, self.txs, self.bytes, self.file_name)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoreStats {
    pub total_bytes: u64,
    pub block_files: usize,
    pub block_bytes: u64,
    pub mempool_files: usize,
    pub mempool_bytes: u64,
    pub wallet_files: usize,
    pub wallet_bytes: u64,
    pub state_bytes: u64,
    pub metadata_bytes: u64,
    pub height: u64,
    pub tip_hash: Hash,
    pub state_root: Hash,
}

impl StoreStats {
    pub fn render(&self, verbose: bool) -> String {
        let mut out = String::new();
        out.push_str(&format!("height: {}\n", self.height));
        out.push_str(&format!("tip_hash: {}\n", self.tip_hash));
        out.push_str(&format!("state_root: {}\n", self.state_root));
        out.push_str(&format!("total_bytes: {}\n", self.total_bytes));
        out.push_str(&format!("block_files: {}\n", self.block_files));
        out.push_str(&format!("mempool_files: {}\n", self.mempool_files));
        out.push_str(&format!("wallet_files: {}\n", self.wallet_files));
        if verbose {
            out.push_str(&format!("block_bytes: {}\n", self.block_bytes));
            out.push_str(&format!("mempool_bytes: {}\n", self.mempool_bytes));
            out.push_str(&format!("wallet_bytes: {}\n", self.wallet_bytes));
            out.push_str(&format!("state_bytes: {}\n", self.state_bytes));
            out.push_str(&format!("metadata_bytes: {}\n", self.metadata_bytes));
        }
        out
    }
}

#[derive(Clone, Debug)]
pub struct DustStore {
    root: PathBuf,
}

impl DustStore {
    pub fn open(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn init(&self) -> Result<()> {
        fs::create_dir_all(self.wallets_dir())?;
        fs::create_dir_all(self.blocks_dir())?;
        fs::create_dir_all(self.mempool_dir())?;
        fs::create_dir_all(self.metadata_dir())?;
        let config = ChainConfig::default_for(self.root.clone());
        write_text_atomic(&self.config_path(), &config.render())?;
        let empty = State::new();
        self.save_genesis_state(&empty)?;
        self.save_state(&empty)?;
        self.write_metadata(&StoreMetadata::fresh(config.chain_id, &empty))?;
        self.rebuild_index()?;
        Ok(())
    }

    pub fn config(&self) -> Result<ChainConfig> {
        let path = self.config_path();
        if !path.exists() {
            return Ok(ChainConfig::default_for(self.root.clone()));
        }
        let text = fs::read_to_string(path)?;
        let chain_id = find_value(&text, "chain_id").unwrap_or_else(|| "dust-local".to_string());
        let mut policy = FeePolicy::default();
        policy.base_fee = find_value(&text, "base_fee").and_then(|v| v.parse().ok()).unwrap_or(policy.base_fee);
        policy.fee_per_kb = find_value(&text, "fee_per_kb").and_then(|v| v.parse().ok()).unwrap_or(policy.fee_per_kb);
        policy.included_bytes = find_value(&text, "included_bytes").and_then(|v| v.parse().ok()).unwrap_or(policy.included_bytes);
        policy.max_priority_fee = find_value(&text, "max_priority_fee").and_then(|v| v.parse().ok()).unwrap_or(policy.max_priority_fee);
        policy.max_block_size_bytes = find_value(&text, "max_block_size_bytes").and_then(|v| v.parse().ok()).unwrap_or(policy.max_block_size_bytes);
        policy.max_tx_size_bytes = find_value(&text, "max_tx_size_bytes").and_then(|v| v.parse().ok()).unwrap_or(policy.max_tx_size_bytes);
        policy.max_memo_bytes = find_value(&text, "max_memo_bytes").and_then(|v| v.parse().ok()).unwrap_or(policy.max_memo_bytes);
        Ok(ChainConfig { chain_id, policy, data_dir: self.root.clone() })
    }

    pub fn create_wallet(&self, name: &str) -> Result<WalletRecord> {
        validate_wallet_name(name)?;
        fs::create_dir_all(self.wallets_dir())?;
        let key = KeyMaterial::generate();
        let address = address_from_public_key(&key.public_key);
        let wallet = WalletRecord { name: name.to_string(), key, address };
        write_text_atomic(&self.wallet_path(name), &render_wallet(&wallet))?;
        let mut state = self.load_state()?;
        state.ensure_account(address);
        self.save_state(&state)?;
        Ok(wallet)
    }

    pub fn wallet(&self, name: &str) -> Result<WalletRecord> {
        validate_wallet_name(name)?;
        let path = self.wallet_path(name);
        if !path.exists() {
            return Err(StoreError::WalletNotFound(name.to_string()));
        }
        parse_wallet(&fs::read_to_string(path)?)
    }

    pub fn wallet_by_address(&self, address: Address) -> Result<Option<WalletRecord>> {
        for wallet in self.wallets()? {
            if wallet.address == address {
                return Ok(Some(wallet));
            }
        }
        Ok(None)
    }

    pub fn wallets(&self) -> Result<Vec<WalletRecord>> {
        fs::create_dir_all(self.wallets_dir())?;
        let mut wallets = Vec::new();
        for entry in fs::read_dir(self.wallets_dir())? {
            let entry = entry?;
            if entry.path().extension().and_then(|x| x.to_str()) == Some("wallet") {
                wallets.push(parse_wallet(&fs::read_to_string(entry.path())?)?);
            }
        }
        wallets.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(wallets)
    }

    pub fn resolve_address(&self, value: &str) -> Result<Address> {
        if let Ok(wallet) = self.wallet(value) {
            return Ok(wallet.address);
        }
        value.parse().map_err(|_| StoreError::Parse(format!("not a wallet name or address: {value}")))
    }

    pub fn load_genesis_state(&self) -> Result<State> {
        let path = self.genesis_state_path();
        if !path.exists() {
            return Ok(State::new());
        }
        parse_state(&fs::read_to_string(path)?)
    }

    pub fn save_genesis_state(&self, state: &State) -> Result<()> {
        write_text_atomic(&self.genesis_state_path(), &render_state(state))?;
        Ok(())
    }

    pub fn load_state(&self) -> Result<State> {
        let path = self.state_path();
        if !path.exists() {
            return Ok(State::new());
        }
        parse_state(&fs::read_to_string(path)?)
    }

    pub fn save_state(&self, state: &State) -> Result<()> {
        write_text_atomic(&self.state_path(), &render_state(state))?;
        if self.metadata_path().exists() {
            let mut metadata = self.metadata()?;
            metadata.state_root = state.root_hash();
            metadata.updated_at_unix = now_unix();
            self.write_metadata(&metadata)?;
        }
        Ok(())
    }

    pub fn export_state_text(&self) -> Result<String> {
        Ok(render_state(&self.load_state()?))
    }

    pub fn load_chain(&self) -> Result<Chain> {
        let cfg = self.config()?;
        let genesis_state = self.load_genesis_state()?;
        let mut chain = Chain::with_genesis_state(cfg.chain_id, cfg.policy, genesis_state);
        chain.blocks.truncate(1);
        chain.blocks.extend(self.load_blocks_ordered()?.into_iter().map(|(_, block)| block));
        chain.state = self.load_state()?;
        Ok(chain)
    }

    pub fn load_blocks_ordered(&self) -> Result<Vec<(PathBuf, Block)>> {
        let mut blocks = Vec::new();
        if self.blocks_dir().exists() {
            for entry in fs::read_dir(self.blocks_dir())? {
                let entry = entry?;
                if entry.path().extension().and_then(|x| x.to_str()) == Some("dblk") {
                    let block = decode_block_file(&read_file(entry.path())?)?;
                    blocks.push((entry.path(), block));
                }
            }
        }
        blocks.sort_by_key(|(_, block)| block.header.height);
        for (idx, (_, block)) in blocks.iter().enumerate() {
            let expected = idx as u64 + 1;
            if block.header.height != expected {
                return Err(StoreError::Layout(format!("missing or non-contiguous block height; expected {expected}, received {}", block.header.height)));
            }
        }
        Ok(blocks)
    }

    pub fn append_block(&self, block: &Block) -> Result<PathBuf> {
        fs::create_dir_all(self.blocks_dir())?;
        fs::create_dir_all(self.metadata_dir())?;
        let file_name = format!("{:08}.dblk", block.header.height);
        let path = self.blocks_dir().join(&file_name);
        write_bytes_atomic(&path, &block_file_bytes(block))?;
        self.rebuild_index()?;
        let mut metadata = self.metadata().unwrap_or_else(|_| StoreMetadata::fresh(block.header.chain_id.clone(), &State::new()));
        metadata.chain_id = block.header.chain_id.clone();
        metadata.height = block.header.height;
        metadata.tip_hash = block.header_hash();
        metadata.state_root = block.header.state_root;
        metadata.blocks = self.load_blocks_ordered()?.len();
        metadata.updated_at_unix = now_unix();
        self.write_metadata(&metadata)?;
        Ok(path)
    }

    pub fn save_pending_tx(&self, tx: &dust_core::SignedTransaction) -> Result<PathBuf> {
        fs::create_dir_all(self.mempool_dir())?;
        let path = self.mempool_dir().join(format!("{}.dtx", tx.hash.short()));
        write_bytes_atomic(&path, &signed_tx_file_bytes(tx))?;
        Ok(path)
    }

    pub fn pending_txs(&self) -> Result<Vec<(PathBuf, dust_core::SignedTransaction)>> {
        fs::create_dir_all(self.mempool_dir())?;
        let mut out = Vec::new();
        for entry in fs::read_dir(self.mempool_dir())? {
            let entry = entry?;
            if entry.path().extension().and_then(|x| x.to_str()) == Some("dtx") {
                let tx = decode_signed_tx_file(&read_file(entry.path())?)?;
                out.push((entry.path(), tx));
            }
        }
        out.sort_by_key(|(_, tx)| (tx.tx.from, tx.tx.nonce, std::cmp::Reverse(tx.tx.priority_fee)));
        Ok(out)
    }

    pub fn clear_pending(&self, mined: &[dust_core::Hash]) -> Result<()> {
        let mined: std::collections::HashSet<_> = mined.iter().copied().collect();
        for (path, tx) in self.pending_txs()? {
            if mined.contains(&tx.hash) {
                fs::remove_file(path)?;
            }
        }
        Ok(())
    }

    pub fn clear_all_pending(&self) -> Result<()> {
        if self.mempool_dir().exists() {
            for entry in fs::read_dir(self.mempool_dir())? {
                let path = entry?.path();
                if path.extension().and_then(|x| x.to_str()) == Some("dtx") {
                    fs::remove_file(path)?;
                }
            }
        }
        Ok(())
    }

    pub fn metadata(&self) -> Result<StoreMetadata> {
        let path = self.metadata_path();
        if !path.exists() {
            let cfg = self.config()?;
            return Ok(StoreMetadata::fresh(cfg.chain_id, &self.load_state()?));
        }
        parse_metadata(&fs::read_to_string(path)?)
    }

    pub fn rebuild_index(&self) -> Result<Vec<BlockIndexEntry>> {
        fs::create_dir_all(self.metadata_dir())?;
        let mut entries = Vec::new();
        for (path, block) in self.load_blocks_ordered()? {
            let bytes = fs::metadata(&path)?.len();
            let file_name = path.file_name().and_then(|v| v.to_str()).unwrap_or("unknown.dblk").to_string();
            entries.push(BlockIndexEntry::from_block(&block, bytes, file_name));
        }
        let mut out = String::from(BlockIndexEntry::render_header());
        for entry in &entries {
            out.push_str(&entry.render());
        }
        write_text_atomic(&self.block_index_path(), &out)?;

        let cfg = self.config()?;
        let state = self.load_state()?;
        let mut metadata = StoreMetadata::fresh(cfg.chain_id, &state);
        if let Some(last) = entries.last() {
            metadata.height = last.height;
            metadata.tip_hash = last.hash;
            metadata.blocks = entries.len();
        }
        self.write_metadata(&metadata)?;
        Ok(entries)
    }

    pub fn db_size_bytes(&self) -> Result<u64> {
        Ok(dir_size(&self.root)?)
    }

    pub fn db_stats(&self) -> Result<StoreStats> {
        let metadata = self.metadata()?;
        let (block_files, block_bytes) = dir_count_and_size(&self.blocks_dir(), Some("dblk"))?;
        let (mempool_files, mempool_bytes) = dir_count_and_size(&self.mempool_dir(), Some("dtx"))?;
        let (wallet_files, wallet_bytes) = dir_count_and_size(&self.wallets_dir(), Some("wallet"))?;
        let state_bytes = file_size(&self.state_path())? + file_size(&self.genesis_state_path())?;
        let metadata_bytes = dir_size(&self.metadata_dir())?;
        Ok(StoreStats {
            total_bytes: self.db_size_bytes()?,
            block_files,
            block_bytes,
            mempool_files,
            mempool_bytes,
            wallet_files,
            wallet_bytes,
            state_bytes,
            metadata_bytes,
            height: metadata.height,
            tip_hash: metadata.tip_hash,
            state_root: metadata.state_root,
        })
    }

    fn write_metadata(&self, metadata: &StoreMetadata) -> Result<()> {
        fs::create_dir_all(self.metadata_dir())?;
        write_text_atomic(&self.metadata_path(), &metadata.render())?;
        Ok(())
    }

    fn config_path(&self) -> PathBuf { self.root.join("config.toml") }
    fn state_path(&self) -> PathBuf { self.root.join("state.snapshot") }
    fn genesis_state_path(&self) -> PathBuf { self.root.join("genesis.snapshot") }
    fn wallets_dir(&self) -> PathBuf { self.root.join("wallets") }
    fn wallet_path(&self, name: &str) -> PathBuf { self.wallets_dir().join(format!("{name}.wallet")) }
    fn blocks_dir(&self) -> PathBuf { self.root.join("blocks") }
    fn mempool_dir(&self) -> PathBuf { self.root.join("mempool") }
    fn metadata_dir(&self) -> PathBuf { self.root.join("metadata") }
    fn metadata_path(&self) -> PathBuf { self.metadata_dir().join("manifest.txt") }
    fn block_index_path(&self) -> PathBuf { self.metadata_dir().join("block-index.tsv") }
}

fn render_wallet(wallet: &WalletRecord) -> String {
    format!("name={}\nsecret_key={}\npublic_key={}\naddress={}\n", wallet.name, wallet.key.secret_hex(), wallet.key.public_hex(), wallet.address)
}

fn parse_wallet(text: &str) -> Result<WalletRecord> {
    let name = find_value(text, "name").ok_or_else(|| StoreError::Parse("wallet missing name".to_string()))?;
    let secret = find_value(text, "secret_key").ok_or_else(|| StoreError::Parse("wallet missing secret_key".to_string()))?;
    let key = KeyMaterial::from_secret_hex(&secret)?;
    let address = address_from_public_key(&key.public_key);
    Ok(WalletRecord { name, key, address })
}

fn render_state(state: &State) -> String {
    let mut out = String::new();
    for account in state.accounts() {
        out.push_str(&format!("{} {} {}\n", account.address, account.balance, account.nonce));
    }
    out
}

fn parse_state(text: &str) -> Result<State> {
    let mut state = State::new();
    for (idx, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() { continue; }
        let parts: Vec<_> = line.split_whitespace().collect();
        if parts.len() != 3 {
            return Err(StoreError::Parse(format!("bad state line {}", idx + 1)));
        }
        let address: Address = parts[0].parse().map_err(|_| StoreError::Parse(format!("bad address on state line {}", idx + 1)))?;
        let balance: u64 = parts[1].parse().map_err(|_| StoreError::Parse(format!("bad balance on state line {}", idx + 1)))?;
        let nonce: u64 = parts[2].parse().map_err(|_| StoreError::Parse(format!("bad nonce on state line {}", idx + 1)))?;
        state.set_account(Account { address, balance, nonce });
    }
    Ok(state)
}

fn parse_metadata(text: &str) -> Result<StoreMetadata> {
    let schema_version = find_value(text, "schema_version").unwrap_or_else(|| "1".to_string()).parse().map_err(|_| StoreError::Parse("bad metadata schema_version".to_string()))?;
    let chain_id = find_value(text, "chain_id").unwrap_or_else(|| "dust-local".to_string());
    let height = find_value(text, "height").unwrap_or_else(|| "0".to_string()).parse().map_err(|_| StoreError::Parse("bad metadata height".to_string()))?;
    let tip_hash = parse_hash_value(text, "tip_hash")?.unwrap_or(Hash::ZERO);
    let state_root = parse_hash_value(text, "state_root")?.unwrap_or(Hash::ZERO);
    let blocks = find_value(text, "blocks").unwrap_or_else(|| "0".to_string()).parse().map_err(|_| StoreError::Parse("bad metadata blocks".to_string()))?;
    let updated_at_unix = find_value(text, "updated_at_unix").unwrap_or_else(|| "0".to_string()).parse().map_err(|_| StoreError::Parse("bad metadata updated_at_unix".to_string()))?;
    Ok(StoreMetadata { schema_version, chain_id, height, tip_hash, state_root, blocks, updated_at_unix })
}

fn parse_hash_value(text: &str, key: &str) -> Result<Option<Hash>> {
    let Some(value) = find_value(text, key) else {
        return Ok(None);
    };
    Ok(Some(value.parse().map_err(|_| StoreError::Parse(format!("bad metadata hash: {key}")))?))
}

fn find_value(text: &str, key: &str) -> Option<String> {
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with('#') || !line.contains('=') { continue; }
        let (k, v) = line.split_once('=')?;
        if k.trim() == key {
            return Some(v.trim().trim_matches('"').to_string());
        }
    }
    None
}

fn validate_wallet_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(StoreError::Parse("wallet name cannot be empty".to_string()));
    }
    if !name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_') {
        return Err(StoreError::Parse("wallet name may only contain letters, numbers, '-' and '_'".to_string()));
    }
    Ok(())
}

fn write_text_atomic(path: &Path, text: &str) -> Result<()> {
    write_bytes_atomic(path, text.as_bytes())
}

fn write_bytes_atomic(path: &Path, bytes: &[u8]) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let tmp = tmp_path(path);
    fs::write(&tmp, bytes)?;
    fs::rename(&tmp, path)?;
    Ok(())
}

fn tmp_path(path: &Path) -> PathBuf {
    let file_name = path.file_name().and_then(|v| v.to_str()).unwrap_or("dust.tmp");
    path.with_file_name(format!(".{file_name}.tmp"))
}

fn file_size(path: &Path) -> Result<u64> {
    if !path.exists() {
        return Ok(0);
    }
    Ok(fs::metadata(path)?.len())
}

fn dir_size(path: &Path) -> std::io::Result<u64> {
    if !path.exists() {
        return Ok(0);
    }
    let mut total = 0;
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let meta = entry.metadata()?;
        if meta.is_dir() {
            total += dir_size(&entry.path())?;
        } else {
            total += meta.len();
        }
    }
    Ok(total)
}

fn dir_count_and_size(path: &Path, extension: Option<&str>) -> Result<(usize, u64)> {
    if !path.exists() {
        return Ok((0, 0));
    }
    let mut count = 0;
    let mut bytes = 0;
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let meta = entry.metadata()?;
        if meta.is_dir() {
            let (nested_count, nested_bytes) = dir_count_and_size(&entry.path(), extension)?;
            count += nested_count;
            bytes += nested_bytes;
        } else {
            let matches = extension.is_none() || entry.path().extension().and_then(|x| x.to_str()) == extension;
            if matches {
                count += 1;
                bytes += meta.len();
            }
        }
    }
    Ok((count, bytes))
}

fn now_unix() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs()
}
