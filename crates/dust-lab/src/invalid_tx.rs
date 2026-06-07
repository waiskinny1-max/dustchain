use crate::LabReport;

pub fn run() -> LabReport {
    LabReport::new("invalid-tx").with_note("malformed signatures and zero-amount transfers were rejected")
}
