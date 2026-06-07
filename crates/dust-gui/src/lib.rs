//! Native desktop wallet and node console for dustchain.
//!
//! The GUI is intentionally conservative: no exchange panels, no market data,
//! no investment language. It is a local wallet, mempool, chain, inspector, and
//! lab console for the experimental protocol implementation.

pub mod actions;
pub mod app;
pub mod components;

use std::path::PathBuf;

pub use app::DustGuiApp;

/// Launch the native GUI.
///
/// `data_dir` defaults to `.dustchain` when omitted. The GUI uses the same
/// storage layout as the CLI, so users can move between `dust` and `dust-gui`
/// without migrations.
pub fn run_gui(data_dir: Option<PathBuf>) -> anyhow::Result<()> {
    let data_dir = data_dir.unwrap_or_else(|| PathBuf::from(".dustchain"));
    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("dustchain")
            .with_inner_size([1180.0, 760.0])
            .with_min_inner_size([980.0, 620.0]),
        ..Default::default()
    };

    eframe::run_native(
        "dustchain",
        native_options,
        Box::new(move |cc| Ok(Box::new(DustGuiApp::new(cc, data_dir.clone())))),
    )
    .map_err(|err| anyhow::anyhow!(err.to_string()))
}
