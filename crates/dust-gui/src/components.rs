use eframe::egui::{self, Color32, RichText, Stroke, Ui};

pub const PANEL: Color32 = Color32::from_rgb(18, 19, 22);
pub const PANEL_SOFT: Color32 = Color32::from_rgb(24, 25, 29);
pub const TEXT_MUTED: Color32 = Color32::from_rgb(150, 154, 163);
pub const ACCENT: Color32 = Color32::from_rgb(230, 230, 224);
pub const BORDER: Color32 = Color32::from_rgb(48, 50, 56);

pub fn install_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(10.0, 10.0);
    style.spacing.button_padding = egui::vec2(12.0, 7.0);
    style.visuals = egui::Visuals::dark();
    style.visuals.panel_fill = Color32::from_rgb(9, 10, 12);
    style.visuals.window_fill = PANEL;
    style.visuals.faint_bg_color = PANEL_SOFT;
    style.visuals.extreme_bg_color = Color32::from_rgb(6, 7, 9);
    style.visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, BORDER);
    style.visuals.widgets.inactive.bg_fill = PANEL_SOFT;
    style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(34, 36, 41);
    style.visuals.widgets.active.bg_fill = Color32::from_rgb(48, 50, 56);
    style.visuals.selection.bg_fill = Color32::from_rgb(78, 82, 92);
    ctx.set_style(style);
}

pub fn section(ui: &mut Ui, title: &str, subtitle: &str, add: impl FnOnce(&mut Ui)) {
    egui::Frame::default()
        .fill(PANEL)
        .stroke(Stroke::new(1.0, BORDER))
        .rounding(16.0)
        .inner_margin(egui::Margin::same(16.0))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new(title).size(18.0).strong().color(ACCENT));
                    if !subtitle.is_empty() {
                        ui.label(RichText::new(subtitle).size(12.0).color(TEXT_MUTED));
                    }
                });
            });
            ui.add_space(10.0);
            add(ui);
        });
}

pub fn metric(ui: &mut Ui, label: &str, value: impl ToString) {
    egui::Frame::default()
        .fill(PANEL_SOFT)
        .stroke(Stroke::new(1.0, BORDER))
        .rounding(14.0)
        .inner_margin(egui::Margin::same(12.0))
        .show(ui, |ui| {
            ui.label(RichText::new(label).size(11.0).color(TEXT_MUTED));
            ui.label(RichText::new(value.to_string()).size(20.0).strong().color(ACCENT));
        });
}

pub fn mono_label(ui: &mut Ui, text: impl ToString) {
    ui.label(RichText::new(text.to_string()).monospace().size(12.0).color(Color32::from_rgb(205, 207, 212)));
}

pub fn status_line(ui: &mut Ui, text: &str) {
    let color = if text.to_ascii_lowercase().contains("error") || text.to_ascii_lowercase().contains("failed") {
        Color32::from_rgb(238, 126, 126)
    } else {
        Color32::from_rgb(159, 209, 164)
    };
    ui.label(RichText::new(text).monospace().size(12.0).color(color));
}
