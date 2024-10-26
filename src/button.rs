use gpui::prelude::FluentBuilder;
use gpui::*;

const BORDER_COLOR: u32 = 0x2D3039;
const BACKGROUND_COLOR: u32 = 0x3B82F6;
const FOREGROUND_COLOR: u32 = 0xFFFFFF;
const HOVER_COLOR: u32 = 0x60A5FA;

#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    label: SharedString,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>, label: SharedString) -> Self {
        Button {
            id: id.into(),
            label,
            on_click: None,
        }
    }

    pub fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Button {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        div()
            .id(self.id)
            .flex()
            .text_xl()
            .border_2()
            .p_2()
            .rounded_lg()
            .border_color(rgb(BORDER_COLOR))
            .cursor_pointer()
            .text_color(rgb(FOREGROUND_COLOR))
            .bg(rgb(BACKGROUND_COLOR))
            .hover(|style| style.bg(rgb(HOVER_COLOR)))
            .when_some(self.on_click, |this, on_click| {
                this.on_click(move |evt, cx| (on_click)(evt, cx))
            })
            .child(self.label)
    }
}
