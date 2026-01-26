use gpui::{prelude::FluentBuilder, *};
use gpui_component::StyledExt;

use crate::ui::{
    ACCENT_RED, BG_DEEP, BG_ELEVATED, BG_PANEL, BORDER, TEXT_BRIGHT, TEXT_MUTED, TEXT_PRIMARY,
    TEXT_SECONDARY, svg_icon,
};

// 历史日志页面（空状态）
pub(crate) fn history_content_empty() -> impl IntoElement {
    div()
        .flex_1()
        .bg(rgb(BG_DEEP))
        .flex()
        .flex_col()
        .p_5()
        .overflow_hidden()
        // 标题
        .child(
            div().mb_4().child(
                div()
                    .text_lg()
                    .font_semibold()
                    .text_color(rgb(TEXT_BRIGHT))
                    .child("运行日志"),
            ),
        )
        // 筛选栏
        .child(filter_bar())
        // 表格容器
        .child(table_container())
}

// 筛选栏
fn filter_bar() -> impl IntoElement {
    div()
        .flex()
        .items_end()
        .gap_4()
        .mb_4()
        .child(filter_group("筛选服务器", filter_select()))
        .child(filter_group("筛选关键字", filter_input()))
        .child(filter_actions())
}

// 筛选组（标签 + 输入）
fn filter_group(label: &'static str, input: impl IntoElement) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .child(div().text_xs().text_color(rgb(TEXT_MUTED)).child(label))
        .child(input)
}

// 下拉选择框
fn filter_select() -> impl IntoElement {
    div()
        .w(px(240.0))
        .h(px(34.0))
        .flex()
        .items_center()
        .px_3()
        .bg(rgb(BG_PANEL))
        .border_1()
        .border_color(rgb(BORDER))
        .rounded(px(4.0))
        .cursor_pointer()
        .hover(|style| style.border_color(rgb(TEXT_MUTED)))
        .child(
            div()
                .flex_1()
                .min_w_0()
                .text_sm()
                .text_color(rgb(TEXT_PRIMARY))
                .child("全部"),
        )
        .child(
            div()
                .w(px(14.0))
                .flex()
                .items_center()
                .justify_center()
                .child(svg_icon("icons/chevron-down.svg", 14.0, TEXT_MUTED)),
        )
}

// 输入框
fn filter_input() -> impl IntoElement {
    div()
        .w(px(240.0))
        .h(px(34.0))
        .flex()
        .items_center()
        .px_3()
        .bg(rgb(BG_PANEL))
        .border_1()
        .border_color(rgb(BORDER))
        .rounded(px(4.0))
        .hover(|style| style.border_color(rgb(TEXT_MUTED)))
        .child(
            div()
                .flex_1()
                .min_w_0()
                .text_sm()
                .text_color(rgb(TEXT_MUTED))
                .child(""),
        )
        .child(div().w(px(14.0)))
}

// 操作按钮组
fn filter_actions() -> impl IntoElement {
    div()
        .flex()
        .gap_2()
        .child(filter_btn("icons/refresh.svg", "刷新"))
        .child(filter_btn("icons/trash.svg", "重置"))
}

// 筛选按钮
fn filter_btn(icon: &'static str, _title: &'static str) -> impl IntoElement {
    div()
        .id(SharedString::from(_title))
        .size(px(34.0))
        .flex()
        .items_center()
        .justify_center()
        .bg(rgb(BG_PANEL))
        .border_1()
        .border_color(rgb(BORDER))
        .rounded(px(4.0))
        .cursor_pointer()
        .hover(|style| style.bg(rgb(BG_ELEVATED)).border_color(rgb(TEXT_MUTED)))
        .child(svg_icon(icon, 16.0, TEXT_SECONDARY))
}

// 表格容器
fn table_container() -> impl IntoElement {
    div()
        .flex_1()
        .flex()
        .flex_col()
        .overflow_hidden()
        .relative()
        // 表头
        .child(table_header())
        // 表格内容（空状态）
        .child(empty_state())
}

// 表头
fn table_header() -> impl IntoElement {
    div()
        .flex()
        .bg(rgb(BG_ELEVATED))
        .border_b_1()
        .border_color(rgb(BORDER))
        .child(table_th("执行时间", 180.0, true))
        .child(table_th("服务器", 160.0, false))
        .child(table_th_flex("命令"))
        .child(table_th("耗时", 100.0, false))
}

// 固定宽度表头单元格
fn table_th(text: &'static str, width: f32, active: bool) -> impl IntoElement {
    let text_color = if active { ACCENT_RED } else { TEXT_SECONDARY };

    div()
        .w(px(width))
        .px_4()
        .py_3()
        .flex()
        .items_center()
        .gap_1()
        .cursor_pointer()
        .child(
            div()
                .text_sm()
                .font_medium()
                .text_color(rgb(text_color))
                .child(text),
        )
        .when(active, |el| {
            el.child(svg_icon("icons/arrow-up.svg", 14.0, text_color))
        })
}

// 弹性宽度表头单元格
fn table_th_flex(text: &'static str) -> impl IntoElement {
    div().flex_1().px_4().py_3().child(
        div()
            .text_sm()
            .font_medium()
            .text_color(rgb(TEXT_SECONDARY))
            .child(text),
    )
}

// 空状态
fn empty_state() -> impl IntoElement {
    div()
        .flex_1()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap_3()
        .child(div().size(px(48.0)).opacity(0.5).child(svg_icon(
            "icons/monitor-off.svg",
            48.0,
            TEXT_MUTED,
        )))
        .child(div().text_sm().text_color(rgb(TEXT_MUTED)).child("无数据"))
}
