#![allow(unused)]

mod engine;

use crate::engine::{chunk::{self, Chunk}, raycast, shader::load_shader};

use glfw::{Action, Context, Key, MouseButton};
use glam::{Mat4, Vec2, Vec3};


fn main() {

    let mut time_last_frame = std::time::Instant::now();
    let mut delta: f32;

    let mut cube_rot: f32 = 0.0;
    let mut frame_acc = 0;
    let mut time_acc = 0.0;

    let vertices: [f32; 108] = [
        // back
        -0.5, -0.5, -0.5,
        0.5, -0.5, -0.5,
        0.5,  0.5, -0.5,
        0.5,  0.5, -0.5,
        -0.5,  0.5, -0.5,
        -0.5, -0.5, -0.5,

        // front
        -0.5, -0.5,  0.5,
        0.5, -0.5,  0.5,
        0.5,  0.5,  0.5,
        0.5,  0.5,  0.5,
        -0.5,  0.5,  0.5,
        -0.5, -0.5,  0.5,

        // left
        -0.5,  0.5,  0.5,
        -0.5,  0.5, -0.5,
        -0.5, -0.5, -0.5,
        -0.5, -0.5, -0.5,
        -0.5, -0.5,  0.5,
        -0.5,  0.5,  0.5,

        // right
        0.5,  0.5,  0.5,
        0.5,  0.5, -0.5,
        0.5, -0.5, -0.5,
        0.5, -0.5, -0.5,
        0.5, -0.5,  0.5,
        0.5,  0.5,  0.5,

        // bottom
        -0.5, -0.5, -0.5,
        0.5, -0.5, -0.5,
        0.5, -0.5,  0.5,
        0.5, -0.5,  0.5,
        -0.5, -0.5,  0.5,
        -0.5, -0.5, -0.5,

        // top
        -0.5,  0.5, -0.5,
        0.5,  0.5, -0.5,
        0.5,  0.5,  0.5,
        0.5,  0.5,  0.5,
        -0.5,  0.5,  0.5,
        -0.5,  0.5, -0.5,
    ];
    
    let width = 800;
    let height = 600;

    let mut camera = engine::camera::Camera::new(Vec3::new(0.0, 0.0, 3.0), 180.0, 0.0, width, height, 75.0);

    let mut chunk = Chunk::new(0, 0);

    let mut last_mouse_x = width as f32 / 2.0;
    let mut last_mouse_y = height as f32 / 2.0;
    let mut first_mouse = true;

    let mut chunk = Chunk::new(0, 0);

    // cube
    let model = Mat4::IDENTITY;
    let mut vao = 0;
    let mut vbo = 0;
    
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    
    let (mut window, events) = glfw
        .create_window(width, height, "rusted-voxels", glfw::WindowMode::Windowed)
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

    let mesh = chunk.build_mesh();

    unsafe {
        gl::Viewport(0, 0, width as i32, height as i32);
        gl::Enable(gl::DEPTH_TEST);
        
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (mesh.len() as usize * std::mem::size_of::<f32>()) as isize,
            mesh.as_ptr() as *const _,
             gl::STATIC_DRAW
        );

        /*
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as isize,
            vertices.as_ptr() as *const _, gl::STATIC_DRAW);
        */
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<f32>() as i32, std::ptr::null());
        
        
        gl::EnableVertexAttribArray(0);
        gl::BindVertexArray(0);
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
                    let result = raycast::cast_ray(camera.position, camera.front, 10.0, &chunk);
                    if result.hit {
                        &chunk.set_block(
                            result.hit_position.x as i32,
                            result.hit_position.y as i32,
                            result.hit_position.z as i32,
                            0
                        );
                        //println!("{}", result.hit_position);
                        let mesh = chunk.build_mesh();
                        unsafe {


                            gl::DeleteVertexArrays(1, &vao);
                            gl::DeleteBuffers(1, &vbo);

                            gl::GenVertexArrays(1, &mut vao);
                            gl::GenBuffers(1, &mut vbo);

                            gl::BindVertexArray(vao);

                            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

                            gl::BufferData(
                                gl::ARRAY_BUFFER,
                                (mesh.len() as usize * std::mem::size_of::<f32>()) as isize,
                                mesh.as_ptr() as *const _,
                                    gl::STATIC_DRAW
                            );

                            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<f32>() as i32, std::ptr::null());


                            gl::EnableVertexAttribArray(0);
                            gl::BindVertexArray(0);
                        }
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

                    camera.yaw += x_offset;
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

        let input = Vec2::new((front - back) as f32, (right - left) as f32);
        let speed = 5.0;
        camera.position += camera.front * input.x * delta * speed;
        camera.position += camera.right * input.y * delta * speed;
        camera.recalculate_view();
        //println!("{}", input);
        
        // recalculate the model matrix of the rotating cube
        cube_rot += delta;
        //let model = Mat4::from_rotation_y(cube_rot) * Mat4::from_rotation_x(cube_rot * 0.5);
        
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            
            gl::UseProgram(base_shader);
            
            gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model.to_cols_array().as_ptr());
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, camera.view.to_cols_array().as_ptr());
            gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, camera.projection.to_cols_array().as_ptr());
            
            gl::UseProgram(base_shader);
            gl::BindVertexArray(vao);
            
            gl::DrawArrays(gl::TRIANGLES, 0, (mesh.len() / 3) as i32);
            
            gl::BindVertexArray(0);
        }
        
        window.swap_buffers();
        time_last_frame = now;
        frame_acc += 1;
        time_acc += delta;
    }
}