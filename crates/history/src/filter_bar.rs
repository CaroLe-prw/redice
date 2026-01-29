use gpui::*;
use gpui_component::{
    button::Button, input::Input, select::Select, ActiveTheme, IconName, Sizable,
};
use ui::svg_icon;

use super::HistoryPage;

pub(super) fn filter_bar(view: &HistoryPage, cx: &mut Context<HistoryPage>) -> impl IntoElement {
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
