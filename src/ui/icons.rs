use std::{borrow::Cow, collections::BTreeSet};

use crate::ui::{ACCENT_BLUE, BG_ELEVATED, BG_PANEL, BORDER, TEXT_MUTED, TEXT_PRIMARY};
use gpui::*;
use gpui_component::{Icon, IconName, Sizable, StyledExt};
use rust_embed::RustEmbed;

#[derive(Clone)]
pub enum IconSource {
    Name(IconName),
    Path(&'static str),
}

impl From<IconName> for IconSource {
    fn from(name: IconName) -> Self {
        IconSource::Name(name)
    }
}

impl From<&'static str> for IconSource {
    fn from(path: &'static str) -> Self {
        IconSource::Path(path)
    }
}

#[derive(RustEmbed)]
#[folder = "./assets"]
#[include = "icons/**/*.svg"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }

        if let Some(f) = Assets::get(path) {
            return Ok(Some(f.data));
        }

        match gpui_component_assets::Assets.load(path) {
            Ok(v) => Ok(v),
            Err(_) => Ok(None),
        }
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let mut set = BTreeSet::<SharedString>::new();

        for p in Assets::iter().filter(|p| p.starts_with(path)) {
            set.insert(p.into());
        }

        for p in gpui_component_assets::Assets.list(path)? {
            set.insert(p);
        }
        Ok(set.into_iter().collect())
    }
}

pub fn svg_icon(icon: impl Into<IconSource>, px_size: f32, color: u32) -> Icon {
    let icon = match icon.into() {
        IconSource::Name(name) => Icon::new(name),
        IconSource::Path(path) => Icon::new(IconName::CircleX).path(path),
    };
    icon.with_size(px(px_size))
        .text_color(rgb(color))
        .flex_shrink_0()
}

pub fn footer_icon_btn(icon: impl Into<IconSource>, color: u32) -> impl IntoElement {
    div()
        .size(px(28.0))
        .flex()
        .items_center()
        .justify_center()
        .rounded(px(4.0))
        .child(svg_icon(icon, 16.0, color))
}

pub fn sidebar_divider() -> impl IntoElement {
    div()
        .w(px(28.0))
        .h(px(1.0))
        .my_2()
        .bg(rgb(BORDER))
        .opacity(0.7)
}

pub fn sidebar_icon_btn(
    active: bool,
    icon: impl Into<IconSource>,
    bg_active: u32,
    color_active: u32,
    color_idle: u32,
) -> impl IntoElement {
    let icon = icon.into();
    let mut el = div()
        .size(px(36.0))
        .flex()
        .items_center()
        .justify_center()
        .rounded(px(8.0));

    if active {
        el = el.bg(rgb(bg_active));
        el.child(svg_icon(icon, 22.0, color_active))
            .into_any_element()
    } else {
        el.hover(|style| style.bg(rgb(bg_active)))
            .child(svg_icon(icon, 22.0, color_idle))
            .into_any_element()
    }
}

pub fn quick_action_card(
    icon_path: &'static str,
    color: u32,
    bg_color: u32,
    title: &'static str,
    desc: &'static str,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .items_center()
        .gap_3()
        .px_8()
        .py_6()
        .min_w(px(160.0))
        .bg(rgb(BG_PANEL))
        .border_1()
        .border_color(rgb(BORDER))
        .rounded(px(12.0))
        .cursor_pointer()
        .hover(|style| {
            style
                .bg(rgb(BG_ELEVATED))
                .border_color(rgb(ACCENT_BLUE))
                .mt(px(-2.0))
                .mb(px(2.0))
        })
        .child(
            div()
                .size(px(40.0))
                .flex()
                .items_center()
                .justify_center()
                .rounded(px(10.0))
                .bg(rgb(bg_color))
                .child(svg_icon(icon_path, 20.0, color)),
        )
        .child(
            div()
                .text_sm()
                .font_medium()
                .text_color(rgb(TEXT_PRIMARY))
                .child(title),
        )
        .child(div().text_xs().text_color(rgb(TEXT_MUTED)).child(desc))
}

pub fn feature_item(
    icon_path: &'static str,
    title: &'static str,
    desc: &'static str,
) -> impl IntoElement {
    div()
        .flex()
        .items_start()
        .gap_3()
        .max_w(px(240.0))
        .child(
            div()
                .size(px(32.0))
                .flex()
                .items_center()
                .justify_center()
                .rounded(px(8.0))
                .bg(rgb(BG_ELEVATED))
                .flex_shrink_0()
                .child(svg_icon(icon_path, 16.0, ACCENT_BLUE)),
        )
        .child(
            div()
                .child(
                    div()
                        .text_sm()
                        .font_semibold()
                        .text_color(rgb(TEXT_PRIMARY))
                        .mb_1()
                        .child(title),
                )
                .child(div().text_xs().text_color(rgb(TEXT_MUTED)).child(desc)),
        )
}
