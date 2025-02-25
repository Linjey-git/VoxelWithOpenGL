use crate::meshes::base_mesh::BaseMesh;
use crate::world_objects::Chunk;
use crate::meshes::chunk_mesh_builder::build_chunk_mesh;

pub struct ChunkMesh {
    base: BaseMesh,
}

impl ChunkMesh {
    pub fn new(chunk: &Chunk) -> Self {
        let format_size = 5; // 3u1 (позиція) + 1u1 (voxel_id) + 1u1 (face_id)
        let vertex_data = build_chunk_mesh(&chunk.voxels, format_size);
        let vertex_count = vertex_data.len() as i32 / format_size;
        let attrs = [(0, 3), (1, 1), (2, 1)]; // Позиція, voxel_id, face_id
        Self {
            base: BaseMesh::new(chunk.shader_program.chunk_program(), &vertex_data, &attrs, vertex_count),
        }
    }

    pub fn render(&self) {
        self.base.render();
    }
}