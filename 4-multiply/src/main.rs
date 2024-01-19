use bevy::{
    input::{
        common_conditions::input_pressed,
        mouse::{MouseMotion, MouseWheel},
    },
    prelude::*,
    render::mesh::shape::Cube,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<BoxAssets>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, init_box_assets)
        .add_systems(Startup, spawn_lit_box.after(init_box_assets))
        .add_systems(
            Update,
            rotate_camera_with_mouse.run_if(input_pressed(MouseButton::Left)),
        )
        .add_systems(Update, rotate_camera_with_wheel)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let camera_pivot = commands
        .spawn((
            CameraPivot::default(),
            TransformBundle {
                local: Transform::default().with_rotation(
                    Quat::from_axis_angle(Vec3::Y, (20 as f32).to_radians())
                        * Quat::from_axis_angle(Vec3::X, (30 as f32).to_radians()),
                ),
                ..default()
            },
        ))
        .id();
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .set_parent(camera_pivot);
}

fn spawn_lit_box(mut commands: Commands, box_assets: Res<BoxAssets>) {
    for i in 0..5 {
        spawn_box_at(&mut commands, &box_assets, Vec3::X * (i as f32) * 1.5);
    }
    commands.spawn(DirectionalLightBundle {
        transform: Transform::default().looking_to(
            Vec3 {
                x: 3.0,
                y: -1.0,
                z: 1.0,
            },
            Vec3::Y,
        ),
        ..default()
    });
}

#[derive(Component, Default, Debug)]
struct CameraPivot {}

fn rotate_camera_with_mouse(
    mut ev_mouse_motion: EventReader<MouseMotion>,
    mut cameras: Query<(&CameraPivot, &mut Transform)>,
) {
    if let Some(motion) = ev_mouse_motion.read().last() {
        for (_pivot, mut transform) in cameras.iter_mut() {
            transform.rotate_axis(Vec3::Y, -motion.delta.x / 50.0)
        }
    };
}

fn rotate_camera_with_wheel(
    mut ev_mouse_wheel: EventReader<MouseWheel>,
    mut cameras: Query<(&CameraPivot, &mut Transform)>,
) {
    for ev in ev_mouse_wheel.read() {
        let (_, mut transform) = cameras.single_mut();
        transform.rotate_axis(Vec3::Y, -ev.x / 200.0)
    }
}

#[derive(Resource, Clone, Default)]
struct BoxAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

fn init_box_assets(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut box_assets: ResMut<BoxAssets>,
) {
    box_assets.mesh = meshes.add(Mesh::from(Cube::default()));
    box_assets.material = materials.add(Color::BLUE.with_l(0.2).into());
}

fn spawn_box_at(commands: &mut Commands, box_assets: &Res<BoxAssets>, position: Vec3) {
    commands.spawn(PbrBundle {
        mesh: box_assets.mesh.clone(),
        material: box_assets.material.clone(),
        transform: Transform::from_translation(position + Vec3::Y * 0.5),
        ..default()
    });
}
