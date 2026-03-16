mod app_context;
pub(crate) mod config;
mod egui_response_ext;
mod interact_rect_handler;
mod main_application;
mod navigator;
//mod router;
mod screen;
pub(super) mod theme;
mod ui_event_proxy;
mod ui_state;
mod virtual_keyboard;
mod widget;

pub(super) use app_context::*;
pub(super) use egui_response_ext::{
    EguiInnerResponseExt, EguiResponseExt, set_egui_response_ext_interact_rect_handler,
};
pub(super) use interact_rect_handler::InteractRectHandler;
pub(crate) use main_application::MainApplication;
pub(super) use navigator::{NavigateMessage, Navigator, NavigatorState};
pub(super) use ui_event_proxy::UiEventProxy;
pub(crate) use ui_state::*;
pub(super) use virtual_keyboard::VirtualKeyboard;

pub(crate) type ExtEvent = Box<dyn FnOnce(&mut MainApplication) + Send>;
