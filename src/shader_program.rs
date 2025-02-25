use crate::player::Player;
use glam::Mat4;

#[derive(Clone)]
pub struct ShaderProgram {
    chunk: u32,
}

impl ShaderProgram {
    pub fn new(player: &Player) -> Self {
        let chunk = unsafe {
            let vertex_shader =
                compile_shader(include_str!("shaders/chunk.vert"), gl::VERTEX_SHADER);
            let fragment_shader =
                compile_shader(include_str!("shaders/chunk.frag"), gl::FRAGMENT_SHADER);
            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
            program
        };

        let mut this = Self { chunk };
        this.set_uniforms_on_init(player);
        this
    }

    fn set_uniforms_on_init(&mut self, player: &Player) {
        unsafe {
            gl::UseProgram(self.chunk);
            set_uniform_mat4(self.chunk, "m_proj", player.m_proj());
            set_uniform_mat4(self.chunk, "m_model", &Mat4::IDENTITY);
        }
    }

    pub fn update(&self, player: &Player) {
        unsafe {
            gl::UseProgram(self.chunk);
            set_uniform_mat4(self.chunk, "m_view", player.m_view());
        }
    }

    pub fn chunk_program(&self) -> u32 { self.chunk }
}

unsafe fn compile_shader(source: &str, shader_type: u32) -> u32 {
    let shader = gl::CreateShader(shader_type);
    gl::ShaderSource(
        shader,
        1,
        &(source.as_ptr() as *const _),
        &(source.len() as i32),
    );
    gl::CompileShader(shader);
    let mut success = 0;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
    if success == gl::FALSE as i32 {
        let mut len = 0;
        gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
        let mut buffer = Vec::with_capacity(len as usize);
        buffer.set_len((len as usize) - 1);
        gl::GetShaderInfoLog(
            shader,
            len,
            std::ptr::null_mut(),
            buffer.as_mut_ptr() as *mut _,
        );
        panic!(
            "Shader compilation error: {}",
            String::from_utf8_lossy(&buffer)
        );
    }
    shader
}

unsafe fn set_uniform_mat4(program: u32, name: &str, matrix: &Mat4) {
    let loc = gl::GetUniformLocation(program, format!("{}\0", name).as_ptr() as *const _);
    gl::UniformMatrix4fv(loc, 1, gl::FALSE, matrix.as_ref().as_ptr());
}
