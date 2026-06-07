use dust_store::DustStore;
use dust_wire::signed_tx_file_bytes;

use crate::{LocalClient, Result};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GossipReport {
    pub peer: String,
    pub attempted: usize,
    pub accepted: usize,
    pub failed: usize,
}

impl GossipReport {
    pub fn render(&self) -> String {
        format!(
            "peer: {}\nattempted: {}\naccepted: {}\nfailed: {}\n",
            self.peer, self.attempted, self.accepted, self.failed
        )
    }
}

pub async fn gossip_mempool_once(store: &DustStore, client: &LocalClient, peer: &str) -> Result<GossipReport> {
    let pending = store.pending_txs()?;
    let mut report = GossipReport { peer: peer.to_string(), attempted: pending.len(), accepted: 0, failed: 0 };
    for (_, tx) in pending {
        match client.submit_tx_file(peer, &signed_tx_file_bytes(&tx)).await {
            Ok(_) => report.accepted += 1,
            Err(_) => report.failed += 1,
        }
    }
    Ok(report)
}
