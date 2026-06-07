use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let data_dir = std::env::args()
        .skip(1)
        .find_map(|arg| arg.strip_prefix("--data-dir=").map(PathBuf::from));
    dust_gui::run_gui(data_dir)
}
