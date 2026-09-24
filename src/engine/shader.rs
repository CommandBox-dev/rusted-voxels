use std::ffi::CString;

pub fn load_shader(vert_path: &str, frag_path: &str) -> u32 {

    let vertex_source = std::fs::read_to_string(vert_path).expect("Failed to read vertex shader");
    let fragment_source = std::fs::read_to_string(frag_path).expect("Failed to read fragment shader");

    let vertex_shader = compile_shader(&vertex_source, gl::VERTEX_SHADER).expect("Vertex shader compilation failed");
    let fragment_shader = compile_shader(&fragment_source, gl::FRAGMENT_SHADER).expect("Fragment shader compilation failed");

    let program = link_program(vertex_shader,fragment_shader).expect("Shader linking failed");

    // the shaders aren't needed anymore once linked
    unsafe {
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }

    program
}

fn compile_shader(source: &str, shader_type: u32) -> Result<u32, String> {

    let c_source = CString::new(source).map_err(|_| "Shader source contains a null byte".to_string())?;
    let shader;

    unsafe {
        shader = gl::CreateShader(shader_type);

        gl::ShaderSource(shader, 1, &c_source.as_ptr(), std::ptr::null());

        gl::CompileShader(shader);

        let mut success = 0;

        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

        if success == 0 {
            let mut len = 0;

            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

            let mut buffer = vec![0u8; len as usize];

            gl::GetShaderInfoLog(shader, len, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut i8,);
            gl::DeleteShader(shader);
            let log = String::from_utf8_lossy(&buffer).trim_end_matches('\0').to_string();

            return Err(log);
        }
    }

    Ok(shader)
}

fn link_program(vertex_shader: u32, fragment_shader: u32) -> Result<u32, String> {
    let program;

    unsafe {
        program = gl::CreateProgram();

        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);

        gl::LinkProgram(program);

        let mut success = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);

        // failed linking
        if success == 0 {
            let mut len = 0;

            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);

            let mut buffer = vec![0u8; len as usize];

            gl::GetProgramInfoLog(program, len, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut i8);
            gl::DeleteProgram(program);

            let log = String::from_utf8_lossy(&buffer).trim_end_matches('\0').to_string();

            return Err(log);
        }
    }

    Ok(program)
}