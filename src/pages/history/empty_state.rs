use gpui::*;
use gpui_component::ActiveTheme;

use super::HistoryPage;
use crate::ui::svg_icon;

pub(super) fn empty_state(cx: &mut Context<HistoryPage>) -> impl IntoElement {
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
