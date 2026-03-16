use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};
use xframe::egui;

struct Inner {
    interactive_rect_list: Vec<egui::Rect>,
    interactive_on_last_pass: bool,
    last_was_interactive: bool,
}

pub struct InteractRectHandler {
    egui_ctx: egui::Context,
    inner: RefCell<Inner>,
    // disable the rect handler for tests or debug
    disabled: bool,
}
impl InteractRectHandler {
    pub fn new(egui_ctx: egui::Context) -> Rc<Self> {
        Self {
            egui_ctx,
            inner: RefCell::new(Inner {
                interactive_rect_list: vec![],
                interactive_on_last_pass: false,
                last_was_interactive: false,
            }),
            disabled: true,
        }
        .into()
    }

    pub fn ignore_touch(&self, next_tl_x: f64, next_tl_y: f64) -> bool {
        if self.disabled {
            return false;
        }

        let mut this = self.inner.borrow_mut();

        if self.position_is_on_interactive_area(&mut this, next_tl_x, next_tl_y) {
            this.last_was_interactive = true;
            return false;
        }

        if this.last_was_interactive {
            this.last_was_interactive = false;
            return false;
        }

        true
    }

    pub fn clear_interactive_rect_list(&self) {
        if self.disabled {
            return;
        }

        /*println!(
            "## last interactive rects: {}",
            self.inner.borrow().interactive_rect_list.len()
        );*/
        self.inner.borrow_mut().interactive_rect_list.clear();
    }

    pub fn set_interactive_on_last_pass(&self) {
        if self.disabled {
            return;
        }

        self.inner.borrow_mut().interactive_on_last_pass = true;
    }

    pub fn push_rect_if_not_set(&self, rect: egui::Rect) {
        if self.disabled {
            return;
        }

        let mut this = self.inner.borrow_mut();

        if !this.interactive_rect_list.contains(&rect) {
            this.interactive_rect_list.push(rect);
        }
    }

    fn position_is_on_interactive_area(
        &self,
        inner: &mut RefMut<'_, Inner>,
        position_x: f64,
        position_y: f64,
    ) -> bool {
        //let mut this = self.inner.borrow_mut();

        let interactive_on_last_pass = inner.interactive_on_last_pass;
        inner.interactive_on_last_pass = false;

        if
        /*wants_keyboard_input ||*/
        interactive_on_last_pass {
            return true;
        }

        // x-position distance from the left side which must be not ignored
        // for 'swipe' starting at the left side.
        let swipe_back_edge_width: f32 = 50.0;

        // x-position distance from the right side which must be not ignored
        // for 'swipe' starting at the right side.
        let swipe_forward_edge_width: f32 = 50.0;

        //let interactive_areas = self.interactive_rect_list.drain(..);

        let egui_ctx = &self.egui_ctx;

        let ppp = egui_ctx.pixels_per_point();
        let pos_x = (position_x / ppp as f64) as f32;
        let pos_y = (position_y / ppp as f64) as f32;

        //println!("--- pos_x: {pos_x}, pos_y: {pos_y} ---");

        if egui_ctx.dragged_id().is_some() {
            return true;
        }

        let right_swipe_area_start_point =
            egui_ctx.viewport_rect().max.x - swipe_forward_edge_width;
        if (pos_x <= swipe_back_edge_width) || (pos_x >= right_swipe_area_start_point) {
            //println!("--- position is in swipe area");
            return true;
        }

        for v in &inner.interactive_rect_list {
            //
            let min_x = v.min.x;
            let min_y = v.min.y;

            let max_x = v.max.x;
            let max_y = v.max.y;

            //println!("min_x: {min_x}, max_x: {max_x}");
            //println!("min_y: {min_y}, max_y: {max_y}");

            let is_inside_x_range = min_x <= pos_x && max_x >= pos_x;
            let is_inside_y_range = min_y <= pos_y && max_y >= pos_y;

            if is_inside_x_range && is_inside_y_range {
                return true;
            }
        }

        false
    }
}
