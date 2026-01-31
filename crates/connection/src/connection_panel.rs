use gpui::*;
use gpui_component::{
    ActiveTheme, IconName, StyledExt, WindowExt,
    button::{Button, ButtonVariants},
    h_flex,
    scroll::ScrollableElement,
};
use ui::{icon_btn_interactive, svg_icon};

use crate::connection_dialog::ConnectionDialog;

use super::{ConnStatus, HomePage};

pub fn connection_panel(view: &mut HomePage, cx: &mut Context<HomePage>) -> impl IntoElement {
    let theme = cx.theme();
    div()
        .w(px(280.0))
        .bg(theme.sidebar)
        .border_r_1()
        .border_color(theme.border)
        .flex()
        .flex_col()
        .child(
            div()
                .flex_1()
                .flex()
                .flex_col()
                .child(if view.items.is_empty() {
                    div()
                        .flex_1()
                        .flex()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .p_6()
                        .child(
                            div()
                                .size(px(48.0))
                                .mb_4()
                                .text_color(theme.muted_foreground)
                                .opacity(0.4)
                                .child(svg_icon("icons/monitor.svg", 48.0, theme.muted_foreground)),
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(theme.muted_foreground)
                                .text_center()
                                .child("还没有保存的连接"),
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(theme.muted_foreground)
                                .text_center()
                                .child("点击下方 + 添加"),
                        )
                        .into_any_element()
                } else {
                    div()
                        .overflow_y_scrollbar()
                        .p_2()
                        .children(view.items.iter().map(|item| {
                            let status_color = match item.status {
                                ConnStatus::Connected => theme.success,
                                ConnStatus::Warning => theme.warning,
                                ConnStatus::Disconnected => theme.danger,
                            };
                            div()
                                .flex()
                                .items_center()
                                .gap_3()
                                .px_3()
                                .py_2()
                                .mb_1()
                                .rounded(px(6.0))
                                .child(div().size(px(8.0)).bg(status_color).rounded(px(999.0)))
                                .child(
                                    div()
                                        .flex_1()
                                        .min_w_0()
                                        .child(
                                            div()
                                                .text_color(theme.foreground)
                                                .text_sm()
                                                .font_medium()
                                                .truncate()
                                                .child(item.name.clone()),
                                        )
                                        .child(
                                            div()
                                                .text_color(theme.secondary_foreground)
                                                .text_xs()
                                                .truncate()
                                                .child(item.host.clone()),
                                        ),
                                )
                        }))
                        .into_any_element()
                }),
        )
        .child(
            div()
                .px_3()
                .py_2()
                .flex()
                .items_center()
                .gap_2()
                .border_t_1()
                .border_color(theme.border)
                .bg(theme.sidebar)
                .child(
                    div()
                        .id("add-connection")
                        .cursor_pointer()
                        .child(icon_btn_interactive(
                            "add-btn",
                            IconName::Plus,
                            theme.muted_foreground,
                            theme.danger,
                        ))
                        .on_click(cx.listener(|_, _, window, cx| {
                            let dialog_view = cx.new(|cx| ConnectionDialog::new(window, cx));
                            dialog_view.update(cx, |dialog, cx| {
                                dialog.active_tab = 0;
                                cx.notify();
                            });
                            window.open_dialog(cx, move |dialog, window, _cx| {
                                let view_height = window.viewport_size().height;
                                let mut dialog_height = px(600.);
                                if dialog_height > view_height {
                                    dialog_height = view_height;
                                }
                                let margin_top = (view_height - dialog_height) / 2.;

                                dialog
                                    .w(px(800.))
                                    .h(dialog_height)
                                    .margin_top(margin_top)
                                    .title("新建连接")
                                    .child(dialog_view.clone())
                                    .footer(|_, _, _, _| {
                                        vec![
                                            h_flex()
                                                .w_full()
                                                .justify_between()
                                                .child(
                                                    Button::new("test")
                                                        .success()
                                                        .cursor_pointer()
                                                        .outline()
                                                        .label("测试连接"),
                                                )
                                                .child(
                                                    h_flex()
                                                        .gap_2()
                                                        .child(
                                                            Button::new("cancel")
                                                                .outline()
                                                                .cursor_pointer()
                                                                .label("取消")
                                                                .on_click(|_, window, cx| {
                                                                    window.close_dialog(cx);
                                                                }),
                                                        )
                                                        .child(
                                                            Button::new("confirm")
                                                                .primary()
                                                                .cursor_pointer()
                                                                .outline()
                                                                .label("确认")
                                                                .on_click(|_, window, cx| {
                                                                    window.close_dialog(cx);
                                                                }),
                                                        ),
                                                )
                                                .into_any_element(),
                                        ]
                                    })
                            });
                        })),
                )
                .child(icon_btn_interactive(
                    "folder-btn",
                    "icons/folder.svg",
                    theme.muted_foreground,
                    theme.danger,
                ))
                .child(
                    div()
                        .flex_1()
                        .flex()
                        .items_center()
                        .gap_2()
                        .px_2()
                        .py_1()
                        .bg(theme.sidebar)
                        .border_1()
                        .border_color(theme.border)
                        .rounded(px(4.0))
                        .child(svg_icon(IconName::Search, 14.0, theme.muted_foreground))
                        .child(
                            div()
                                .text_color(theme.muted_foreground)
                                .text_xs()
                                .child("筛选"),
                        ),
                )
                .child(icon_btn_interactive(
                    "more-btn",
                    IconName::EllipsisVertical,
                    theme.muted_foreground,
                    theme.danger,
                )),
        )
}
