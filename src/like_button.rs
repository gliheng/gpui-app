use crate::button::Button;
use gpui::*;

pub struct LikeButton {}
impl LikeButton {
    pub fn new(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| LikeButton {})
    }
    fn handle_like(&mut self, _event: &ClickEvent, cx: &mut ViewContext<Self>) {
        let confirmation = cx.prompt(PromptLevel::Critical, "Shit", None, &["Delete", "Cancel"]);

        cx.spawn(|this, mut cx| async move {
            if let Some(ans) = confirmation.await.ok() {
                dbg!("Got answer", ans);
            }
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }
}
impl Render for LikeButton {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        Button::new("like-button", "Like".into()).on_click(cx.listener(Self::handle_like))
    }
}
