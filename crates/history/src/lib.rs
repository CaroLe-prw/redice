//! Monitor view for Redice - history, slowlog, command monitor

mod empty_state;
mod filter_bar;
mod table;

use gpui::*;
use gpui_component::{ActiveTheme, StyledExt, input::InputState, select::SelectState};

use filter_bar::filter_bar;
use table::table_container;

pub struct HistoryPage {
    pub server_select: Entity<SelectState<Vec<&'static str>>>,
    pub keyword_input: Entity<InputState>,
}

impl HistoryPage {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
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
            .child(filter_bar(self, cx))
            .child(table_container(cx))
    }
}
