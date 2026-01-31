use gpui::*;
use gpui_component::{
    form::{field, v_form},
    h_flex,
    input::Input,
    select::Select,
};

use crate::connection_dialog::ConnectionDialog;

/// General Configuration
impl ConnectionDialog {
    pub(super) fn render_general_tab(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        v_form()
            .child(
                field()
                    .label("连接名")
                    .child(Input::new(&self.config.general.name)),
            )
            .child(
                field()
                    .label("分组")
                    .child(Select::new(&self.config.general.group).cursor_pointer()),
            )
            .child(
                field().label("连接地址").child(
                    h_flex()
                        .gap_3()
                        .child(
                            div()
                                .w(px(130.))
                                .child(Select::new(&self.config.general.protocol).cursor_pointer()),
                        )
                        .child(div().flex_1().child(Input::new(&self.config.general.host)))
                        .child(
                            div()
                                .w(px(100.))
                                .child(Input::new(&self.config.general.port)),
                        ),
                ),
            )
            .child(
                field()
                    .label("用户名")
                    .child(Input::new(&self.config.general.username)),
            )
            .child(
                field()
                    .label("密码")
                    .child(Input::new(&self.config.general.password).mask_toggle()),
            )
    }
}
