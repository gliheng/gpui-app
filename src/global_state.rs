use crate::input::TextInput;
use gpui::*;

impl Global for TextStore {}
#[derive(Clone)]
pub struct TextStore {
    text: SharedString,
}

impl TextStore {
    fn new() -> Self {
        Self {
            text: "hello".into(),
        }
    }
}

pub struct GlobalStateUi {
    text_input: View<TextInput>,
    text_display: View<TextDisplay>,
    focus_handle: FocusHandle,
}

impl FocusableView for GlobalStateUi {
    fn focus_handle(&self, _: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl GlobalStateUi {
    pub fn new(cx: &mut WindowContext) -> View<Self> {
        cx.set_global(TextStore::new());
        cx.new_view(|cx| {
            let input = TextInput::new(cx);

            cx.observe(&input, |_, input, cx| {
                let ipt = input.read(cx);
                let text = ipt.text();
                cx.update_global::<TextStore, ()>(|store, cx| {
                    store.text = text;
                });
            })
            .detach();

            let display = TextDisplay::new(cx);
            GlobalStateUi {
                text_input: input,
                text_display: display,
                focus_handle: cx.focus_handle(),
            }
        })
    }
}

impl Render for GlobalStateUi {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .p_2()
            .child(self.text_input.clone())
            .child(self.text_display.clone())
    }
}

struct TextDisplay {
    text: SharedString,
}

impl TextDisplay {
    fn new(cx: &mut WindowContext) -> View<TextDisplay> {
        let view = cx.new_view(|cx| TextDisplay { text: "".into() });
        let copy = view.clone();
        cx.observe_global::<TextStore>(move |cx| {
            let text = cx.global::<TextStore>().text.clone();
            copy.update(cx, move |v, _| {
                v.text = text;
            });
        })
        .detach();
        view
    }
}

impl Render for TextDisplay {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .text_color(rgb(0xffffff))
            .child(format!("Text: {}", self.text))
    }
}
