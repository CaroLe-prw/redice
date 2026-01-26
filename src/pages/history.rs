use gpui::*;

use crate::components::history_content_empty;

pub(crate) struct HistoryPage;

impl Render for HistoryPage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        history_content_empty()
    }
}

impl HistoryPage {
    pub(crate) fn new() -> Self {
        Self
    }
}
