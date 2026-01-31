use gpui::*;
use gpui_component::{
    checkbox::Checkbox,
    form::{field, v_form},
    h_flex,
    input::Input,
    v_flex,
};
use theme::ActiveTheme;

use crate::connection_dialog::ConnectionDialog;

/// Advanced Tab Configuration
impl ConnectionDialog {
    pub(super) fn render_advanced_tab(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let Some(advanced) = &self.config.advanced else {
            return div().into_any_element();
        };

        v_form()
            .columns(2)
            .child(
                field()
                    .label("默认数据库")
                    .description("连接后默认选择的数据库索引")
                    .col_span(2)
                    .child(Input::new(&advanced.default_db)),
            )
            .child(
                field()
                    .label("连接超时")
                    .child(Input::new(&advanced.connect_timeout).suffix("秒")),
            )
            .child(
                field()
                    .label("执行超时")
                    .child(Input::new(&advanced.exec_timeout).suffix("秒")),
            )
            .child(
                field()
                    .label("键分隔符")
                    .description("用于在键浏览器中分组显示键")
                    .child(Input::new(&advanced.key_separator)),
            )
            .child(
                field()
                    .label("每次加载键数量")
                    .description("SCAN 命令每次迭代返回的键数量")
                    .child(Input::new(&advanced.scan_count)),
            )
            .child(
                field().label("").col_span(2).child(
                    h_flex()
                        .gap_20()
                        .items_start()
                        .child(
                            v_flex()
                                .gap_1()
                                .child(
                                    Checkbox::new("readonly-mode")
                                        .label("启用只读模式")
                                        .cursor_pointer()
                                        .checked(advanced.readonly_mode)
                                        .on_click(cx.listener(|view, checked, _, cx| {
                                            if let Some(adv) = &mut view.config.advanced
                                                && adv.readonly_mode != *checked
                                            {
                                                adv.readonly_mode = *checked;
                                                cx.notify();
                                            }
                                        })),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(cx.theme().muted_foreground)
                                        .ml_5()
                                        .child("禁止执行写入操作"),
                                ),
                        )
                        .child(
                            Checkbox::new("auto-reconnect")
                                .label("自动重连")
                                .cursor_pointer()
                                .checked(advanced.auto_reconnect)
                                .on_click(cx.listener(|view, checked, _, cx| {
                                    if let Some(adv) = &mut view.config.advanced
                                        && adv.auto_reconnect != *checked
                                    {
                                        adv.auto_reconnect = *checked;
                                        cx.notify();
                                    }
                                })),
                        )
                        .child(
                            Checkbox::new("auto-connect")
                                .label("启动时自动连接")
                                .cursor_pointer()
                                .checked(advanced.auto_connect)
                                .on_click(cx.listener(|view, checked, _, cx| {
                                    if let Some(adv) = &mut view.config.advanced
                                        && adv.auto_connect != *checked
                                    {
                                        adv.auto_connect = *checked;
                                        cx.notify();
                                    }
                                })),
                        ),
                ),
            )
            .into_any_element()
    }
}
