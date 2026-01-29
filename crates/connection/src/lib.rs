//! Browser view for Redice - connection list and key browser

mod connection_panel;
mod main_content;

use gpui::*;

pub use connection_panel::connection_panel;
pub use main_content::main_content;

#[derive(Clone, Copy)]
pub enum ConnStatus {
    Connected,
    Warning,
    Disconnected,
}

#[derive(Clone)]
pub struct ConnectionItem {
    pub name: String,
    pub host: String,
    pub status: ConnStatus,
}

pub struct HomePage {
    pub items: Vec<ConnectionItem>,
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
    pub fn new() -> Self {
        Self { items: vec![] }
    }
}

impl Default for HomePage {
    fn default() -> Self {
        Self::new()
    }
}
