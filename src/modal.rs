mod modal_layer;

use crate::{Button, HelloWorld};
use gpui::*;
use modal_layer::{ModalLayer, ModalView};

struct MyModal {
    focus_handle: FocusHandle,
}

impl Render for MyModal {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().child("Hello from modal")
    }
}

impl MyModal {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl EventEmitter<DismissEvent> for MyModal {}

impl FocusableView for MyModal {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ModalView for MyModal {}

pub struct ModalExample {
    modal_layer: View<ModalLayer>,
}
impl ModalExample {
    pub fn new(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| ModalExample {
            modal_layer: cx.new_view(|_| ModalLayer::new()),
        })
    }
    fn toggle_modal(&mut self, _event: &ClickEvent, cx: &mut ViewContext<Self>) {
        self.modal_layer.update(cx, |modal_layer, cx| {
            modal_layer.toggle_modal(cx, |cx| MyModal::new(cx));
        })
    }
}

impl Render for ModalExample {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .w(rems(32.))
            .h_32()
            .bg(rgb(0xff0000))
            .child(self.modal_layer.clone())
            .child(
                Button::new("toggle-modal", "Toggle Modal".into())
                    .on_click(cx.listener(Self::toggle_modal)),
            )
    }
}
