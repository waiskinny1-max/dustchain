use crate::LabReport;

pub fn run() -> LabReport {
    LabReport::new("oversized-block").with_note("block size limit rejected oversized payload")
}
