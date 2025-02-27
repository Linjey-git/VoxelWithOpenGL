use crate::settings::{CHUNK_AREA, CHUNK_SIZE, CHUNK_VOL, MIN_Y};
use crate::world::World;
use crate::world_objects::Chunk;
use glam::IVec3;

// fn is_void(
//     voxel_pos: (i32, i32, i32),
//     chunk_voxels: &[u8],
//     world_voxel_pos: (i32, i32, i32),
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

fn is_void(
    voxel_pos: (i32, i32, i32),
    chunk_voxels: &[u8],
    world_voxel_pos: (i32, i32, i32),
    world: &World,
) -> bool {
    let (x, y, z) = voxel_pos;
    let (wx, wy, wz) = world_voxel_pos;

    // Якщо воксель у межах поточного чанка
    if x >= 0
        && x < CHUNK_SIZE as i32
        && y >= 0
        && y < CHUNK_SIZE as i32
        && z >= 0
        && z < CHUNK_SIZE as i32
    {
        let index = (x + CHUNK_SIZE as i32 * z + CHUNK_AREA as i32 * y) as usize;
        return chunk_voxels[index] == 0;
    }

    // Якщо воксель поза межами чанка, перевіряємо сусідній чанк
    let cx = wx.div_euclid(CHUNK_SIZE as i32);
    let cy = wy.div_euclid(CHUNK_SIZE as i32);
    let cz = wz.div_euclid(CHUNK_SIZE as i32);

    if let Some(chunk) = world.chunks.get(&IVec3::new(cx, cy, cz)) {
        let local_x = wx.rem_euclid(CHUNK_SIZE as i32);
        let local_y = wy.rem_euclid(CHUNK_SIZE as i32);
        let local_z = wz.rem_euclid(CHUNK_SIZE as i32);
        let index = (local_x + CHUNK_SIZE as i32 * local_z + CHUNK_AREA as i32 * local_y) as usize;
        return chunk.voxels[index] == 0;
    }

    // Якщо чанк не згенерований і нижче MIN_Y, вважаємо суцільним
    // if wy < MIN_Y * CHUNK_SIZE as i32{
    //     return false;
    // }

    // Інакше вважаємо порожнім
    true
}

fn get_chunk(world_voxel_pos: (i32, i32, i32), world: &World) -> Option<&Chunk> {
    let (wx, wy, wz) = world_voxel_pos;
    let cx = (wx as f32 / CHUNK_SIZE as f32).floor() as i32;
    let cy = (wy as f32 / CHUNK_SIZE as f32).floor() as i32;
    let cz = (wz as f32 / CHUNK_SIZE as f32).floor() as i32;
    world.chunks.get(&IVec3::new(cx, cy, cz))
}

// fn is_void(
//     local_voxel_pos: (i32, i32, i32),
//     world_voxel_pos: (i32, i32, i32),
//     world: &World,
// ) -> bool {
//     let (_, wy, _) = world_voxel_pos;
//
//     // Якщо нижче MIN_Y * CHUNK_SIZE, вважаємо заповненим
//     if wy < MIN_Y * CHUNK_SIZE as i32 {
//         return false;
//     }
//
//     let chunk = get_chunk(world_voxel_pos, world);
//     if chunk.is_none() {
//         return true; // Незгенерований чанк = порожній
//     }
//     let chunk_voxels = &chunk.unwrap().voxels;
//
//     let (x, y, z) = local_voxel_pos;
//     let voxel_index = (x.rem_euclid(CHUNK_SIZE as i32)
//         + z.rem_euclid(CHUNK_SIZE as i32) * CHUNK_SIZE as i32
//         + y.rem_euclid(CHUNK_SIZE as i32) * CHUNK_AREA as i32) as usize;
//
//     if chunk_voxels[voxel_index] != 0 {
//         return false;
//     }
//     true
// }

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
    let mut triangle_count = 0;

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

                let wx = chunk_pos.x * CHUNK_SIZE as i32 + x as i32;
                let wy = chunk_pos.y * CHUNK_SIZE as i32 + y as i32;
                let wz = chunk_pos.z * CHUNK_SIZE as i32 + z as i32;

                // top face ok
                if is_void(
                    (x as i32, y as i32 + 1, z as i32),
                    chunk_voxels,
                    (wx, wy + 1, wz),
                    world,
                ) {
                    #[rustfmt::skip]
                    let vertices = [
                        (x    , y + 1, z    , voxel_id, 0), //v0
                        (x    , y + 1, z + 1, voxel_id, 0), //v3
                        (x + 1, y + 1, z + 1, voxel_id, 0), //v2
                        (x    , y + 1, z    , voxel_id, 0), //v0
                        (x + 1, y + 1, z + 1, voxel_id, 0), //v2
                        (x + 1, y + 1, z    , voxel_id, 0), //v1
                    ];
                    add_data(&mut vertex_data, &vertices);
                    triangle_count += 2;
                }

                // bottom face ok
                if is_void(
                    (x as i32, y as i32 - 1, z as i32),
                    chunk_voxels,
                    (wx, wy - 1, wz),
                    world,
                ) {
                    #[rustfmt::skip]
                    let vertices = [
                        (x    , y, z    , voxel_id, 1), //v0
                        (x + 1, y, z + 1, voxel_id, 1), //v2
                        (x    , y, z + 1, voxel_id, 1), //v3
                        (x    , y, z    , voxel_id, 1), //v0
                        (x + 1, y, z    , voxel_id, 1), //v1
                        (x + 1, y, z + 1, voxel_id, 1), //v2
                    ];
                    add_data(&mut vertex_data, &vertices);
                    triangle_count += 2;
                }

                // right face ok
                if is_void(
                    (x as i32 + 1, y as i32, z as i32),
                    chunk_voxels,
                    (wx + 1, wy, wz),
                    world,
                ) {
                    #[rustfmt::skip]
                    let vertices = [
                        (x + 1, y    , z    , voxel_id, 2), //v0
                        (x + 1, y + 1, z    , voxel_id, 2), //v1
                        (x + 1, y + 1, z + 1, voxel_id, 2), //v2
                        (x + 1, y    , z    , voxel_id, 2), //v0
                        (x + 1, y + 1, z + 1, voxel_id, 2), //v2
                        (x + 1, y    , z + 1, voxel_id, 2), //v3
                    ];
                    add_data(&mut vertex_data, &vertices);
                    triangle_count += 2;
                }

                // left face ok
                if is_void(
                    (x as i32 - 1, y as i32, z as i32),
                    chunk_voxels,
                    (wx - 1, wy, wz),
                    world,
                ) {
                    #[rustfmt::skip]
                    let vertices = [
                        (x, y    , z    , voxel_id, 3), //v0
                        (x, y + 1, z + 1, voxel_id, 3), //v2
                        (x, y + 1, z    , voxel_id, 3), //v1
                        (x, y    , z    , voxel_id, 3), //v0
                        (x, y    , z + 1, voxel_id, 3), //v3
                        (x, y + 1, z + 1, voxel_id, 3), //v2
                    ];
                    add_data(&mut vertex_data, &vertices);
                    triangle_count += 2;
                }

                // back face ok
                if is_void(
                    (x as i32, y as i32, z as i32 - 1),
                    chunk_voxels,
                    (wx, wy, wz - 1),
                    world,
                ) {
                    #[rustfmt::skip]
                    let vertices = [
                        (x    , y    , z, voxel_id, 4), //v0
                        (x    , y + 1, z, voxel_id, 4), //v1
                        (x + 1, y + 1, z, voxel_id, 4), //v2
                        (x    , y    , z, voxel_id, 4), //v0
                        (x + 1, y + 1, z, voxel_id, 4), //v2
                        (x + 1, y    , z, voxel_id, 4), //v3
                    ];
                    add_data(&mut vertex_data, &vertices);
                    triangle_count += 2;
                }

                // front face ok
                if is_void(
                    (x as i32, y as i32, z as i32 + 1),
                    chunk_voxels,
                    (wx, wy, wz + 1),
                    world,
                ) {
                    #[rustfmt::skip]
                    let vertices = [
                        (x    , y    , z + 1, voxel_id, 5), //v0
                        (x + 1, y + 1, z + 1, voxel_id, 5), //v2
                        (x    , y + 1, z + 1, voxel_id, 5), //v1
                        (x    , y    , z + 1, voxel_id, 5), //v0
                        (x + 1, y    , z + 1, voxel_id, 5), //v3
                        (x + 1, y + 1, z + 1, voxel_id, 5), //v2
                    ];
                    add_data(&mut vertex_data, &vertices);
                    triangle_count += 2;
                }
            }
        }
    }

    println!("Chunk at {:?}: {} triangles", chunk_pos, triangle_count);
    vertex_data
}
