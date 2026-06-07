mod commands;
mod output;

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use dust_core::{Address, Hash, SignedTransaction, Transaction};
use dust_crypto::{sign, verify_signed_transaction};
use dust_store::DustStore;
use dust_wire::{inspect_block_file, inspect_tx_file, read_file, signed_tx_payload, transaction_signing_payload};

#[derive(Parser, Debug)]
#[command(name = "dust")]
#[command(about = "Terminal-first experimental low-fee blockchain node")]
struct Cli {
    #[arg(long, global = true, default_value = ".dustchain")]
    data_dir: PathBuf,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Init,
    Wallet { #[command(subcommand)] command: WalletCommand },
    Faucet { wallet: String, amount: u64 },
    Tx { #[command(subcommand)] command: TxCommand },
    Mine,
    Chain { #[command(subcommand)] command: ChainCommand },
    Balance { address_or_wallet: String },
    Fee { #[command(subcommand)] command: FeeCommand },
    Mempool { #[command(subcommand)] command: MempoolCommand },
    Inspect { #[command(subcommand)] command: InspectCommand },
    Bench { #[arg(long)] markdown: bool },
    Node { #[command(subcommand)] command: NodeCommand },
    Peer { #[command(subcommand)] command: PeerCommand },
    Tui,
    Gui,
    Lab { #[command(subcommand)] command: LabCommand },
}

#[derive(Subcommand, Debug)]
enum WalletCommand {
    New { name: String },
    List,
    Show { name: String },
}

#[derive(Subcommand, Debug)]
enum TxCommand {
    Send {
        from: String,
        to: String,
        amount: u64,
        #[arg(long, default_value_t = 0)]
        priority_fee: u64,
        #[arg(long, default_value = "")]
        memo: String,
    },
    Inspect { tx_file: PathBuf },
}

#[derive(Subcommand, Debug)]
enum ChainCommand {
    Height,
    Verify,
    Inspect { height: u64 },
    DbStats { #[arg(long)] verbose: bool },
    Reindex,
    ExportState { #[arg(long)] output: Option<PathBuf> },
}

#[derive(Subcommand, Debug)]
enum FeeCommand {
    Estimate {
        #[arg(long, default_value_t = 100)]
        amount: u64,
        #[arg(long, default_value = "")]
        memo: String,
        #[arg(long, default_value_t = 0)]
        priority_fee: u64,
    },
}

#[derive(Subcommand, Debug)]
enum MempoolCommand {
    List,
    Clear,
}

#[derive(Subcommand, Debug)]
enum InspectCommand {
    Tx { path: PathBuf },
    Block { path: PathBuf },
}

#[derive(Subcommand, Debug)]
enum NodeCommand {
    Start {
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        #[arg(long, default_value_t = 3030)]
        port: u16,
        #[arg(long = "peer")]
        peers: Vec<String>,
        #[arg(long)]
        allow_non_loopback: bool,
    },
    Status {
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        #[arg(long, default_value_t = 3030)]
        port: u16,
    },
}

#[derive(Subcommand, Debug)]
enum PeerCommand {
    Add { address: String },
    List,
    Probe { address: Option<String> },
    FetchBlock { peer: String, height: u64, #[arg(long)] output: Option<PathBuf> },
    GossipMempool { peer: String },
}

#[derive(Subcommand, Debug)]
enum LabCommand {
    Spam { #[arg(long, default_value_t = 50_000)] txs: u64 },
    Replay,
    InvalidTx,
    InvalidBlock,
    OversizedBlock,
    Fork,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let store = DustStore::open(&cli.data_dir);

    match cli.command {
        Command::Init => {
            store.init()?;
            println!("initialized dustchain store");
            println!("data_dir: {}", store.root().display());
            println!("metadata: metadata/manifest.txt");
        }
        Command::Wallet { command } => wallet_command(&store, command)?,
        Command::Faucet { wallet, amount } => faucet(&store, &wallet, amount)?,
        Command::Tx { command } => tx_command(&store, command)?,
        Command::Mine => mine(&store)?,
        Command::Chain { command } => chain_command(&store, command)?,
        Command::Balance { address_or_wallet } => balance(&store, &address_or_wallet)?,
        Command::Fee { command } => fee_command(&store, command)?,
        Command::Mempool { command } => mempool_command(&store, command)?,
        Command::Inspect { command } => inspect_command(command)?,
        Command::Bench { markdown } => bench(&store, markdown)?,
        Command::Node { command } => node_command(&store, command).await?,
        Command::Peer { command } => peer_command(&store, command).await?,
        Command::Tui => print!("{}", dust_tui::run_once(store.root())?),
        Command::Gui => dust_gui::run_gui(Some(store.root().to_path_buf()))?,
        Command::Lab { command } => lab_command(command),
    }

    Ok(())
}

fn wallet_command(store: &DustStore, command: WalletCommand) -> Result<()> {
    match command {
        WalletCommand::New { name } => {
            let wallet = store.create_wallet(&name)?;
            println!("wallet created");
            println!("name: {}", wallet.name);
            println!("address: {}", wallet.address);
            println!("public_key: {}", wallet.key.public_hex());
        }
        WalletCommand::List => {
            for wallet in store.wallets()? {
                println!("{}  {}", wallet.name, wallet.address);
            }
        }
        WalletCommand::Show { name } => {
            let wallet = store.wallet(&name)?;
            println!("name: {}", wallet.name);
            println!("address: {}", wallet.address);
            println!("public_key: {}", wallet.key.public_hex());
        }
    }
    Ok(())
}

fn faucet(store: &DustStore, wallet: &str, amount: u64) -> Result<()> {
    let address = store.resolve_address(wallet)?;
    let mut state = store.load_state()?;
    state.credit(address, amount);
    store.save_state(&state)?;
    let mut genesis = store.load_genesis_state()?;
    genesis.credit(address, amount);
    store.save_genesis_state(&genesis)?;
    println!("faucet credited");
    println!("address: {}", address);
    println!("amount: {} dust", amount);
    Ok(())
}

fn tx_command(store: &DustStore, command: TxCommand) -> Result<()> {
    match command {
        TxCommand::Send { from, to, amount, priority_fee, memo } => send_tx(store, &from, &to, amount, priority_fee, memo.as_bytes())?,
        TxCommand::Inspect { tx_file } => inspect_tx_path(&tx_file)?,
    }
    Ok(())
}

fn send_tx(store: &DustStore, from: &str, to: &str, amount: u64, priority_fee: u64, memo: &[u8]) -> Result<()> {
    let cfg = store.config()?;
    let wallet = store.wallet(from)?;
    let to = store.resolve_address(to)?;
    let state = store.load_state()?;
    let nonce = state.get(&wallet.address).nonce;

    let mut tx = Transaction::transfer(&cfg.chain_id, wallet.address, to, amount, nonce, 0, priority_fee, memo.to_vec());
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
    println!("transaction accepted");
    println!("tx_hash: {}", signed.hash);
    println!("encoded_size: {} bytes", fee.encoded_size);
    println!("required_fee: {} dust", fee.required_fee);
    println!("priority_fee: {} dust", fee.priority_fee);
    println!("total_fee: {} dust", fee.paid_fee);
    println!("file: {}", path.display());
    Ok(())
}

fn mine(store: &DustStore) -> Result<()> {
    let mut chain = store.load_chain()?;
    let pending = store.pending_txs()?;
    let txs: Vec<_> = pending.into_iter().map(|(_, tx)| tx).collect();
    let producer = Address::ZERO;
    let verifier = |s: &SignedTransaction| verify_signed_transaction(s, |inner| transaction_signing_payload(&inner.tx));
    let block = chain.mine(producer, txs, verifier)?;
    let hashes: Vec<_> = block.transactions.iter().map(|tx| tx.hash).collect();
    let path = store.append_block(&block)?;
    store.save_state(&chain.state)?;
    store.clear_pending(&hashes)?;
    println!("block mined");
    println!("height: {}", block.header.height);
    println!("hash: {}", block.header_hash());
    println!("state_root: {}", block.header.state_root);
    println!("txs: {}", block.transactions.len());
    println!("file: {}", path.display());
    println!("index: metadata/block-index.tsv");
    Ok(())
}

fn chain_command(store: &DustStore, command: ChainCommand) -> Result<()> {
    match command {
        ChainCommand::Height => {
            let chain = store.load_chain()?;
            println!("height: {}", chain.height());
        }
        ChainCommand::Verify => {
            let chain = store.load_chain()?;
            let verifier = |s: &SignedTransaction| verify_signed_transaction(s, |inner| transaction_signing_payload(&inner.tx));
            chain.verify(verifier)?;
            println!("chain status: valid");
        }
        ChainCommand::Inspect { height } => {
            let chain = store.load_chain()?;
            let block = chain.blocks.iter().find(|b| b.header.height == height).context("block not found")?;
            println!("height: {}", block.header.height);
            println!("hash: {}", block.header_hash());
            println!("previous: {}", block.header.previous_block_hash);
            println!("tx_root: {}", block.header.tx_root);
            println!("state_root: {}", block.header.state_root);
            println!("txs: {}", block.transactions.len());
        }
        ChainCommand::DbStats { verbose } => {
            println!("data_dir: {}", store.root().display());
            print!("{}", store.db_stats()?.render(verbose));
        }
        ChainCommand::Reindex => {
            let entries = store.rebuild_index()?;
            println!("block index rebuilt");
            println!("indexed_blocks: {}", entries.len());
        }
        ChainCommand::ExportState { output } => {
            let text = store.export_state_text()?;
            if let Some(path) = output {
                std::fs::write(&path, text)?;
                println!("state exported");
                println!("file: {}", path.display());
            } else {
                print!("{}", text);
            }
        }
    }
    Ok(())
}

fn balance(store: &DustStore, address_or_wallet: &str) -> Result<()> {
    let address = store.resolve_address(address_or_wallet)?;
    let state = store.load_state()?;
    let account = state.get(&address);
    println!("address: {}", address);
    println!("balance: {} dust", account.balance);
    println!("nonce: {}", account.nonce);
    Ok(())
}

fn fee_command(store: &DustStore, command: FeeCommand) -> Result<()> {
    match command {
        FeeCommand::Estimate { amount: _, memo, priority_fee } => {
            let cfg = store.config()?;
            let estimated_size = 180 + memo.len();
            let fee = cfg.policy.breakdown(estimated_size, priority_fee);
            println!("estimated_size: {} bytes", fee.encoded_size);
            println!("required_fee: {} dust", fee.required_fee);
            println!("priority_fee: {} dust", fee.priority_fee);
            println!("total_fee: {} dust", fee.paid_fee);
        }
    }
    Ok(())
}

fn mempool_command(store: &DustStore, command: MempoolCommand) -> Result<()> {
    match command {
        MempoolCommand::List => {
            let cfg = store.config()?;
            let pending = store.pending_txs()?;
            for (path, tx) in pending {
                let fee = tx.fee_breakdown(&cfg.policy);
                println!("{}  from={}  to={}  amount={}  fee={}  size={}  file={}", tx.hash.short(), tx.tx.from.short(), tx.tx.to.short(), tx.tx.amount, fee.paid_fee, fee.encoded_size, path.display());
            }
        }
        MempoolCommand::Clear => {
            store.clear_all_pending()?;
            println!("mempool cleared");
        }
    }
    Ok(())
}

fn inspect_command(command: InspectCommand) -> Result<()> {
    match command {
        InspectCommand::Tx { path } => inspect_tx_path(&path)?,
        InspectCommand::Block { path } => inspect_block_path(&path)?,
    }
    Ok(())
}

fn inspect_tx_path(path: &PathBuf) -> Result<()> {
    let bytes = read_file(path)?;
    let info = inspect_tx_file(&bytes)?;
    println!("Transaction File: {}", path.display());
    println!("Magic: {}", info.magic);
    println!("Version: {}", info.version);
    println!("Hash: {}", info.tx_hash);
    println!("From: {}", info.from);
    println!("To: {}", info.to);
    println!("Amount: {}", info.amount);
    println!("Nonce: {}", info.nonce);
    println!("Payload size: {} bytes", info.payload_len);
    println!("Memo bytes: {}", info.memo_bytes);
    println!("Checksum: valid");
    Ok(())
}

fn inspect_block_path(path: &PathBuf) -> Result<()> {
    let bytes = read_file(path)?;
    let info = inspect_block_file(&bytes)?;
    println!("Block File: {}", path.display());
    println!("Magic: {}", info.magic);
    println!("Version: {}", info.version);
    println!("Height: {}", info.height);
    println!("Previous: {}", info.previous);
    println!("Transactions: {}", info.tx_count);
    println!("Payload size: {} bytes", info.payload_len);
    println!("Merkle root: {}", info.tx_root);
    println!("State root: {}", info.state_root);
    println!("Producer: {}", info.producer);
    println!("Checksum: valid");
    Ok(())
}

fn bench(store: &DustStore, markdown: bool) -> Result<()> {
    let cfg = store.config()?;
    let start = std::time::Instant::now();
    let samples = 10_000usize;
    let mut total_size = 0usize;
    for i in 0..samples {
        let tx = Transaction::transfer(&cfg.chain_id, Address::ZERO, Address::ZERO, 1, i as u64, 1, 0, Vec::<u8>::new());
        total_size += transaction_signing_payload(&tx).len() + 96;
    }
    let elapsed = start.elapsed();
    let avg_size = total_size / samples;
    let min_fee = cfg.policy.required_fee(avg_size);
    let txs_per_mb = cfg.policy.max_block_size_bytes / avg_size.max(1);

    if markdown {
        println!("| Metric | Result |");
        println!("|---|---:|");
        println!("| Samples | {} |", samples);
        println!("| Average tx size | {} bytes |", avg_size);
        println!("| Minimum transfer fee | {} dust |", min_fee);
        println!("| Txs per 1MB block | {} |", txs_per_mb);
        println!("| Encoding loop time | {:.2?} |", elapsed);
    } else {
        println!("dustchain benchmark report");
        println!("transactions generated:       {}", samples);
        println!("average tx size:              {} bytes", avg_size);
        println!("minimum fee:                  {} dust", min_fee);
        println!("txs per 1MB block:            {}", txs_per_mb);
        println!("encoding loop time:           {:.2?}", elapsed);
    }
    Ok(())
}

async fn node_command(store: &DustStore, command: NodeCommand) -> Result<()> {
    match command {
        NodeCommand::Start { host, port, peers, allow_non_loopback } => {
            let mut configured_peers = read_peers(store)?;
            configured_peers.extend(peers);
            configured_peers.sort();
            configured_peers.dedup();

            let mut config = dust_node::NodeConfig { host, port, data_dir: store.root().to_path_buf(), ..dust_node::NodeConfig::default() };
            config.allow_non_loopback = allow_non_loopback;
            let node = dust_node::Node::new(config);
            let status = node.local_status(configured_peers.len()).await?;
            println!("node starting in local P2P mode");
            println!("listening_on: {}", status.listening_on);
            println!("height: {}", status.height);
            println!("tip_hash: {}", status.tip_hash);
            println!("mempool_txs: {}", status.mempool_txs);
            println!("configured_peers: {}", status.configured_peers);
            println!("local_only_default: true");
            node.start_localnet(configured_peers).await?;
        }
        NodeCommand::Status { host, port } => {
            let config = dust_node::NodeConfig { host, port, data_dir: store.root().to_path_buf(), ..dust_node::NodeConfig::default() };
            let node = dust_node::Node::new(config);
            let status = node.local_status(read_peers(store)?.len()).await?;
            println!("listening_on: {}", status.listening_on);
            println!("height: {}", status.height);
            println!("tip_hash: {}", status.tip_hash);
            println!("mempool_txs: {}", status.mempool_txs);
            println!("p2p_enabled: {}", status.p2p_enabled);
            println!("configured_peers: {}", status.configured_peers);
        }
    }
    Ok(())
}

async fn peer_command(store: &DustStore, command: PeerCommand) -> Result<()> {
    match command {
        PeerCommand::Add { address } => {
            let mut peers = read_peers(store)?;
            if !peers.contains(&address) {
                peers.push(address.clone());
                peers.sort();
                write_peers(store, &peers)?;
            }
            println!("peer saved");
            println!("address: {}", address);
            println!("file: {}", peers_path(store).display());
        }
        PeerCommand::List => {
            let peers = read_peers(store)?;
            if peers.is_empty() {
                println!("no peers configured");
            } else {
                for peer in peers {
                    println!("{}", peer);
                }
            }
        }
        PeerCommand::Probe { address } => {
            let peers = if let Some(address) = address { vec![address] } else { read_peers(store)? };
            let client = default_client();
            if peers.is_empty() {
                println!("no peers to probe");
            }
            for peer in peers {
                match client.status(&peer).await {
                    Ok(status) => {
                        println!("peer: {}", peer);
                        println!("chain_id: {}", status.chain_id);
                        println!("height: {}", status.height);
                        println!("tip_hash: {}", status.tip_hash);
                        println!("mempool_txs: {}", status.mempool_txs);
                    }
                    Err(err) => {
                        println!("peer: {}", peer);
                        println!("status: unreachable");
                        println!("error: {}", err);
                    }
                }
            }
        }
        PeerCommand::FetchBlock { peer, height, output } => {
            let client = default_client();
            match client.fetch_block(&peer, height).await? {
                Some(bytes) => {
                    let path = output.unwrap_or_else(|| store.root().join("synced").join(format!("peer-{height:08}.dblk")));
                    if let Some(parent) = path.parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                    std::fs::write(&path, bytes)?;
                    println!("block fetched");
                    println!("peer: {}", peer);
                    println!("height: {}", height);
                    println!("file: {}", path.display());
                    println!("note: fetched block was not appended to the chain; inspect and verify before importing");
                }
                None => {
                    println!("block not found on peer");
                    println!("peer: {}", peer);
                    println!("height: {}", height);
                }
            }
        }
        PeerCommand::GossipMempool { peer } => {
            let client = default_client();
            let report = dust_node::gossip_mempool_once(store, &client, &peer).await?;
            print!("{}", report.render());
        }
    }
    Ok(())
}

fn peers_path(store: &DustStore) -> PathBuf {
    store.root().join("peers.txt")
}

fn read_peers(store: &DustStore) -> Result<Vec<String>> {
    let path = peers_path(store);
    if !path.exists() {
        return Ok(Vec::new());
    }
    let text = std::fs::read_to_string(path)?;
    let mut peers = text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    peers.sort();
    peers.dedup();
    Ok(peers)
}

fn write_peers(store: &DustStore, peers: &[String]) -> Result<()> {
    std::fs::create_dir_all(store.root())?;
    let mut text = String::from("# dustchain local peers; use loopback addresses by default\n");
    for peer in peers {
        text.push_str(peer);
        text.push('\n');
    }
    std::fs::write(peers_path(store), text)?;
    Ok(())
}

fn default_client() -> dust_node::LocalClient {
    let cfg = dust_node::NodeConfig::default();
    dust_node::LocalClient::new(cfg.max_frame_bytes, cfg.connect_timeout_ms)
}

fn lab_command(command: LabCommand) {
    let report = match command {
        LabCommand::Spam { txs } => dust_lab::run_named("spam", txs),
        LabCommand::Replay => dust_lab::run_named("replay", 0),
        LabCommand::InvalidTx => dust_lab::run_named("invalid-tx", 0),
        LabCommand::InvalidBlock => dust_lab::run_named("invalid-block", 0),
        LabCommand::OversizedBlock => dust_lab::run_named("oversized-block", 0),
        LabCommand::Fork => dust_lab::run_named("fork", 0),
    };
    print!("{}", report.render());
}
