use gpui::*;
use gpui_component::{ActiveTheme, IconName};

use crate::{HomeShell, ui::sidebar_icon_btn};

pub(crate) fn icon_sidebar(active_index: usize, cx: &mut Context<HomeShell>) -> impl IntoElement {
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
                            active_index == 0,
                            "icons/monitor.svg",
                            theme.muted,
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
                            active_index == 1,
                            "icons/clock.svg",
                            theme.muted,
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
                    false,
                    IconName::Settings,
                    theme.muted,
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
                    false,
                    IconName::GitHub,
                    theme.muted,
                    theme.danger,
                    theme.muted_foreground,
                ))
                .on_click(|_, _, cx| {
                    cx.stop_propagation();
                    let _ = open::that("https://github.com/CaroLe-prw/redice");
                }),
        )
}
