use std::time::Instant;

use glfw::*;

use crate::engine::input::Input;

pub struct GameWindow {
    pub handle: PWindow,
    pub events: GlfwReceiver<(f64, WindowEvent)>,
    pub glfw: Glfw,
    // window, logical coordinates
    pub width: u32,
    pub height: u32,
    // framebuffer, in pixels
    pub fb_width: i32,
    pub fb_height: i32,

    // viewport is a rendering region on framebuffer
    // ui, mouse and window should use window coordinate system
    // rendering, viewport and rendering resolutions should use framebuffer pixels coordinate system

    pub time: f32,
    pub delta: f32,
    start_time: Instant,
    time_last_frame: Instant,
}

impl GameWindow { // renamed Window to GameWindow to avoid confusion with the glfw Window struct
    pub fn new(title: &str, width: u32, height: u32) -> Self {

        // glfw initialization
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

        // window creation
        let (mut handle, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create window");

        let (fb_width, fb_height) = handle.get_framebuffer_size();

        handle.make_current();
        handle.set_key_polling(true);
        handle.set_mouse_button_polling(true);
        handle.set_cursor_pos_polling(true);

        // opengl initialization (temporary, move into renderer script later)
        gl::load_with(|symbol| {
            handle
            .get_proc_address(symbol)
            .map_or(std::ptr::null(), |p| p as *const _)
        });

        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Viewport(0, 0, fb_width, fb_height);
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);
        }

        Self {
            handle,
            events,
            glfw,
            width,
            height,
            fb_width,
            fb_height,
            time: 0.0,
            delta: 0.0,
            start_time: Instant::now(),
            time_last_frame: Instant::now(),
        }
    }

    pub fn should_close(&self) -> bool {
        self.handle.should_close()
    }

    pub fn set_cursor_mode(&mut self, mode: glfw::CursorMode) {
        self.handle.set_cursor_mode(mode)
    }

    pub fn poll_events(&mut self) {
        self.glfw.poll_events();

        let now = Instant::now();
        self.time = self.start_time.elapsed().as_secs_f32();
        self.delta = now.duration_since(self.time_last_frame).as_secs_f32();
        self.time_last_frame = now;
    }

    pub fn swap_buffers(&mut self) {
        self.handle.swap_buffers();
    }
}