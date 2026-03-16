use std::rc::Rc;
use xframe::{
    egui::{
        Color32, Context, CornerRadius,
        FontFamily::{Monospace, Proportional},
        FontId, Shadow, Spacing, Stroke, Style, TextStyle, Theme, Visuals,
        style::{
            HandleShape, NumericColorSpace, Selection, TextCursorStyle, WidgetVisuals, Widgets,
        },
        vec2,
    },
    epaint::{AlphaFromCoverage, TextOptions},
};

use crate::app::theme::fonts::font_definitions;

/// Used by the [eframe::App::clear_color] implementation.
/// This is the background color of windows if you don’t set a central panel.
/// See [eframe::App::clear_color] for details.
//pub const EGUI_CLEAR_COLOR: Color32 = COLOR_PRIMARY_BG;

/* ******* Purple ******
// Hex: #622A84
pub const COLOR_PRIMARY: Color32 = Color32::from_rgb(98, 42, 132);
// Hex: #52276C - darker then primary
pub const COLOR_PRIMARY_BG: Color32 = Color32::from_rgb(82, 39, 108);
// Hex: #8E24AA - lighter then primary
pub const COLOR_PRIMARY_ACCENT: Color32 = Color32::from_rgb(142, 36, 170);*/
// *********************

// #0D47A1

// ******* Blue ********
// Hex: #1565C0
pub const COLOR_PRIMARY: Color32 = Color32::from_rgb(21, 101, 192);
// Hex: #0D47A1 - darker then primary
pub const COLOR_PRIMARY_BG: Color32 = Color32::from_rgb(13, 71, 161);
// Hex: #1E88E5 - lighter then primary
pub const COLOR_PRIMARY_ACCENT: Color32 = Color32::from_rgb(30, 136, 229);
// *********************

// Hex: #12664f
pub const COLOR_SUCCESS: Color32 = Color32::from_rgb(18, 102, 79);
// Hex: #b77e33
pub const COLOR_WARNING: Color32 = Color32::from_rgb(183, 126, 51);
// Hex: #c3423f
pub const COLOR_DANGER: Color32 = Color32::from_rgb(195, 66, 63);

pub fn color_text_light() -> Color32 {
    Color32::from_white_alpha(200)
}

pub fn color_text_dark() -> Color32 {
    Color32::from_black_alpha(200)
}

pub const STROKE_WIDTH_THIN: f32 = 0.5;
pub const STROKE_WIDTH_NORMAL: f32 = 1.;
pub const STROKE_WIDTH_THICK: f32 = 2.;

pub trait EguiThemeExt {
    fn color_background_page(&self) -> Color32;
    fn color_background_top_bar(&self) -> Color32;
    fn color_text(&self) -> Color32;
}
impl EguiThemeExt for Theme {
    fn color_background_page(&self) -> Color32 {
        match self {
            Self::Dark => Color32::from_gray(55),
            Self::Light => Color32::from_hex("#fcfcfc").unwrap(),
        }
    }

    fn color_background_top_bar(&self) -> Color32 {
        match self {
            Self::Dark => Color32::from_gray(50),
            Self::Light => COLOR_PRIMARY,
        }
    }

    fn color_text(&self) -> Color32 {
        match self {
            Self::Dark => color_text_light(),
            Self::Light => color_text_dark(),
        }
    }
}

pub fn init_egui_theme(egui_ctx: &Context) {
    egui_material_icons::initialize(&egui_ctx);

    egui_ctx.set_fonts(font_definitions());

    let dark_style = style_dark();
    let light_style = style_light_with_dark(&dark_style);

    egui_ctx.options_mut(|o| {
        o.dark_style = Rc::new(dark_style);
        o.light_style = Rc::new(light_style);
        o.fallback_theme = Theme::Light;
    });
}

/// Default dark theme.
fn style_dark() -> Style {
    Style {
        //override_text_style: todo!(),
        //override_font_id: (),
        //override_text_valign: (),
        text_styles: [
            (TextStyle::Small, FontId::new(12., Proportional)),
            (TextStyle::Body, FontId::new(16., Proportional)),
            (TextStyle::Button, FontId::new(16., Proportional)),
            (TextStyle::Heading, FontId::new(20.0, Proportional)),
            (TextStyle::Monospace, FontId::new(16.0, Monospace)),
        ]
        .into(),
        //drag_value_text_style: (),
        //number_formatter: (),
        //wrap_mode: (),
        spacing: Spacing {
            item_spacing: vec2(0., 0.),
            //window_margin: todo!(),
            button_padding: vec2(10., 6.),
            //menu_margin: todo!(),
            //indent: todo!(),
            //interact_size: todo!(),
            //slider_width: todo!(),
            slider_rail_height: 4.,
            //combo_width: todo!(),
            //text_edit_width: todo!(),
            //icon_width: todo!(),
            //icon_width_inner: todo!(),
            //icon_spacing: todo!(),
            //default_area_size: todo!(),
            //tooltip_width: todo!(),
            //menu_width: todo!(),
            //menu_spacing: todo!(),
            //indent_ends_with_horizontal_line: todo!(),
            //combo_height: todo!(),
            //scroll: todo!(),
            ..Default::default()
        },
        //interaction: (),
        visuals: visuals_dark(),
        //animation_time: (),
        //debug: (),
        //explanation_tooltips: (),
        //url_in_tooltip: (),
        //always_scroll_the_only_direction: (),
        //scroll_animation: (),
        //compact_menu_style: (),
        ..Default::default()
    }
}

/// Default light theme - with defaults from dark theme.
fn style_light_with_dark(dark: &Style) -> Style {
    let dark = dark.clone();
    Style {
        visuals: visuals_light_with_dark(&dark.visuals),
        ..dark
    }
}

/// Default dark theme.
fn visuals_dark() -> Visuals {
    Visuals {
        dark_mode: true,
        text_options: TextOptions {
            alpha_from_coverage: AlphaFromCoverage::DARK_MODE_DEFAULT,
            ..Default::default()
        },
        override_text_color: Some(color_text_light()),
        weak_text_alpha: 0.6,
        weak_text_color: None,
        widgets: widgets_dark(),
        selection: selection_dark(),
        hyperlink_color: COLOR_PRIMARY_ACCENT,
        faint_bg_color: Color32::from_additive_luminance(5), // visible, but barely so
        extreme_bg_color: Color32::from_white_alpha(40),     // e.g. TextEdit background
        text_edit_bg_color: Some(Color32::TRANSPARENT),      // use `extreme_bg_color` by default
        code_bg_color: Color32::from_gray(64),
        warn_fg_color: COLOR_WARNING, // orange
        error_fg_color: COLOR_DANGER, // red

        window_corner_radius: CornerRadius::same(6),
        window_shadow: Shadow {
            offset: [10, 20],
            blur: 15,
            spread: 0,
            color: Color32::from_black_alpha(96),
        },
        window_fill: Color32::from_gray(45), // Color32::from_gray(230)
        window_stroke: Stroke::new(STROKE_WIDTH_NORMAL, Color32::from_gray(60)),
        window_highlight_topmost: true,

        menu_corner_radius: CornerRadius::same(6),

        panel_fill: Color32::from_gray(27),

        popup_shadow: Shadow {
            offset: [6, 10],
            blur: 8,
            spread: 0,
            color: Color32::from_black_alpha(96),
        },

        resize_corner_size: 12.0,

        text_cursor: text_cursor_style_dark(),

        clip_rect_margin: 3.0, // should be at least half the size of the widest frame stroke + max WidgetVisuals::expansion
        button_frame: true,
        collapsing_header_frame: false,
        indent_has_left_vline: true,

        striped: false,

        slider_trailing_fill: true,
        handle_shape: HandleShape::Circle,

        interact_cursor: None,

        image_loading_spinners: true,

        numeric_color_space: NumericColorSpace::GammaByte,
        disabled_alpha: 0.5,
    }
}

/// Default light theme - with defaults from dark theme.
fn visuals_light_with_dark(dark: &Visuals) -> Visuals {
    Visuals {
        dark_mode: false,
        text_options: TextOptions {
            alpha_from_coverage: AlphaFromCoverage::LIGHT_MODE_DEFAULT,
            ..Default::default()
        },
        override_text_color: Some(color_text_dark()),
        widgets: widgets_light_with_dark(&dark.widgets),
        selection: selection_light_with_dark(&dark.selection),
        //hyperlink_color: COLOR_PRIMARY_ACCENT,
        faint_bg_color: Color32::from_additive_luminance(5), // visible, but barely so
        extreme_bg_color: Color32::from_black_alpha(40),     // e.g. TextEdit background
        code_bg_color: Color32::from_gray(230),
        //warn_fg_color: Color32::from_rgb(255, 100, 0), // slightly orange red. it's difficult to find a warning color that pops on bright background.
        //error_fg_color: Color32::from_rgb(255, 0, 0),  // red
        window_shadow: Shadow {
            offset: [10, 20],
            blur: 15,
            spread: 0,
            color: Color32::from_black_alpha(25),
        },
        window_fill: COLOR_PRIMARY_BG,
        window_stroke: Stroke::new(1.0, Color32::from_gray(190)),

        panel_fill: Color32::from_gray(248),

        popup_shadow: Shadow {
            offset: [6, 10],
            blur: 8,
            spread: 0,
            color: Color32::from_black_alpha(25),
        },

        text_cursor: text_cursor_style_light_with_dark(&dark.text_cursor),

        ..*dark
    }
}

fn selection_dark() -> Selection {
    Selection {
        // Background color behind selected text and other selectable buttons.
        bg_fill: COLOR_PRIMARY_ACCENT,
        // Color of selected text.
        stroke: Stroke::new(STROKE_WIDTH_NORMAL, Color32::WHITE),
    }
}

fn selection_light_with_dark(dark: &Selection) -> Selection {
    Selection {
        bg_fill: dark.bg_fill,
        stroke: Stroke::new(dark.stroke.width, Color32::BLACK),
    }
}

fn text_cursor_style_dark() -> TextCursorStyle {
    TextCursorStyle {
        stroke: Stroke::new(STROKE_WIDTH_NORMAL, Color32::WHITE),
        preview: false,
        blink: true,
        on_duration: 0.5,
        off_duration: 0.5,
    }
}

fn text_cursor_style_light_with_dark(dark: &TextCursorStyle) -> TextCursorStyle {
    TextCursorStyle {
        stroke: Stroke::new(dark.stroke.width, Color32::BLACK),
        ..*dark
    }
}

fn widgets_dark() -> Widgets {
    Widgets {
        noninteractive: WidgetVisuals {
            weak_bg_fill: Color32::from_gray(27),
            bg_fill: Color32::from_gray(27),
            bg_stroke: Stroke::new(STROKE_WIDTH_NORMAL, Color32::from_gray(90)), // separators, indentation lines
            fg_stroke: Stroke::new(STROKE_WIDTH_NORMAL, Color32::from_gray(140)), // normal text color
            corner_radius: CornerRadius::same(2),
            expansion: 0.0,
        },
        inactive: WidgetVisuals {
            weak_bg_fill: Color32::from_gray(60), // button background
            bg_fill: Color32::from_gray(60),      // checkbox background
            bg_stroke: Stroke::new(STROKE_WIDTH_NORMAL, Color32::from_gray(150)),
            fg_stroke: Stroke::new(STROKE_WIDTH_NORMAL, Color32::from_gray(180)), // button text
            corner_radius: CornerRadius::same(2),
            expansion: 0.0,
        },
        hovered: WidgetVisuals {
            weak_bg_fill: Color32::from_gray(70),
            bg_fill: Color32::from_gray(70),
            bg_stroke: Stroke::new(STROKE_WIDTH_NORMAL, Color32::from_gray(180)), // e.g. hover over window edge or button
            fg_stroke: Stroke::new(STROKE_WIDTH_NORMAL, Color32::from_gray(240)),
            corner_radius: CornerRadius::same(3),
            expansion: 0.0,
        },
        active: WidgetVisuals {
            weak_bg_fill: Color32::from_gray(55),
            bg_fill: Color32::from_gray(55),
            bg_stroke: Stroke::new(STROKE_WIDTH_NORMAL, Color32::WHITE),
            fg_stroke: Stroke::new(STROKE_WIDTH_NORMAL, Color32::WHITE),
            corner_radius: CornerRadius::same(2),
            expansion: 0.0,
        },
        open: WidgetVisuals {
            weak_bg_fill: Color32::from_gray(45),
            bg_fill: Color32::from_gray(27),
            bg_stroke: Stroke::new(STROKE_WIDTH_NORMAL, Color32::from_gray(60)),
            fg_stroke: Stroke::new(STROKE_WIDTH_NORMAL, Color32::from_gray(210)),
            corner_radius: CornerRadius::same(2),
            expansion: 0.0,
        },
    }
}

fn widgets_light_with_dark(dark: &Widgets) -> Widgets {
    Widgets {
        noninteractive: WidgetVisuals {
            weak_bg_fill: Color32::from_gray(248),
            bg_fill: Color32::from_gray(248),
            bg_stroke: Stroke::new(dark.noninteractive.bg_stroke.width, Color32::from_gray(190)), // separators, indentation lines
            fg_stroke: Stroke::new(dark.noninteractive.fg_stroke.width, Color32::from_gray(80)), // normal text color
            corner_radius: dark.noninteractive.corner_radius,
            expansion: dark.noninteractive.expansion,
        },
        inactive: WidgetVisuals {
            weak_bg_fill: Color32::from_gray(230), // button background
            bg_fill: Color32::from_gray(230),      // checkbox background
            bg_stroke: dark.inactive.bg_stroke,
            fg_stroke: Stroke::new(dark.inactive.fg_stroke.width, Color32::from_gray(60)), // button text
            corner_radius: dark.inactive.corner_radius,
            expansion: dark.inactive.expansion,
        },
        hovered: WidgetVisuals {
            weak_bg_fill: Color32::from_gray(220),
            bg_fill: Color32::from_gray(220),
            bg_stroke: Stroke::new(dark.hovered.bg_stroke.width, Color32::from_gray(105)), // e.g. hover over window edge or button
            fg_stroke: Stroke::new(dark.hovered.fg_stroke.width, Color32::BLACK),
            corner_radius: dark.hovered.corner_radius,
            expansion: dark.hovered.expansion,
        },
        active: WidgetVisuals {
            weak_bg_fill: Color32::from_gray(165),
            bg_fill: Color32::from_gray(165),
            bg_stroke: Stroke::new(dark.active.bg_stroke.width, Color32::BLACK),
            fg_stroke: Stroke::new(dark.active.fg_stroke.width, Color32::BLACK),
            corner_radius: dark.active.corner_radius,
            expansion: dark.active.expansion,
        },
        open: WidgetVisuals {
            weak_bg_fill: Color32::from_gray(220),
            bg_fill: Color32::from_gray(220),
            bg_stroke: Stroke::new(dark.open.bg_stroke.width, Color32::from_gray(160)),
            fg_stroke: Stroke::new(dark.open.fg_stroke.width, Color32::BLACK),
            corner_radius: dark.open.corner_radius,
            expansion: dark.open.expansion,
        },
    }
}
