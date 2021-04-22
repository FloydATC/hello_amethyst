// Originally based upon
// https://crsaracco.github.io/amethyst-3d-tutorial/empty-game.html
// Extended and updated to work with Amethyst 0.15.3 by floyd@atc.no

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
use amethyst::renderer::Mesh;
use amethyst::renderer::MaterialDefaults;
use amethyst::renderer::rendy::mesh::{Normal, Position, Tangent, TexCoord};
//use amethyst::renderer::rendy::mesh::Indices;
use amethyst::renderer::light::{Light, PointLight};
use amethyst::renderer::palette::rgb::Rgb;
//use amethyst::renderer::types::MeshData;
use amethyst::window::DisplayConfig;
use amethyst::StateData;
use amethyst::GameData;
use amethyst::prelude::World;
use amethyst::prelude::WorldExt;
use amethyst::prelude::Builder;
use amethyst::assets::AssetLoaderSystemData;
use amethyst::core::{Transform, TransformBundle};
//use amethyst::core::SystemDesc;
use amethyst::core::timing::Time;
use amethyst::derive::SystemDesc;
use amethyst::ecs::Component;
//use amethyst::ecs::NullStorage;
use amethyst::ecs::DenseVecStorage;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use nalgebra::geometry::{Translation3, UnitQuaternion};
use nalgebra::base::Vector3;


struct GameState;
impl SimpleState for GameState {
    fn on_start(&mut self, state_data: StateData<'_, GameData<'_, '_>>) {
        initialize_camera(state_data.world);
        initialize_cube(state_data.world);
        //initialize_mesh(state_data.world);
        //initialize_sphere(state_data.world);
        initialize_light(state_data.world);
    }
}


// The following component is used to give entities 
// linear and/or axial momentum to be applied continuously
pub struct Momentum {
    linear:     Vector3<f32>,
    axial:      UnitQuaternion<f32>,
}

impl Component for Momentum {
  type Storage = DenseVecStorage<Self>;
}

impl Momentum {
    fn new(linear: Vector3<f32>, axial: UnitQuaternion<f32>) -> Self {
        Momentum {
            linear,
            axial,
        }
    }
    fn translation(&self) -> &Vector3<f32> {
        &self.linear
    }
    fn rotation(&self) -> &UnitQuaternion<f32> {
        &self.axial
    }
}

impl Default for Momentum {
    fn default() -> Self {
        Momentum {
            linear:     Vector3::new(0., 0., 0.),
            axial:      UnitQuaternion::identity(),
        }
    }
}


// The following system continuously applies rotation/translation
// to entities that have a Transform + Momentum
#[derive(SystemDesc)]
pub struct MomentumSystem;

impl<'s> System<'s> for MomentumSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Momentum>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, momentums, time): Self::SystemData) {
        let t = time.delta_seconds();
        for (transform, momentum) in (&mut transforms, &momentums).join() {
            let rotation = momentum.rotation();
            // Apply rotation, if any
            match (rotation.axis(), rotation.angle()) {
                (Some(axis), angle) => {
                    transform.append_rotation(axis, angle * t);
                }
                _ => {}
            }
            // Apply translation
            let translation = momentum.translation();
            transform.append_translation(translation * t);
        }
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

    // Set up a minimal GameDataBuilder
    let game_data = GameDataBuilder::default()
    .with_bundle(TransformBundle::new())?
    .with_bundle(
        RenderingBundle::<DefaultBackend>::new() // Backend selected in Cargo.toml
            .with_plugin(
                RenderToWindow::from_config(display_config)
                    .with_clear([0.029, 0.008, 0.08, 1.0]), // [R,G,B,A]
            )
            .with_plugin(RenderPbr3D::default()), // Physically Based Rendering
    )?
    .with(MomentumSystem, "momentum_system", &[]);

    // Run the game!
    let mut game = Application::new(assets_dir, GameState, game_data)?;
    game.run();

    Ok(())
}


fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 5.0);

    world.create_entity()
        .with(Camera::standard_3d(640.0, 400.0))
        .with(transform)
        .build();
}


fn initialize_cube(world: &mut World) {
    let mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        loader.load_from_data(
            Shape::Cube
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

    let position = Translation3::new(0.0, 0.0, 0.0);
    let rotation = UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0);
    let scale = Vector3::new(1.0, 1.0, 1.0);
    
    let transform = Transform::new(position, rotation, scale);

    let linear = Vector3::new(0.0, 0.0, 0.0);
    let axial = UnitQuaternion::from_euler_angles(0.43, 0.27, 0.13);
    let momentum = Momentum::new(linear, axial);

    world.create_entity()
        .with(mesh)
        .with(material)
        .with(transform)
        .with(momentum)
        .build();
}


// User defined mesh (unused)
// Maybe experiment with WaveFront .OBJ files next?
fn _initialize_mesh(world: &mut World) {

    let geometry = vec![
        Position([0., 0., 0.]),
        Position([1., 0., 0.]),
        Position([0., 1., 0.]),
    ];
    let normals = vec![
        Normal([0., 0., 1.]),
        Normal([0., 0., 1.]),
        Normal([0., 0., 1.]),
    ];
    let tangents = vec![
        Tangent([0., 1., 0., 0.]),
        Tangent([0., 1., 0., 0.]),
        Tangent([0., 1., 0., 0.]),
    ];
    let texcoords = vec![
        TexCoord([0., 0.]), 
        TexCoord([1., 1.]), 
        TexCoord([0., 1.]),
    ];

    let mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        loader.load_from_data(
            amethyst::renderer::types::MeshData(
                amethyst::renderer::rendy::mesh::MeshBuilder::new()
                    .with_vertices(geometry)
                    .with_vertices(normals)
                    .with_vertices(tangents)
                    .with_vertices(texcoords)
            ),
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


// Sphere from the original tutorial. (unused)
// Not particularly useful for experimenting with rotation.
fn _initialize_sphere(world: &mut World) {
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


fn initialize_light(world: &mut World) {
    let light: Light = PointLight {
        intensity: 10.0,
        color: Rgb::new(1.0, 1.0, 1.0),
        ..PointLight::default()
    }.into();

    let mut transform = Transform::default();
    transform.set_translation_xyz(5.0, 5.0, 20.0);

    world
        .create_entity()
        .with(light)
        .with(transform)
        .build();
}
