use crate::{
    app::{
        AppContext, EguiResponseExt,
        widget::{
            egui_menu_bar, mobile_top_bar_content_default, scaffold_desktop_page,
            scaffold_mobile_layout,
        },
    },
    ui_state,
};

use egui_router::{Route, RouteArgument};
use xframe::{
    EguiContextExt,
    egui::{self},
};

pub struct ScreenEditMessage;
impl ScreenEditMessage {
    pub fn title() -> &'static str {
        "Edit Message"
    }

    pub fn path() -> &'static str {
        "/edit-message"
    }

    pub fn route() -> impl Route<AppContext> {
        |ui: &mut egui::Ui, app_ctx: &mut AppContext, _route_arg: RouteArgument<'_>| {
            let screen_state = ui_state!(|| ScreenEditMessage);

            let is_mobile_layout = app_ctx.current_layout().is_mobile();

            if is_mobile_layout {
                scaffold_mobile_layout(
                    ui,
                    egui::Id::new(crate::unique_int!()),
                    |ui_1| Self::mobile_top_ui(ui_1),
                    |ui_2| screen_state.page_ui(ui_2, app_ctx),
                );
            } else {
                scaffold_desktop_page(ui, |ui| screen_state.page_ui(ui, app_ctx));
            }
        }
    }

    fn mobile_top_ui(ui: &mut egui::Ui) {
        mobile_top_bar_content_default(ui, Self::title().into());
    }

    fn page_ui(&mut self, ui: &mut egui::Ui, app_ctx: &mut AppContext) {
        let base_padding = 10_f32;
        let sa = ui.ctx().safe_area_insets();
        let left_padding = (sa.left + base_padding) as i8;
        let right_padding = (sa.right + base_padding) as i8;
        let bottom_padding = (sa.bottom + base_padding) as i8;

        egui::Frame::NONE
            .inner_margin(egui::Margin {
                left: left_padding,
                right: right_padding,
                top: base_padding as i8,
                bottom: bottom_padding,
            })
            .show(ui, |ui| {
                ui.scope(|ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(5., 5.);

                    let msg_edit = crate::ui_state!(|| MessageEdit::new());

                    egui_menu_bar(ui);

                    ui.heading("Edit Message");
                    ui.text_edit_singleline(crate::ui_state!(|| "n/a".to_string()))
                        .interaction_ctx();

                    if ui.button("Save").interaction_ctx().clicked() {
                        app_ctx.navigator.back();
                    }
                });
            });
    }
}

struct MessageEdit {
    message: String,
}
impl MessageEdit {
    fn new() -> Self {
        println!("---> App/MessageEdit --- NEW ---");

        Self {
            message: "n/a".into(),
        }
    }
}
impl Drop for MessageEdit {
    fn drop(&mut self) {
        println!("---> App/MessageEdit --- DROPPED ---");
    }
}
