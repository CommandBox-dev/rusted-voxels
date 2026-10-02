use glfw::{ffi::{GLFW_KEY_LAST, GLFW_MOUSE_BUTTON_LAST, glfwSetKeyCallback}};
use glfw::*;
use crate::engine::window::*;

pub struct Input {
    // keys
    down: [bool; (GLFW_KEY_LAST + 1) as usize],
    pressed: [bool; (GLFW_KEY_LAST + 1) as usize],
    released: [bool; (GLFW_KEY_LAST + 1) as usize],
    // mouse
    first_mouse: bool,
    last_mouse_x: f32,
    last_mouse_y: f32,
    mouse_move_x: f32,
    mouse_move_y: f32,
    mouse_down: [bool; (GLFW_MOUSE_BUTTON_LAST + 1) as usize],
    mouse_pressed: [bool; (GLFW_MOUSE_BUTTON_LAST + 1) as usize],
    mouse_released: [bool; (GLFW_MOUSE_BUTTON_LAST + 1) as usize],
}

impl Input {

    pub fn new(window: &GameWindow) -> Self {
        Self {
            down: [false; (GLFW_KEY_LAST + 1) as usize],
            pressed: [false; (GLFW_KEY_LAST + 1) as usize],
            released: [false; (GLFW_KEY_LAST + 1) as usize],

            last_mouse_x: window.width as f32 / 2.0,
            last_mouse_y: window.height as f32 / 2.0,
            mouse_move_x: 0.0,
            mouse_move_y: 0.0,
            first_mouse: true,

            mouse_down: [false; (GLFW_MOUSE_BUTTON_LAST + 1) as usize],
            mouse_pressed: [false; (GLFW_MOUSE_BUTTON_LAST + 1) as usize],
            mouse_released: [false; (GLFW_MOUSE_BUTTON_LAST + 1) as usize],
        }
    }

    pub fn update(&mut self, window: &GameWindow) {
        self.pressed.fill(false);
        self.released.fill(false);
        self.mouse_pressed.fill(false);
        self.mouse_released.fill(false);
        self.mouse_move_x = 0.0;
        self.mouse_move_y = 0.0;

        for (_, event) in glfw::flush_messages(&window.events) {

            if let WindowEvent::Key(key, _, action, _) = event {
                let index = key as usize;

                match action {
                    Action::Press =>  {
                        self.pressed[index] = true;
                        self.down[index] = true;
                    }
                    Action::Release =>  {
                        self.released[index] = true;
                        self.down[index] = false;
                    }
                    Action::Repeat =>  {}
                }

            } else if let glfw::WindowEvent::CursorPos(x, y) = event {

                let x = x as f32;
                let y = y as f32;

                if self.first_mouse {
                    self.last_mouse_x = x;
                    self.last_mouse_y = y;
                    self.first_mouse = false;
                    continue;
                }

                let mut x_offset = x - self.last_mouse_x;
                let mut y_offset = y - self.last_mouse_y;

                self.last_mouse_x = x;
                self.last_mouse_y = y;

                let sensitivity = 0.1;

                x_offset *= sensitivity;
                y_offset *= sensitivity;

                self.mouse_move_x = x_offset;
                self.mouse_move_y = y_offset;

            } else if let glfw::WindowEvent::MouseButton(button, action, _) = event {
                let index = button as usize;

                match action {
                    Action::Press =>  {
                        self.mouse_pressed[index] = true;
                        self.mouse_down[index] = true;
                    }
                    Action::Release =>  {
                        self.mouse_released[index] = true;
                        self.mouse_down[index] = false;
                    }
                    Action::Repeat =>  {}
                }
            }
        }
    }

    pub fn is_down(&self, key: i32) -> bool {
        return self.down[key as usize];
    }

    pub fn is_pressed(&self, key: i32) -> bool {
        return self.pressed[key as usize];
    }

    pub fn is_released(&self, key: i32) -> bool {
        return self.released[key as usize];
    }

    pub fn is_mouse_button_down(&self, key: i32) -> bool {
        return self.mouse_down[key as usize];
    }

    pub fn is_mouse_button_pressed(&self, key: i32) -> bool {
        return self.mouse_pressed[key as usize];
    }

    pub fn is_mouse_button_released(&self, key: i32) -> bool {
        return self.mouse_released[key as usize];
    }

    pub fn get_last_mouse_movement(&self) -> (f32, f32) {
        (self.mouse_move_x, self.mouse_move_y)
    }
}

pub struct Keybind {
    key_id: i32 // just a single key per keybind for now
}