use crate::app::theme::EguiThemeExt;
use xframe::{
    EguiContextExt,
    egui::{self, Align},
};

/// Arg `id`: A unique id per page - used to store the top-bar height
/// which can vary from page to page on mobile.
pub fn scaffold_mobile_layout(
    ui: &mut egui::Ui,
    id: egui::Id,
    top_bar_content: impl FnOnce(&mut egui::Ui),
    main_content: impl FnOnce(&mut egui::Ui),
) {
    //ui.set_width(ui.available_width());
    //ui.set_height(ui.available_height());

    egui::Frame::NONE
        //.fill(theme::EGUI_CLEAR_COLOR)
        .shadow(egui::Shadow {
            offset: [0, 0],
            blur: 15,
            spread: 5,
            color: egui::Color32::from_black_alpha(25),
        })
        .show(ui, |ui| {
            let top_bar_height = ui.data_mut(|d| *d.get_temp_mut_or_default::<f32>(id));

            ui.with_layout(egui::Layout::bottom_up(Align::Min), |ui| {
                main_content_frame(ui, top_bar_height, main_content);

                let next_top_bar_height = top_bar_frame(ui, top_bar_content);

                if top_bar_height != next_top_bar_height {
                    ui.data_mut(|d| {
                        d.insert_temp(id, next_top_bar_height);
                    });

                    if !ui.ctx().will_discard() {
                        ui.ctx().request_discard("set top-bar height sizing pass");
                    }
                }
            });
        });
}

/// Returns the top-bar height.
fn top_bar_frame(ui: &mut egui::Ui, top_bar_content: impl FnOnce(&mut egui::Ui)) -> f32 {
    let theme = ui.theme();
    let sa = ui.ctx().safe_area_insets();
    let base_padding = 10_f32;

    let height = egui::Frame::NONE
        .shadow(egui::Shadow {
            offset: [0, 2],
            blur: 20,
            spread: 0,
            color: egui::Color32::from_black_alpha(100),
        })
        .fill(theme.color_background_top_bar())
        .inner_margin(egui::Margin {
            left: (sa.left + base_padding) as i8,
            right: (sa.right + base_padding) as i8,
            top: (sa.top + 3.) as i8,
            bottom: 5,
        })
        .show(ui, |ui| {
            ui.vertical(|ui| {
                ui.set_width(ui.available_width());
                top_bar_content(ui);
            });
        })
        .response
        .rect
        .height();

    if height < 0.0 {
        return 0.;
    }

    height
}

fn main_content_frame(
    ui: &mut egui::Ui,
    top_bar_height: f32,
    main_content: impl FnOnce(&mut egui::Ui),
) {
    let theme = ui.theme();

    egui::Frame::NONE
        .fill(theme.color_background_page())
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            //ui.set_height(ui.available_height());

            ui.vertical(|ui| {
                ui.add_space(top_bar_height);

                main_content(ui);
            });
        });
}
