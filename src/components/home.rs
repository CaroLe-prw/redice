use gpui::*;
use gpui_component::{ActiveTheme, IconName, StyledExt, scroll::ScrollableElement};

use crate::{
    HomeShell,
    pages::HomePage,
    ui::{feature_item, footer_icon_btn, quick_action_card, sidebar_icon_btn, svg_icon},
};

fn open_url(url: &str) {
    let _ = open::that(url);
}

pub(crate) fn connection_panel(
    view: &mut HomePage,
    cx: &mut Context<HomePage>,
) -> impl IntoElement {
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
                                crate::pages::ConnStatus::Connected => theme.success,
                                crate::pages::ConnStatus::Warning => theme.warning,
                                crate::pages::ConnStatus::Disconnected => theme.danger,
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
                .child(footer_icon_btn(IconName::Plus, theme.muted_foreground))
                .child(footer_icon_btn("icons/folder.svg", theme.muted_foreground))
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
                .child(footer_icon_btn(
                    IconName::EllipsisVertical,
                    theme.muted_foreground,
                )),
        )
}

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

pub(crate) fn main_content(cx: &App) -> impl IntoElement {
    let theme = cx.theme();
    div()
        .flex_1()
        .bg(theme.sidebar)
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .p_10()
        .child(
            div()
                .mb_6()
                .flex()
                .flex_col()
                .items_center()
                .child(
                    div()
                        .size(px(120.0))
                        .mb_3()
                        .child(img("icons/logo.svg").size(px(120.0))),
                )
                .child(
                    div()
                        .text_3xl()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.accent_foreground)
                        .mb_2()
                        .child("REDICE"),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(theme.secondary_foreground)
                        .child("现代化的 Redis 桌面客户端"),
                ),
        )
        .child(quick_actions(cx))
        .child(features(cx))
        .child(welcome_footer(cx))
}

fn quick_actions(cx: &App) -> impl IntoElement {
    let theme = cx.theme();
    div()
        .flex()
        .gap_4()
        .mb_12()
        .child(quick_action_card(
            "icons/plus.svg",
            theme.danger,
            theme.danger.opacity(0.15),
            "新建连接",
            "添加 Redis 服务器",
            cx,
        ))
        .child(quick_action_card(
            "icons/import.svg",
            theme.success,
            theme.success.opacity(0.15),
            "导入配置",
            "从文件导入连接",
            cx,
        ))
        .child(quick_action_card(
            "icons/book.svg",
            theme.info,
            theme.info.opacity(0.15),
            "使用文档",
            "查看帮助文档",
            cx,
        ))
}

fn features(cx: &App) -> impl IntoElement {
    div()
        .flex()
        .gap_8()
        .mb_12()
        .child(feature_item(
            "icons/key.svg",
            "键值浏览",
            "树形结构浏览，支持所有 Redis 数据类型",
            cx,
        ))
        .child(feature_item(
            "icons/chart.svg",
            "实时监控",
            "服务器状态、内存、命令统计",
            cx,
        ))
        .child(feature_item(
            "icons/terminal.svg",
            "命令行",
            "内置 CLI，支持命令自动补全",
            cx,
        ))
        .child(feature_item(
            "icons/lock.svg",
            "安全连接",
            "支持 SSL/TLS、SSH 隧道",
            cx,
        ))
}

fn welcome_footer(cx: &App) -> impl IntoElement {
    let theme = cx.theme();
    div()
        .flex()
        .flex_col()
        .items_center()
        .child(
            div()
                .flex()
                .gap_2()
                .text_xs()
                .mb_2()
                .child(
                    div()
                        .id("link-github")
                        .text_color(theme.link)
                        .cursor_pointer()
                        .child("GitHub")
                        .on_click(|_, _, cx| {
                            cx.stop_propagation();
                            open_url("https://github.com/CaroLe-prw/redice");
                        }),
                )
                .child(div().text_color(theme.muted_foreground).child("·"))
                .child(
                    div()
                        .id("link-issues")
                        .text_color(theme.link)
                        .cursor_pointer()
                        .child("反馈问题")
                        .on_click(|_, _, cx| {
                            cx.stop_propagation();
                            open_url("https://github.com/CaroLe-prw/redice/issues");
                        }),
                )
                .child(div().text_color(theme.muted_foreground).child("·"))
                .child(
                    div()
                        .id("link-changelog")
                        .text_color(theme.link)
                        .cursor_pointer()
                        .child("更新日志")
                        .on_click(|_, _, cx| {
                            cx.stop_propagation();
                            open_url("https://github.com/CaroLe-prw/redice/releases");
                        }),
                ),
        )
        .child(
            div()
                .text_xs()
                .text_color(theme.muted_foreground)
                .child("v0.1.0"),
        )
}
