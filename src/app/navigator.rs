use crate::app::{AppContext, screen::Screen};
use egui_router::EguiRouter;
use std::{
    any::Any,
    cell::{Cell, RefCell},
    rc::Rc,
};
use xframe::egui;

pub enum NavigateMessage {
    Navigate(String, Option<Box<dyn Any>>),
    Back,
}

#[derive(Clone)]
pub struct Navigator(Rc<NavigatorState>);
impl Navigator {
    pub fn new(navigator_state: Rc<NavigatorState>) -> Self {
        Self(navigator_state)
    }

    pub fn navigate(&self, to: &str, route_arg: Option<Box<dyn Any>>) {
        self.0.navigate(to, route_arg);
    }

    pub fn back(&self) {
        self.0.back();
    }

    pub fn can_go_back(&self) -> bool {
        self.0.can_go_back()
    }

    pub fn current_screen(&self) -> Screen {
        self.0.current_screen()
    }
}

pub struct NavigatorState {
    egui_ctx: egui::Context,
    //history: RefCell<Vec<Box<dyn Any>>>,
    navigate_msg: RefCell<Option<NavigateMessage>>,
    // Can be a empty string!
    current_route: RefCell<String>,
    current_screen: RefCell<Screen>,
    can_go_back: Cell<bool>,
}
impl NavigatorState {
    pub fn new(egui_ctx: egui::Context) -> Rc<Self> {
        Self {
            egui_ctx,
            //history:
            navigate_msg: Default::default(),
            current_route: Default::default(),
            current_screen: Screen::Start.into(),
            can_go_back: Default::default(),
        }
        .into()

        //Self((Default::default(), Default::default(), egui_ctx)).into()
    }

    pub fn can_go_back(&self) -> bool {
        self.can_go_back.get()
    }

    pub fn current_screen(&self) -> Screen {
        self.current_screen.borrow().clone()
    }

    pub fn on_egui_begin_pass(
        &self,
        egui_router: &mut EguiRouter<AppContext>,
        app_ctx: &mut AppContext,
    ) {
        // Handle navigation invocation if a navigate message is set.
        if let Some(msg) = self.navigate_msg.borrow_mut().take() {
            match msg {
                NavigateMessage::Navigate(route, arg) => {
                    let _ = egui_router.navigate(app_ctx, route, arg);
                }
                NavigateMessage::Back => {
                    let _ = egui_router.back();
                }
            };
        }

        self.can_go_back.set(egui_router.history_len() > 1);

        if let Some((active_route_path, _)) = egui_router.active_route() {
            if self.current_route.borrow().as_str() != active_route_path {
                let screen = Screen::from_path(active_route_path);
                self.current_screen.replace(screen);
                self.current_route.replace(active_route_path.into());
            }
        }
    }

    fn navigate(&self, to: &str, arg: Option<Box<dyn Any>>) {
        let _ = self
            .navigate_msg
            .borrow_mut()
            .insert(NavigateMessage::Navigate(to.to_string(), arg));
        //self.0.2.request_repaint();
    }

    fn back(&self) {
        let _ = self.navigate_msg.borrow_mut().insert(NavigateMessage::Back);
        //self.0.2.request_repaint();
    }
}
