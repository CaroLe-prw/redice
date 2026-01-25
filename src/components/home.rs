use gpui::*;
use gpui_component::{IconName, StyledExt, scroll::ScrollableElement};

use crate::{
    HomeShell,
    ui::{
        ACCENT_BLUE, ACCENT_PURPLE, ACCENT_PURPLE_BG, ACCENT_RED, ACCENT_RED_BG, ACCENT_TEAL,
        ACCENT_TEAL_BG, BG_DEEP, BG_ELEVATED, BG_PANEL, BORDER, TEXT_BRIGHT, TEXT_MUTED,
        TEXT_PRIMARY, TEXT_SECONDARY, feature_item, footer_icon_btn, quick_action_card,
        sidebar_divider, sidebar_icon_btn, svg_icon,
    },
};

fn open_url(url: &str) {
    let _ = open::that(url);
}

pub fn connection_panel(view: &mut HomeShell, _cx: &mut Context<HomeShell>) -> impl IntoElement {
    div()
        .w(px(280.0))
        .bg(rgb(BG_DEEP))
        .border_r_1()
        .border_color(rgb(BORDER))
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
                                .text_color(rgb(TEXT_MUTED))
                                .opacity(0.4)
                                .child(svg_icon("icons/monitor.svg", 48.0, TEXT_MUTED)),
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(rgb(TEXT_MUTED))
                                .text_center()
                                .child("还没有保存的连接"),
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(rgb(TEXT_MUTED))
                                .text_center()
                                .child("点击下方 + 添加"),
                        )
                        .into_any_element()
                } else {
                    div()
                        .overflow_y_scrollbar()
                        .p_2()
                        .children(view.items.iter().map(|item| {
                            div()
                                .flex()
                                .items_center()
                                .gap_3()
                                .px_3()
                                .py_2()
                                .mb_1()
                                .rounded(px(6.0))
                                .child(
                                    div()
                                        .size(px(8.0))
                                        .bg(rgb(item.status.color()))
                                        .rounded(px(999.0)),
                                )
                                .child(
                                    div()
                                        .flex_1()
                                        .min_w_0()
                                        .child(
                                            div()
                                                .text_color(rgb(TEXT_PRIMARY))
                                                .text_sm()
                                                .font_medium()
                                                .truncate()
                                                .child(item.name.clone()),
                                        )
                                        .child(
                                            div()
                                                .text_color(rgb(TEXT_SECONDARY))
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
                .border_color(rgb(BORDER))
                .bg(rgb(BG_PANEL))
                .child(footer_icon_btn(IconName::Plus, TEXT_MUTED))
                .child(footer_icon_btn("icons/folder.svg", TEXT_MUTED))
                .child(
                    div()
                        .flex_1()
                        .flex()
                        .items_center()
                        .gap_2()
                        .px_2()
                        .py_1()
                        .bg(rgb(BG_DEEP))
                        .border_1()
                        .border_color(rgb(BORDER))
                        .rounded(px(4.0))
                        .child(svg_icon(IconName::Search, 14.0, TEXT_MUTED))
                        .child(div().text_color(rgb(TEXT_MUTED)).text_xs().child("筛选")),
                )
                .child(footer_icon_btn(IconName::EllipsisVertical, TEXT_MUTED)),
        )
}

pub fn icon_sidebar(active_index: usize, cx: &mut Context<HomeShell>) -> impl IntoElement {
    div()
        .w(px(48.0))
        .bg(rgb(BG_DEEP))
        .border_r_1()
        .border_color(rgb(BORDER))
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
                        .cursor_pointer()
                        .child(sidebar_icon_btn(
                            active_index == 0,
                            "icons/monitor.svg",
                            BG_ELEVATED,
                            ACCENT_RED,
                            TEXT_MUTED,
                        ))
                        .on_click(cx.listener(|view, _, _, _| {
                            view.sidebar_active = 0;
                        })),
                )
                .child(
                    div()
                        .id("sidebar-history")
                        .cursor_pointer()
                        .child(sidebar_icon_btn(
                            active_index == 1,
                            "icons/clock.svg",
                            BG_ELEVATED,
                            ACCENT_RED,
                            TEXT_MUTED,
                        ))
                        .on_click(cx.listener(|view, _, _, _| {
                            view.sidebar_active = 1;
                        })),
                ),
        )
        .child(div().flex_1())
        .child(div().child(sidebar_icon_btn(
            false,
            IconName::Settings,
            BG_ELEVATED,
            ACCENT_RED,
            TEXT_MUTED,
        )))
        .child(sidebar_divider())
        .child(
            div()
                .id("sidebar-github")
                .cursor_pointer()
                .child(sidebar_icon_btn(
                    false,
                    IconName::GitHub,
                    BG_ELEVATED,
                    ACCENT_RED,
                    TEXT_MUTED,
                ))
                .on_click(|_, _, cx| {
                    cx.stop_propagation();
                    let _ = open::that("https://github.com/CaroLe-prw/redice");
                }),
        )
}
pub fn main_content_empty() -> impl IntoElement {
    div()
        .flex_1()
        .bg(rgb(BG_DEEP))
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .p_10()
        .child(
            div()
                .mb_8()
                .flex()
                .flex_col()
                .items_center()
                .child(
                    div()
                        .size(px(80.0))
                        .mb_4()
                        .child(img("icons/logo.svg").size(px(80.0)).mb_4()),
                )
                .child(
                    div()
                        .text_3xl()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(TEXT_BRIGHT))
                        .mb_2()
                        .child("REDICE"),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(TEXT_SECONDARY))
                        .child("现代化的 Redis 桌面客户端"),
                ),
        )
        .child(quick_actions())
        .child(features())
        .child(welcome_footer())
}

fn quick_actions() -> impl IntoElement {
    div()
        .flex()
        .gap_4()
        .mb_12()
        .child(quick_action_card(
            "icons/plus.svg",
            ACCENT_RED,
            ACCENT_RED_BG,
            "新建连接",
            "添加 Redis 服务器",
        ))
        .child(quick_action_card(
            "icons/import.svg",
            ACCENT_TEAL,
            ACCENT_TEAL_BG,
            "导入配置",
            "从文件导入连接",
        ))
        .child(quick_action_card(
            "icons/book.svg",
            ACCENT_PURPLE,
            ACCENT_PURPLE_BG,
            "使用文档",
            "查看帮助文档",
        ))
}

fn features() -> impl IntoElement {
    div()
        .flex()
        .gap_8()
        .mb_12()
        .child(feature_item(
            "icons/key.svg",
            "键值浏览",
            "树形结构浏览，支持所有 Redis 数据类型",
        ))
        .child(feature_item(
            "icons/chart.svg",
            "实时监控",
            "服务器状态、内存、命令统计",
        ))
        .child(feature_item(
            "icons/terminal.svg",
            "命令行",
            "内置 CLI，支持命令自动补全",
        ))
        .child(feature_item(
            "icons/lock.svg",
            "安全连接",
            "支持 SSL/TLS、SSH 隧道",
        ))
}

fn welcome_footer() -> impl IntoElement {
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
                        .text_color(rgb(ACCENT_BLUE))
                        .cursor_pointer()
                        .child("GitHub")
                        .on_click(|_, _, cx| {
                            cx.stop_propagation();
                            open_url("https://github.com/CaroLe-prw/redice");
                        }),
                )
                .child(div().text_color(rgb(TEXT_MUTED)).child("·"))
                .child(
                    div()
                        .id("link-issues")
                        .text_color(rgb(ACCENT_BLUE))
                        .cursor_pointer()
                        .child("反馈问题")
                        .on_click(|_, _, cx| {
                            cx.stop_propagation();
                            open_url("https://github.com/CaroLe-prw/redice/issues");
                        }),
                )
                .child(div().text_color(rgb(TEXT_MUTED)).child("·"))
                .child(
                    div()
                        .id("link-changelog")
                        .text_color(rgb(ACCENT_BLUE))
                        .cursor_pointer()
                        .child("更新日志")
                        .on_click(|_, _, cx| {
                            cx.stop_propagation();
                            open_url("https://github.com/CaroLe-prw/redice/releases");
                        }),
                ),
        )
        .child(div().text_xs().text_color(rgb(TEXT_MUTED)).child("v0.1.0"))
}
