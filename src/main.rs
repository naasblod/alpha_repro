use bevy::{
    core_pipeline::prepass::DepthPrepass,
    math,
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, MyExtension>,
        >::default())
        .add_systems(Startup, (setup, setup_ui))
        .add_systems(Update, (rotate_things, toggle_depth_prepass))
        .run();
}
fn setup_ui(mut commands: Commands) {
    commands.spawn(TextBundle::from_section(
        "Press space to remove / insert DepthPrepass to camera.",
        TextStyle {
            font_size: 72.0,
            color: Color::WHITE,
            ..default()
        },
    ));
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, MyExtension>>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(math::primitives::Circle::new(3.0)),
        transform: Transform::from_xyz(0.0, 0.0, -1.0),
        material: standard_materials.add(StandardMaterial {
            base_color: Color::BLUE,
            ..Default::default()
        }),
        ..default()
    });
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(math::primitives::Circle::new(1.0)),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: Color::RED,
                alpha_mode: AlphaMode::Mask(0.5),
                ..Default::default()
            },
            extension: MyExtension {},
        }),
        ..default()
    });

    // light
    commands.spawn((
        DirectionalLightBundle {
            transform: Transform::from_xyz(1.0, 1.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Rotate,
    ));

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        DepthPrepass,
    ));
}
fn toggle_depth_prepass(
    mut commands: Commands,
    query: Query<(Entity, Option<&DepthPrepass>)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        for (entity, opt_prepass) in &query {
            if opt_prepass.is_some() {
                commands.entity(entity).remove::<DepthPrepass>();
            } else {
                commands.entity(entity).insert(DepthPrepass);
            }
        }
    }
}

#[derive(Component)]
struct Rotate;

fn rotate_things(mut q: Query<&mut Transform, With<Rotate>>, time: Res<Time>) {
    for mut t in &mut q {
        t.rotate_y(time.delta_seconds());
    }
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
struct MyExtension {}

impl MaterialExtension for MyExtension {
    fn fragment_shader() -> ShaderRef {
        "shaders/extended_material.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "shaders/extended_material.wgsl".into()
    }
}
