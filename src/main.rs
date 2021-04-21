// https://crsaracco.github.io/amethyst-3d-tutorial/empty-game.html

use amethyst::utils::application_root_dir;
use amethyst::SimpleState;
use amethyst::GameDataBuilder;
use amethyst::Application;


struct GameState;
impl SimpleState for GameState {}


fn main() -> amethyst::Result<()> {

    // Set up the Amethyst logger
    // https://docs-src.amethyst.rs/stable/amethyst/struct.Logger.html
    amethyst::start_logger(Default::default());

    // Set up the assets directory (PathBuf)
    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");

    // Set up an empty (for now) GameDataBuilder
    let game_data = GameDataBuilder::default();

    // Run the game!
    let mut game = Application::new(assets_dir, GameState, game_data)?;
    game.run();

    Ok(())
}
