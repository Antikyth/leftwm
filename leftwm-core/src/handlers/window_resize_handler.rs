use super::{Manager, Window, WindowHandle};
use crate::config::Config;
use crate::display_servers::DisplayServer;

impl<C: Config, SERVER: DisplayServer> Manager<C, SERVER> {
    pub fn window_resize_handler(
        &mut self,
        handle: &WindowHandle,
        offset_w: i32,
        offset_h: i32,
    ) -> bool {
        if let Some(w) = self.state.windows.iter_mut().find(|w| &w.handle == handle) {
            process_window(w, offset_w, offset_h);
            return true;
        }
        false
    }
}

fn process_window(window: &mut Window, offset_w: i32, offset_h: i32) {
    window.set_floating(true);
    let mut offset = window.get_floating_offsets().unwrap_or_default();
    let start = window.start_loc.unwrap_or_default();
    //offset.clear_minmax();
    offset.set_w(start.w() + offset_w);
    offset.set_h(start.h() + offset_h);
    window.set_floating_offsets(Some(offset));
}
