pub struct BaseMesh {
    vao: u32,
    vertex_count: i32,
}

impl BaseMesh {
    pub fn new(program: u32, vertex_data: &[f32], attrs: &[(u32, i32)], vertex_count: i32) -> Self {
        let mut vao = 0;
        let mut vbo = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertex_data.len() * std::mem::size_of::<f32>()) as isize,
                vertex_data.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let mut offset = 0;
            let stride = attrs.iter().map(|(_, size)| size).sum::<i32>() * std::mem::size_of::<f32>() as i32;
            for (location, size) in attrs {
                gl::VertexAttribPointer(*location, *size, gl::FLOAT, gl::FALSE, stride, offset as *const _);
                gl::EnableVertexAttribArray(*location);
                offset += *size as usize * std::mem::size_of::<f32>();
            }

            gl::BindVertexArray(0);
        }

        Self { vao, vertex_count }
    }

    pub fn render(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count);
        }
    }
}