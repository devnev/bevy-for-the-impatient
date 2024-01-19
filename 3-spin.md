# Spin

> Spin the viewpoint around the cube using the mouse or touchpad

Add a type to tag your camera pivot entity:

```rust
#[derive(Component, Default, Debug)]
struct CameraPivot {}
```

Change the camera pivot spawn to include the tag in `fn spawn_camera`:

```rust
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
```

Add a behaviour for rotating with the mouse:

```rust
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
```

Add a behaviour for rotating with the touchpad or horizontal scroll:

```rust
fn rotate_camera_with_wheel(
    mut ev_mouse_wheel: EventReader<MouseWheel>,
    mut cameras: Query<(&CameraPivot, &mut Transform)>,
) {
    for ev in ev_mouse_wheel.read() {
        let (_, mut transform) = cameras.single_mut();
        transform.rotate_axis(Vec3::Y, -ev.x / 200.0)
    }
}
```

Add both behaviours to the application's update loop:

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_lit_box)
        .add_systems(
            Update,
            rotate_camera_with_mouse.run_if(input_pressed(MouseButton::Left)),
        )
        .add_systems(Update, rotate_camera_with_wheel)
        .run();
}
```

## Navigation

[Code](./3-spin) / [Prev](2-on-screen.md) / [Next](4-multiply.md)

## Detailed Walk-Through


---

This work is licensed under [CC BY 4.0](http://creativecommons.org/licenses/by/4.0)
