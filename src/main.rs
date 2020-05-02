mod states;
mod systems;

use amethyst::{
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    prelude::*,
    renderer::{
        plugins::{RenderDebugLines, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};
use states::Main;
use systems::{NextGenerationSystem, RenderTilesSystem};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let root = application_root_dir()?;
    let window_config_path = root.join("config/display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(window_config_path)?
                        .with_clear([0.13, 0.13, 0.13, 1.0]),
                )
                .with_plugin(RenderDebugLines::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with(RenderTilesSystem, "render_tiles_system", &[])
        .with(
            NextGenerationSystem,
            "next_gen_system",
            &["render_tiles_system"],
        );

    let mut game = Application::build("assets", Main)?
        .with_frame_limit(FrameRateLimitStrategy::Sleep, 140)
        .build(game_data)?;
    game.run();
    Ok(())
}
