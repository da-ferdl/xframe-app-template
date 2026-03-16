//! With Sidebar open / close animation.
//!
//! Animation logic example is taken from the gist:
//! https://gist.github.com/fgimian/aaf3fe7c2db4e19170f581cf1378b7a8

use xframe::egui::{self, Color32, CornerRadius, Stroke, Widget};

use crate::app::{Navigator, widget::SideBarState};

pub fn desktop_top_bar_content(
    ui: &mut egui::Ui,
    sidebar_state: &mut SideBarState,
    navigator: &Navigator,
) {
    let current_screen = navigator.current_screen();
    let can_go_back = navigator.can_go_back();
    let title = current_screen.desktop_title();

    ui.horizontal_centered(|ui| {
        if side_panel_button(ui, sidebar_state.is_opening_or_open()) {
            //println!("-- Side-Panel-Button clicked --");
            sidebar_state.toggle();
        }

        // Handle SideBar animation.
        sidebar_state.handle_animation();

        ui.add_space(40.);

        if back_button(ui, can_go_back) {
            //println!("-- Back-Clicked - canGoBack: {can_go_back}");
            navigator.back();
        }

        ui.add_space(5.);

        ui.label(
            egui::RichText::new(title)
                .strong()
                .color(egui::Color32::from_white_alpha(225)),
        );
    });
}

/// Returns `true` if clicked.
fn side_panel_button(ui: &mut egui::Ui, is_opening_or_open: bool) -> bool {
    ui.scope(|ui| {
        let icon_color = if is_opening_or_open {
            Color32::from_white_alpha(100)
        } else {
            Color32::from_white_alpha(220)
        };

        let style = ui.style_mut();

        let inactive = &mut style.visuals.widgets.inactive;
        let hovered = &mut style.visuals.widgets.hovered;
        let active = &mut style.visuals.widgets.active;

        inactive.bg_stroke = Stroke::NONE;
        hovered.bg_stroke = Stroke::NONE;
        active.bg_stroke = Stroke::NONE;

        inactive.fg_stroke = Stroke::new(0., icon_color);
        hovered.fg_stroke = Stroke::new(0., icon_color);
        active.fg_stroke = Stroke::new(0., icon_color);

        inactive.corner_radius = CornerRadius::ZERO;
        hovered.corner_radius = CornerRadius::ZERO;
        active.corner_radius = CornerRadius::ZERO;

        inactive.weak_bg_fill = Color32::TRANSPARENT;
        hovered.weak_bg_fill = Color32::from_black_alpha(30);
        active.weak_bg_fill = Color32::from_black_alpha(60);

        //ui.style_mut().visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
        //ui.style_mut().visuals.widgets.hovered.weak_bg_fill = Color32::RED;

        egui::Button::new(
            egui::RichText::new(egui_material_icons::icons::ICON_SIDE_NAVIGATION)
                .size(20.0)
                .strong(),
        )
        .frame(true)
        .ui(ui)
    })
    .inner
    .clicked()
}

/// Returns `true` if clicked.
fn back_button(ui: &mut egui::Ui, can_go_back: bool) -> bool {
    ui.scope(|ui| {
        let style = ui.style_mut();

        let inactive = &mut style.visuals.widgets.inactive;
        let hovered = &mut style.visuals.widgets.hovered;
        let active = &mut style.visuals.widgets.active;

        inactive.bg_stroke = Stroke::NONE;
        hovered.bg_stroke = Stroke::NONE;
        active.bg_stroke = Stroke::NONE;

        inactive.fg_stroke = Stroke::new(0., Color32::from_white_alpha(10));
        hovered.fg_stroke = Stroke::new(0., Color32::WHITE);
        active.fg_stroke = Stroke::new(0., Color32::WHITE);

        inactive.corner_radius = CornerRadius::ZERO;
        hovered.corner_radius = CornerRadius::ZERO;
        active.corner_radius = CornerRadius::ZERO;

        inactive.weak_bg_fill = Color32::TRANSPARENT;
        hovered.weak_bg_fill = Color32::from_black_alpha(30);
        active.weak_bg_fill = Color32::from_black_alpha(60);

        //ui.style_mut().visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
        //ui.style_mut().visuals.widgets.hovered.weak_bg_fill = Color32::RED;

        ui.add_enabled(
            can_go_back,
            egui::Button::new(
                egui::RichText::new(egui_material_icons::icons::ICON_ARROW_BACK_IOS_NEW)
                    .size(20.0)
                    .strong(),
            )
            .frame(true),
        )
    })
    .inner
    .clicked()
}
