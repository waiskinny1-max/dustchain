use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct NodeConfig {
    pub host: String,
    pub port: u16,
    pub data_dir: PathBuf,
    pub max_frame_bytes: usize,
    pub invalid_message_limit: usize,
    pub connect_timeout_ms: u64,
    pub allow_non_loopback: bool,
}

impl NodeConfig {
    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3030,
            data_dir: ".dustchain".into(),
            max_frame_bytes: 4 * 1024 * 1024,
            invalid_message_limit: 8,
            connect_timeout_ms: 1_500,
            allow_non_loopback: false,
        }
    }
}
