// https://crsaracco.github.io/amethyst-3d-tutorial/empty-game.html

use amethyst::utils::application_root_dir;
use amethyst::SimpleState;
use amethyst::GameDataBuilder;
use amethyst::Application;
use amethyst::renderer::plugins::RenderPbr3D;
use amethyst::renderer::plugins::RenderToWindow;
use amethyst::renderer::types::DefaultBackend;
use amethyst::renderer::RenderingBundle;
use amethyst::renderer::shape::Shape;
use amethyst::renderer::Camera;
use amethyst::renderer::Material;
use amethyst::renderer::MaterialDefaults;
use amethyst::renderer::rendy::mesh::{Normal, Position, Tangent, TexCoord};
use amethyst::window::DisplayConfig;
use amethyst::StateData;
use amethyst::GameData;
use amethyst::prelude::World;
use amethyst::prelude::WorldExt;
use amethyst::prelude::Builder;
use amethyst::assets::AssetLoaderSystemData;
use amethyst::core::{Transform, TransformBundle};


struct GameState;
impl SimpleState for GameState {
    fn on_start(&mut self, state_data: StateData<'_, GameData<'_, '_>>) {
        initialize_camera(state_data.world);
        initialize_sphere(state_data.world);
    }
}


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
    .with_bundle(TransformBundle::new())?
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


fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 10.0);

    world.create_entity()
        .with(Camera::standard_3d(1024.0, 768.0))
        .with(transform)
        .build();
}


fn initialize_sphere(world: &mut World) {
    let mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        loader.load_from_data(
            Shape::Sphere(100, 100)
                .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                .into(),
            (),
        )
    });

    let material_defaults = world.read_resource::<MaterialDefaults>().0.clone();
    let material = world.exec(|loader: AssetLoaderSystemData<'_, Material>| {
        loader.load_from_data(
                Material {
                    ..material_defaults
                },
                (),
            )
        },
    );

    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 0.0);

    world.create_entity()
        .with(mesh)
        .with(material)
        .with(transform)
        .build();
}
