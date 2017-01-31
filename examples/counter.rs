#[macro_use]
extern crate limn;

mod util;

use std::any::Any;

use limn::widget::{EventHandler, EventArgs};
use limn::widget::builder::WidgetBuilder;
use limn::widget::layout::{LinearLayout, Orientation};
use limn::widgets::text::{self, TextDrawable, TEXT_STYLE_DEFAULT};
use limn::widgets::button::PushButtonBuilder;
use limn::event::{self, EventId, Event, Signal, EventAddress};
use limn::resources::Id;
use limn::color::*;

const COUNTER: EventId = EventId("COUNTER");
const COUNT: EventId = EventId("COUNT");

fn main() {
    let (window, ui) = util::init_default("Limn counter demo");
    let font_id = util::load_default_font();
    
    let mut root_widget = WidgetBuilder::new();

    let mut linear_layout = LinearLayout::new(Orientation::Horizontal, &root_widget.layout);
    let mut left_spacer = WidgetBuilder::new();
    left_spacer.layout.width(50.0);
    linear_layout.add_widget(&mut left_spacer.layout);
    root_widget.add_child(Box::new(left_spacer));

    struct CountHandler {}
    impl EventHandler for CountHandler {
        fn event_id(&self) -> EventId {
            COUNT
        }
        fn handle_event(&mut self, args: EventArgs) {
            let count = args.event.data::<u32>();
            args.state.update(|state: &mut TextDrawable| state.text = format!("{}", count));
        }
    }
    let text_drawable = TextDrawable::new_style(TEXT_STYLE_DEFAULT.clone().with_text("0").with_background_color(WHITE));
    let text_dims = text_drawable.measure_dims_no_wrap();
    let mut text_widget = WidgetBuilder::new()
        .set_drawable(text::draw_text, Box::new(text_drawable))
        .add_handler(Box::new(CountHandler {}));
    text_widget.layout.width(80.0);
    text_widget.layout.height(text_dims.height);
    text_widget.layout.center_vertical(&root_widget.layout);
    linear_layout.add_widget(&mut text_widget.layout);

    let mut button_container = WidgetBuilder::new();
    linear_layout.add_widget(&mut button_container.layout);
    struct PushButtonHandler {
        receiver_id: Id,
    }
    impl EventHandler for PushButtonHandler {
        fn event_id(&self) -> EventId {
            event::WIDGET_PRESS
        }
        fn handle_event(&mut self, args: EventArgs) {
            let event = Signal::new(COUNTER);
            args.event_queue.push(EventAddress::Widget(self.receiver_id), COUNTER, Box::new(event));
        }
    }
    let mut button_widget = PushButtonBuilder::new()
        .set_text("Count", font_id)
        .widget.add_handler(Box::new(PushButtonHandler { receiver_id: root_widget.id }));
    button_widget.layout.center(&button_container.layout);
    button_widget.layout.pad(50.0, &button_container.layout);
    button_container.add_child(Box::new(button_widget));
    root_widget.add_child(Box::new(text_widget));
    root_widget.add_child(Box::new(button_container));


    event!(CountEvent, u32);
    struct CounterHandler {
        count: u32,
    }
    impl CounterHandler {
        fn new() -> Self {
            CounterHandler { count: 0 }
        }
    }
    impl EventHandler for CounterHandler {
        fn event_id(&self) -> EventId {
            COUNTER
        }
        fn handle_event(&mut self, args: EventArgs) {
            self.count += 1;
            let event = CountEvent::new(COUNT, self.count);
            args.event_queue.push(EventAddress::SubTree(args.widget_id), COUNT, Box::new(event));
        }
    }
    root_widget.event_handlers.push(Box::new(CounterHandler::new()));

    util::set_root_and_loop(window, ui, root_widget);
}
