use crate::settings::{CHUNK_AREA, CHUNK_SIZE, CHUNK_VOL};
use crate::world::World;
use crate::world_objects::Chunk;
use glam::IVec3;

// fn is_void(
//     voxel_pos: (i32, i32, i32),
//     chunk_voxels: &[u8],
//     chunk_pos: IVec3,
//     world: &World,
// ) -> bool {
//     let (x, y, z) = voxel_pos;
//     if x >= 0
//         && x < CHUNK_SIZE as i32
//         && y >= 0
//         && y < CHUNK_SIZE as i32
//         && z >= 0
//         && z < CHUNK_SIZE as i32
//     {
//         if chunk_voxels[(x + CHUNK_SIZE as i32 * z + CHUNK_AREA as i32 * y) as usize] != 0 {
//             return false;
//         }
//     }
//     true
// }

fn get_chunk_index(world_voxel_pos: (i32, i32, i32), world: &World) -> Option<&Chunk> {
    let (wx, wy, wz) = world_voxel_pos;
    let cx = wx.div_euclid(CHUNK_SIZE as i32); // Цілочисельне ділення (аналог // у Python)
    let cy = wy.div_euclid(CHUNK_SIZE as i32);
    let cz = wz.div_euclid(CHUNK_SIZE as i32);

    // У Python є межі світу (WORLD_W, WORLD_H, WORLD_D), у Rust їх немає, тому просто повертаємо чанк із HashMap
    world.chunks.get(&IVec3::new(cx, cy, cz))
}

fn is_void(
    local_voxel_pos: (i32, i32, i32),
    world_voxel_pos: (i32, i32, i32),
    world: &World,
) -> bool {
    let chunk = get_chunk_index(world_voxel_pos, world);
    if chunk.is_none() {
        // Аналог chunk_index == -1 у Python, але в Rust для нескінченного світу повертаємо true (порожньо)
        return true;
    }
    let chunk_voxels = &chunk.unwrap().voxels;

    let (x, y, z) = local_voxel_pos;
    // Обчислюємо локальний індекс із врахуванням "обгортання" через % (modulo)
    let voxel_index = (x.rem_euclid(CHUNK_SIZE as i32)
        + z.rem_euclid(CHUNK_SIZE as i32) * CHUNK_SIZE as i32
        + y.rem_euclid(CHUNK_SIZE as i32) * CHUNK_AREA as i32) as usize;

    // Перевіряємо воксель (true, якщо порожній; false, якщо заповнений)
    chunk_voxels[voxel_index] == 0
}

fn add_data(vertex_data: &mut Vec<u8>, vertices: &[(u8, u8, u8, u8, u8)]) {
    for vertex in vertices {
        vertex_data.extend_from_slice(&[vertex.0, vertex.1, vertex.2, vertex.3, vertex.4]);
    }
}

pub fn build_chunk_mesh(
    chunk_voxels: &[u8],
    format_size: i32,
    chunk_pos: IVec3,
    world: &World,
) -> Vec<u8> {
    let mut vertex_data = Vec::with_capacity((CHUNK_VOL as usize * 18 * format_size as usize));

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let voxel_id = chunk_voxels[(x + CHUNK_SIZE * z + CHUNK_AREA * y) as usize];
                if voxel_id == 0 {
                    continue;
                }

                let x = x as u8;
                let y = y as u8;
                let z = z as u8;

                // Обчислюємо глобальні координати вокселя
                let wx = chunk_pos.x * CHUNK_SIZE as i32 + x as i32;
                let wy = chunk_pos.y * CHUNK_SIZE as i32 + y as i32;
                let wz = chunk_pos.z * CHUNK_SIZE as i32 + z as i32;

                // top face ok
                if is_void((x as i32, y as i32 + 1, z as i32), (wx, wy + 1, wz), world) {
                    #[rustfmt::skip]
                    let vertices = [
                        (x    , y + 1, z    , voxel_id, 0), //v0
                        (x    , y + 1, z + 1, voxel_id, 0), //v3
                        (x + 1, y + 1, z + 1, voxel_id, 0), //v2
                        (x    , y + 1, z    , voxel_id, 0), //v0
                        (x + 1, y + 1, z + 1, voxel_id, 0), //v2
                        (x + 1, y + 1, z    , voxel_id, 0), //v1
                    ];
                    add_data(&mut vertex_data, &vertices); // v0, v3, v2, v0, v2, v1
                }

                // bottom face ok
                if is_void((x as i32, y as i32 - 1, z as i32), (wx, wy - 1, wz), world) {
                    #[rustfmt::skip]
                    let vertices = [
                        (x    , y, z    , voxel_id, 1), //v0
                        (x + 1, y, z + 1, voxel_id, 1), //v2
                        (x    , y, z + 1, voxel_id, 1), //v3
                        (x    , y, z    , voxel_id, 1), //v0
                        (x + 1, y, z    , voxel_id, 1), //v1
                        (x + 1, y, z + 1, voxel_id, 1), //v2
                    ];
                    add_data(&mut vertex_data, &vertices); //v0, v2, v3, v0, v1, v2
                }

                // right face ok
                if is_void((x as i32 + 1, y as i32, z as i32), (wx + 1, wy, wz), world) {
                    #[rustfmt::skip]
                    let vertices = [
                        (x + 1, y    , z    , voxel_id, 2), //v0
                        (x + 1, y + 1, z    , voxel_id, 2), //v1
                        (x + 1, y + 1, z + 1, voxel_id, 2), //v2
                        (x + 1, y    , z    , voxel_id, 2), //v0
                        (x + 1, y + 1, z + 1, voxel_id, 2), //v2
                        (x + 1, y    , z + 1, voxel_id, 2), //v3
                    ];
                    add_data(&mut vertex_data, &vertices); // v0, v1, v2, v0, v2, v3
                }

                // left face ok
                if is_void((x as i32 - 1, y as i32, z as i32), (wx - 1, wy, wz), world) {
                    #[rustfmt::skip]
                    let vertices = [
                        (x, y    , z    , voxel_id, 3), //v0
                        (x, y + 1, z + 1, voxel_id, 3), //v2
                        (x, y + 1, z    , voxel_id, 3), //v1
                        (x, y    , z    , voxel_id, 3), //v0
                        (x, y    , z + 1, voxel_id, 3), //v3
                        (x, y + 1, z + 1, voxel_id, 3), //v2
                    ];
                    add_data(&mut vertex_data, &vertices); // v0, v2, v1, v0, v3, v2
                }

                // back face ok
                if is_void((x as i32, y as i32, z as i32 - 1), (wx, wy, wz - 1), world) {
                    #[rustfmt::skip]
                    let vertices = [
                        (x    , y    , z, voxel_id, 4), //v0
                        (x    , y + 1, z, voxel_id, 4), //v1
                        (x + 1, y + 1, z, voxel_id, 4), //v2
                        (x    , y    , z, voxel_id, 4), //v0
                        (x + 1, y + 1, z, voxel_id, 4), //v2
                        (x + 1, y    , z, voxel_id, 4), //v3
                    ];
                    add_data(&mut vertex_data, &vertices); // v0, v1, v2, v0, v2, v3
                }

                // front face
                if is_void((x as i32, y as i32, z as i32 + 1), (wx, wy, wz + 1), world) {
                    #[rustfmt::skip]
                    let vertices = [
                        (x    , y    , z + 1, voxel_id, 5), //v0
                        (x + 1, y + 1, z + 1, voxel_id, 5), //v2
                        (x    , y + 1, z + 1, voxel_id, 5), //v1
                        (x    , y    , z + 1, voxel_id, 5), //v0
                        (x + 1, y    , z + 1, voxel_id, 5), //v3
                        (x + 1, y + 1, z + 1, voxel_id, 5), //v2
                    ];
                    add_data(&mut vertex_data, &vertices); // v0, v2, v1, v0, v3, v2
                }
            }
        }
    }

    vertex_data
}
