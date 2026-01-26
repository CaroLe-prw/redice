use gpui::*;

use crate::components::{connection_panel, main_content};

#[derive(Clone, Copy)]
pub(crate) enum ConnStatus {
    Connected,
    Warning,
    Disconnected,
}

#[derive(Clone)]
pub(crate) struct ConnectionItem {
    pub(crate) name: String,
    pub(crate) host: String,
    pub(crate) status: ConnStatus,
}

pub(crate) struct HomePage {
    pub(crate) items: Vec<ConnectionItem>,
}

impl Render for HomePage {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .child(connection_panel(self, cx))
            .child(main_content(cx))
    }
}

impl HomePage {
    pub(crate) fn new() -> Self {
        Self { items: vec![] }
    }
}
