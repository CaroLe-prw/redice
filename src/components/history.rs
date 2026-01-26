use gpui::{prelude::FluentBuilder, *};
use gpui_component::{
    ActiveTheme, IconName, Sizable, StyledExt,
    button::Button,
    input::{Input, InputState},
    select::{Select, SelectState},
};

use crate::ui::svg_icon;

pub(crate) struct HistoryPage {
    server_select: Entity<SelectState<Vec<&'static str>>>,
    keyword_input: Entity<InputState>,
}

impl HistoryPage {
    pub(crate) fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let server_select = cx.new(|cx| {
            SelectState::new(
                vec!["全部"],
                Some(gpui_component::IndexPath::default()),
                window,
                cx,
            )
        });
        let keyword_input = cx.new(|cx| InputState::new(window, cx).placeholder("搜索关键字..."));
        Self {
            server_select,
            keyword_input,
        }
    }
}

impl Render for HistoryPage {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        history_content_empty(self, cx)
    }
}

fn history_content_empty(view: &HistoryPage, cx: &mut Context<HistoryPage>) -> impl IntoElement {
    let theme = cx.theme();
    div()
        .size_full()
        .bg(theme.sidebar)
        .flex()
        .flex_col()
        .p_5()
        .overflow_hidden()
        .child(
            div().mb_4().child(
                div()
                    .text_lg()
                    .font_semibold()
                    .text_color(theme.accent_foreground)
                    .child("运行日志"),
            ),
        )
        .child(filter_bar(view, cx))
        .child(table_container(cx))
}

fn filter_bar(view: &HistoryPage, cx: &mut Context<HistoryPage>) -> impl IntoElement {
    let theme = cx.theme();
    div()
        .flex()
        .items_end()
        .gap_4()
        .mb_4()
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.muted_foreground)
                        .child("筛选服务器"),
                )
                .child(Select::new(&view.server_select).w(px(200.0))),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.muted_foreground)
                        .child("筛选关键字"),
                )
                .child(
                    Input::new(&view.keyword_input)
                        .w(px(200.0))
                        .prefix(gpui_component::Icon::new(IconName::Search).small()),
                ),
        )
        .child(
            div()
                .flex()
                .gap_2()
                .child(
                    Button::new("refresh")
                        .icon(svg_icon(
                            "icons/refresh.svg",
                            16.0,
                            cx.theme().secondary_foreground,
                        ))
                        .tooltip("刷新"),
                )
                .child(
                    Button::new("reset")
                        .icon(svg_icon(
                            "icons/trash.svg",
                            16.0,
                            cx.theme().secondary_foreground,
                        ))
                        .tooltip("重置"),
                ),
        )
}

fn table_container(cx: &mut Context<HistoryPage>) -> impl IntoElement {
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

fn empty_state(cx: &mut Context<HistoryPage>) -> impl IntoElement {
    let theme = cx.theme();
    div()
        .flex_1()
        .h_full()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap_4()
        .child(div().size(px(80.0)).opacity(0.5).child(svg_icon(
            "icons/monitor-off.svg",
            80.0,
            theme.muted_foreground,
        )))
        .child(
            div()
                .text_base()
                .text_color(theme.muted_foreground)
                .child("无数据"),
        )
}
