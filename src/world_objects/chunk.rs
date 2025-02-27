use crate::meshes::chunk_mesh::ChunkMesh;
use crate::settings::{CHUNK_AREA, CHUNK_SIZE, CHUNK_VOL};
use crate::shader_program::{ShaderProgram, set_uniform_mat4};
use crate::world::World;
use gl;
use glam::{IVec3, Mat4};

pub struct Chunk {
    pub voxels: Vec<u8>,
    pub mesh: Option<ChunkMesh>,
    pub shader_program: ShaderProgram,
    pub position: IVec3,
    pub m_model: Mat4,
    pub world: *const World,
}

impl Chunk {
    pub fn new(shader_program: &ShaderProgram, position: IVec3, voxels: Vec<u8>, world: &World) -> Self {
        let m_model = Mat4::from_translation(position.as_vec3() * CHUNK_SIZE as f32);
        let mut chunk = Self {
            voxels,
            mesh: None,
            shader_program: shader_program.clone(),
            position,
            m_model,
            world: world as *const World,
        };
        chunk.build_mesh();
        chunk
    }

    pub fn build_mesh(&mut self) {
        self.mesh = Some(ChunkMesh::new(self));
    }

    pub fn render(&self) {
        if let Some(mesh) = &self.mesh {
            unsafe {
                gl::UseProgram(self.shader_program.chunk_program());
                set_uniform_mat4(self.shader_program.chunk_program(), "m_model", &self.m_model);
            }
            mesh.render();
        }
    }
}