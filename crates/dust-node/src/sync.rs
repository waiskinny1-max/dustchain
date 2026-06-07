use dust_core::Hash;

use crate::{LocalClient, PeerStatus, Result};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SyncPlan {
    pub peer: String,
    pub local_height: u64,
    pub remote_height: u64,
    pub remote_tip: Hash,
    pub blocks_to_fetch: Vec<u64>,
}

impl SyncPlan {
    pub fn is_caught_up(&self) -> bool {
        self.blocks_to_fetch.is_empty()
    }

    pub fn render(&self) -> String {
        let range = if self.blocks_to_fetch.is_empty() {
            "none".to_string()
        } else {
            format!("{}..={}", self.blocks_to_fetch[0], self.blocks_to_fetch[self.blocks_to_fetch.len() - 1])
        };
        format!(
            "peer: {}\nlocal_height: {}\nremote_height: {}\nremote_tip: {}\nblocks_to_fetch: {}\n",
            self.peer, self.local_height, self.remote_height, self.remote_tip, range
        )
    }
}

pub fn plan_from_status(peer: impl Into<String>, local_height: u64, remote: PeerStatus) -> SyncPlan {
    let blocks_to_fetch = if remote.height > local_height {
        ((local_height + 1)..=remote.height).collect()
    } else {
        Vec::new()
    };
    SyncPlan { peer: peer.into(), local_height, remote_height: remote.height, remote_tip: remote.tip_hash, blocks_to_fetch }
}

pub async fn plan_from_peer(client: &LocalClient, peer: &str, local_height: u64) -> Result<SyncPlan> {
    let remote = client.status(peer).await?;
    Ok(plan_from_status(peer.to_string(), local_height, remote))
}
