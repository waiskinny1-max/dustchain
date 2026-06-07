use std::path::PathBuf;

use dust_core::{Address, Block, SignedTransaction, State};
use dust_lab::LabReport;
use dust_store::{DustStore, StoreStats, WalletRecord};
use eframe::egui::{self, Align, Layout, RichText, ScrollArea, TextEdit};

use crate::{actions, components::*};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Screen {
    Overview,
    Wallet,
    Send,
    Chain,
    Mempool,
    Fees,
    Inspector,
    Lab,
    Node,
}

impl Screen {
    const ALL: [Screen; 9] = [
        Screen::Overview,
        Screen::Wallet,
        Screen::Send,
        Screen::Chain,
        Screen::Mempool,
        Screen::Fees,
        Screen::Inspector,
        Screen::Lab,
        Screen::Node,
    ];

    fn label(self) -> &'static str {
        match self {
            Screen::Overview => "Overview",
            Screen::Wallet => "Wallet",
            Screen::Send => "Send",
            Screen::Chain => "Chain",
            Screen::Mempool => "Mempool",
            Screen::Fees => "Fees",
            Screen::Inspector => "Inspector",
            Screen::Lab => "Lab",
            Screen::Node => "Node",
        }
    }
}

pub struct DustGuiApp {
    store: DustStore,
    data_dir: PathBuf,
    screen: Screen,
    status: String,
    stats: Option<StoreStats>,
    wallets: Vec<WalletRecord>,
    pending: Vec<(PathBuf, SignedTransaction)>,
    blocks: Vec<(PathBuf, Block)>,
    state: State,
    new_wallet_name: String,
    faucet_target: String,
    faucet_amount: String,
    send_from: String,
    send_to: String,
    send_amount: String,
    send_priority_fee: String,
    send_memo: String,
    fee_amount: String,
    fee_memo: String,
    inspect_path: String,
    inspect_output: String,
    lab_scenario: String,
    lab_txs: String,
    lab_report: Option<LabReport>,
}

impl DustGuiApp {
    pub fn new(cc: &eframe::CreationContext<'_>, data_dir: PathBuf) -> Self {
        install_style(&cc.egui_ctx);
        let store = DustStore::open(&data_dir);
        let mut app = Self {
            store,
            data_dir,
            screen: Screen::Overview,
            status: "ready".to_string(),
            stats: None,
            wallets: Vec::new(),
            pending: Vec::new(),
            blocks: Vec::new(),
            state: State::new(),
            new_wallet_name: "alice".to_string(),
            faucet_target: "alice".to_string(),
            faucet_amount: "1000".to_string(),
            send_from: "alice".to_string(),
            send_to: "bob".to_string(),
            send_amount: "100".to_string(),
            send_priority_fee: "0".to_string(),
            send_memo: String::new(),
            fee_amount: "100".to_string(),
            fee_memo: String::new(),
            inspect_path: String::new(),
            inspect_output: String::new(),
            lab_scenario: "spam".to_string(),
            lab_txs: "50000".to_string(),
            lab_report: None,
        };
        app.refresh();
        app
    }

    fn refresh(&mut self) {
        self.stats = self.store.db_stats().ok();
        self.wallets = self.store.wallets().unwrap_or_default();
        self.pending = self.store.pending_txs().unwrap_or_default();
        self.blocks = self.store.load_blocks_ordered().unwrap_or_default();
        self.state = self.store.load_state().unwrap_or_default();
    }

    fn set_status(&mut self, result: anyhow::Result<String>) {
        self.status = match result {
            Ok(value) => value,
            Err(err) => format!("error: {err}"),
        };
        self.refresh();
    }

    fn parse_amount(value: &str) -> anyhow::Result<u64> {
        value.trim().parse::<u64>().map_err(|_| anyhow::anyhow!("expected a positive integer amount"))
    }

    fn account_balance(&self, address: Address) -> String {
        let account = self.state.get(&address);
        format!("{} dust · nonce {}", account.balance, account.nonce)
    }
}

impl eframe::App for DustGuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top")
            .frame(egui::Frame::default().fill(egui::Color32::from_rgb(9, 10, 12)).inner_margin(egui::Margin::symmetric(18.0, 12.0)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("dustchain").size(22.0).strong().color(ACCENT));
                    ui.label(RichText::new("local wallet + protocol console").size(12.0).color(TEXT_MUTED));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui.button("Refresh").clicked() {
                            self.refresh();
                            self.status = actions::short_status(&self.store);
                        }
                        ui.label(RichText::new(self.data_dir.display().to_string()).monospace().size(12.0).color(TEXT_MUTED));
                    });
                });
            });

        egui::SidePanel::left("nav")
            .resizable(false)
            .exact_width(178.0)
            .frame(egui::Frame::default().fill(egui::Color32::from_rgb(11, 12, 14)).inner_margin(egui::Margin::symmetric(12.0, 16.0)))
            .show(ctx, |ui| {
                ui.label(RichText::new("LOCAL NODE").size(11.0).strong().color(TEXT_MUTED));
                ui.add_space(10.0);
                for screen in Screen::ALL {
                    let selected = self.screen == screen;
                    let response = ui.selectable_label(selected, RichText::new(screen.label()).size(14.0));
                    if response.clicked() {
                        self.screen = screen;
                    }
                }
                ui.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
                    status_line(ui, &self.status);
                });
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(egui::Color32::from_rgb(9, 10, 12)).inner_margin(egui::Margin::same(18.0)))
            .show(ctx, |ui| {
                ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| match self.screen {
                    Screen::Overview => self.overview(ui),
                    Screen::Wallet => self.wallet_screen(ui),
                    Screen::Send => self.send_screen(ui),
                    Screen::Chain => self.chain_screen(ui),
                    Screen::Mempool => self.mempool_screen(ui),
                    Screen::Fees => self.fees_screen(ui),
                    Screen::Inspector => self.inspector_screen(ui),
                    Screen::Lab => self.lab_screen(ui),
                    Screen::Node => self.node_screen(ui),
                });
            });
    }
}

impl DustGuiApp {
    fn overview(&mut self, ui: &mut egui::Ui) {
        let stats = self.stats.clone();
        ui.horizontal(|ui| {
            metric(ui, "height", stats.as_ref().map(|s| s.height).unwrap_or(0));
            metric(ui, "mempool", self.pending.len());
            metric(ui, "wallets", self.wallets.len());
            metric(ui, "db bytes", stats.as_ref().map(|s| s.total_bytes).unwrap_or(0));
        });
        ui.add_space(12.0);
        section(ui, "Protocol surface", "Compact payments, binary files, predictable fees, local-first operation.", |ui| {
            ui.columns(2, |columns| {
                columns[0].label(RichText::new("Latest block").strong());
                if let Some((path, block)) = self.blocks.last() {
                    mono_label(&mut columns[0], format!("#{} · {} txs", block.header.height, block.transactions.len()));
                    mono_label(&mut columns[0], block.header_hash());
                    mono_label(&mut columns[0], path.display());
                } else {
                    mono_label(&mut columns[0], "no mined blocks yet");
                }
                columns[1].label(RichText::new("Store").strong());
                if let Some(stats) = &self.stats {
                    mono_label(&mut columns[1], format!("tip: {}", stats.tip_hash.short()));
                    mono_label(&mut columns[1], format!("state: {}", stats.state_root.short()));
                    mono_label(&mut columns[1], format!("blocks: {}", stats.block_files));
                } else {
                    mono_label(&mut columns[1], "not initialized");
                }
            });
        });
        ui.add_space(12.0);
        section(ui, "Fast actions", "Same storage as the CLI.", |ui| {
            ui.horizontal(|ui| {
                if ui.button("Init store").clicked() {
                    let result = actions::init_store(&self.store);
                    self.set_status(result);
                }
                if ui.button("Mine pending block").clicked() {
                    let result = actions::mine_block(&self.store);
                    self.set_status(result);
                }
                if ui.button("Clear mempool").clicked() {
                    let result = actions::clear_mempool(&self.store);
                    self.set_status(result);
                }
            });
        });
    }

    fn wallet_screen(&mut self, ui: &mut egui::Ui) {
        section(ui, "Wallets", "Local development keys. Do not use for real funds.", |ui| {
            egui::Grid::new("wallet-table").striped(true).num_columns(4).show(ui, |ui| {
                ui.label(RichText::new("name").strong());
                ui.label(RichText::new("address").strong());
                ui.label(RichText::new("balance").strong());
                ui.label(RichText::new("public key").strong());
                ui.end_row();
                for wallet in &self.wallets {
                    mono_label(ui, &wallet.name);
                    mono_label(ui, wallet.address);
                    mono_label(ui, self.account_balance(wallet.address));
                    mono_label(ui, wallet.key.public_hex());
                    ui.end_row();
                }
            });
        });
        ui.add_space(12.0);
        section(ui, "Create wallet", "Names are local aliases stored under the data directory.", |ui| {
            ui.horizontal(|ui| {
                ui.add(TextEdit::singleline(&mut self.new_wallet_name).hint_text("wallet name"));
                if ui.button("Create").clicked() {
                    let name = self.new_wallet_name.clone();
                    let result = actions::create_wallet(&self.store, &name);
                    self.set_status(result);
                }
            });
        });
        ui.add_space(12.0);
        section(ui, "Development faucet", "Credits local state only. This is not a public faucet.", |ui| {
            ui.horizontal(|ui| {
                ui.add(TextEdit::singleline(&mut self.faucet_target).hint_text("wallet or address"));
                ui.add(TextEdit::singleline(&mut self.faucet_amount).hint_text("amount"));
                if ui.button("Credit").clicked() {
                    let result = Self::parse_amount(&self.faucet_amount)
                        .and_then(|amount| actions::faucet(&self.store, &self.faucet_target, amount));
                    self.set_status(result);
                }
            });
        });
    }

    fn send_screen(&mut self, ui: &mut egui::Ui) {
        section(ui, "Send transfer", "Signs a compact local transaction and writes a .dtx file into mempool.", |ui| {
            ui.columns(2, |columns| {
                columns[0].label("from wallet");
                columns[0].add(TextEdit::singleline(&mut self.send_from));
                columns[0].label("to wallet/address");
                columns[0].add(TextEdit::singleline(&mut self.send_to));
                columns[0].label("amount");
                columns[0].add(TextEdit::singleline(&mut self.send_amount));
                columns[1].label("priority fee");
                columns[1].add(TextEdit::singleline(&mut self.send_priority_fee));
                columns[1].label("memo");
                columns[1].add(TextEdit::multiline(&mut self.send_memo).desired_rows(5));
            });
            ui.add_space(10.0);
            if ui.button("Sign and queue").clicked() {
                let result = Self::parse_amount(&self.send_amount).and_then(|amount| {
                    let priority = Self::parse_amount(&self.send_priority_fee)?;
                    actions::send_transfer(&self.store, &self.send_from, &self.send_to, amount, priority, &self.send_memo)
                });
                self.set_status(result);
            }
        });
    }

    fn chain_screen(&mut self, ui: &mut egui::Ui) {
        section(ui, "Blocks", "Local binary block files written as .dblk.", |ui| {
            egui::Grid::new("blocks-table").striped(true).num_columns(6).show(ui, |ui| {
                ui.label(RichText::new("height").strong());
                ui.label(RichText::new("hash").strong());
                ui.label(RichText::new("txs").strong());
                ui.label(RichText::new("state root").strong());
                ui.label(RichText::new("tx root").strong());
                ui.label(RichText::new("file").strong());
                ui.end_row();
                for (path, block) in self.blocks.iter().rev().take(50) {
                    mono_label(ui, block.header.height);
                    mono_label(ui, block.header_hash().short());
                    mono_label(ui, block.transactions.len());
                    mono_label(ui, block.header.state_root.short());
                    mono_label(ui, block.header.tx_root.short());
                    mono_label(ui, path.file_name().and_then(|v| v.to_str()).unwrap_or("block"));
                    ui.end_row();
                }
            });
        });
    }

    fn mempool_screen(&mut self, ui: &mut egui::Ui) {
        section(ui, "Mempool", "Pending .dtx files sorted by account/nonce.", |ui| {
            egui::Grid::new("mempool-table").striped(true).num_columns(7).show(ui, |ui| {
                ui.label(RichText::new("tx hash").strong());
                ui.label(RichText::new("from").strong());
                ui.label(RichText::new("to").strong());
                ui.label(RichText::new("amount").strong());
                ui.label(RichText::new("size").strong());
                ui.label(RichText::new("fee").strong());
                ui.label(RichText::new("file").strong());
                ui.end_row();
                let policy = self.store.config().ok().map(|cfg| cfg.policy).unwrap_or_default();
                for (path, tx) in &self.pending {
                    let fee = tx.fee_breakdown(&policy);
                    mono_label(ui, tx.hash.short());
                    mono_label(ui, tx.tx.from.short());
                    mono_label(ui, tx.tx.to.short());
                    mono_label(ui, tx.tx.amount);
                    mono_label(ui, fee.encoded_size);
                    mono_label(ui, fee.paid_fee);
                    mono_label(ui, path.file_name().and_then(|v| v.to_str()).unwrap_or("tx"));
                    ui.end_row();
                }
            });
        });
    }

    fn fees_screen(&mut self, ui: &mut egui::Ui) {
        let policy = self.store.config().ok().map(|cfg| cfg.policy).unwrap_or_default();
        ui.horizontal(|ui| {
            metric(ui, "base fee", format!("{} dust", policy.base_fee));
            metric(ui, "fee / KB", format!("{} dust", policy.fee_per_kb));
            metric(ui, "max tx", format!("{} bytes", policy.max_tx_size_bytes));
            metric(ui, "max memo", format!("{} bytes", policy.max_memo_bytes));
        });
        ui.add_space(12.0);
        section(ui, "Fee estimator", "The first included KB is covered by the protocol base fee.", |ui| {
            ui.horizontal(|ui| {
                ui.label("amount");
                ui.add(TextEdit::singleline(&mut self.fee_amount));
                ui.label("memo");
                ui.add(TextEdit::singleline(&mut self.fee_memo));
            });
            let estimated_size = 180 + self.fee_memo.as_bytes().len();
            let fee = policy.breakdown(estimated_size, 0);
            ui.add_space(8.0);
            mono_label(ui, format!("estimated_size: {} bytes", estimated_size));
            mono_label(ui, format!("required_fee: {} dust", fee.required_fee));
            mono_label(ui, format!("fee_per_byte_microunits: {}", fee.fee_per_byte_microunits));
        });
    }

    fn inspector_screen(&mut self, ui: &mut egui::Ui) {
        section(ui, "Binary inspector", "Reads .dtx and .dblk files and verifies their headers/checksums.", |ui| {
            ui.horizontal(|ui| {
                ui.add(TextEdit::singleline(&mut self.inspect_path).hint_text("./.dustchain/blocks/00000001.dblk"));
                if ui.button("Inspect").clicked() {
                    match actions::inspect_path(&self.inspect_path) {
                        Ok(out) => {
                            self.inspect_output = out;
                            self.status = "inspected binary file".to_string();
                        }
                        Err(err) => {
                            self.inspect_output.clear();
                            self.status = format!("error: {err}");
                        }
                    }
                }
            });
            ui.add_space(8.0);
            ui.add(TextEdit::multiline(&mut self.inspect_output).font(egui::TextStyle::Monospace).desired_rows(18).desired_width(f32::INFINITY));
        });
    }

    fn lab_screen(&mut self, ui: &mut egui::Ui) {
        section(ui, "Local adversarial lab", "Reports protocol behavior against local synthetic inputs only.", |ui| {
            ui.horizontal(|ui| {
                egui::ComboBox::from_label("scenario")
                    .selected_text(&self.lab_scenario)
                    .show_ui(ui, |ui| {
                        for name in ["spam", "replay", "invalid-tx", "invalid-block", "oversized-block", "fork"] {
                            ui.selectable_value(&mut self.lab_scenario, name.to_string(), name);
                        }
                    });
                ui.label("txs");
                ui.add(TextEdit::singleline(&mut self.lab_txs).desired_width(90.0));
                if ui.button("Run").clicked() {
                    let txs = self.lab_txs.parse::<u64>().unwrap_or(50_000);
                    self.lab_report = Some(dust_lab::run_named(&self.lab_scenario, txs));
                    self.status = format!("ran lab scenario {}", self.lab_scenario);
                }
            });
            ui.add_space(8.0);
            if let Some(report) = &self.lab_report {
                let mut rendered = report.render();
                ui.add(TextEdit::multiline(&mut rendered).font(egui::TextStyle::Monospace).desired_rows(16).desired_width(f32::INFINITY));
            }
        });
    }

    fn node_screen(&mut self, ui: &mut egui::Ui) {
        section(ui, "Localnet", "The GUI is a console over local files. Use the CLI to run TCP nodes.", |ui| {
            mono_label(ui, "dust node start --port 3030");
            mono_label(ui, "dust node start --port 3031 --peer 127.0.0.1:3030");
            mono_label(ui, "dust peer probe 127.0.0.1:3031");
            mono_label(ui, "dust peer fetch-block 127.0.0.1:3031 1");
        });
        ui.add_space(12.0);
        section(ui, "Safety boundary", "Local-first defaults stay on loopback unless explicitly overridden.", |ui| {
            mono_label(ui, "No public mining network.");
            mono_label(ui, "No market data or investment UI.");
            mono_label(ui, "Malformed network frames are rejected before decoding payloads.");
        });
    }
}
