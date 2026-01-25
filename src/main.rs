mod components;
mod ui;
use gpui::*;
use gpui_component::Root;

use crate::{
    components::{connection_panel, history_content_empty, icon_sidebar, main_content_empty},
    ui::{Assets, BG_DEEP},
};

const STATUS_CONNECTED: u32 = 0x3FB950;
const STATUS_WARNING: u32 = 0xF0883E;
const STATUS_ERROR: u32 = 0xF85149;

#[derive(Clone, Copy)]
enum ConnStatus {
    Connected,
    Warning,
    Disconnected,
}

impl ConnStatus {
    fn color(self) -> u32 {
        match self {
            ConnStatus::Connected => STATUS_CONNECTED,
            ConnStatus::Warning => STATUS_WARNING,
            ConnStatus::Disconnected => STATUS_ERROR,
        }
    }
}

#[derive(Clone)]
struct ConnectionItem {
    name: String,
    host: String,
    status: ConnStatus,
}

struct HomeShell {
    items: Vec<ConnectionItem>,
    sidebar_active: usize,
}

impl Render for HomeShell {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let sidebar_active = self.sidebar_active;
        div()
            .size_full()
            .flex()
            .bg(rgb(BG_DEEP))
            .child(icon_sidebar(sidebar_active, cx))
            .child(if sidebar_active == 0 {
                div()
                    .flex_1()
                    .flex()
                    .child(connection_panel(self, cx))
                    .child(main_content_empty())
                    .into_any_element()
            } else {
                history_content_empty().into_any_element()
            })
    }
}

impl HomeShell {
    fn new() -> Self {
        Self {
            items: vec![],
            sidebar_active: 0,
        }
    }
}

fn main() {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|_| HomeShell::new());
                cx.new(|cx| Root::new(view, window, cx))
            })?;
            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
