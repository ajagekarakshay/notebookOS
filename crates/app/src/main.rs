use gpui_kit::*;

struct NotebookOs;

impl Render for NotebookOs {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child("NotebookOS")
    }
}

fn main() {
    application().with_assets(assets::Assets).run(|cx| {
        init(cx);

        open_window(WindowOptions::default(), cx, |_, cx| cx.new(|_| NotebookOs))
            .expect("failed to open the NotebookOS window");
    });
}
