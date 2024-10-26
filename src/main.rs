mod action;
mod button;
mod counter;
mod counter_simple;
mod draw;
mod global_state;
mod input;
mod like_button;
mod modal;
mod scroll_list;
mod share_state;
mod subscribe;

use action::ActionExample;
use button::Button;
use counter_simple::CounterSimple;
use global_state::GlobalStateUi;
use gpui::*;
use like_button::LikeButton;
use modal::ModalExample;
use scroll_list::ScrollList;
use share_state::ShareStateUi;
use subscribe::SubscribeExample;

struct HelloWorld {
    counter_simple: View<CounterSimple>,
    like_button: View<LikeButton>,
    share_state: View<ShareStateUi>,
    global_state: View<GlobalStateUi>,
    subscribe: View<SubscribeExample>,
    action: View<ActionExample>,
    modal: View<ModalExample>,
}

impl HelloWorld {
    fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            counter_simple: CounterSimple::new(cx),
            like_button: LikeButton::new(cx),
            share_state: ShareStateUi::new(cx),
            global_state: GlobalStateUi::new(cx),
            subscribe: SubscribeExample::new(cx),
            action: ActionExample::new(cx),
            modal: ModalExample::new(cx),
        }
    }
}

impl Render for HelloWorld {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .w_full()
            .flex()
            .flex_col()
            .items_center()
            .child(self.counter_simple.clone())
            .child(self.like_button.clone())
            .child(self.share_state.clone())
            .child(self.global_state.clone())
            .child(self.subscribe.clone())
            .child(self.action.clone())
            .child(self.modal.clone())
    }
}

// Associate actions using the `actions!` macro (or `impl_actions!` macro)
actions!(set_menus, [Quit]);

// Define the quit function that is registered with the AppContext
fn quit(_: &Quit, cx: &mut AppContext) {
    println!("Gracefully quitting the application . . .");
    cx.quit();
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.activate(true);
        // Register the `quit` function so it can be referenced by the `MenuItem::action` in the menu bar
        cx.on_action(quit);
        // Add menu items
        cx.set_menus(vec![Menu {
            name: "set_menus".into(),
            items: vec![MenuItem::action("Quit", Quit)],
        }]);
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|cx| HelloWorld::new(cx))
        })
        .unwrap();
        // cx.open_window(WindowOptions::default(), |cx| ScrollList::new(cx))
        //     .unwrap();
        // cx.open_window(WindowOptions::default(), |cx| {
        //     cx.new_view(|_cx| draw::Draw {})
        // })
        // .unwrap();
    });
}
