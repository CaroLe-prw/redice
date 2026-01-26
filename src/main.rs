mod components;
mod pages;
mod ui;
use gpui::{prelude::FluentBuilder, *};
use gpui_component::Root;

use crate::{
    components::icon_sidebar,
    pages::{HistoryPage, HomePage},
    ui::{Assets, BG_DEEP},
};

struct HomeShell {
    sidebar_active: usize,
    home_page: Entity<HomePage>,
    history_page: Entity<HistoryPage>,
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
            .child(
                div()
                    .flex_1()
                    .relative()
                    .overflow_hidden()
                    .child(
                        div()
                            .absolute()
                            .inset_0()
                            .opacity(if sidebar_active == 0 { 1.0 } else { 0.0 })
                            .child(self.home_page.clone())
                            .when(sidebar_active != 0, |el| {
                                el.child(div().absolute().inset_0())
                            }),
                    )
                    .child(
                        div()
                            .absolute()
                            .inset_0()
                            .opacity(if sidebar_active == 1 { 1.0 } else { 0.0 })
                            .child(self.history_page.clone())
                            .when(sidebar_active != 1, |el| {
                                el.child(div().absolute().inset_0())
                            }),
                    ),
            )
    }
}

impl HomeShell {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            sidebar_active: 0,
            home_page: cx.new(|_| HomePage::new()),
            history_page: cx.new(|_| HistoryPage::new()),
        }
    }

    pub(crate) fn set_active_page(&mut self, index: usize, cx: &mut Context<Self>) {
        if self.sidebar_active == index {
            return;
        }
        self.sidebar_active = index;
        cx.notify();
    }
}

fn main() {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(HomeShell::new);
                cx.new(|cx| Root::new(view, window, cx))
            })?;
            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
