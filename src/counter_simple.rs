use gpui::*;

const BACKGROUND_COLOR: u32 = 0x1E2027;
const FOREGROUND_COLOR: u32 = 0xE6E6E6;
const BORDER_COLOR: u32 = 0x2D3039;
const HOVER_BACKGROUND_COLOR: u32 = 0x3D4049;

pub struct CounterSimple {
    v: i32,
}

impl CounterSimple {
    pub fn new(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_| Self { v: 0 })
    }
    fn inc(&mut self, _event: &ClickEvent, cx: &mut ViewContext<Self>) {
        self.v += 1;
        cx.notify();
    }
}

impl Render for CounterSimple {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .id("counter")
            .w(rems(10.0))
            .flex()
            .items_center()
            .content_center()
            .border_2()
            .bg(rgb(BACKGROUND_COLOR))
            .border_color(rgb(BORDER_COLOR))
            .hover(|style| style.bg(rgb(HOVER_BACKGROUND_COLOR)))
            .text_color(rgb(FOREGROUND_COLOR))
            .text_xl()
            .p_2()
            .child(format!("# {}", self.v))
            .rounded_lg()
            .on_click(cx.listener(Self::inc))
    }
}
