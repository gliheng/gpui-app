use crate::Button;
use gpui::*;

#[derive(Debug)]
pub struct Counter(i32);

#[derive(Clone)]
pub struct CounterModel {
    pub inner: Model<Counter>,
}

impl CounterModel {
    pub fn new(cx: &mut WindowContext) -> Self {
        let this = Self {
            inner: cx.new_model(|_| Counter(0)),
        };
        this
    }
}

pub struct RenderCounter {
    state: CounterModel,
}

impl RenderCounter {
    pub fn new(cx: &mut WindowContext, state: CounterModel) -> View<Self> {
        let view = cx.new_view(|cx| {
            // let state = CounterModel::new(cx);
            // cx.observe(&state.inner, |_, _, cx| {
            //     println!("model update!!!");
            //     cx.notify();
            // })
            // .detach();
            RenderCounter { state }
        });
        view
    }

    fn inc(&mut self, _event: &ClickEvent, cx: &mut ViewContext<Self>) {
        self.state.inner.update(cx, |m, _| {
            m.0 += 1;
        });
        cx.notify();
    }

    fn dec(&mut self, _event: &ClickEvent, cx: &mut ViewContext<Self>) {
        self.state.inner.update(cx, |m, _| {
            m.0 -= 1;
        });
        cx.notify();
    }
}

impl Render for RenderCounter {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let n = cx.read_model(&self.state.inner, |m, _| return m.0);

        let increment_button =
            Button::new("inc-btn", "Inc".into()).on_click(cx.listener(Self::inc));
        let decrement_button =
            Button::new("dec-btn", "Dec".into()).on_click(cx.listener(Self::dec));
        div()
            .flex()
            .bg(rgb(0x2e7d32))
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(
                div()
                    .flex()
                    .child(increment_button)
                    .child(
                        div()
                            .id("count")
                            .px_2()
                            .bg(rgb(0x4caf50))
                            .text_color(rgb(0xffffff))
                            .flex()
                            .items_center()
                            .child(format!("Count: {}", n)),
                    )
                    .child(decrement_button),
            )
    }
}
