use amethyst::{
    core::{transform::Transform, Named},
    ecs::prelude::*,
    prelude::*,
    renderer::camera::Camera,
    window::ScreenDimensions,
};
use rand::prelude::*;

pub const TILE_COLOR: (f32, f32, f32, f32) = (0.2, 0.8, 1.0, 1.0);
const TILES_IN_COL: usize = 120;
const TILES_IN_ROW: usize = 120;
const INIT_POPULATION: usize = 10_000;

pub type TilesMap = [[i32; TILES_IN_COL]; TILES_IN_ROW];

pub struct TileSize {
    pub width: f32,
    pub height: f32,
}

pub struct Main;

impl SimpleState for Main {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let (width, height) = {
            let dim = data.world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };
        init_camera(data.world, width, height);

        let tile_size = TileSize {
            width: width as f32 / TILES_IN_COL as f32,
            height: height as f32 / TILES_IN_ROW as f32,
        };
        data.world.insert(tile_size);

        let mut tiles_map: TilesMap = [[0; TILES_IN_COL]; TILES_IN_ROW];
        init_life(&mut tiles_map, INIT_POPULATION);
        data.world.insert(tiles_map);
    }
}

fn init_life(tiles_map: &mut TilesMap, init_population: usize) {
    let mut rng = thread_rng();
    let row = tiles_map.len();
    let col = tiles_map[0].len();
    for _ in 0..init_population {
        tiles_map[rng.gen_range(0, col) as usize][rng.gen_range(0, row) as usize] = 1;
    }
}

fn init_camera(world: &mut World, w: f32, h: f32) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(w * 0.5, h * 0.5, 1.0);
    world
        .create_entity()
        .with(Camera::standard_2d(w, h))
        .with(transform)
        .named("camera")
        .build();
}
