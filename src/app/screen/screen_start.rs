use crate::{
    app::{
        AppContext, EguiResponseExt,
        screen::Screen,
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
    egui::{self, Margin, TextEdit, Widget, vec2},
};

pub struct ScreenStart;
impl ScreenStart {
    pub fn title() -> &'static str {
        crate::app::config::CONF_APP_NAME
    }

    pub fn path() -> &'static str {
        "/"
    }

    pub fn route() -> impl Route<AppContext> {
        |ui: &mut egui::Ui, app_ctx: &mut AppContext, _route_arg: RouteArgument<'_>| {
            let screen_state = ui_state!(|| ScreenStart);

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

        //ui.add_space(100.0);
        //
        egui::ScrollArea::vertical()
            .content_margin(Margin {
                left: left_padding,
                right: right_padding,
                top: base_padding as i8,
                bottom: bottom_padding,
            })
            .show(ui, |ui| {
                ui.scope(|ui| {
                    ui.spacing_mut().item_spacing = vec2(5., 5.);

                    egui_menu_bar(ui);

                    ui.heading("Home!");

                    ui.add_space(12.0);

                    ui.horizontal(|ui| {
                        ui.label("Write something: ");
                        TextEdit::singleline(&mut app_ctx.label)
                            .margin(Margin::symmetric(8, 8))
                            .ui(ui)
                            .interaction_ctx();
                    });

                    ui.add(egui::Slider::new(&mut app_ctx.value, 0.0..=10.0).text("value"))
                        .interaction_ctx();

                    if ui.button("Increment").interaction_ctx().clicked() {
                        app_ctx.value += 1.0;
                    }

                    ui.add_space(12.0);

                    ui.separator();

                    ui.add_space(12.0);

                    let xframe::CurrentPlatform {
                        is_android,
                        is_ios,
                        is_mobile,
                        is_desktop,
                        is_macos,
                        is_linux,
                        is_windows,
                    } = app_ctx.egui_ctx().current_platform();

                    ui.label(format!("is_android: {is_android}"));
                    ui.label(format!("is_ios: {is_ios}"));

                    ui.add_space(12.0);

                    ui.label(format!("is_mobile: {is_mobile}"));

                    ui.add_space(12.0);

                    ui.label(format!("is_macos: {is_macos}"));
                    ui.label(format!("is_linux: {is_linux}"));
                    ui.label(format!("is_windows: {is_windows}"));

                    ui.add_space(12.0);

                    ui.label(format!("is_desktop: {is_desktop}"));

                    ui.label(format!("Message: {}", app_ctx.label));

                    ui.label(format!("Id: {:?}", ui.next_auto_id()));

                    if ui.link("Edit Message").interaction_ctx().clicked() {
                        app_ctx.navigator.navigate(Screen::EditMessage.path(), None);
                    }

                    ui.add_space(12.0);

                    ui.label("Navigate to post:");

                    if ui.link("Post-ID 1").interaction_ctx().clicked() {
                        app_ctx
                            .navigator
                            .navigate(Screen::Post.path(), Some(Box::new(1_u8)));
                    }

                    if ui.link("Post-ID 2").interaction_ctx().clicked() {
                        app_ctx
                            .navigator
                            .navigate(Screen::Post.path(), Some(Box::new(2_u8)));
                    }

                    if ui.link("Invalid Post-ID 3").interaction_ctx().clicked() {
                        app_ctx
                            .navigator
                            .navigate(Screen::Post.path(), Some(Box::new(3_u8)));
                    }

                    if ui.link("No Post-ID argument").interaction_ctx().clicked() {
                        app_ctx.navigator.navigate(Screen::Post.path(), None);
                    }

                    ui.add_space(12.0);

                    if ui.link("About").interaction_ctx().clicked() {
                        app_ctx.navigator.navigate(Screen::About.path(), None);
                    }
                });
            });
    }
}
