use gpui::*;
use gpui_component::{
    form::{Field, v_form},
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
                Field::new()
                    .label("连接名")
                    .child(Input::new(&self.config.general.name)),
            )
            .child(
                Field::new()
                    .label("分组")
                    .child(Select::new(&self.config.general.group)),
            )
            .child(
                Field::new().label("连接地址").child(
                    h_flex()
                        .gap_3()
                        .child(
                            div()
                                .w(px(130.))
                                .child(Select::new(&self.config.general.protocol)),
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
                Field::new()
                    .label("用户名")
                    .child(Input::new(&self.config.general.username)),
            )
            .child(
                Field::new()
                    .label("密码")
                    .child(Input::new(&self.config.general.password).mask_toggle()),
            )
    }
}
