use crate::counter::{CounterModel, RenderCounter};
use gpui::*;

pub struct ShareStateUi {
    counter_view: View<RenderCounter>,
    counter_view2: View<RenderCounter>,
}

impl ShareStateUi {
    pub fn new(cx: &mut WindowContext) -> View<Self> {
        let state = CounterModel::new(cx);

        cx.new_view(|cx| ShareStateUi {
            counter_view: RenderCounter::new(cx, state.clone()),
            counter_view2: RenderCounter::new(cx, state.clone()),
        })
    }
}

impl Render for ShareStateUi {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        return div()
            .child(self.counter_view.clone())
            .child(self.counter_view2.clone());
    }
}
