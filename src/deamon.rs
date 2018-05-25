use app_state::AppState;
use std::sync::{Arc, Mutex};
use traits::LayoutScreen;
use window::WindowId;
use dom::UpdateScreen;

pub struct DeamonCallback<T: LayoutScreen> {
    callback: fn(Arc<Mutex<T>>) -> UpdateScreen,
}

impl<T: LayoutScreen> Clone for DeamonCallback<T>
{
    fn clone(&self) -> Self {
        Self { callback: self.callback.clone() }
    }
}

/// Run all currently registered deamons on an `Arc<Mutex<AppState<T>>`
pub(crate) fn run_all_deamons<T: LayoutScreen>(app_state: &mut AppState<T>) -> UpdateScreen {
    let mut should_update_screen = UpdateScreen::DontRedraw;
    for deamon in app_state.deamons.values().cloned() {
        let arc_clone = app_state.data.clone();
        let should_update = (deamon.callback)(arc_clone);
        if should_update == UpdateScreen::Redraw &&
           should_update_screen == UpdateScreen::DontRedraw {
            should_update_screen = UpdateScreen::Redraw;
        }
    }
    should_update_screen
}