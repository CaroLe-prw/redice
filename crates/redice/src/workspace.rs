//! Main workspace layout

use connection::HomePage;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{ActiveTheme, IconName, Root, TitleBar};
use history::HistoryPage;
use ui::sidebar_icon_btn;

pub struct HomeShell {
    sidebar_active: usize,
    home_page: Entity<HomePage>,
    history_page: Entity<HistoryPage>,
}

impl Render for HomeShell {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let sidebar_active = self.sidebar_active;
        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(cx.theme().sidebar)
            .child(TitleBar::new().bg(cx.theme().sidebar))
            .child(
                div()
                    .flex_1()
                    .flex()
                    .overflow_hidden()
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
                    ),
            )
            .children(Root::render_dialog_layer(window, cx))
    }
}

impl HomeShell {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            sidebar_active: 0,
            home_page: cx.new(|_| HomePage::new()),
            history_page: cx.new(|cx| HistoryPage::new(window, cx)),
        }
    }

    pub fn set_active_page(&mut self, index: usize, cx: &mut Context<Self>) {
        if self.sidebar_active == index {
            return;
        }
        self.sidebar_active = index;
        cx.notify();
    }
}

fn icon_sidebar(active_index: usize, cx: &mut Context<HomeShell>) -> impl IntoElement {
    let theme = cx.theme();
    div()
        .w(px(48.0))
        .bg(theme.sidebar)
        .border_r_1()
        .border_color(theme.border)
        .flex()
        .flex_col()
        .items_center()
        .py_3()
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .id("sidebar-connections")
                        .mb_2()
                        .cursor_pointer()
                        .child(sidebar_icon_btn(
                            "conn-icon",
                            active_index == 0,
                            "icons/monitor.svg",
                            theme.danger,
                            theme.muted_foreground,
                        ))
                        .on_click(cx.listener(|view, _, _, cx| {
                            view.set_active_page(0, cx);
                        })),
                )
                .child(
                    div()
                        .id("sidebar-history")
                        .cursor_pointer()
                        .child(sidebar_icon_btn(
                            "history-icon",
                            active_index == 1,
                            "icons/clock.svg",
                            theme.danger,
                            theme.muted_foreground,
                        ))
                        .on_click(cx.listener(|view, _, _, cx| {
                            view.set_active_page(1, cx);
                        })),
                ),
        )
        .child(div().flex_1())
        .child(
            div()
                .child(sidebar_icon_btn(
                    "settings-icon",
                    false,
                    IconName::Settings,
                    theme.danger,
                    theme.muted_foreground,
                ))
                .mb_2(),
        )
        .child(
            div()
                .id("sidebar-github")
                .cursor_pointer()
                .child(sidebar_icon_btn(
                    "github-icon",
                    false,
                    IconName::GitHub,
                    theme.danger,
                    theme.muted_foreground,
                ))
                .on_click(|_, _, cx| {
                    cx.stop_propagation();
                    let _ = open::that("https://github.com/CaroLe-prw/redice");
                }),
        )
}
