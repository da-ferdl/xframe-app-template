use crate::app::AppContext;

use super::navigator::{ERHistory, NavigatorState};
use egui_router::{
    EguiRouter, MakeHandler, Route, RouterBuilder as EguiRouterBuilder, TransitionConfig,
};
use std::rc::Rc;

pub struct Router<State> {
    navigator_state: Rc<NavigatorState>,
    inner_router: EguiRouter<State>,
}

pub struct RouterBuilder<State> {
    inner_builder: EguiRouterBuilder<State, ERHistory>,
    default_path: String,
}
impl<State: 'static> RouterBuilder<State> {
    pub fn new(default_path: String) -> Self {
        let inner_builder = EguiRouter::builder().default_path(&default_path);

        Self {
            inner_builder,
            default_path,
        }
    }

    /// Set the transition for both forward and backward transitions
    pub fn transition(mut self, transition: TransitionConfig) -> Self {
        let inner_builder = self.inner_builder;
        self.inner_builder = inner_builder.transition(transition);
        self
    }

    /// Set the default duration for transitions
    pub fn default_duration(mut self, duration: f32) -> Self {
        let inner_builder = self.inner_builder;
        self.inner_builder = inner_builder.default_duration(duration);
        self
    }

    /// Enable or disable the iOS-style swipe-to-go-back gesture (disabled by default)
    pub fn swipe_back_gesture(mut self, enabled: bool) -> Self {
        let inner_builder = self.inner_builder;
        self.inner_builder = inner_builder.swipe_back_gesture(enabled);
        self
    }

    /// Add a route.
    pub fn route<HandlerArgs, Han: MakeHandler<State, HandlerArgs> + 'static>(
        mut self,
        route: &str,
        handler: Han,
    ) -> Self {
        let inner_builder = self.inner_builder;
        self.inner_builder = inner_builder.route(route, handler);
        self
    }

    pub fn routes<HandlerArgs, Han: MakeHandler<State, HandlerArgs> + 'static>(
        mut self,
        routes: Vec<(&str, Han)>,
        //route: &str,
        //handler: Han,
    ) -> Self {
        for h in routes {
            let inner_builder = self.inner_builder;
            self.inner_builder = inner_builder.route(h.0, h.1);
        }

        self
    }

    pub fn route() -> impl Route<AppContext> {
        |ui: &mut egui::Ui, app_ctx: &mut AppContext| {
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
}
