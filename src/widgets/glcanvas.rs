use widget::Widget;
pub use draw::glcanvas::GLCanvasState;

#[derive(Debug, Copy, Clone)]
pub struct GLCanvasBuilder;

impl GLCanvasBuilder {
    /// Creates a new `GLCanvasBuilder`, returns it in form of a `Widget`
    pub fn new(texture_id: u64) -> Widget {
        let image_draw_state = GLCanvasState::new(texture_id);
        let mut widget = Widget::new("glcanvas");
        widget.set_draw_state(image_draw_state);
        widget
    }
}
