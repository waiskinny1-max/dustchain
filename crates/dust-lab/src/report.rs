#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LabReport {
    pub scenario: String,
    pub generated: u64,
    pub accepted: u64,
    pub rejected_underpriced: u64,
    pub rejected_nonce: u64,
    pub rejected_mempool_full: u64,
    pub rejected_malformed: u64,
    pub node_status: String,
    pub panic: bool,
    pub notes: Vec<String>,
}

impl LabReport {
    pub fn new(scenario: impl Into<String>) -> Self {
        Self {
            scenario: scenario.into(),
            generated: 0,
            accepted: 0,
            rejected_underpriced: 0,
            rejected_nonce: 0,
            rejected_mempool_full: 0,
            rejected_malformed: 0,
            node_status: "healthy".to_string(),
            panic: false,
            notes: Vec::new(),
        }
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    pub fn render(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("scenario: {}\n", self.scenario));
        out.push_str(&format!("generated: {}\n", self.generated));
        out.push_str(&format!("accepted: {}\n", self.accepted));
        out.push_str(&format!("rejected_underpriced: {}\n", self.rejected_underpriced));
        out.push_str(&format!("rejected_nonce: {}\n", self.rejected_nonce));
        out.push_str(&format!("rejected_mempool_full: {}\n", self.rejected_mempool_full));
        out.push_str(&format!("rejected_malformed: {}\n", self.rejected_malformed));
        out.push_str(&format!("node_status: {}\n", self.node_status));
        out.push_str(&format!("panic: {}\n", self.panic));
        for note in &self.notes {
            out.push_str(&format!("note: {}\n", note));
        }
        out
    }
}
