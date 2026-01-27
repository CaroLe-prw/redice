mod db;
mod pages;
mod ui;
use std::path::PathBuf;

use gpui::{prelude::FluentBuilder, *};
use gpui_component::{ActiveTheme, Root, Theme, ThemeRegistry};
use tracing_subscriber::{EnvFilter, fmt};

use crate::{
    pages::{HistoryPage, HomePage, home::icon_sidebar},
    ui::Assets,
};

struct HomeShell {
    sidebar_active: usize,
    home_page: Entity<HomePage>,
    history_page: Entity<HistoryPage>,
}

impl Render for HomeShell {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let sidebar_active = self.sidebar_active;
        div()
            .size_full()
            .flex()
            .bg(cx.theme().sidebar)
            .child(icon_sidebar(sidebar_active, cx))
            .child(
                div()
                    .flex_1()
                    .relative()
                    .overflow_hidden()
                    .child(
                        div()
                            .absolute()
                            .inset_0()
                            .opacity(if sidebar_active == 0 { 1.0 } else { 0.0 })
                            .child(self.home_page.clone())
                            .when(sidebar_active != 0, |el| {
                                el.child(div().absolute().inset_0())
                            }),
                    )
                    .child(
                        div()
                            .absolute()
                            .inset_0()
                            .opacity(if sidebar_active == 1 { 1.0 } else { 0.0 })
                            .child(self.history_page.clone())
                            .when(sidebar_active != 1, |el| {
                                el.child(div().absolute().inset_0())
                            }),
                    ),
            )
    }
}

impl HomeShell {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            sidebar_active: 0,
            home_page: cx.new(|_| HomePage::new()),
            history_page: cx.new(|cx| HistoryPage::new(window, cx)),
        }
    }

    pub(crate) fn set_active_page(&mut self, index: usize, cx: &mut Context<Self>) {
        if self.sidebar_active == index {
            return;
        }
        self.sidebar_active = index;
        cx.notify();
    }
}

fn main() {
    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        gpui_component::init(cx);

        // Load custom theme
        let theme_name = SharedString::from("Dark");
        let themes_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("themes");
        if let Err(err) = ThemeRegistry::watch_dir(themes_dir, cx, move |cx| {
            if let Some(theme) = ThemeRegistry::global(cx).themes().get(&theme_name).cloned() {
                Theme::global_mut(cx).apply_config(&theme);
            }
        }) {
            tracing::error!("Failed to watch themes directory: {}", err);
        }

        cx.spawn(async move |cx| {
            // init db
            if let Err(e) = db::init_db().await {
                tracing::error!("init_db failed: {:?}", e);
            }

            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|cx| HomeShell::new(window, cx));
                cx.new(|cx| Root::new(view, window, cx))
            })?;
            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
