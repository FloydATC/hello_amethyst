// https://crsaracco.github.io/amethyst-3d-tutorial/empty-game.html

use amethyst::utils::application_root_dir;
use amethyst::SimpleState;
use amethyst::GameDataBuilder;
use amethyst::Application;
use amethyst::renderer::plugins::RenderPbr3D;
use amethyst::renderer::plugins::RenderToWindow;
use amethyst::renderer::types::DefaultBackend;
use amethyst::renderer::RenderingBundle;
use amethyst::window::DisplayConfig;


struct GameState;
impl SimpleState for GameState {}


fn main() -> amethyst::Result<()> {

    // Set up the Amethyst logger
    // https://docs-src.amethyst.rs/stable/amethyst/struct.Logger.html
    amethyst::start_logger(Default::default());

    // Set up the assets directory (PathBuf)
    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");

    // Set up the display configuration
    let display_config = DisplayConfig {
        title: "Hello Amethyst".to_string(),
        dimensions: Some((640, 400)),
        ..Default::default()
    };

    // Set up an empty (for now) GameDataBuilder
    let game_data = GameDataBuilder::default()
    .with_bundle(
        RenderingBundle::<DefaultBackend>::new() // "vulkan" selected in Cargo.toml
            .with_plugin(
                RenderToWindow::from_config(display_config)
                    .with_clear([0.529, 0.808, 0.98, 1.0]), // [R,G,B,A]
            )
            .with_plugin(RenderPbr3D::default()), // Physically Based Rendering
    )?;

    // Run the game!
    let mut game = Application::new(assets_dir, GameState, game_data)?;
    game.run();

    Ok(())
}
