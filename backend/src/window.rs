use std::time::Duration;
use super::gfx::{GfxContext, G2d};
use super::shader_version::OpenGL;

pub use super::events::WindowEvents;
use graphics::Viewport;

use glutin;

use super::gl;
use glutin::ContextError;
use std::error::Error;

pub use super::graphics::Context;

/// Contains everything required for controlling window, graphics, event loop.
pub struct Window {
    /// The window.
    pub window: glutin::Window,
    /// Stores state associated with Gfx.
    pub context: GfxContext,
}

impl Window {
    pub fn new<T, S>(title: T, size: S, min_size: Option<S>) -> Self 
    where T: Into<String>,
          S: Into<(u32, u32)>,
    {
        let size: (u32, u32) = size.into();
        
        let builder = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(size.0, size.1);
        
        let builder = {
            if let Some(min_size) = min_size {
                let min_size: (u32, u32) = min_size.into();
                builder.with_min_dimensions(min_size.0, min_size.1)
            } else {
                builder
            }
        };
        let mut window = builder.build().unwrap();
        unsafe { window.make_current() };
        gl::load_with(|s| window.get_proc_address(s) as *const _);

        let opengl = OpenGL::V3_2;
        let samples = 4;
        let context = GfxContext::new(&mut window, opengl, samples);

        Window {
            window: window,
            context: context,
        }
    }
    pub fn viewport(&self) -> Viewport {
        Viewport {
            rect: [0, 0, self.draw_size().0 as i32, self.draw_size().1 as i32],
            window_size: [self.size().0, self.size().1],
            draw_size: [self.draw_size().0, self.draw_size().1],
        }
    }
    fn size(&self) -> (u32, u32) {
        self.window.get_inner_size().unwrap_or((0, 0)).into()
    }
    fn draw_size(&self) -> (u32, u32) {
        self.window.get_inner_size_pixels().unwrap_or((0, 0)).into()
    }

    /// Renders 2D graphics.
    pub fn draw_2d<F, U>(&mut self, f: F) -> U where
        F: FnOnce(Context, &mut G2d) -> U
    {
        self.make_current();
        let viewport = self.viewport();
        let res = self.context.draw_2d(f, viewport);
        self.window.swap_buffers();
        self.context.after_render();
        res
    }    
    pub fn window_resized(&mut self) {
        let draw_size = self.draw_size();
        self.context.check_resize(draw_size);
    }

    fn make_current(&mut self) {
        unsafe {
            self.window.make_current().unwrap()
        }
    }
}