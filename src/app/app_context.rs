use crate::app::VirtualKeyboard;

use super::Navigator;
use xframe::{
    EguiContextExt,
    egui::{self, Theme, ThemePreference},
};

#[derive(PartialEq)]
pub enum CustomKeyboardPreference {
    OnlyOnAndroid,
    Allways,
    Never,
}
impl CustomKeyboardPreference {
    pub fn should_use_keyboard(&self, egui_ctx: &egui::Context) -> bool {
        match self {
            Self::OnlyOnAndroid => egui_ctx.current_platform().is_android,
            Self::Allways => true,
            Self::Never => false,
        }
    }
}

#[derive(PartialEq)]
pub enum LayoutPreference {
    Auto,
    Mobile,
    Desktop,
}
impl LayoutPreference {
    pub fn as_layout(&self, egui_ctx: &egui::Context) -> Layout {
        match self {
            Self::Auto => {
                if egui_ctx.current_platform().is_mobile {
                    Layout::Mobile
                } else {
                    Layout::Desktop
                }
            }
            Self::Mobile => Layout::Mobile,
            Self::Desktop => Layout::Desktop,
        }
    }
}

pub enum Layout {
    Mobile,
    Desktop,
}
impl Layout {
    pub fn is_mobile(&self) -> bool {
        match self {
            Self::Mobile => true,
            Self::Desktop => false,
        }
    }

    pub fn is_desktop(&self) -> bool {
        match self {
            Self::Mobile => false,
            Self::Desktop => true,
        }
    }
}

pub struct AppContext {
    pub label: String,
    pub value: f32,
    pub navigator: Navigator,
    egui_ctx: egui::Context,
    selected_custom_keyboard_preference: CustomKeyboardPreference,
    custom_virtual_keyboard: Option<VirtualKeyboard>,
    selected_layout_preference: LayoutPreference,
    selected_layout: Layout,
}
impl AppContext {
    pub fn new(
        egui_ctx: egui::Context,
        navigator: Navigator,
        theme_preference: ThemePreference,
        layout_preference: LayoutPreference,
        custom_keyboard_preference: CustomKeyboardPreference,
    ) -> Self {
        let mut custom_virtual_keyboard = None;
        if custom_keyboard_preference.should_use_keyboard(&egui_ctx) {
            custom_virtual_keyboard = Some(VirtualKeyboard::default());
        }

        let selected_layout = layout_preference.as_layout(&egui_ctx);

        let this = Self {
            label: "Hello World!".to_owned(),
            value: 2.7,
            navigator,
            egui_ctx,
            selected_custom_keyboard_preference: custom_keyboard_preference,
            custom_virtual_keyboard,
            selected_layout_preference: layout_preference,
            selected_layout,
        };

        this.set_theme_preference(theme_preference);

        this
    }

    pub fn egui_ctx(&self) -> &egui::Context {
        &self.egui_ctx
    }

    /// The currently active theme (may depend on the system theme).
    pub fn current_theme(&self) -> Theme {
        self.egui_ctx.theme()
    }

    /// The currently set theme-preference.
    pub fn current_theme_preference(&self) -> ThemePreference {
        self.egui_ctx.options(|opt| opt.theme_preference.into())
    }

    /// The current active layout.
    pub fn current_layout(&self) -> &Layout {
        &self.selected_layout
    }

    /// The currently set layout-preference.
    pub fn current_layout_preference(&self) -> &LayoutPreference {
        &self.selected_layout_preference
    }

    /// The currently set custom-keyboard-preference.
    pub fn current_custom_keyboard_preference(&self) -> &CustomKeyboardPreference {
        &self.selected_custom_keyboard_preference
    }

    pub fn set_theme_preference(&self, preference: ThemePreference) {
        let current = self.current_theme_preference();
        if current == preference {
            return;
        }

        self.egui_ctx
            .options_mut(|opt| opt.theme_preference = preference);
        self.egui_ctx.request_repaint();
    }

    pub fn set_layout_preference(&mut self, preference: LayoutPreference) {
        if self.selected_layout_preference == preference {
            return;
        }

        let layout = preference.as_layout(&self.egui_ctx);
        self.selected_layout = layout;
        self.selected_layout_preference = preference;

        self.egui_ctx.request_repaint();
    }

    pub fn set_custom_keyboard_preference(&mut self, preference: CustomKeyboardPreference) {
        if self.selected_custom_keyboard_preference == preference {
            return;
        }

        let is_currently_keyboard_set = self.custom_virtual_keyboard.is_some();
        let should_use_keyboard = preference.should_use_keyboard(&self.egui_ctx);

        self.selected_custom_keyboard_preference = preference;

        if should_use_keyboard {
            let _ = self
                .custom_virtual_keyboard
                .get_or_insert_with(|| VirtualKeyboard::default());
        } else {
            self.custom_virtual_keyboard = None;
        }

        if is_currently_keyboard_set != should_use_keyboard {
            self.egui_ctx.request_repaint();
        }
    }

    pub fn handle_custom_keyboard_bump_events(&mut self, raw_input: &mut egui::RawInput) {
        if let Some(v_keyboard) = &mut self.custom_virtual_keyboard {
            v_keyboard.bump_events(raw_input);
        }
    }

    pub fn handle_custom_keyboard_ui(&mut self, ui: &mut egui::Ui) {
        if let Some(v_keyboard) = &mut self.custom_virtual_keyboard {
            v_keyboard.show_if_active(ui);
        }

        if self.custom_virtual_keyboard.is_some() {
            // Removes 'ime' which prevents display of the mobile (touch) virtual keyboard display.
            // -> check after updates if still functional.
            let _ = ui.output_mut(|v| v.ime.take());
        }
    }
}
