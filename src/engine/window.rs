use glfw::*;

pub struct Window {
    pub handle: PWindow,
    pub events: GlfwReceiver<(f64, WindowEvent)>,
    // window, logical coordinates
    pub width: u32,
    pub height: u32,
    // framebuffer, in pixels
    pub fb_width: i32,
    pub fb_height: i32,

    // viewport is a rendering region on framebuffer
    // ui, mouse and window should use window coordinate system
    // rendering, viewport and rendering resolutions should use framebuffer pixels coordinate system
}

impl Window {
    fn new(title: &str, width: u32, height: u32, glfw: &mut Glfw) -> Self {

        let (mut handle, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create window");

        let (fb_width, fb_height) = handle.get_framebuffer_size();

        handle.make_current();
        handle.set_key_polling(true);
        handle.set_mouse_button_polling(true);
        handle.set_cursor_pos_polling(true);

        Self {
            handle,
            events,
            width,
            height,
            fb_width,
            fb_height
        }
    }

    fn should_close(&self) -> bool {
        self.handle.should_close()
    }

    fn set_cursor_mode(&mut self, mode: glfw::CursorMode) {
        self.handle.set_cursor_mode(mode)
    }
}