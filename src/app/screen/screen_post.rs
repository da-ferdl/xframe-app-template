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

pub struct ScreenPost;
impl ScreenPost {
    pub fn title() -> &'static str {
        "Post"
    }

    pub fn path() -> &'static str {
        "/post"
    }

    pub fn route() -> impl Route<AppContext> {
        |ui: &mut egui::Ui, app_ctx: &mut AppContext, arg: RouteArgument<'_>| {
            let screen_view = ui_state!(|| ScreenPostView::new(arg.get()));

            let is_mobile_layout = app_ctx.current_layout().is_mobile();

            if is_mobile_layout {
                scaffold_mobile_layout(
                    ui,
                    egui::Id::new(crate::unique_int!()),
                    |ui_1| Self::mobile_top_ui(ui_1),
                    |ui_2| screen_view.ui(ui_2, app_ctx),
                );
            } else {
                scaffold_desktop_page(ui, |ui| screen_view.ui(ui, app_ctx));
            }
        }
    }

    fn mobile_top_ui(ui: &mut egui::Ui) {
        mobile_top_bar_content_default(ui, Self::title().into());
    }
}

struct ScreenPostView {
    post_msg: String,
}
impl ScreenPostView {
    fn new(arg: Option<&u8>) -> Self {
        let valid_post_ids = vec![1_u8, 2];

        let post_msg = match arg {
            Some(id) => {
                if !valid_post_ids.contains(id) {
                    format!("Post-ID '{id}' is not valid")
                } else {
                    format!("Selected Post-ID: {id}")
                }
            }
            None => "No 'Post-ID' route argument set".to_string(),
        };

        Self { post_msg }
    }

    fn ui(&mut self, ui: &mut egui::Ui, app_ctx: &mut AppContext) {
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

                    ui.label(&self.post_msg);
                    if ui.button("back").interaction_ctx().clicked() {
                        app_ctx.navigator.back();
                    }
                });
            });
    }
}
