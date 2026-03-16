use crate::app::EguiInnerResponseExt;
use xframe::egui;

pub fn egui_menu_bar(ui: &mut egui::Ui) {
    egui::MenuBar::new()
        .ui(ui, |ui| {
            egui::widgets::global_theme_preference_buttons(ui);
        })
        .interaction_ctx();
}
