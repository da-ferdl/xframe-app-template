use crate::app::theme::EguiThemeExt;
use xframe::egui::{self, Align, Id};

const SIDEBAR_WIDTH: f32 = 90.;
const MAIN_OUTER_MARGIN: i8 = 3;

pub fn scaffold_desktop_page(ui: &mut egui::Ui, page_content: impl FnOnce(&mut egui::Ui)) {
    let theme = ui.theme();

    let side_bar_width = get_mut_side_bar_state(ui.ctx()).current_width;

    egui::Frame::NONE
        .outer_margin(egui::Margin {
            left: side_bar_width as i8,
            right: MAIN_OUTER_MARGIN,
            top: 0,
            bottom: MAIN_OUTER_MARGIN,
        })
        .inner_margin(egui::Margin {
            left: 0,
            right: 0,
            top: 0,
            bottom: MAIN_OUTER_MARGIN,
        })
        .fill(theme.color_background_page())
        .corner_radius(egui::CornerRadius {
            nw: 0,
            ne: 0,
            sw: 8,
            se: 8,
        })
        .shadow(egui::Shadow {
            offset: [0, 0],
            blur: 5,
            spread: 2,
            color: egui::Color32::from_black_alpha(50),
        })
        .show(ui, |ui| {
            ui.set_height(ui.available_height());

            ui.vertical(|ui| {
                page_content(ui);
            });
        });
}

pub fn scaffold_desktop_layout(
    ui: &mut egui::Ui,
    top_bar_content: impl FnOnce(&mut egui::Ui, &mut SideBarState),
    side_bar_content: impl FnOnce(&mut egui::Ui),
    main_content: impl FnOnce(&mut egui::Ui),
) {
    let theme = ui.theme();
    let top_bar_height = crate::ui_state!(|| 0_f32);

    ui.with_layout(egui::Layout::bottom_up(Align::Min), |ui| {
        ui.vertical(|ui| {
            ui.add_space(*top_bar_height);
            ui.with_layout(egui::Layout::left_to_right(Align::Min), |ui| {
                ui.vertical(|ui| {
                    egui::Frame::new()
                        .fill(theme.color_background_top_bar())
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            ui.set_height(ui.available_height());

                            if get_mut_side_bar_state(ui.ctx()).show_side_bar_content() {
                                side_bar_content(ui);
                            }
                        });
                });
                main_content(ui);
            });
        });

        let next_top_bar_height = top_bar_frame(ui, top_bar_content);

        if *top_bar_height != next_top_bar_height {
            *top_bar_height = next_top_bar_height;

            if !ui.ctx().will_discard() {
                ui.ctx().request_discard("set top-bar height sizing pass");
            }
        }
    });
}

fn top_bar_frame(
    ui: &mut egui::Ui,
    top_bar_content: impl FnOnce(&mut egui::Ui, &mut SideBarState),
) -> f32 {
    let theme = ui.theme();

    let height = egui::Frame::new()
        .shadow(egui::Shadow {
            offset: [0, 0],
            blur: 3,
            spread: 2,
            color: egui::Color32::from_black_alpha(50),
        })
        .fill(theme.color_background_top_bar())
        .inner_margin(egui::Margin {
            left: 70,
            right: 10,
            top: 0,
            bottom: 0,
        })
        .show(ui, |ui| {
            ui.vertical(|ui| {
                ui.set_width(ui.available_width());
                top_bar_content(ui, get_mut_side_bar_state(ui.ctx()));
            });
        })
        .response
        .rect
        .height();

    if height < 0.0 {
        return 0.;
    }

    height
}

pub struct SideBarState {
    id: egui::Id,
    egui_ctx: egui::Context,
    anim_dir: AnimDir,
    current_width: f32,
}
impl SideBarState {
    fn new(egui_ctx: &egui::Context) -> Self {
        let id = Id::new(crate::unique_int!());
        let current_width = MAIN_OUTER_MARGIN as f32;

        egui_ctx.animate_value_with_time(id, current_width, 0.0);

        Self {
            id,
            egui_ctx: egui_ctx.clone(),
            current_width: current_width,
            anim_dir: AnimDir::Close,
        }
    }

    /// Toggle open / close
    pub fn toggle(&mut self) {
        // Switch animation direction.
        self.anim_dir = match self.anim_dir {
            AnimDir::Open => AnimDir::Close,
            AnimDir::Close => AnimDir::Open,
        };

        // To ensure smooth transitions with different times, we need to clear the
        // animation manager and then provide it the current value so it knows where to
        // animate from.
        self.egui_ctx.clear_animations();
        self.egui_ctx
            .animate_value_with_time(self.id, self.current_width, 0.0);
        self.egui_ctx.request_repaint();
    }

    // Handles SideBar open / close animation if needed.
    // Performs the open or close transition if required.
    pub fn handle_animation(&mut self) {
        let anim_time = 0.15_f32;

        if self.anim_dir == AnimDir::Open && self.current_width != SIDEBAR_WIDTH {
            self.current_width =
                self.egui_ctx
                    .animate_value_with_time(self.id, SIDEBAR_WIDTH, anim_time);
        } else if self.anim_dir == AnimDir::Close
            && self.current_width != (MAIN_OUTER_MARGIN as f32)
        {
            self.current_width =
                self.egui_ctx
                    .animate_value_with_time(self.id, MAIN_OUTER_MARGIN as f32, anim_time);
        }
    }

    pub fn show_side_bar_content(&self) -> bool {
        self.current_width != (MAIN_OUTER_MARGIN as f32)
    }

    /// Returns `true` if the side-bar panel is opening or open, otherwise `false`.
    pub fn is_opening_or_open(&self) -> bool {
        self.anim_dir == AnimDir::Open
    }
}

fn get_mut_side_bar_state<'s>(egui_ctx: &egui::Context) -> &'s mut SideBarState {
    static mut SIDE_BAR_STATE: Option<SideBarState> = None;

    #[expect(static_mut_refs)]
    match unsafe { &mut SIDE_BAR_STATE } {
        Some(s) => s,
        None => unsafe { &mut SIDE_BAR_STATE }.insert(SideBarState::new(egui_ctx)),
    }
}

#[derive(PartialEq)]
enum AnimDir {
    Open,
    Close,
}
