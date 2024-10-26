use gpui::*;

pub struct Draw {}

impl Render for Draw {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .w_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgb(0x999999))
            .child(div().w(rems(40.)).h(rems(100.)).bg(rgb(0xcccccc)))
    }
}
