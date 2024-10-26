use gpui::*;

use crate::button::Button;

pub struct SubscribeExample {}

#[derive(Debug)]
enum MyEvent {
    One,
    Two,
}

impl SubscribeExample {
    pub fn new(cx: &mut WindowContext) -> View<SubscribeExample> {
        let view = cx.new_view(|cx| SubscribeExample {});
        cx.subscribe(&view, |_, evt, _| {
            println!("Got event {evt:?}");
        })
        .detach();
        view
    }

    fn emit_one(&mut self, _: &ClickEvent, cx: &mut ViewContext<Self>) {
        cx.emit(MyEvent::One);
    }
    fn emit_two(&mut self, _: &ClickEvent, cx: &mut ViewContext<Self>) {
        cx.emit(MyEvent::Two);
    }
}

impl EventEmitter<MyEvent> for SubscribeExample {}

impl Render for SubscribeExample {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .child(
                Button::new("emit-one", "Emit one".into())
                    .on_click(cx.listener(SubscribeExample::emit_one)),
            )
            .child(
                Button::new("emit-two", "Emit two".into())
                    .on_click(cx.listener(SubscribeExample::emit_two)),
            )
    }
}
