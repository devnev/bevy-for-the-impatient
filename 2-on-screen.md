# On-Screen

> Show a coloured and lit cube from a raised viewpoint

Add a behaviour for spawning a camera:

```rust
fn spawn_camera(mut commands: Commands) {
    let camera_pivot = commands
        .spawn((TransformBundle {
            local: Transform::default().with_rotation(
                Quat::from_axis_angle(Vec3::Y, (20 as f32).to_radians())
                    * Quat::from_axis_angle(Vec3::X, (30 as f32).to_radians()),
            ),
            ..default()
        },))
        .id();
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .set_parent(camera_pivot);
}
```

Add a behaviour for spawning a lit box:

```rust
fn spawn_lit_box(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cube::default())),
        material: materials.add(Color::BLUE.with_l(0.2).into()),
        ..default()
    });
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
```

Change main to create a Bevy app that will call the above functions during startup:

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_lit_box)
        .run();
}
```

And now run the app:

```sh
cargo run
```

## Navigation

[Code](./2-on-screen) / [Prev](1-install.md) / [Next](3-spin.md)

## Detailed Walk-Through


---

This work is licensed under [CC BY 4.0](http://creativecommons.org/licenses/by/4.0)
