use super::InteractRectHandler;
use std::rc::Rc;
use xframe::egui;

pub trait EguiInnerResponseExt<R> {
    fn interaction_ctx(self) -> Self;
}
impl<R> EguiInnerResponseExt<R> for egui::InnerResponse<R> {
    fn interaction_ctx(mut self) -> Self {
        self.response = self.response.interaction_ctx();
        self
    }
}

pub trait EguiResponseExt {
    fn interaction_ctx(self) -> Self;
}
impl EguiResponseExt for egui::Response {
    fn interaction_ctx(self) -> Self {
        let has_interaction = self.contains_pointer() || self.hovered() || self.has_focus();
        if has_interaction {
            rect_handler().set_interactive_on_last_pass();
        }

        //let interact_id = self.id;
        let interact_rect = self.interact_rect;
        rect_handler().push_rect_if_not_set(interact_rect);

        /*/let egui_ctx = &self.ctx;

        // Interacted indicating properties.
        let r_clicked = self.clicked();
        let r_middle_clicked = self.middle_clicked();
        let r_secondary_clicked = self.secondary_clicked();
        let r_triple_clicked = self.triple_clicked();
        let r_clicked_elsewhere = self.clicked_elsewhere();
        let r_clicked_with_open_in_background = self.clicked_with_open_in_background();
        let r_contains_pointer = self.contains_pointer();
        let r_double_clicked = self.double_clicked();
        let r_drag_started = self.drag_started();
        let r_drag_stopped = self.drag_stopped();
        let r_dragged = self.dragged();
        let r_gained_focus = self.gained_focus();
        let r_has_focus = self.has_focus(); // -
        let r_lost_focus = self.lost_focus();
        let r_hovered = self.hovered(); // -
        let r_is_pointer_button_down_on = self.is_pointer_button_down_on(); // -
        let r_long_touched = self.long_touched();

        println!("----- Interaction Info -----");
        println!("- contains_pointer: {r_contains_pointer}");
        println!("- is_pointer_button_down_on: {r_is_pointer_button_down_on}");
        println!("- has_focus: {r_has_focus}");
        println!("- hovered: {r_hovered}");
        println!("- clicked: {r_clicked}");
        println!("- double_clicked: {r_double_clicked}");
        println!("- long_touched: {r_long_touched}");
        println!("- clicked_elsewhere: {r_clicked_elsewhere}");
        println!("      ---------");
        println!("- gained_focus: {r_gained_focus}");
        println!("- lost_focus: {r_lost_focus}");
        println!("- middle_clicked: {r_middle_clicked}");
        println!("- secondary_clicked: {r_secondary_clicked}");
        println!("- triple_clicked: {r_triple_clicked}");
        println!("- clicked_with_open_in_background: {r_clicked_with_open_in_background}");
        println!("- dragged: {r_dragged}");
        println!("- drag_started: {r_drag_started}");
        println!("- drag_stopped: {r_drag_stopped}");
        println!("----------------------------");
        */

        self
    }
}

pub fn set_egui_response_ext_interact_rect_handler(handler: Rc<InteractRectHandler>) {
    unsafe { INTERACT_RECT_HANDLER = Some(handler) };
}

fn rect_handler<'s>() -> &'s InteractRectHandler {
    #[expect(static_mut_refs)]
    unsafe { &INTERACT_RECT_HANDLER }
        .as_ref()
        .expect("InteractRectHandler must be set at this point")
}

static mut INTERACT_RECT_HANDLER: Option<Rc<InteractRectHandler>> = None;
