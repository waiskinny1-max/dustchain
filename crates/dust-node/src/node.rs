use crate::{NodeConfig, Result};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeStatus {
    pub listening_on: String,
    pub height: u64,
    pub mempool_txs: usize,
    pub p2p_enabled: bool,
}

pub struct Node {
    config: NodeConfig,
}

impl Node {
    pub fn new(config: NodeConfig) -> Self {
        Self { config }
    }

    pub async fn start_local_stub(&self) -> Result<NodeStatus> {
        let store = dust_store::DustStore::open(&self.config.data_dir);
        let chain = store.load_chain()?;
        let mempool_txs = store.pending_txs()?.len();
        Ok(NodeStatus {
            listening_on: format!("{}:{}", self.config.host, self.config.port),
            height: chain.height(),
            mempool_txs,
            p2p_enabled: false,
        })
    }
}
