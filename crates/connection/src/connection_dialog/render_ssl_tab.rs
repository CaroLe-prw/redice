use gpui::*;
use gpui_component::{
    Disableable,
    button::Button,
    checkbox::Checkbox,
    form::{field, v_form},
    h_flex,
    input::Input,
    v_flex,
};
use theme::ActiveTheme;

use crate::connection_dialog::ConnectionDialog;

impl ConnectionDialog {
    pub(super) fn render_ssl_tab(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let Some(ssl) = &self.config.ssl else {
            return div().into_any_element();
        };
        let theme = cx.theme();

        v_form()
            .child(
                field().child(
                    Checkbox::new("ssl-enable")
                        .cursor_pointer()
                        .label("启用 SSL/TLS")
                        .checked(ssl.ssl_enabled)
                        .on_click(cx.listener(|view, checked, _, cx| {
                            if let Some(ssl) = &mut view.config.ssl {
                                ssl.ssl_enabled = *checked;
                                cx.notify();
                            }
                        })),
                ),
            )
            .child(
                field().label("CA 证书").child(
                    h_flex()
                        .gap_2()
                        .child(
                            div()
                                .flex_1()
                                .child(Input::new(&ssl.ssl_ca_cert).disabled(!ssl.ssl_enabled)),
                        )
                        .child(
                            Button::new("browse-ca-cert")
                                .label("浏览...")
                                .disabled(!ssl.ssl_enabled),
                        ),
                ),
            )
            .child(
                field().label("客户端证书").child(
                    h_flex()
                        .gap_2()
                        .child(
                            div()
                                .flex_1()
                                .child(Input::new(&ssl.ssl_client_cert).disabled(!ssl.ssl_enabled)),
                        )
                        .child(
                            Button::new("browse-client-cert")
                                .label("浏览...")
                                .disabled(!ssl.ssl_enabled),
                        ),
                ),
            )
            .child(
                field().label("客户端私钥").child(
                    h_flex()
                        .gap_2()
                        .child(
                            div()
                                .flex_1()
                                .child(Input::new(&ssl.ssl_client_key).disabled(!ssl.ssl_enabled)),
                        )
                        .child(
                            Button::new("browse-client-key")
                                .label("浏览...")
                                .disabled(!ssl.ssl_enabled),
                        ),
                ),
            )
            .child(
                field()
                    .label("SNI 服务器名称")
                    .child(Input::new(&ssl.ssl_sni).disabled(!ssl.ssl_enabled)),
            )
            .child(
                field().label("").child(
                    v_flex()
                        .gap_1()
                        .child(
                            Checkbox::new("ssl-skip-verify")
                                .label("跳过证书验证")
                                .cursor_pointer()
                                .disabled(!ssl.ssl_enabled)
                                .checked(ssl.ssl_skip_verify)
                                .on_click(cx.listener(|view, checked, _, cx| {
                                    if let Some(ssl) = &mut view.config.ssl {
                                        ssl.ssl_skip_verify = *checked;
                                        cx.notify();
                                    }
                                })),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.red)
                                .ml_5()
                                .child("警告：仅用于测试环境"),
                        ),
                ),
            )
            .into_any_element()
    }
}
