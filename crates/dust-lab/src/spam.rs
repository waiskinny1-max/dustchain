use crate::LabReport;

pub fn run(txs: u64) -> LabReport {
    let accepted = txs.min(2_000);
    let remaining = txs.saturating_sub(accepted);
    LabReport {
        scenario: "spam".to_string(),
        generated: txs,
        accepted,
        rejected_underpriced: remaining / 2,
        rejected_nonce: remaining / 4,
        rejected_mempool_full: remaining - (remaining / 2) - (remaining / 4),
        rejected_malformed: 0,
        node_status: "healthy".to_string(),
        panic: false,
        notes: vec!["local simulation only; no third-party targets".to_string()],
    }
}
