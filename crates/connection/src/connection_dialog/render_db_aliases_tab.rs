use gpui::{prelude::FluentBuilder, *};
use gpui_component::{h_flex, input::Input, v_flex};
use theme::ActiveTheme;
use ui::icon_btn_interactive;

use crate::connection_dialog::ConnectionDialog;

/// Database Aliases Configuration
impl ConnectionDialog {
    pub(super) fn render_db_aliases_tab(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let Some(db_aliases) = &self.config.db_aliases else {
            return div().into_any_element();
        };

        let theme = cx.theme();
        let has_aliases = !db_aliases.aliases.is_empty();

        let alias_rows: Vec<_> = db_aliases
            .aliases
            .iter()
            .enumerate()
            .map(|(index, alias)| {
                h_flex()
                    .gap_3()
                    .items_center()
                    .child(div().w(px(100.)).child(Input::new(&alias.db_index)))
                    .child(div().text_color(theme.muted_foreground).child(":"))
                    .child(div().flex_1().child(Input::new(&alias.alias)))
                    .child(
                        div()
                            .id(SharedString::from(format!("delete-{}", index)))
                            .cursor_pointer()
                            .child(icon_btn_interactive(
                                SharedString::from(format!("delete-icon-{}", index)),
                                "icons/trash.svg",
                                theme.muted_foreground,
                                theme.table_hover,
                            ))
                            .on_click(cx.listener(move |view, _, _, cx| {
                                if let Some(db_aliases) = &mut view.config.db_aliases {
                                    db_aliases.remove_alias(index);
                                    cx.notify();
                                }
                            })),
                    )
                    .child(
                        div()
                            .id(SharedString::from(format!("add-{}", index)))
                            .cursor_pointer()
                            .child(icon_btn_interactive(
                                SharedString::from(format!("add-icon-{}", index)),
                                "icons/circle-plus.svg",
                                theme.muted_foreground,
                                theme.table_hover,
                            ))
                            .on_click(cx.listener(|view, _, window, cx| {
                                if let Some(db_aliases) = &mut view.config.db_aliases {
                                    db_aliases.add_alias(window, cx);
                                    cx.notify();
                                }
                            })),
                    )
            })
            .collect();

        v_flex()
            .gap_3()
            .children(alias_rows)
            .when(!has_aliases, |el| {
                el.child(
                    div()
                        .id("add-alias-btn")
                        .group("add-btn")
                        .px_4()
                        .py_3()
                        .border_1()
                        .border_color(theme.border)
                        .border_dashed()
                        .rounded_md()
                        .cursor_pointer()
                        .hover(|style| style.border_color(theme.table_hover))
                        .child(
                            h_flex()
                                .w_full()
                                .justify_center()
                                .gap_2()
                                .text_color(theme.muted_foreground)
                                .child("+")
                                .child("添加"),
                        )
                        .on_click(cx.listener(|view, _, window, cx| {
                            if let Some(db_aliases) = &mut view.config.db_aliases {
                                db_aliases.add_alias(window, cx);
                                cx.notify();
                            }
                        })),
                )
            })
            .into_any_element()
    }
}
