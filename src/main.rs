#![allow(unused)]

mod engine;

use crate::engine::{chunk::{self, Chunk}, chunk_renderer::{self, render_chunks}, raycast, shader::load_shader, world};
use crate::engine::world::World;

use glfw::{Action, Context, Key, MouseButton, ffi::glfwGetFramebufferSize};
use glam::{Mat4, Vec2, Vec3};
use rand::rand_core::utils::Word;


fn main() {

    let mut time_last_frame = std::time::Instant::now();
    let mut delta: f32;

    let mut frame_acc = 0;
    let mut time_acc = 0.0;
    
    let window_width = 800;
    let window_height = 600;

    //let viewport_width = 0;
    //let viewport_height = 0;

    let mut last_mouse_x = window_width as f32 / 2.0;
    let mut last_mouse_y = window_height as f32 / 2.0;
    let mut first_mouse = true;
    
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    
    let (mut window, events) = glfw
        .create_window(window_width, window_height, "rusted-voxels", glfw::WindowMode::Windowed)
        .expect("Failed to create window");

    window.make_current();
    window.set_key_polling(true);
    window.set_mouse_button_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    // load opengl function pointers.
    gl::load_with(|symbol| {
        window
        .get_proc_address(symbol)
        .map_or(std::ptr::null(), |p| p as *const _)
    });

    let (fb_width, fb_height) = window.get_framebuffer_size();

    let mut camera = engine::camera::Camera::new(
        Vec3::new((world::RENDER_DISTANCE / 2) as f32, 258.0, (world::RENDER_DISTANCE / 2) as f32),
        180.0,
        0.0,
        fb_width as u32,
        fb_height as u32,
        75.0
    );

    unsafe {
        gl::Viewport(0, 0, fb_width, fb_height);
        gl::Enable(gl::DEPTH_TEST);
        //gl::Enable(gl::CULL_FACE);
        //gl::CullFace(gl::BACK);
    }

    // load shader and get shader uniform locations
    let base_shader = load_shader("res/shaders/basic.vert", "res/shaders/basic.frag");

    let model_loc = unsafe {
        gl::GetUniformLocation(base_shader, c"u_model".as_ptr())
    };

    let view_loc = unsafe {
        gl::GetUniformLocation(base_shader, c"u_view".as_ptr())
    };

    let projection_loc = unsafe {
        gl::GetUniformLocation(base_shader, c"u_projection".as_ptr())
    };

    let mut world = World::new();
    
    // game loop
    while !window.should_close() {
        glfw.poll_events();
        
        let now = std::time::Instant::now();
        delta = now.duration_since(time_last_frame).as_secs_f32();

        if time_acc > 1.0 {
            //println!("{}", frame_acc);
            frame_acc = 0;
            time_acc = 0.0;
        }
        //println!("{}", delta);

        // handle keybord input
        
        for (_, event) in glfw::flush_messages(&events) {
            //println!("{:?}", event);
            match event{

                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }

                glfw::WindowEvent::MouseButton(MouseButton::Left, Action::Press, _) => {
                    //println!("{:?}, {:?}", button, action);
                    let result = raycast::cast_ray(camera.position, camera.front, 15.0, &mut world);
                    if result.hit {
                        &world.set_block(
                            result.hit_position.x,
                            result.hit_position.y,
                            result.hit_position.z,
                            0
                        );
                    }
                }

                glfw::WindowEvent::MouseButton(MouseButton::Right, Action::Press, _) => {
                    //println!("{:?}, {:?}", button, action);
                    let result = raycast::cast_ray(camera.position, camera.front, 15.0, &mut world);
                    if result.hit {
                        &world.set_block(
                            result.hit_position.x + result.hit_normal.x,
                            result.hit_position.y + result.hit_normal.y,
                            result.hit_position.z + result.hit_normal.z,
                            2
                        );
                    }
                }

                glfw::WindowEvent::CursorPos(x, y) => {
                    let x = x as f32;
                    let y = y as f32;

                    if first_mouse {
                        last_mouse_x = x;
                        last_mouse_y = y;
                        first_mouse = false;
                        continue;
                    }

                    let mut x_offset = x - last_mouse_x;
                    let mut y_offset = y - last_mouse_y;

                    last_mouse_x = x;
                    last_mouse_y = y;

                    let sensitivity = 0.1;

                    x_offset *= sensitivity;
                    y_offset *= sensitivity;

                    camera.yaw += -x_offset;
                    camera.pitch += -y_offset;

                    camera.pitch = camera.pitch.clamp(-89.0, 89.0);
                    camera.recalculate_vectors();
                    //println!("mmx: {}, mmy: {}", x_offset, y_offset);
                }

                _ => {}
            }
        }

        let mut front: i32 = 0;
        let mut back: i32 = 0;
        let mut right: i32 = 0;
        let mut left: i32 = 0;

        if window.get_key(Key::W) == Action::Press {
            front = 1;
        } else {
            front = 0;
        }
        if window.get_key(Key::S) == Action::Press {
            back = 1;
        } else {
            back = 0;
        }
        if window.get_key(Key::D) == Action::Press {
            right = 1;
        } else {
            right = 0;
        }
        if window.get_key(Key::A) == Action::Press {
            left = 1;
        } else {
            left = 0;
        }

        let input = Vec2::new((front - back) as f32, -(right - left) as f32);
        let mut speed = 8.0;

        if window.get_key(Key::LeftShift) == Action::Press {
            speed *= 3.0;
        }

        camera.position += camera.front * input.x * delta * speed;
        camera.position += camera.right * input.y * delta * speed;
        camera.recalculate_view();
        
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            
            gl::UseProgram(base_shader);
            
            // bind camera matrices
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, camera.view.to_cols_array().as_ptr());
            gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, camera.projection.to_cols_array().as_ptr());

            chunk_renderer::render_chunks(&mut world, model_loc);
            
            gl::BindVertexArray(0);
        }
        
        window.swap_buffers();
        time_last_frame = now;
        frame_acc += 1;
        time_acc += delta;
    }
}