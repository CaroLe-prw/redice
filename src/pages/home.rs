use gpui::*;

use crate::components::{connection_panel, main_content};

const STATUS_CONNECTED: u32 = 0x3FB950;
const STATUS_WARNING: u32 = 0xF0883E;
const STATUS_ERROR: u32 = 0xF85149;

#[derive(Clone, Copy)]
pub(crate) enum ConnStatus {
    Connected,
    Warning,
    Disconnected,
}

impl ConnStatus {
    pub(crate) fn color(self) -> u32 {
        match self {
            ConnStatus::Connected => STATUS_CONNECTED,
            ConnStatus::Warning => STATUS_WARNING,
            ConnStatus::Disconnected => STATUS_ERROR,
        }
    }
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
            .child(main_content())
    }
}

impl HomePage {
    pub(crate) fn new() -> Self {
        Self { items: vec![] }
    }
}
