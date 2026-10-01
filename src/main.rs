#![allow(unused)]

mod engine;

use crate::engine::*;
use crate::engine::chunk::*;
use crate::engine::world::*;
use crate::engine::shader::*;
use crate::engine::texture::*;
use crate::engine::block_model::*;
use crate::engine::physics::*;
use crate::engine::physics::aabb::move_and_collide;
use crate::engine::physics::aabb::*;
use crate::engine::physics::aabb::AABB;

use glfw::{Action, Context, Key, MouseButton, ffi::glfwGetFramebufferSize};
use glam::{Mat4, Vec2, Vec3, IVec3};
use rand::rand_core::utils::Word;

fn main() {

    let hit_box = AABB::HUMANOID;

    let mut time_last_frame = std::time::Instant::now();
    let mut delta: f32;

    let mut frame_acc = 0;
    let mut time_acc = 0.0;
    
    let window_width = 1920;
    let window_height = 1080;

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

    let mut is_on_floor = false;
    let mut velocity = Vec3::new(0.0, 0.0, 0.0);

    let mut escape = false;

    gl::load_with(|symbol| {
        window
        .get_proc_address(symbol)
        .map_or(std::ptr::null(), |p| p as *const _)
    });

    let (fb_width, fb_height) = window.get_framebuffer_size();

    let mut camera = engine::camera::Camera::new(
        Vec3::new(((world::RENDER_DISTANCE / 2) * CHUNK_WIDTH as u32) as f32, 105.0, ((world::RENDER_DISTANCE / 2) * CHUNK_WIDTH as u32) as f32),
        180.0,
        0.0,
        fb_width as u32,
        fb_height as u32,
        75.0
    );

    unsafe {
        gl::Viewport(0, 0, fb_width, fb_height);
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::CULL_FACE);
        gl::CullFace(gl::BACK);
    }

    let base_shader = load_shader("./res/shaders/basic.vert", "./res/shaders/basic.frag");
    let texture_atlas = load_texture_atlas("./res/textures");

    let mut selected_block: u32 = 3;

    let base_texture = texture_atlas.id;

    let mut last_place_time: f32 = 0.0;
    let mut locked_placing_axis = false;
    let mut locked_placing_dir = IVec3::new(0, 0, 0);
    let mut locked_placing_normal = IVec3::new(0, 0, 0);

    //let uv_cobblestone = texture_atlas.get_tile(String::from("cobblestone"));
    let uv_cobblestone = texture_atlas.get_tile(String::from("stone"));
    let uv_wall = texture_atlas.get_tile(String::from("wall2"));
    let uv_crate = texture_atlas.get_tile(String::from("crate"));
    let uv_grass_top = texture_atlas.get_tile(String::from("grass"));
    let uv_grass_side = texture_atlas.get_tile(String::from("grass_side"));
    let uv_log_side = texture_atlas.get_tile(String::from("log"));

    let block_models = vec![
        BlockModel::new_cube(uv_cobblestone),
        BlockModel::new_cube(uv_wall),
        BlockModel::new_cube(uv_crate),
        BlockModel::new_cube_sides_top_bottom(
            uv_grass_side,
            uv_grass_top,
            uv_cobblestone,
        ),
        BlockModel::new_cube(uv_log_side),
    ];

    let model_loc = unsafe {
        gl::GetUniformLocation(base_shader, c"u_model".as_ptr())
    };

    let view_loc = unsafe {
        gl::GetUniformLocation(base_shader, c"u_view".as_ptr())
    };

    let projection_loc = unsafe {
        gl::GetUniformLocation(base_shader, c"u_projection".as_ptr())
    };

    let texture_loc = unsafe {
        gl::GetUniformLocation(base_shader, c"baseTexture".as_ptr())
    };

    let mut world = World::new(&block_models);
    
    // game loop
    while !window.should_close() {
        glfw.poll_events();
        
        let now = std::time::Instant::now();
        delta = now.duration_since(time_last_frame).as_secs_f32();

        if time_acc > 1.0 {
            frame_acc = 0;
            time_acc = 0.0;
        }
        
        for (_, event) in glfw::flush_messages(&events) {
            //println!("{:?}", event);
            match event {

                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    //window.set_should_close(true);
                    escape = !escape;
                    if escape {
                        window.set_cursor_mode(glfw::CursorMode::Normal);
                    } else {
                        window.set_cursor_mode(glfw::CursorMode::Disabled);
                    }
                }

                glfw::WindowEvent::MouseButton(MouseButton::Left, Action::Press, _) => {
                    //println!("{:?}, {:?}", button, action);
                    let result = raycast::cast_ray(camera.position, camera.front, 15.0, &mut world);
                    if result.hit {
                        &world.set_block(
                            result.hit_position.x,
                            result.hit_position.y,
                            result.hit_position.z,
                            0,
                        );
                    }
                }

                glfw::WindowEvent::MouseButton(MouseButton::Right, Action::Press, _) => {
                    let result = raycast::cast_ray(camera.position, camera.front, 15.0, &mut world);
                    if result.hit {
                        &world.set_block(
                            result.hit_position.x + result.hit_normal.x,
                            result.hit_position.y + result.hit_normal.y,
                            result.hit_position.z + result.hit_normal.z,
                            selected_block,
                        );
                    }
                }

                glfw::WindowEvent::MouseButton(MouseButton::Middle, Action::Press, _) => {
                    let result = raycast::cast_ray(camera.position, camera.front, 15.0, &mut world);
                    if result.hit {
                        selected_block = result.hit_block;
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

                    if !escape {
                        camera.yaw += -x_offset;
                        camera.pitch += -y_offset;

                        camera.pitch = camera.pitch.clamp(-89.0, 89.0);
                        camera.recalculate_vectors();
                    }
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
        }
        if window.get_key(Key::S) == Action::Press {
            back = 1;
        }
        if window.get_key(Key::D) == Action::Press {
            right = 1;
        }
        if window.get_key(Key::A) == Action::Press {
            left = 1;
        }
        if window.get_key(Key::Space) == Action::Press {
            velocity.y = 0.1;
        } else {
            velocity.y += (-9.8 * 0.045 * delta);
        }
        if window.get_mouse_button(MouseButton::Right) == Action::Press {
            if last_place_time > 0.1 {
                if locked_placing_axis == false {
                    let result = raycast::cast_ray(camera.position, camera.front, 15.0, &mut world);
                    if result.hit {
                        &world.set_block(
                            result.hit_position.x + result.hit_normal.x,
                            result.hit_position.y + result.hit_normal.y,
                            result.hit_position.z + result.hit_normal.z,
                            selected_block,
                        );
                        last_place_time = 0.0;
                        locked_placing_axis = true;
                        locked_placing_dir = result.hit_position + result.hit_normal + result.hit_normal;
                        locked_placing_normal = result.hit_normal;
                    }
                } else {
                    let result = raycast::cast_ray(camera.position, camera.front, 15.0, &mut world);
                    if result.hit &&
                        result.hit_position + result.hit_normal == locked_placing_dir
                    {
                        &world.set_block(
                            result.hit_position.x + result.hit_normal.x,
                            result.hit_position.y + result.hit_normal.y,
                            result.hit_position.z + result.hit_normal.z,
                            selected_block,
                        );
                        locked_placing_dir = result.hit_position + locked_placing_normal;
                        last_place_time = 0.0;
                    }

                }
            }
        } else {
            last_place_time = 0.0;
            locked_placing_axis = false;
        }

        last_place_time += delta;
        
        let input = Vec2::new((front - back) as f32, -(right - left) as f32);
        let mut speed = 8.0;
        
        if window.get_key(Key::LeftShift) == Action::Press {
            speed *= 3.0;
        }
        
        let vel_y = velocity.y;
        velocity.y *= 0.90;

        velocity = camera.forward * input.x * delta * speed;
        velocity += camera.right * input.y * delta * speed;
        velocity.y += vel_y;

        let result = move_and_collide(&world, &camera.position, &mut velocity, &hit_box);
        is_on_floor = result.is_on_floor;

        camera.position += velocity;

        camera.recalculate_view();

        world.update_dirty_chunks(&block_models);
        
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            
            gl::UseProgram(base_shader);
            bind(base_texture, 0);
            
            // bind camera matrices
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, camera.view.to_cols_array().as_ptr());
            gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, camera.projection.to_cols_array().as_ptr());

            gl::Uniform1i(texture_loc, 0);

            chunk_renderer::render_chunks(&mut world, model_loc);
            
            gl::BindVertexArray(0);
        }
        
        window.swap_buffers();
        time_last_frame = now;
        frame_acc += 1;
        time_acc += delta;
    }
}