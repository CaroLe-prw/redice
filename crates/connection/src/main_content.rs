use gpui::*;
use gpui_component::ActiveTheme;
use ui::{feature_item, quick_action_card};

fn open_url(url: &str) {
    let _ = open::that(url);
}

pub fn main_content(cx: &App) -> impl IntoElement {
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
