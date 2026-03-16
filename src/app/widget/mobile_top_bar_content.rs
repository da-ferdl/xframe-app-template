use crate::app::theme;
use xframe::egui;

pub fn mobile_top_bar_content_default(ui: &mut egui::Ui, title: String) {
    ui.label(
        egui::RichText::new(title)
            .heading()
            .strong()
            .color(theme::color_text_light()),
    );
    ui.label("text");
}
