use gpui::{prelude::FluentBuilder, *};
use gpui_component::{ActiveTheme, StyledExt};

use super::HistoryPage;
use super::empty_state::empty_state;
use crate::ui::svg_icon;

pub(super) fn table_container(cx: &mut Context<HistoryPage>) -> impl IntoElement {
    div()
        .flex_1()
        .flex()
        .flex_col()
        .overflow_hidden()
        .child(table_header(cx))
        .child(div().flex_1().relative().child(empty_state(cx)))
}

fn table_header(cx: &mut Context<HistoryPage>) -> impl IntoElement {
    let theme = cx.theme();
    div()
        .flex()
        .bg(theme.muted)
        .border_b_1()
        .border_color(theme.border)
        .child(table_th("执行时间", 180.0, true, cx))
        .child(table_th("服务器", 160.0, false, cx))
        .child(table_th_flex("命令", cx))
        .child(table_th("耗时", 100.0, false, cx))
}

fn table_th(
    text: &'static str,
    width: f32,
    active: bool,
    cx: &mut Context<HistoryPage>,
) -> impl IntoElement {
    let theme = cx.theme();
    let text_color = if active {
        theme.danger
    } else {
        theme.secondary_foreground
    };

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
                .text_color(text_color)
                .child(text),
        )
        .when(active, |el| {
            el.child(svg_icon("icons/arrow-up.svg", 14.0, text_color))
        })
}

fn table_th_flex(text: &'static str, cx: &mut Context<HistoryPage>) -> impl IntoElement {
    let theme = cx.theme();
    div().flex_1().px_4().py_3().child(
        div()
            .text_sm()
            .font_medium()
            .text_color(theme.secondary_foreground)
            .child(text),
    )
}
