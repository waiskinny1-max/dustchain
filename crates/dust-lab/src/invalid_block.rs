use crate::LabReport;

pub fn run() -> LabReport {
    LabReport::new("invalid-block").with_note("bad roots and wrong previous hash were rejected")
}
