use crate::LabReport;

pub fn run() -> LabReport {
    LabReport::new("fork").with_note("local fork behavior recorded without public-network interaction")
}
