use amethyst::{ecs::prelude::*, renderer::debug_drawing::DebugLines};

use crate::states::{TileSize, TilesMap, TILE_COLOR};

pub struct NextGenerationSystem;

impl<'a> System<'a> for NextGenerationSystem {
    type SystemData = WriteExpect<'a, TilesMap>;

    fn run(&mut self, mut tiles_map: Self::SystemData) {
        let mut buffer = *tiles_map;
        let size = (tiles_map.len(), tiles_map[0].len());
        for i in 0..size.0 {
            for j in 0..size.1 {
                let neighbours = count_neighbours(&tiles_map, size, (i, j));
                if neighbours < 2 || neighbours > 3 {
                    buffer[i][j] = 0;
                } else if neighbours == 3 {
                    buffer[i][j] = 1;
                }
            }
        }
        *tiles_map = buffer;
    }
}

fn count_neighbours(tiles_map: &TilesMap, size: (usize, usize), pos: (usize, usize)) -> i32 {
    let mut count = 0;
    for i in 0..3 {
        for j in 0..3 {
            let x = ((pos.0 + i + size.0) as i32 - 1) as usize % size.0;
            let y = ((pos.1 + j + size.1) as i32 - 1) as usize % size.1;
            count += tiles_map[x][y];
        }
    }
    count - tiles_map[pos.0][pos.1]
}

pub struct RenderTilesSystem;

impl<'a> System<'a> for RenderTilesSystem {
    type SystemData = (
        ReadExpect<'a, TilesMap>,
        ReadExpect<'a, TileSize>,
        Write<'a, DebugLines>,
    );
    fn run(&mut self, (tiles_map, tile_size, mut debug_lines): Self::SystemData) {
        for i in 0..tiles_map.len() {
            for j in 0..tiles_map[0].len() {
                if tiles_map[i][j] == 1 {
                    let x = tile_size.width * i as f32;
                    let y = tile_size.height * j as f32;
                    debug_lines.draw_rectangle(
                        [x, y].into(),
                        [x + tile_size.width, y + tile_size.height].into(),
                        0.0,
                        TILE_COLOR.into(),
                    );
                }
            }
        }
    }
}
