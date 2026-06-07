use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct NodeConfig {
    pub host: String,
    pub port: u16,
    pub data_dir: PathBuf,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self { host: "127.0.0.1".to_string(), port: 3030, data_dir: ".dustchain".into() }
    }
}
