use std::{fs, path::{Path, PathBuf}};

use dust_core::{Account, Address, Chain, FeePolicy, State};
use dust_crypto::{address_from_public_key, KeyMaterial};
use dust_wire::{block_file_bytes, decode::decode_block_file, decode::decode_signed_tx_file, read_file, signed_tx_file_bytes, write_file};

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
        let config = ChainConfig::default_for(self.root.clone());
        fs::write(self.config_path(), config.render())?;
        let empty = State::new();
        self.save_genesis_state(&empty)?;
        self.save_state(&empty)?;
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
        fs::create_dir_all(self.wallets_dir())?;
        let key = KeyMaterial::generate();
        let address = address_from_public_key(&key.public_key);
        let wallet = WalletRecord { name: name.to_string(), key, address };
        fs::write(self.wallet_path(name), render_wallet(&wallet))?;
        let mut state = self.load_state()?;
        state.ensure_account(address);
        self.save_state(&state)?;
        Ok(wallet)
    }

    pub fn wallet(&self, name: &str) -> Result<WalletRecord> {
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
        if let Some(parent) = self.genesis_state_path().parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(self.genesis_state_path(), render_state(state))?;
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
        if let Some(parent) = self.state_path().parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(self.state_path(), render_state(state))?;
        Ok(())
    }

    pub fn load_chain(&self) -> Result<Chain> {
        let cfg = self.config()?;
        let genesis_state = self.load_genesis_state()?;
        let mut chain = Chain::with_genesis_state(cfg.chain_id, cfg.policy, genesis_state);
        chain.blocks.truncate(1);
        let mut blocks = Vec::new();
        if self.blocks_dir().exists() {
            for entry in fs::read_dir(self.blocks_dir())? {
                let entry = entry?;
                if entry.path().extension().and_then(|x| x.to_str()) == Some("dblk") {
                    blocks.push(decode_block_file(&read_file(entry.path())?)?);
                }
            }
        }
        blocks.sort_by_key(|b| b.header.height);
        chain.blocks.extend(blocks);
        chain.state = self.load_state()?;
        Ok(chain)
    }

    pub fn append_block(&self, block: &dust_core::Block) -> Result<PathBuf> {
        fs::create_dir_all(self.blocks_dir())?;
        let path = self.blocks_dir().join(format!("{:08}.dblk", block.header.height));
        write_file(&path, &block_file_bytes(block))?;
        Ok(path)
    }

    pub fn save_pending_tx(&self, tx: &dust_core::SignedTransaction) -> Result<PathBuf> {
        fs::create_dir_all(self.mempool_dir())?;
        let path = self.mempool_dir().join(format!("{}.dtx", tx.hash.short()));
        write_file(&path, &signed_tx_file_bytes(tx))?;
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
        out.sort_by_key(|(_, tx)| tx.tx.nonce);
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

    pub fn db_size_bytes(&self) -> Result<u64> {
        fn walk(path: &Path) -> std::io::Result<u64> {
            if !path.exists() { return Ok(0); }
            let mut total = 0;
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let meta = entry.metadata()?;
                if meta.is_dir() {
                    total += walk(&entry.path())?;
                } else {
                    total += meta.len();
                }
            }
            Ok(total)
        }
        Ok(walk(&self.root)?)
    }

    fn config_path(&self) -> PathBuf { self.root.join("config.toml") }
    fn state_path(&self) -> PathBuf { self.root.join("state.snapshot") }
    fn genesis_state_path(&self) -> PathBuf { self.root.join("genesis.snapshot") }
    fn wallets_dir(&self) -> PathBuf { self.root.join("wallets") }
    fn wallet_path(&self, name: &str) -> PathBuf { self.wallets_dir().join(format!("{name}.wallet")) }
    fn blocks_dir(&self) -> PathBuf { self.root.join("blocks") }
    fn mempool_dir(&self) -> PathBuf { self.root.join("mempool") }
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
