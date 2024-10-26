use crate::Button;
use gpui::*;

pub struct ActionExample {
    focus_handle: FocusHandle,
}

actions!(wtf, [ActionOne, ActionTwo]);

impl ActionExample {
    pub fn new(cx: &mut WindowContext) -> View<Self> {
        let view = cx.new_view(|cx| ActionExample {
            focus_handle: cx.focus_handle(),
        });
        cx.bind_keys([KeyBinding::new("ctrl-a", ActionOne, None)]);

        view
    }

    fn on_action(&mut self, _: &ActionOne, cx: &mut ViewContext<Self>) {
        println!("Action one");
    }

    fn on_click(&mut self, _: &ClickEvent, cx: &mut ViewContext<Self>) {
        println!("Dispatch");
        cx.dispatch_action(Box::new(ActionOne));
    }
}

impl Render for ActionExample {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .child(
                Button::new("action-btn", "Dispatch".into()).on_click(cx.listener(Self::on_click)),
            )
            .on_action(cx.listener(Self::on_action))
    }
}

impl FocusableView for ActionExample {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}
