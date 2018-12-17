use crate::window::WindowImpl;
use crate::window::WindowDimensions;
use crate::element::Element;
use crate::platform;
use crate::mouse::{Mouse, MouseHandler, MouseButton, MouseCursor};

pub struct WindowHandle {
}

pub fn create_platform_window() -> Box<WindowImpl> {
    unimplemented!();
}

pub struct X11 {
}

impl WindowImpl for X11 {
    fn open(&mut self, dimensions: WindowDimensions, title: &str, parent: Option<platform::WindowHandle>) {
        unimplemented!();
    }

    fn draw(&mut self, force_redraw: bool) -> bool {
        unimplemented!();
    }

    fn focus_element(&mut self, element: &mut Element) {
        unimplemented!();
    }

    fn lock(&mut self) {
        unimplemented!();
    }

    fn unlock(&mut self) {
        unimplemented!();
    }

    fn reinit(&mut self) {
        unimplemented!();
    }
}

impl Drop for X11 {
    fn drop(&mut self) {
        unimplemented!();
    }
}

impl MouseHandler for X11 {
    fn mouse_press(&mut self, button: MouseButton, x: isize, y: isize) {
        unimplemented!();
    }

    fn mouse_release(&mut self, button: MouseButton, x: isize, y: isize) {
        unimplemented!();
    }

    fn mouse_motion(&mut self, x: isize, y: isize) {
        unimplemented!();
    }

    fn mouse_wheel(&mut self, x: isize, y: isize, delta: f32) {
        unimplemented!();
    }

    fn mouse_enter_window(&mut self, x: isize, y: isize) {
        unimplemented!();
    }

    fn mouse_leave_window(&mut self, x: isize, y: isize) {
        unimplemented!();
    }

    fn mouse_double_click_interval(&self) -> i64 {
        unimplemented!();
    }

    fn set_cursor(&mut self, mouse: &mut Mouse, cursor: MouseCursor) {
        unimplemented!();
    }

    fn mouse_pointer_wrap(&self, x: isize, y: isize) {
        unimplemented!();
    }

    fn copy_to_clipboard(&self, buffer: &[u8]) {
        unimplemented!();
    }

    fn paste_from_clipboard(&self, buffer: &mut [u8]) -> isize {
        unimplemented!();
    }
}