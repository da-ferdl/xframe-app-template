use super::{
    AppContext, ExtEvent, InteractRectHandler, Navigator, NavigatorState, UiEventProxy,
    set_egui_response_ext_interact_rect_handler, theme,
};
use crate::app::screen::{ScreenAbout, ScreenEditMessage, ScreenPost, ScreenStart};
use crate::app::widget::{desktop_top_bar_content, scaffold_desktop_layout};
use crate::app::{CustomKeyboardPreference, LayoutPreference, screen, widget};
use egui_router::{EguiRouter, TransitionConfig};
use std::{rc::Rc, sync::Arc};
use xframe::egui::Widget;
use xframe::{
    EguiContextExt, XFrameProxy, egui,
    winit::event::{DeviceEvent, WindowEvent},
};

pub struct MainApplication {
    app_ctx: AppContext,
    navigator_state: Rc<NavigatorState>,
    navigator: Navigator,
    egui_router: EguiRouter<AppContext>,
    ui_event_proxy: UiEventProxy,
    interact_rect_handler: Rc<InteractRectHandler>,
}

impl MainApplication {
    pub fn new(egui_ctx: egui::Context, xframe_proxy: Arc<XFrameProxy<ExtEvent>>) -> Self {
        theme::init_egui_theme(&egui_ctx);

        let platform = egui_ctx.current_platform();

        let interact_rect_handler = InteractRectHandler::new(egui_ctx.clone());
        set_egui_response_ext_interact_rect_handler(interact_rect_handler.clone());

        let navigator_state = NavigatorState::new(egui_ctx.clone());
        let navigator = Navigator::new(navigator_state.clone());

        let ui_event_proxy = UiEventProxy::new(xframe_proxy);

        let theme_preference = egui::ThemePreference::System;
        let layout_preference = LayoutPreference::Auto;
        //let layout_preference = LayoutPreference::Mobile;

        let mut custom_keyboard_preference = CustomKeyboardPreference::OnlyOnAndroid;
        if platform.is_android || platform.is_macos {
            custom_keyboard_preference = CustomKeyboardPreference::Allways;
        }

        let mut app_ctx = AppContext::new(
            egui_ctx.clone(),
            navigator.clone(),
            theme_preference,
            layout_preference,
            custom_keyboard_preference,
        );

        // --------------------------------------------------------------------
        // Router setup

        let mut transition_config = TransitionConfig::fade();
        let mut swipe_back_gesture = false;
        let mut transition_default_duration = 0.15_f32;

        if platform.is_mobile {
            swipe_back_gesture = true;
            transition_default_duration = 0.25;
            if platform.is_ios {
                transition_default_duration = 0.3;
                transition_config = TransitionConfig::slide();
            } else {
                transition_config = TransitionConfig::fade_up();
            }
        }

        let egui_router = build_router(
            &mut app_ctx,
            transition_config,
            swipe_back_gesture,
            transition_default_duration,
        );

        // --------------------------------------------------------------------

        Self {
            app_ctx,
            navigator_state,
            navigator,
            egui_router,
            ui_event_proxy,
            interact_rect_handler,
        }
    }

    fn reload_router(&mut self) {
        self.egui_router = build_router(&mut self.app_ctx, TransitionConfig::slide(), true, 0.25);
        self.app_ctx.egui_ctx().request_repaint();
    }
}

impl xframe::XFrameApp<ExtEvent> for MainApplication {
    fn on_start(&mut self, start_context: &xframe::StartContext<'_>) {
        println!("---> App/on_start ");

        let _ = start_context;
    }

    fn raw_input_hook(
        &mut self,
        _ctx: &xframe::egui::Context,
        raw_input: &mut xframe::egui::RawInput,
    ) {
        // The interact-rect list is cleared here and repopulated on
        // 'xframe::App::ui' by the ui elements that have interactive elements.
        //
        // Clearing must be done so elements that are no longer available are removed
        // and elements that moved (eg. on a scroll view) are updated.
        self.interact_rect_handler.clear_interactive_rect_list();

        self.app_ctx.handle_custom_keyboard_bump_events(raw_input);
    }

    fn logic(&mut self, ctx: &xframe::egui::Context, frame: &mut xframe::Frame) {
        _ = (ctx, frame);
    }

    fn ui(&mut self, ui: &mut xframe::egui::Ui, _frame: &mut xframe::Frame) {
        // ---------------------------------------------------------
        // Handles navigation invocation and internal navigator state.
        self.navigator_state
            .on_egui_begin_pass(&mut self.egui_router, &mut self.app_ctx);

        let is_desktop_layout = self.app_ctx.current_layout().is_desktop();

        if is_desktop_layout {
            scaffold_desktop_layout(
                ui,
                |ui, side_bar_state| desktop_top_bar_content(ui, side_bar_state, &self.navigator),
                |ui| {
                    ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                        ui.label("text 1");
                        ui.label("text 2");
                        ui.label("text 3");
                        ui.label("text 4");
                    });

                    ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                        /*egui_material_icons::icon_button(
                            ui,
                            egui_material_icons::icons::ICON_ARROW_BACK_IOS,
                        );*/

                        ui.label("d-text 1");
                        ui.label("d-text 2");
                        egui::Button::new(
                            egui::RichText::new(
                                egui_material_icons::icons::ICON_ARROW_BACK_IOS_NEW,
                            )
                            .size(22.0)
                            .strong(),
                        )
                        .frame(true)
                        .ui(ui);
                    });
                },
                |ui| self.egui_router.ui(ui, &mut self.app_ctx),
            );
        } else {
            self.egui_router.ui(ui, &mut self.app_ctx);
        }

        ui.scope(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(5., 5.);
            self.app_ctx.handle_custom_keyboard_ui(ui);
        });

        crate::app::ui_state::state_holder().on_egui_end_pass();
    }

    fn on_user_event(&mut self, event: ExtEvent) {
        event(self);
    }

    fn on_app_life_cycle_state_change(&mut self, next_state: xframe::AppLifeCycleState) {
        let state = match next_state {
            xframe::AppLifeCycleState::Background { did_exit } => {
                format!("Background - did_exit: {did_exit}")
            }
            xframe::AppLifeCycleState::ForegroundPaused { was_active } => {
                format!("ForegroundPaused - was_active: {was_active}")
            }
            xframe::AppLifeCycleState::ForegroundActive => "ForegroundActive".to_string(),
        };

        println!("---> App/on_app_life_cycle - {state}");
    }

    fn on_save(&mut self, storage: &mut dyn xframe::Storage) {
        println!("---> App/on_save ");

        let _ = storage;
    }

    fn on_exit(&mut self) {
        println!("---> App/on_exit ");
    }

    fn clear_color(&self, visuals: &xframe::egui::Visuals) -> [f32; 4] {
        visuals.window_fill.to_normalized_gamma_f32()
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn winit_window_event_hook(&mut self, event: WindowEvent) -> Option<WindowEvent> {
        let platform = self.app_ctx.egui_ctx().current_platform();
        let is_mobile = platform.is_mobile;
        let is_desktop = platform.is_desktop;

        let ignore = match &event {
            WindowEvent::Touch(t) => {
                if is_mobile {
                    let loc_x = t.location.x;
                    let loc_y = t.location.y;

                    self.interact_rect_handler.ignore_touch(loc_x, loc_y)
                } else {
                    true
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                if is_desktop {
                    let p_x = position.x;
                    let p_y = position.y;

                    self.interact_rect_handler.ignore_touch(p_x, p_y)
                } else {
                    true
                }
            }
            _ => false,
        };

        if ignore {
            return None;
        }

        Some(event)
    }

    fn winit_device_event_hook(&mut self, event: DeviceEvent) -> Option<DeviceEvent> {
        match &event {
            // - Not handled by egui on macOS (but still repaints and increases cpu)
            DeviceEvent::MouseMotion { .. }
            | DeviceEvent::MouseWheel { .. }
            | DeviceEvent::Motion { .. } => {
                return None;
            }
            _ => (),
        }

        Some(event)
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

fn build_router(
    app_ctx: &mut AppContext,
    transition: TransitionConfig,
    swipe_back_gesture: bool,
    transition_default_duration: f32,
) -> EguiRouter<AppContext> {
    let mut egui_router = EguiRouter::builder(ScreenStart::path(), None)
        //.transition(TransitionConfig::slide().with_easing(egui_animation::easing::quad_out))
        .transition(transition)
        .swipe_back_gesture(swipe_back_gesture)
        .default_duration(transition_default_duration);

    egui_router = egui_router
        .route(ScreenStart::path(), ScreenStart::route)
        .route(ScreenEditMessage::path(), ScreenEditMessage::route)
        .route(ScreenPost::path(), ScreenPost::route)
        .route(ScreenAbout::path(), ScreenAbout::route);

    egui_router.build(app_ctx)
}
