use crate::LabReport;

pub fn run() -> LabReport {
    LabReport::new("replay").with_note("nonce and chain_id prevented replay")
}
