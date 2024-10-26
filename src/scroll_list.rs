use gpui::*;

pub struct ScrollList {
    message_list: ListState,
}

impl ScrollList {
    pub fn new(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let view = cx.view().downgrade();

            let message_list = ListState::new(
                200,
                gpui::ListAlignment::Bottom,
                px(1000.),
                move |ix, cx| {
                    if let Some(view) = view.upgrade() {
                        view.update(cx, |view: &mut ScrollList, cx| {
                            println!("{}", ix);
                            view.render_message(ix, cx).into_any_element()
                        })
                    } else {
                        div().into_any()
                    }
                },
            );
            message_list.set_scroll_handler(cx.listener(|this, event: &ListScrollEvent, cx| {
                println!("scroll {}", event.visible_range.start);
            }));

            return ScrollList { message_list };
        })
    }

    fn render_message(&mut self, ix: usize, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().h(rems(2. + ix as f32 / 20.)).child(format!("{}", ix))
    }
}

impl Render for ScrollList {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().size_full().flex().bg(rgb(0xffffee)).child(
            div()
                .flex_1()
                .child(list(self.message_list.clone()).size_full()),
        )
    }
}
