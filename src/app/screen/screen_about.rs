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
use xframe::{EguiContextExt, egui};

pub struct ScreenAbout;
impl ScreenAbout {
    pub fn title() -> &'static str {
        "About"
    }

    pub fn path() -> &'static str {
        "/about"
    }

    pub fn route() -> impl Route<AppContext> {
        |ui: &mut egui::Ui, app_ctx: &mut AppContext, _route_arg: RouteArgument<'_>| {
            let screen_state = ui_state!(|| ScreenAbout);

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

                    egui_menu_bar(ui);

                    ui.heading("About this App:");

                    if ui.button("back").interaction_ctx().clicked() {
                        app_ctx.navigator.back();
                    }
                });
            });
    }
}
