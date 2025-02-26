use crate::meshes::chunk_mesh::ChunkMesh;
use crate::settings::{CHUNK_AREA, CHUNK_SIZE, CHUNK_VOL};
use crate::shader_program::{ShaderProgram, set_uniform_mat4};
use crate::world::World;
use glam::{IVec3, Mat4};
use noise::{NoiseFn, Simplex};
use gl;
use rand::Rng;

pub struct Chunk {
    pub voxels: Vec<u8>,
    pub mesh: Option<ChunkMesh>,
    pub shader_program: ShaderProgram,
    pub position: IVec3,
    pub m_model: Mat4,
    pub world: *const World, // Для майбутньої оптимізації
}

impl Chunk {
    pub fn new(shader_program: &ShaderProgram, position: IVec3, world: &World) -> Self {
        let m_model = Mat4::from_translation(position.as_vec3() * CHUNK_SIZE as f32);
        let voxels = Self::build_voxels(position);
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

    fn build_voxels(position: IVec3) -> Vec<u8> {
        let mut voxels = vec![0u8; CHUNK_VOL as usize];
        let simplex = Simplex::new(0); // Можна передати сід із World, якщо потрібно
        let (cx, cy, cz) = (position.x, position.y, position.z);

        let rng = rand::thread_rng().gen_range(1..=100);

        for x in 0..CHUNK_SIZE {
            let wx = x as f32 + cx as f32 * CHUNK_SIZE as f32;
            for z in 0..CHUNK_SIZE {
                let wz = z as f32 + cz as f32 * CHUNK_SIZE as f32;
                let noise_value = simplex.get([wx as f64 * 0.01, wz as f64 * 0.01]);
                let world_height = (noise_value * 32.0 + 32.0) as i32;
                let local_height = (world_height - cy * CHUNK_SIZE as i32).clamp(0, CHUNK_SIZE as i32);

                for y in 0..local_height as u32 {
                    let wy = y as f32 + cy as f32 * CHUNK_SIZE as f32;
                    voxels[(x + CHUNK_SIZE * z + CHUNK_AREA * y) as usize] = rng as u8;
                }
            }
        }
        voxels
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