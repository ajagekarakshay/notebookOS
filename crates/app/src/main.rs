mod app;
mod theme;

use app::NotebookOs;
use gpui_kit::{AppContext as _, WindowOptions, application, assets, init, open_window};

fn main() {
    application().with_assets(assets::Assets).run(|cx| {
        init(cx);

        open_window(WindowOptions::default(), cx, |window, cx| {
            cx.new(|cx| NotebookOs::new(window, cx))
        })
        .expect("failed to open the NotebookOS window");
    });
}
