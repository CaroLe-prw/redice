mod logging;
mod workspace;

use gpui::*;
use gpui_component::{Root, TitleBar};

use ui::Assets;
use workspace::HomeShell;

fn main() {
    logging::init_logging();
    runtime::init();

    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        gpui_component::init(cx);
        theme::init(cx);

        cx.spawn(async move |cx| {
            if let Err(e) = db::init_db().await {
                tracing::error!("init_db failed: {:?}", e);
            }

            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitleBar::title_bar_options()),
                    ..Default::default()
                },
                |window, cx| {
                    let view = cx.new(|cx| HomeShell::new(window, cx));
                    cx.new(|cx| Root::new(view, window, cx))
                },
            )?;
            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
