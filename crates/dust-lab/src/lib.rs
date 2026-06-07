pub mod fork;
pub mod invalid_block;
pub mod invalid_tx;
pub mod oversized_block;
pub mod replay;
pub mod report;
pub mod spam;

pub use report::LabReport;

pub fn run_named(name: &str, txs: u64) -> LabReport {
    match name {
        "spam" => spam::run(txs),
        "replay" => replay::run(),
        "invalid-tx" => invalid_tx::run(),
        "invalid-block" => invalid_block::run(),
        "oversized-block" => oversized_block::run(),
        "fork" => fork::run(),
        _ => LabReport::new(name).with_note("unknown scenario"),
    }
}
